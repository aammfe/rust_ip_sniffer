use std::env::{self};
use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::{process, io};
use std::sync::mpsc :: {Sender, channel};
use std::thread;

const MAX : u16 = 65535;

struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ip_addr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ip_addr,
                threads: 4,
            });
        } else if f.contains("-h") || f.contains("-help") && args.len() == 2 {
            println!(
                "Usage: -j to select how many thread you want to
                \r\n -h or -help to show this help message"
            );
            return Err("Help");
        } else if f.contains("-h") || f.contains("-help") {
            return Err("too many arguments");
        } else if f.contains("-j") {
            let ip_addr = match IpAddr::from_str(&args[3]) {
                Ok(s) => s,
                Err(_) => return Err("not a valid IPAddres; must be IPv4 or Ipv6"),
            };

            let threads = match args[2].parse::<u16>() {
                Ok(s) => s,
                Err(_) => return Err("failed to parse thread numbers"),
            };

            return Ok(Arguments {
                flag: String::from(""),
                ip_addr,
                threads,
            });
        }
        return Err("invalid syntax");
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else( 
        |err| {
            if err.contains("help") {
                process::exit(0);
            }else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        });

    let num_threads= arguments.threads;
    let (tx,rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ip_addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    print!("");
    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}

fn scan(tx : Sender<u16>, start_port :u16, addr : IpAddr, num_threads: u16){
    let mut port :u16 = start_port + 1;

    loop {
        match TcpStream :: connect ((addr, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if(MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}


