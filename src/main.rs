use rand::Rng;
use std::fs::remove_file;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;
use uuid::Uuid;

const BINARY_PATH: &str = "../silly_rusty_kv/target/release/silly_rusty_kv";
const SAMPLE_SIZE: usize = 4;
const BODY_SIZE: usize = 100;

fn uuid_without_dash() -> String {
    Uuid::new_v4().to_string().replace('-', "")
}

fn random_string() -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..BODY_SIZE)
        .map(|_| rng.gen_range(65..91) as u8 as char)
        .collect();
    return random_string;
}

fn main() {
    remove_file("data.db").unwrap_or_else(|_| println!("No file to delete"));

    let start = Instant::now();
    // Generate pairs of uuids
    let pairs: Vec<(String, String)> = (0..SAMPLE_SIZE)
        .map(|_| (uuid_without_dash(), random_string().to_string()))
        .collect();

    let cmds: String = pairs
        .iter()
        .map(|(k, v)| format!("PUT {} \"{}\" \n", k.to_string().replace('-', ""), v))
        .collect();

    println!("Preparation took {:?}", start.elapsed());

    let write_start = Instant::now();
    let mut child = Command::new(BINARY_PATH)
        .stdin(Stdio::piped())
        // .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    let mut stdin = child.stdin.take().expect("failed to get stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(cmds.as_bytes())
            .expect("failed to write to stdin");

        // Dunno why this seems to stack the time instead of running concurrently
        // I think this might be indicative of the db getting stuck from reading?
        // std::thread::sleep(std::time::Duration::from_secs(10))
    });
    let output = child.wait_with_output().expect("failed to wait on child");
    println!("child finished: {:?}", output);
    println!("Writing took {:?}", write_start.elapsed());

    println!("Reading...");
    let reversed_pairs: Vec<(String, String)> = pairs.into_iter().collect();
    let read_commands: String = reversed_pairs
        .iter()
        .map(|(k, _)| format!("GET {} \n", k))
        .collect();

    let read_start = Instant::now();
    let mut child = Command::new(BINARY_PATH)
        .stdin(Stdio::piped())
        // .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    println!("Child spawned");
    let mut stdin = child.stdin.take().expect("failed to get stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(read_commands.as_bytes())
            .expect("failed to write to stdin");
    });
    let output = child.wait().expect("failed to wait on child");
    println!("child finished: {:?}", output);
    println!("Reading took {:?}", read_start.elapsed());
}
