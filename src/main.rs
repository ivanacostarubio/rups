use std::env;
use std::net::TcpStream;

/*
 * TODO:
 * - query to ports in parallel
 */

struct ConnectionAttempt {
    host: String,
    port: String,
}

impl ConnectionAttempt {
    fn peek(&self) -> Result<TcpStream, std::io::Error> {
        let r = self.uri();
        TcpStream::connect(r)
    }

    fn uri(&self) -> String {
        let h = self.host.to_string();
        let c = ":".to_string();
        let p = self.port.to_string();
        [h, c, p].concat()
    }

    fn print(&self) {
        let result = self.peek();

        if result.is_ok() {
            println!("{0: <10} {1: <10}", "OPEN", self.port)
        }
    }
}

fn print_help() {
    eprintln!("Usage: rusty_scan host");
}

fn scan_ports(h: String, start: u32, end: u32) {
    for port in start..end {
        let host = h.to_string();
        let port = port.to_string();

        let attempt = ConnectionAttempt { host, port };
        attempt.print();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let s: String = args[1].parse().unwrap();

    println!("{0: <10} {1: <10}", "HOST", s);

    match args.len() {
        1 => {
            print_help();
        }
        2 => match args[1].parse() {
            Ok(h) => scan_ports(h, 0, 100),
            _ => eprintln!("Error parsing host"),
        },
        3 => match args[1].parse() {
            Ok(h) => match args[2].parse() {
                Ok(start) => scan_ports(h, start, 100),
                _ => eprintln!("Error parsing start port"),
            },
            _ => eprintln!("Error parsing host"),
        },
        4 => match args[1].parse() {
            Ok(h) => match args[2].parse() {
                Ok(start) => match args[3].parse() {
                    Ok(end) => scan_ports(h, start, end),
                    _ => eprintln!("Error parsing end port"),
                },
                _ => eprintln!("Error parsing start port"),
            },
            _ => eprintln!("Error parsing host"),
        },
        _ => {
            print_help();
        }
    }
}
