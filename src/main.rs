mod ui;

use log::{info, warn, error, debug};

pub static EXIT_SUCCESS: i32 = 0;
pub static EXIT_FAILURE: i32 = 1;

pub fn exit_program(exit_code: i32) -> ! {
    info!("Exiting...");
    info!("Exited with exit code: {}", exit_code);
    std::process::exit(exit_code);
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    info!("Logger initialized!");
    info!("Starting...");

    ui::main();

    exit_program(EXIT_SUCCESS);
}