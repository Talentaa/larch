mod cli;
mod isa;
mod utils;


use clap::Parser;
use cli::{CmdExector, Opts};


fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    opts.cmd.execute()
}