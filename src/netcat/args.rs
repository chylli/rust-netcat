extern crate getopts;

use self::getopts::Options;

pub enum Args {
    SendMode(String, u16),
    ListenMode(u16),
    Usage(String, Options),
}

pub fn usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    println!("{}",opts.usage(&brief))
}

pub fn parse(args: Vec<String>) -> Result<Args, String> {
    let program = &args[0];
    let mut opts = Options::new();
    opts.optflag("h","help", "print usage info");
    opts.optopt("l", "", "listen mode on specified TCP-port", "PORT");

    let matches = try!(opts.parse(&args[1..]).map_err(|err|err.to_string()));
    if matches.opt_present("h"){
        return Ok(Args::Usage(program.clone(), opts));
    }

    if matches.opt_present("l") {
        let listen_port: u16 =
            try!(
                try!(
                    matches.opt_str("l").ok_or("You should specify TCP-port for listen mode.") // ok_or ?
                ).parse().map_err(|err| "The port should be an integer.")
            );
        Ok(Args::ListenMode(listen_port))
    }
    else {
        if matches.free.len() == 0 && !matches.opt_present("h"){
            Ok(Args::Usage(program.clone(), opts))
        }
        else if matches.free.len() < 2 {
            Err("You should specify host name and TCP-port for send mode.".to_owned())
        }
        else {
            let hostname = &matches.free[0];
            let port: u16 = try!(matches.free[1].parse().map_err(|err| "Port should be an ingeter"));
            Ok(Args::SendMode(hostname.clone(), port))
        }
    }
}

#[test]
fn test_parse(){
    let args : Vec<String> = vec!("neccat","-h", "1234").iter().map(|s| s.to_string()).collect();
    let p : String = args[0].clone();
    match parse(args) {
        Ok(Args::Usage(program, opts)) => assert!(program == p, "usage"),
        _ => assert!(false),
    }

    let args : Vec<String> = vec!("neccat","1234").iter().map(|s| s.to_string()).collect();
    let p : String = args[0].clone();
    match parse(args) {
        Err(_) => assert!(true), //TODO use Error
        _ => assert!(false),
    }

    let args : Vec<String> = vec!("neccat","host", "1234").iter().map(|s| s.to_string()).collect();
    let p : String = args[0].clone();
    match parse(args) {
        Ok(Args::SendMode(_,_)) => assert!(true), 
        _ => assert!(false),
    }

    let args : Vec<String> = vec!("neccat","host", "abcd").iter().map(|s| s.to_string()).collect();
    let p : String = args[0].clone();
    match parse(args) {
        Err(_) => assert!(true), 
        _ => assert!(false),
    }

    let args : Vec<String> = vec!("neccat", "-l", "1234").iter().map(|s| s.to_string()).collect();
    let p : String = args[0].clone();
    match parse(args) {
        Ok(Args::ListenMode(p)) => assert_eq!(p, 1234), 
        _ => assert!(false),
    }

    let args : Vec<String> = vec!("neccat", "-l", "abcd").iter().map(|s| s.to_string()).collect();
    let p : String = args[0].clone();
    match parse(args) {
        Err(_) => assert!(true),
        _ => assert!(false),
    }

    let args : Vec<String> = vec!("neccat", "abcd").iter().map(|s| s.to_string()).collect();
    let p : String = args[0].clone();
    match parse(args) {
        Err(_) => assert!(true),
        _ => assert!(false),
    }

}
