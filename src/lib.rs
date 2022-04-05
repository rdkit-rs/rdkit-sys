#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use crate::molecule::Molecule;
use std::fs::read_to_string;

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
    let sd_file = read_to_string(sd_file).expect("Could not load file.");
    let mut mol_list: Vec<Option<Molecule>> = Vec::new();
    let molblock_list: Vec<&str> = sd_file.split("$$$$").collect();
    for s in molblock_list.iter() {
        let s_mod = s.trim();
        if s_mod.len() == 0 {
            mol_list.push(None);
            continue;
        };
        let mol_opt = Molecule::new(s_mod, "");

        // this avoids hard to catch exceptions later on...
        //match mol_opt.as_mut() {
        //    Some(mut mol_opt) => {mol_opt.cleanup(""); Some(mol_opt)},
        //    None => None,
        //};
        mol_list.push(mol_opt);
    }
    mol_list
}
