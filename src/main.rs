extern crate minecraft_server_info;

use std::net::TcpStream;
use std::time::Duration;

fn main() {
    let host = "silvermoon.online";
    let port = 25565;

    let mut stream = TcpStream::connect((host,port)).unwrap();
    let _ = stream.set_read_timeout(Some(Duration::from_secs(3)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(1)));

    let data = minecraft_server_info::query_server(&mut stream, &String::from(host), port).unwrap();
    println!("{:?}", &data);

    println!("{}", data.description.text);
    println!("");
    if data.players.online > 0{
        println!("There are {} players online: ", data.players.online);
        for player in data.players.sample.unwrap(){
            println!("{}", player.name);
        }
    }
    else{
        println!("No one is online!");
    }
}
