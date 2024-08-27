use std::{fs::File, io::BufReader};

use crate::isa::LAISA;

static FILE_PATH: &str = "data/isa.json";

pub fn get_la_isa() -> anyhow::Result<LAISA> {
    let reader = BufReader::new(File::open(FILE_PATH)?);
    let la_isa: LAISA = serde_json::from_reader(reader)?;

    anyhow::Ok(la_isa)
}
