use super::*;
use std::io::{read_to_string, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::time::Instant;

fn start_server() -> Child {
    Command::new(BINARY_PATH)
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute process")
}

fn send_cmds(cmd: &str) -> String {
    let mut client = TcpStream::connect("127.0.0.1:5476").expect("Could not connect to server");
    client
        .write_all(cmd.as_bytes())
        .expect("Failed to write to server");
    client
        .shutdown(std::net::Shutdown::Write)
        .expect("Failed to shutdown write");
    read_to_string(client).expect("Failed to read from server")
}

pub fn test_server_basic() {
    println!("Testing server basic");
    reset_db();

    let _s = start_server();
    let pairs = generate_pairs();
    let cmds = pairs_to_put_cmds(&pairs);

    let write_instant = Instant::now();
    let output = send_cmds(&cmds);
    println!("Server output: {}", output);
    println!("Writing took {:?}", write_instant.elapsed());
}
