use password_gen::cli;
use password_gen::generator;

fn main() {
    let config = cli::build_cli();
    let password = generator::generate(&config);
    println!("Generated password: {}", password);
}
