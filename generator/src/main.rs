use clap::Parser;

/// Generate a solution template for a given year and day
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The year to generate a solution for
    #[arg(short, long)]
    year: u16,

    /// The day to generate a solution for
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    println!("Year {}, Day {}", args.year, args.day);
}
