mod netcat;
use self::netcat::args::*;
use std::env;

//let args: Vec<String> = env::args().collect();
//let args = env::args().collect::<Vec<String>>();

fn main() {
    match parse(env::args().collect()){
        Ok(args) => match args {
            Args::SendMode(s,p) => {
                println!("send mode on {} {}",s, p);
            },
            Args::ListenMode(p) => {
                println!("listen mode on {}", p);
            },
            Args::Usage(program,opt) => {
                usage(&program, opt);
            }
        },
        Err(err) => println!("{}",err)
    };

}
