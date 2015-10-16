use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;

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
            println!("{}", String::from_utf8_lossy( &buf[0..r] ));

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

        let ping = Regex::new(r"^PING :(\w+)\r\n").unwrap();
        let privmsg = Regex::new(r"^:(.+)!(.+)@.+ PRIVMSG (.+) :(\w+)\r\n").unwrap();

        let mut buf = [0; 1024];
        let r = self.stream.read(&mut buf).unwrap();

        let msg = String::from_utf8_lossy( &buf[0..r] );

        if let Some( group ) = ping.captures(&*msg)  {
            let server = group.at(1).unwrap();
            return Commands::PING(server.to_string())
        }
        else if let Some( group ) = privmsg.captures(&*msg)  {
            let nick = group.at(1).unwrap();
            let user = group.at(2).unwrap();
            let target = group.at(3).unwrap();
            let message = group.at(4).unwrap();

            println!("{}", user);

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
    // let msg = format!(":monglth!seanlth@spoon.netsoc.tcd.ie PRIVMSG #test :hey\n\r");
    // let privmsg = Regex::new(r"^:(.+)!(.+)@.+ PRIVMSG (.+) :(\w+)\n\r").unwrap();
    //
    // if let Some( group ) = privmsg.captures(&*msg)  {
    //     println!("ad");
    //     if let Some( user ) = group.at(2) {
    //         println!("{}", user);
    //     }
    // }

    let mut irc = IRC::new("irc.netsoc.tcd.ie", "134.226.83.61", "brewbot").unwrap();
    irc.join("tcd2016");
    loop {
        let c = irc.read();
        match c {
            Commands::PING(server) => irc.pong(&*server),
            Commands::PONG(_) => {},
            Commands::PRIVMSG(n, u, t, m) => {
                if u == "seanlth" { irc.mesg(&*t, "^ cool guy") }
                else if u == "mereckaj" { irc.mesg(&*t, "^ cunt") }
            },
            Commands::ERR => {}
        }
        //thread::sleep_ms(50000);
    }
}
