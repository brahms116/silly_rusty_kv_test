use super::*;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

fn run_db_command(input: String) -> String {
    let mut child = Command::new(BINARY_PATH)
        .arg("--stdin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    let mut stdin = child.stdin.take().expect("failed to get stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(input.as_bytes())
            .expect("failed to write to stdin");
    });
    let output = child.wait_with_output().expect("failed to wait on child");
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn test_stdin() {
    println!("Testing stdin...");

    reset_db();
    let start = Instant::now();

    let pairs = generate_pairs();

    let cmds = pairs_to_put_cmds(&pairs);

    println!("Writing...");

    let write_start = Instant::now();
    let _output = run_db_command(cmds);
    println!("Writing took {:?}", write_start.elapsed());

    let read_commands = pairs_to_get_cmds(&pairs);
    println!("Reading...");
    let read_start = Instant::now();
    let output = run_db_command(read_commands);
    println!("Reading took {:?}", read_start.elapsed());
    check_pairs(&pairs, &output);

    let pairs_updated: Vec<(String, String)> = pairs
        .into_iter()
        .map(|(k, _)| (k, random_string()))
        .collect();

    let update_commands = pairs_to_put_cmds(&pairs_updated);
    println!("Updating...");
    let update_start = Instant::now();
    let _output = run_db_command(update_commands);
    println!("Updating took {:?}", update_start.elapsed());

    let get_commands = pairs_to_get_cmds(&pairs_updated);
    println!("Reading updated...");
    let get_start = Instant::now();
    let output = run_db_command(get_commands.clone());
    println!("Reading updated took {:?}", get_start.elapsed());
    check_pairs(&pairs_updated, &output);

    let delete_commands = pairs_to_delete_cmds(&pairs_updated);
    println!("Deleting...");
    let delete_start = Instant::now();
    let _output = run_db_command(delete_commands);
    println!("Deleting took {:?}", delete_start.elapsed());

    let get_commands = pairs_to_get_cmds(&pairs_updated);
    println!("Reading after delete...");
    let get_start = Instant::now();
    let output = run_db_command(get_commands);
    println!("Reading after delete took {:?}", get_start.elapsed());
    check_output_for_deletes(&output);
}
