use std::env;
mod tcp;

fn main() {
    let args : Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() <= 1 {
        println!("ERROR: Enter either 'server'
                 or 'client' to start specific worker");
        std::process::exit(1);
    }

    if args.len() >= 3{
        println!("ERROR: TOO Many Arguments !");
        std::process::exit(1);
    }

    if "server".eq(&args[1]){
        println!("Server Listening on port 8080");
        tcp::tcp::spawn_tcp_server();
    }
    else if "client".eq(&args[1]){
        println!("Starting TCP Client");
        tcp::tcp_client::client();
    }

    std::process::exit(0);
}
