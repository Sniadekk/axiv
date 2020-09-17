use clap::Clap;

use axiv::{run, Settings};

fn main() {
    let settings: Settings = Settings::parse();

    match run(&settings) {
        Ok(()) => println!(
            "The data was successfully parsed and saved at {}",
            &settings.output
        ),
        Err(e) => {
            println!("Error occurred: {}", e);
        }
    }
}
