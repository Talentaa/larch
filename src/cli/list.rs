use std::{fs::File, io::BufReader};

use clap::Parser;

use crate::utils::get_la_isa;

use super::CmdExector;

#[derive(Debug, Parser)]
pub struct ListOpts {}

impl CmdExector for ListOpts {
    fn execute(self) -> anyhow::Result<()> {
        let la_isa = get_la_isa()?;
        for inst in la_isa.insts.iter() {
            println!("{}", inst.name);
        }
        anyhow::Ok(())
    }
}
