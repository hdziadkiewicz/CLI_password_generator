mod cli;
mod generator;
mod utils;

fn main() {
    let config = cli::build_cli();
    let password = generator::generate(&config);
    println!("Generated password: {}", password);
}
