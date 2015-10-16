use std::net::TcpStream;
use std::io::prelude::*;

struct IRC {
    server: String,
    address: String,
    nick: String,
    stream: TcpStream
}

impl IRC {
    fn new(server: &str, address: &str, nick: &str) -> Option<IRC> {

        if let Ok(mut s) = TcpStream::connect( &*format!("{}:{}", address, ":6667") ) {
            let _ = s.write( format!("NICK {}\n\n", nick).as_bytes() );
            let _ = s.write( format!("USER {} 0 * :{}\n\n", nick, nick).as_bytes() );

            let mut buf = [0; 1024];
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
        let _ = self.stream.write( format!("NICK {}\n\n", nick).as_bytes() );
    }

    fn mesg(&mut self, target: &str, message: &str) {
        let _ = self.stream.write( format!(":source PRIVMSG {} :{}\n\n", target, message).as_bytes() );
    }

    fn join(&mut self, channel: &str) {
        let _ = self.stream.write( format!(":source JOIN :#{}\r\n", channel).as_bytes() );
    }

    fn quit(&mut self, reason: &str) {
        let _ = self.stream.write( format!(":source QUIT :{}\r\n", reason).as_bytes() );
    }

}

fn main() {

    let irc = IRC::new("irc.netsoc.tcd.ie", "134.226.83.61", "DckLvr500");


    // let mut stream = TcpStream::connect("127.0.0.1:6667").unwrap();
    //
    // let _ = stream.write( format!("NICK bot\r\n").as_bytes() );
    // let _ = stream.write( format!("USER bot 8 * :bot\r\n").as_bytes() );
    // let _ = stream.flush();
    //
    // let _ = stream.write( format!(":source JOIN :#channel\r\n").as_bytes() );

    // let mut g = 1;
    //     while g > 0 {
    //     let mut buf = [0; 1024];
    //
    //     let r = stream.read(&mut buf).unwrap();
    //
    //     println!("{}", String::from_utf8_lossy( &buf[0..r] ));
    //
    //     // match buf {
    //     //   Some(_) => {
    //     //     if r.clone().unwrap().contains("004")  {
    //     //       isConnected = true;
    //     //     } else if r.clone().unwrap().contains("PING") {
    //     //       println!("PONG.");
    //     //       stream.write( "PONG\r\n".to_bytes() );
    //     //       stream.flush();
    //     //     }
    //     //
    //     //   }
    //     //   None => {
    //     //     println!("End of Stream!");
    //     //     g = 0;
    //     //   }
    //     // }
    // }
    println!("Connection timed out!");
}
