mod info;
mod list;

use enum_dispatch::enum_dispatch;
use info::InfoOpts;
use clap::Parser;
use list::ListOpts;

#[enum_dispatch]
pub trait CmdExector {
  fn execute(self) -> anyhow::Result<()>;
}

#[derive(Debug, Parser)]
#[command(name = "larch", version, about)]
pub struct Opts {
  #[command(subcommand)]
  pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum SubCommand {
  #[command(name = "info", about="Display information about an instruction")]
  Info(InfoOpts),
  #[command(name="list", about="Lists all instructions")]
  List(ListOpts)
}