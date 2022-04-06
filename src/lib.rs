#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use crate::molecule::Molecule;
use std::fs::read_to_string;
use std::io::BufRead;

pub mod bindings;
pub mod molecule;

/* this code taken from https://github.com/chrissly31415/rdkitcffi */

/// read a classical .smi file
pub fn read_smifile(smi_file: &str) -> Vec<Option<Molecule>> {
    let smi_file = read_to_string(smi_file).expect("Could not load file.");
    let mut mol_list: Vec<Option<Molecule>> = Vec::new();
    let smiles_list: Vec<&str> = smi_file.split("\n").collect();
    for s in smiles_list.iter() {
        let s_mod = s.trim();
        if s_mod.len() == 0 {
            mol_list.push(None);
            continue;
        };
        let mol_opt = Molecule::new(s_mod, "");
        mol_list.push(mol_opt);
    }
    mol_list
}

/// read a classical .sdf file
pub fn read_sdfile(sd_file: &str) -> Vec<Option<Molecule>> {
    // let sd_file = read_to_string(sd_file).expect("Could not load file.");

    let sd_file = std::fs::File::open(sd_file).expect("Could not load file");
    let mut sd_file = std::io::BufReader::new(sd_file);

    let mut molblocks = Vec::new();
    let mut molblock_buf = Vec::new();
    while let Ok(_) = sd_file.read_until(b'$', &mut molblock_buf) {
        sd_file.consume(3);

        let molblock = std::str::from_utf8(&molblock_buf).unwrap();
        let mol = Molecule::new(&molblock, "");
        molblocks.push(mol);
    }

    molblocks
}
