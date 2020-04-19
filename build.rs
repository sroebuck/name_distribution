extern crate csv;

use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("names.rs");
    let mut f = File::create(&dest_path)?;

    write_array(
        &mut f,
        "SURNAME",
        "src/data/most-common-surnames-multi-year-data.csv",
    )?;

    Ok(())
}

fn write_array(file: &mut File, constant_name: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    write!(file, "const {}: ArrayType = &[\n", constant_name)?;
    let sorted_map: BTreeMap<String, usize> = reader
        .records()
        .map(|x| {
            let r = x.unwrap();
            let n = &r[1].to_string();
            let name = n.clone();
            let frequency: usize = r[2].parse().unwrap();
            (name, frequency)
        })
        .collect();

    let mut total_frequency: u32 = 0;
    let mut total_count: u16 = 0;
    for (name, frequency) in sorted_map {
        total_count += 1;
        total_frequency += frequency as u32;
        write!(file, "  ({},\"{}\"),\n", total_frequency, name)?;
    }
    write!(file, "];\n")?;
    write!(
        file,
        "const {}_LEN: usize = {};\n",
        constant_name, total_count
    )?;
    Ok(())
}
