use structopt::StructOpt;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    match args::Args::from_args() {
        args::Args::Encode(args) => commands::encode(args),
        args::Args::Decode(args) => commands::decode(args),
        args::Args::Remove(args) => commands::remove(args),
        args::Args::Print(args) => commands::print(args),
    }
}
