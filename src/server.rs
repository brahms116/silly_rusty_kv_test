use super::*;
use std::io::{read_to_string, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn start_server() -> Child {
    let child = Command::new(BINARY_PATH)
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute process");
    sleep(Duration::from_secs(1));
    child
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

fn close_server(mut server: Child) {
    send_cmds("EXIT\n");
    server.kill().expect("Failed to kill server");
    server.wait().expect("Failed to wait for server");
    sleep(Duration::from_secs(1));
}

struct ServerClient {
    client: TcpStream,
    reader: BufReader<TcpStream>,
}

impl ServerClient {
    fn new() -> Self {
        let client = TcpStream::connect("127.0.0.1:5476").expect("Could not connect to server");
        let reader = BufReader::new(client.try_clone().expect("Failed to clone client"));
        Self { client, reader }
    }

    fn send_cmd(&mut self, cmd: &str) -> String {
        let cmd = format!("{}\n", cmd);
        self.client
            .write_all(cmd.as_bytes())
            .expect("Failed to write to server");
        let mut output = String::new();
        self.reader
            .read_line(&mut output)
            .expect("Failed to read from server");
        output.replace("\n", "")
    }

    fn close(self) {
        self.client
            .shutdown(std::net::Shutdown::Write)
            .expect("Failed to shutdown write");
        self.client
            .shutdown(std::net::Shutdown::Read)
            .expect("Failed to shutdown read");
    }
}

pub fn test_server_transaction() {
    println!("Testing server transaction...");
    reset_db();
    let s = start_server();
    let mut client1 = ServerClient::new();
    let mut client2 = ServerClient::new();

    client1.send_cmd("PUT key1 \"value1\"");
    let output = client2.send_cmd("GET key1");
    assert_eq!(output, "value1");

    client1.send_cmd("BEGIN");
    client1.send_cmd("PUT key2 \"value2\"");
    let output = client2.send_cmd("GET key2");
    assert_eq!(output, KEY_NOT_FOUND);

    client1.send_cmd("COMMIT");
    let output = client2.send_cmd("GET key2");
    assert_eq!(output, "value2");
    let output = client1.send_cmd("GET key2");
    assert_eq!(output, "value2");

    client1.send_cmd("BEGIN");
    client1.send_cmd("PUT key3 \"value3\"");
    let output = client2.send_cmd("GET key3");
    assert_eq!(output, KEY_NOT_FOUND);
    assert_ne!(output, "value3");

    client1.send_cmd("ROLLBACK");
    let output = client2.send_cmd("GET key3");
    assert_eq!(output, KEY_NOT_FOUND);
    let output = client1.send_cmd("GET key3");
    assert_eq!(output, KEY_NOT_FOUND);

    client1.send_cmd("PUT deleteme \"deletevalue\"");
    let output = client2.send_cmd("GET deleteme");
    assert_eq!(output, "deletevalue");

    client1.send_cmd("BEGIN");
    client1.send_cmd("DELETE deleteme");
    let output = client2.send_cmd("GET deleteme");
    assert_eq!(output, "deletevalue");

    client1.send_cmd("ROLLBACK");
    let output = client1.send_cmd("GET deleteme");
    assert_eq!(output, "deletevalue");

    client1.send_cmd("BEGIN");
    client1.send_cmd("DELETE deleteme");
    client1.send_cmd("COMMIT");
    let output = client2.send_cmd("GET deleteme");
    assert_eq!(output, KEY_NOT_FOUND);
    let output = client1.send_cmd("GET deleteme");
    assert_eq!(output, KEY_NOT_FOUND);

    client1.close();
    client2.close();
    close_server(s);
}

pub fn test_server_basic() {
    println!("Testing server basic");
    reset_db();
    let s = start_server();
    let pairs = generate_pairs();
    let cmds = pairs_to_put_cmds(&pairs);
    let write_instant = Instant::now();
    let _output = send_cmds(&cmds);
    println!("Writing took {:?}", write_instant.elapsed());
    close_server(s);

    let s = start_server();
    let read_commands = pairs_to_get_cmds(&pairs);
    println!("Reading...");
    let read_start = Instant::now();
    let output = send_cmds(&read_commands);
    println!("Reading took {:?}", read_start.elapsed());
    check_pairs(&pairs, &output);
    close_server(s);
}
