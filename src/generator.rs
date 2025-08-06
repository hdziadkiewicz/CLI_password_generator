use crate::cli::Cli;
use rand::{thread_rng, Rng};

pub fn generate(config: &Cli) -> String {
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");

    if config.uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if config.digits {
        charset.push_str("0123456789");
    }
    if config.symbols {
        charset.push_str("!@#$%^&*()-_=+[]{}<>?");
    }

    let mut rng = thread_rng();
    (0..config.length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}
