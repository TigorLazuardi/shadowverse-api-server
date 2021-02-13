mod cmd;
mod services;
mod shared;

use dotenv;

/// runs the program
pub fn start() {
    dotenv::dotenv().ok();
    cmd::execute();
}
