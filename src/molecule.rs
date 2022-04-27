use cxx::let_cxx_string;

use crate::cxx::bridge::rdmol::ffi as rdmol_ffi;

pub struct Molecule {
    ptr: cxx::SharedPtr<rdmol_ffi::ROMol>,
}

impl Molecule {
    pub fn from_smile(smile: &str) -> Option<Self> {
        let_cxx_string!(smile_cxx_string = smile);
        let ptr = unsafe { rdmol_ffi::mol_from_smiles(&smile_cxx_string) };
        Some(Self { ptr })
    }
}
