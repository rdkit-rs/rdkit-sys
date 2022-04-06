#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use crate::molecule::Molecule;
use flate2::bufread::GzDecoder;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

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

pub fn read_sdfile_gz(sd_file_gz_path: &str) -> Vec<Option<Molecule>> {
    let sd_file = std::fs::File::open(sd_file_gz_path).expect("Could not load file");
    // let mut sd_file_buf_reader = std::io::Gz::new(sd_file);
    let sd_file_decoder = flate2::bufread::GzDecoder::new(std::io::BufReader::new(sd_file));
    let mut sd_file_decoder_buf_read = std::io::BufReader::new(sd_file_decoder);

    let mut molblocks = Vec::new();
    let mut molblock_buf = Vec::new();
    while let Ok(_) = sd_file_decoder_buf_read.read_until(b'$', &mut molblock_buf) {
        sd_file_decoder_buf_read.consume(3);

        let molblock = std::str::from_utf8(&molblock_buf).unwrap();
        let mol = Molecule::new(&molblock, "");
        molblocks.push(mol);
    }

    molblocks
}

pub struct MolBlockIter<R: BufRead> {
    buf: R,
}

impl<R: BufRead> MolBlockIter<R> {
    pub fn new(buf: R) -> Self {
        MolBlockIter { buf }
    }
}

pub type GzBufReader = BufReader<flate2::bufread::GzDecoder<BufReader<File>>>;

impl MolBlockIter<GzBufReader> {
    pub fn from_gz_file(p: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path = p.as_ref().to_owned();

        let file = std::fs::File::open(path).unwrap();
        let buf_reader = std::io::BufReader::new(file);

        let gz_decoder = GzDecoder::new(buf_reader);
        let gz_buf_reader = std::io::BufReader::new(gz_decoder);

        Ok(Self::new(gz_buf_reader))
    }
}

impl<R: BufRead> Iterator for MolBlockIter<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::with_capacity(104);

        let buf = loop {
            buf.clear();
            let read = self.buf.read_until(b'$', &mut buf).unwrap();
            if read == 0 {
                return None;
            } else if read == 1 {
                continue;
            } else {
                break buf;
            }
        };

        let block = std::str::from_utf8(&buf).unwrap();
        let block = block.trim();

        return Some(block.to_owned());
    }
}

/// read a classical .sdf file or sdf.gz
pub fn read_sdfile(sd_file_path: &str) -> Vec<Option<Molecule>> {
    let sd_file = std::fs::File::open(sd_file_path).expect("Could not load file");
    let mut sd_file_buf_reader = std::io::BufReader::new(sd_file);

    let mut molblocks = Vec::new();
    let mut molblock_buf = Vec::new();
    while let Ok(_) = sd_file_buf_reader.read_until(b'$', &mut molblock_buf) {
        sd_file_buf_reader.consume(3);

        let molblock = std::str::from_utf8(&molblock_buf).unwrap();
        let mol = Molecule::new(&molblock, "");
        molblocks.push(mol);
    }

    molblocks
}
