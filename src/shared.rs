use rand::Rng;
use std::fs::remove_file;
use uuid::Uuid;

pub const BINARY_PATH: &str = "../silly_rusty_kv/target/release/silly_rusty_kv";
pub const SAMPLE_SIZE: usize = 10_000;
pub const BODY_SIZE: usize = 100;

pub fn reset_db() {
    remove_file("hash_dir.db").unwrap_or_else(|_| println!("No file to delete"));
    remove_file("hash_data.db").unwrap_or_else(|_| println!("No file to delete"));
}

pub fn uuid_without_dash() -> String {
    Uuid::new_v4().to_string().replace('-', "")
}

pub fn random_string() -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..BODY_SIZE)
        .map(|_| rng.gen_range(65..91) as u8 as char)
        .collect();
    return random_string;
}

pub fn pairs_to_put_cmds(pairs: &Vec<(String, String)>) -> String {
    pairs
        .iter()
        .map(|(k, v)| format!("PUT {} \"{}\" \n", k.to_string(), v))
        .collect()
}

pub fn pairs_to_get_cmds(pairs: &Vec<(String, String)>) -> String {
    pairs.iter().map(|(k, _)| format!("GET {} \n", k)).collect()
}

pub fn pairs_to_delete_cmds(pairs: &Vec<(String, String)>) -> String {
    pairs
        .iter()
        .map(|(k, _)| format!("DELETE {} \n", k))
        .collect()
}

pub fn check_pairs(pairs: &Vec<(String, String)>, output: &str) {
    output.lines().enumerate().for_each(|(i, line)| {
        if pairs[i].1 != line {
            println!(
                "Mismatch at index {} with key {} expected {} , but got {}",
                i, pairs[i].0, pairs[i].1, line
            );
            panic!("Mismatch");
        }
    });
}

pub fn check_output_for_deletes(output: &str) {
    output.lines().for_each(|line| {
        if line != "Key not found" {
            panic!("Delete failed");
        }
    });
}
