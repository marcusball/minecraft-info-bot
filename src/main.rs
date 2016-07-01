extern crate minecraft_server_info;
extern crate slack;

use std::net::TcpStream;
use std::time::Duration;
use std::fmt::Display;


struct MinerBotHandler{
    host: String,
    port: u16
}

impl MinerBotHandler{
    fn new(host: String, port: u16) -> MinerBotHandler{
        MinerBotHandler{
            host: host,
            port: port,
        }
    }

    fn get_stream(&self) -> TcpStream{
        let stream = TcpStream::connect((self.host.as_str(), self.port)).unwrap();
        let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(1)));
        return stream;
    }


    fn get_server_status(&mut self) -> String{
        let server_data = minecraft_server_info::query_server(&mut self.get_stream(), &self.host, self.port);
        if let Ok(data) = server_data{
            return format!("Server is up; description: \"{}\"", data.description.text);
        }
        else{
            return "Failed to receive server info, maybe it's down?".to_string();
        }
    }

    fn get_online_message(&mut self) -> String{
        let server_data = minecraft_server_info::query_server(&mut self.get_stream(), &self.host, self.port);
        if let Ok(data) = server_data{
            if data.players.online > 0{
                let players = join_names(&mut data.players.sample.unwrap().iter().map(|player_data| &player_data.name));
                return format!("{} {} online",players, match data.players.online { 1 => "is", _ => "are"});
            }
            else{
                return "No one is online".to_string();
            }
        }
        else{
            return "Failed to receive server info, maybe it's down?".to_string();
        }
    }
}

#[allow(unused_variables)]
impl slack::EventHandler for MinerBotHandler {
    fn on_event(&mut self, cli: &mut slack::RtmClient, event: Result<&slack::Event, slack::Error>, raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);

        if let Ok(&slack::Event::Message(ref message)) = event{
            match message{
                &slack::Message::Standard{ ref text, ref channel, .. } => {
                    let mes = text.clone().unwrap();
                    let channel = channel.clone().unwrap();

                    if mes.as_str().starts_with("!server"){
                        println!("Received server command");
                        let status_message = self.get_server_status();
                        let _ = cli.send_message(channel.as_str(), status_message.as_str());
                    }
                    else if mes.as_str().starts_with("!online"){
                        println!("Received online status query.");
                        let online_message =  self.get_online_message();
                        let _ = cli.send_message(channel.as_str(), online_message.as_str());
                    }
                    else{
                        println!("Received message \"{}\"", mes);
                    }
                },
                _ => { println!("Received some other message"); }
            }
        }
    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        println!("on_connect");
        // Do a few things using the api:
        // send a message over the real time api websocket
        //let _ = cli.send_message("#automation", "Hello world! (rtm)");
        // post a message as a user to the web api
        //let _ = cli.post_message("#automation", "hello world! (postMessage)", None);
        // set a channel topic via the web api
        // let _ = cli.set_topic("#general", "bots rule!");
    }
}

fn join_names<'a, T: Display, I: ExactSizeIterator<Item=T>>(words: &mut I) -> String{
    let len = words.len();
    words.enumerate().map(|(i, word)| match (i, len) {
        (0, _) => word.to_string(),
        (1, 2) => format!(" and {}", word),
        (i, n) if i == n - 1 => format!(", and {}", word),
        _ => format!(", {}", word),
    }).collect()
}


fn main() {
    let host = "silvermoon.online";
    let port = 25565;

    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run -- <api-key>"),
        x => {
            args[x - 1].clone()
        }
    };

    let mut handler = MinerBotHandler::new(host.to_string(), port);
    let mut cli = slack::RtmClient::new(&api_key);
    let r = cli.login_and_run::<MinerBotHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}

#[test]
fn show_info(){
    let host = "silvermoon.online";
    let port = 25565;

    let mut stream = TcpStream::connect((host,port)).unwrap();
    let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(1)));

    let data = minecraft_server_info::query_server(&mut stream, &String::from(host), port).unwrap();
    println!("{:?}", &data);
}

#[test]
fn show_status(){
    let host = "silvermoon.online";
    let port = 25565;

    let mut handler = MinerBotHandler::new(host.into(), port);

    println!("{}", handler.get_server_status());
}

#[test]
fn show_online(){
    let host = "silvermoon.online";
    let port = 25565;

    let mut handler = MinerBotHandler::new(host.into(), port);

    println!("{}", handler.get_online_message());
}

#[test]
fn test_fold(){
    let items = vec!["rick", "morty", "beth", "jerry", "summer"];

    assert_eq!(join_names(&mut items.iter().take(5)), "rick, morty, beth, jerry, and summer");
    assert_eq!(join_names(&mut items.iter().take(4)), "rick, morty, beth, and jerry");
    assert_eq!(join_names(&mut items.iter().take(3)), "rick, morty, and beth");
    assert_eq!(join_names(&mut items.iter().take(2)), "rick and morty");
    assert_eq!(join_names(&mut items.iter().take(1)), "rick");
}
