extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

#[macro_use]
mod macros;

mod chainhooks;
mod deployments;
mod frontend;
mod generate;
mod integrate;
mod lsp;
mod runner;
mod types;
mod utils;

use frontend::cli;

pub fn main() {
    cli::main();
}
