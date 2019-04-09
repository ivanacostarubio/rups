use std::env;
use std::net::TcpStream;
extern crate chrono;
extern crate threadpool;
use chrono::prelude::*;
use std::time::Duration;

/*
 * TODO:
 * - Display no open ports
 * - SYN Scans
 * - UDP
 */

use threadpool::ThreadPool;

struct ConnectionAttempt {
    host: String,
    port: String,
}

impl ConnectionAttempt {
    fn peek(&self) -> Result<TcpStream, std::io::Error> {
        let r = self.uri();
        let stream = TcpStream::connect(r)?;
        stream
            .set_read_timeout(Some(Duration::new(0, 100)))
            .expect("can't set the connection timeout");

        stream.set_write_timeout(Some(Duration::new(0, 100)))?;

        stream.set_ttl(10)?;

        Ok(stream)
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
            println!("{0: <10} {1: <10}", self.port, "open")
        }
    }
}

fn print_help() {
    eprintln!("Usage: rups host start_port end_port");
}

fn scan_ports(h: String, start: u32, end: u32) {
    let n_workers = 5000;
    let pool = ThreadPool::new(n_workers);

    for p in start..end {
        let host = h.clone();

        pool.execute(move || {
            //println!("DEBUG: {:?}:{:?}", host, p);
            //println!(".");
            let port = p.to_string();
            let attempt = ConnectionAttempt { host, port };

            attempt.print();
            //
        });
    }

    pool.join();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            print_help();
        }
        2 => {
            print_help();
        }
        3 => {
            print_help();
        }

        4 => match args[1].parse() {
            Ok(h) => match args[2].parse() {
                Ok(start) => match args[3].parse() {
                    Ok(end) => {
                        println!("Rups scan report for {:}", h);
                        let now = Local::now();
                        println!(
                            "Starting Rups 0.0.1 at {:}",
                            now.format("%Y-%m-%d %H:%M:%S").to_string()
                        );

                        println!("{0: <10} {1: <10}", "PORT", "STATE");
                        scan_ports(h, start, end);

                        let duration = Local::now().signed_duration_since(now).to_std().unwrap();

                        println!(
                            "Rups done: 1 IP address (1 host up) scanned in {:?} seconds",
                            duration
                        );
                    }
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
