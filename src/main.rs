extern crate rand;

use std::net::TcpStream;
use std::io::prelude::*;
use rand::Rng;


extern crate regex;
use regex::Regex;

#[allow(dead_code)]
enum Commands {
    PING(String), // server
    PONG(String), // server
    PRIVMSG(String, String, String, String), // nick, user, target, message
    ERR
}

#[allow(dead_code)]
struct IRC {
    server: String,
    address: String,
    nick: String,
    stream: TcpStream
}

#[allow(dead_code)]
impl IRC {
    fn new(server: &str, address: &str, nick: &str) -> Option<IRC> {

        if let Ok(mut s) = TcpStream::connect( "134.226.83.61:6667" ) {
            println!("connected");
            let _ = s.write( format!("NICK {}\r\n", nick).as_bytes() );
            let _ = s.write( format!("USER {} 0 * :{}\r\n", nick, nick).as_bytes() );

            let _ = s.flush();

            let mut buf = [0; 4096];
            let r = s.read(&mut buf).unwrap();
            //println!("{}", String::from_utf8_lossy( &buf[0..r] ));

            return Some(IRC {
                server: server.to_string(),
                address: address.to_string(),
                nick: nick.to_string(),
                stream: s
            })
        }
        else {
            return None
        }
    }

    fn nick(&mut self, nick: &str) {
        self.nick = nick.to_string();
        let _ = self.stream.write( format!("NICK {}\r\n", nick).as_bytes() );
    }

    fn mesg(&mut self, target: &str, message: &str) {
        let _ = self.stream.write( format!(":source PRIVMSG {} :{}\r\n", target, message).as_bytes() );
    }

    fn read(&mut self) -> Commands {

        let ping = Regex::new(r"^PING :(.+)\r\n").unwrap();
        let privmsg = Regex::new(r"^:(.+)!(.+)@.+ PRIVMSG (.+) :(.+)\r\n").unwrap();

        let mut buf = [0; 1024];
        let r = self.stream.read(&mut buf).unwrap();

        let msg = String::from_utf8_lossy( &buf[0..r] );

        println!("{}", msg);
        println!("{}", ping.is_match(&*msg));


        if let Some( group ) = ping.captures(&*msg)  {
            let server = group.at(1).unwrap();
            return Commands::PING(server.to_string())
        }
        else if let Some( group ) = privmsg.captures(&*msg)  {
            let nick = group.at(1).unwrap();
            let user = group.at(2).unwrap();
            let target = group.at(3).unwrap();
            let message = group.at(4).unwrap();

            //println!("{}", user);

            return Commands::PRIVMSG(nick.to_string(), user.to_string(), target.to_string(), message.to_string());
        }

        return Commands::ERR
    }

    fn pong(&mut self, server: &str) {
        println!("PONG");
        let _ = self.stream.write( format!("PING {}\r\n", server).as_bytes() );
    }

    fn join(&mut self, channel: &str) {
        let _ = self.stream.write( format!(":source JOIN :#{}\r\n", channel).as_bytes() );
    }

    fn quit(&mut self, reason: &str) {
        let _ = self.stream.write( format!(":source QUIT :{}\r\n", reason).as_bytes() );
    }

}

fn main() {

    let chan = std::env::args().nth(1).unwrap();
    let seanlth = std::env::args().nth(2).unwrap();
    let mereckaj = std::env::args().nth(3).unwrap();
    let duggles = std::env::args().nth(4).unwrap();
    let socbot = std::env::args().nth(5).unwrap();
    let sc = std::env::args().nth(6).unwrap();

    let mut message_string = String::new();

    let mut irc = IRC::new("irc.netsoc.tcd.ie", "134.226.83.61", "brewbot").unwrap();
    irc.join(&*chan);
    loop {
        let c = irc.read();

        let r = rand::random::<u8>();

        match c {
            Commands::PING(server) => irc.pong(&*server),
            Commands::PONG(_) => {},
            Commands::PRIVMSG(n, u, t, m) => {
                if u == "seanlth" && r > 1 { irc.mesg(&*t, &*format!("^ {}", seanlth)) }
                else if u == "mereckaj" && r > 250 { irc.mesg(&*t, &*format!("^ {}", mereckaj)) }
                else if u == "duggles" && r > 250 { irc.mesg(&*t, &*format!("^ {}", duggles)) }
                else if u == "socbot" && r > 100 { irc.mesg(&*t, &*format!("^ {}", socbot)) }
                else if u == "sc" && r > 250 { irc.mesg(&*t, &*format!("^ {}", sc)) }


                let cmd = Regex::new(r"^~(.+)").unwrap();
                if let Some(group) = cmd.captures(&*m) {
                    println!("here");
                    println!("{}, {}", t, chan);
                    if t == chan {
                        let msg = group.at(1).unwrap();
                        println!("{}", msg);
                        message_string = message_string + msg;
                        irc.mesg(&*format!("#{}", chan), &*message_string);
                    }
                }
            },
            Commands::ERR => {}
        }
    }
}
