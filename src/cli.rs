use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Długość hasła
    #[arg(short, long, default_value_t = 12)]
    pub length: usize,

    /// Dodaj cyfry
    #[arg(short, long)]
    pub digits: bool,

    /// Dodaj znaki specjalne
    #[arg(short, long)]
    pub symbols: bool,

    /// Duże litery
    #[arg(short = 'U', long)]
    pub uppercase: bool,

    /// Exclude visually similar characters (0, O, o, 1, l, I)
    #[arg(long)]
    pub no_similar: bool,

    /// Characters to exclude
    #[arg(long)]
    pub exclude: Option<String>,
}

pub fn build_cli() -> Cli {
    Cli::parse()
}
