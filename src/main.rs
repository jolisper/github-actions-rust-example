/// A command line tool to play Marco Polo.
///
/// If the name Marco is given, the program will responds with "Polo".
/// Otherwise, the program will respond with "What is your name?"
use clap::Parser;

#[derive(Parser)]
#[command(
    version = "0.1.0",
    about = "A Marco Polo Game",
    author = "Jorge Luis Perez"
)]
struct Args {
    #[clap(subcommand)]
    name: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "0.1.0", author = "Jorge Luis Perez")]
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.name {
        Some(Commands::Play { name }) => {
            println!("{}", trust::marco_polo(&name));
        }
        None => {
            println!("What is your name?");
        }
    }
}
