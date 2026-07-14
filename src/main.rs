use clap::{Parser, Subcommand};
use quantum_lt::{self, Error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    #[clap(arg_required_else_help = true)]
    Init {
        #[arg(short, long)]
        serial: String,
    },
    List,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    
    match args.subcommand {
        SubCommands::Init { serial } => {
            let ctx = quantum_lt::search(Some(&serial))?
                .into_iter()
                .next();
            if let Some(mut ctx) = ctx {
                quantum_lt::init(&mut ctx)?;
                println!("Success initialization.")
            } else {
                eprintln!("No such device. {}", serial)
            }
        },
        SubCommands::List => {
            let list = quantum_lt::search(None)?;
            for info in &list {
                println!("{}:{}:{} {}", info.bus(), info.port(), info.address(), info.serial());
            }
            if list.len() == 0 {
                eprintln!("Not found.");
            }
        },
    }
    
    Ok(())
}