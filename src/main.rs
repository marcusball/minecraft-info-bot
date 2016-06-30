extern crate minecraft_server_info;
extern crate slack;

use std::net::TcpStream;
use std::time::Duration;



struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &mut slack::RtmClient, event: Result<&slack::Event, slack::Error>, raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);

        if let Ok(&slack::Event::Message(ref message)) = event{
            match message{
                &slack::Message::Standard{ ref text, .. } => {
                    let mes = text.clone().unwrap();
                    println!("Received message \"{}\"", mes);

                    if mes.as_str().starts_with("!test"){
                        println!("Received server command");
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


fn main() {
    let host = "silvermoon.online";
    let port = 25565;

    // let mut stream = TcpStream::connect((host,port)).unwrap();
    // let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
    // let _ = stream.set_write_timeout(Some(Duration::from_secs(1)));
    //
    // let data = minecraft_server_info::query_server(&mut stream, &String::from(host), port).unwrap();
    // println!("{:?}", &data);
    //
    // println!("{}", data.description.text);
    // println!("");
    // if data.players.online > 0{
    //     println!("There are {} players online: ", data.players.online);
    //     for player in data.players.sample.unwrap(){
    //         println!("{}", player.name);
    //     }
    // }
    // else{
    //     println!("No one is online!");
    // }

    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run -- <api-key>"),
        x => {
            args[x - 1].clone()
        }
    };
    
    let mut handler = MyHandler;
    let mut cli = slack::RtmClient::new(&api_key);
    let r = cli.login_and_run::<MyHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
