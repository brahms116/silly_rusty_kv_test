mod shared;
mod stdin;
mod server;

use shared::*;
use stdin::*;
use server::*;

fn main() {
    test_stdin();
    test_server_basic();
    test_server_transaction();
}
