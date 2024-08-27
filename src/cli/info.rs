use clap::Parser;

use crate::utils::get_la_isa;

use super::CmdExector;

#[derive(Debug, Parser)]
pub struct InfoOpts {
    #[arg(default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

impl CmdExector for InfoOpts {
    fn execute(self) -> anyhow::Result<()> {
        let la_isa = get_la_isa()?;
        if let Some(inst) = la_isa
            .insts
            .iter()
            .find(|&inst| inst.name.eq_ignore_ascii_case(&self.input))
        {
            println!("{}", inst.name);
            for f in inst.format.iter() {
                println!("{}", f);
            }
            println!("{}", &inst.detail)
        }

        anyhow::Ok(())
    }
}
