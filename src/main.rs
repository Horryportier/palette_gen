use cli::Cli;

mod cli;
mod error;
mod palette;
mod serializers;
mod args;
mod utils;

fn main() {
   Cli::execute()
}
