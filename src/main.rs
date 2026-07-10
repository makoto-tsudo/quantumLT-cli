use clap::{Parser, Subcommand};
mod quantum_lt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
    #[arg(short, long)]
    serial: String,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Init,
}

fn main() {
    let args = Args::parse();
    
    match quantum_lt::search(args.serial.clone()) {
        Ok(dev) => match args.subcommand {
            SubCommands::Init => {
                match quantum_lt::init(dev) {
                    Ok(()) => {
                        println!("Success initialization.")
                    },
                    Err(err) => {
                        eprint!("Failed to inialize. {}", err)
                    },
                }
            }
        },
        Err(error) => {
            eprintln!("{}", error);
        },
    }
}