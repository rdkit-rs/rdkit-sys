/* this code taken from https://github.com/chrissly31415/rdkitcffi */

//! This is an experimental (and not official) rust wrapper for some functionality of the great open source cheminformatics [RDKit](https://www.rdkit.org/) library.
//!
//! It makes use of its new (and also still experimental) C Foreign Function Interface (CFFI), see also this [blog post](https://greglandrum.github.io/rdkit-blog/technical/2021/05/01/rdkit-cffi-part1.html).
//!
//! Use it at your own risk, its not yet recommended for productive use and only
//! available for linux :-)
//!
//! Please note, that only a limited functionality is being exposed via cffi by
//! RDKit. Structured data is transferred from the backend via the cffi
//! interface as string types. Additional arguments can be passed as json
//! strings. This also means that the structure of objects is different from the
//! C/C++ and python APIs.
//!
//! [github repository](https://github.com/chrissly31415/rdkitcffi).
//!
//! Please have a look at the examples below and the test functions.  
//!
//! # Examples
//!
//! Basic usage:
//!
//!```
//! use rdkit_sys::molecule::Molecule;
//!
//! let smiles = "OCCC#CO";
//! let mol = Molecule::new(smiles, "").unwrap();
//!
//! let natoms = mol.get_numatoms();
//! ```
//!
//! Additional arguments can be passed via json
//!
//!```
//! use rdkit_sys::molecule::Molecule;
//!
//! let json_args = "{\"removeHs\":false,\"canonical\":false}";
//! let mol = Molecule::new("c1cc(O[H])ccc1", json_args).unwrap();
//! ```
//!
//! Working with SD files and filtering invalid molecules (=None):
//!
//! ```
//! use rdkit_sys::{Molecule,read_sdfile};
//!
//! let mut mol_opt_list : Vec<Option<Molecule>>= read_sdfile("data/test.sdf");
//! let mut mol_list: Vec<Molecule> = mol_opt_list.into_iter().filter_map(|m| m).collect();
//! mol_list.iter_mut().for_each(|m| m.remove_all_hs());
//! ```
//!
//! Dealing with invalid molecules (=None)
//!
//! ```
//! use rdkit_sys::Molecule;
//!
//! let result = Molecule::new("OCCO", "");
//! match result {
//!    Some(m) => println!("Result: {:?}", m),
//!    None => println!("Could not get molecule!"),
//! };
//! ```
//!
//!
//! Getting a JSON represenation (via serde_json):
//!
//! ```
//! use rdkit_sys::Molecule;
//!
//! let mol = Molecule::new("OCCO", "").unwrap();
//! println!("json: {:?}", mol.get_json(""));
//! ```
//!
//! Neutralizing a zwitterion
//!
//! ```
//! use rdkit_sys::Molecule;
//!
//! let mut mol = Molecule::new("C(C(=O)[O-])[NH3+]", "").unwrap();
//! mol.neutralize("");
//! println!("{:?}", mol.get_smiles(""));
//! ```
//!
//! Computing RDKit descriptors
//!
//! ```
//! use rdkit_sys::Molecule;
//!
//! let mol = Molecule::new("CCCN", "").unwrap();
//! let desc = mol.get_descriptors_as_dict();
//! let nrot = desc.get("NumRotatableBonds");
//! let logp = desc.get("CrippenClogP");
//! ```
//!
//! Creating a polars dataframe:
//!
//! ```
//! use rdkit_sys::Molecule;
//! use polars::prelude::*;
//! use polars::df;
//!
//! let mut mol_list : Vec<Molecule> = rdkit_sys::read_smifile_unwrap("data/test.smi");
//! let a: Vec<_> = mol_list.iter().map(|m| m.get_smiles("")).collect();
//! let df = df!( "smiles" => a).unwrap();
//! ```

use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fmt::Debug,
    fs::read_to_string,
    mem,
    os::raw::{c_char, c_void},
};

use bindings::{size_t, *};
use libc::free;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;

pub use crate::bindings;
/// Basic class, implementing most functionality as member functions of a
/// molecule object

pub struct Molecule {
    pkl_size: *mut size_t,
    pkl_mol: *mut i8,
}

impl Drop for Molecule {
    fn drop(&mut self) {
        unsafe {
            free(self.pkl_size as *mut c_void);
            free_ptr(self.pkl_mol);
        }
    }
}

impl std::fmt::Debug for Molecule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let smiles = self.get_smiles("");
        write!(f, "{}", smiles)
    }
}

impl Molecule {
    /// Constructor returning an optional molecule
    pub fn new(input: &str, json_info: &str) -> Option<Molecule> {
        let input_cstr = CString::new(input).unwrap();
        let json_info = CString::new(json_info).unwrap();
        let pkl_size: *mut u64 = unsafe { libc::malloc(mem::size_of::<u64>()) as *mut u64 };
        let pkl_mol = unsafe { get_mol(input_cstr.as_ptr(), pkl_size, json_info.as_ptr()) };
        unsafe {
            if pkl_mol.is_null() || *pkl_size == 0 {
                return None;
            }
        }
        Some(Molecule { pkl_size, pkl_mol })
    }

    /// Constructor returning Molecule, panics if None
    pub fn get_mol(input: &str, json_info: &str) -> Molecule {
        let input_cstr = CString::new(input).unwrap();
        let json_info = CString::new(json_info).unwrap();
        let pkl_size: *mut u64 = unsafe { libc::malloc(mem::size_of::<u64>()) as *mut u64 };
        let pkl_mol = unsafe { get_mol(input_cstr.as_ptr(), pkl_size, json_info.as_ptr()) };
        if pkl_mol.is_null() {
            panic!("Could not create molecule!");
        }
        Molecule { pkl_size, pkl_mol }
    }

    /// Gets a commonchem representation as JSON string
    pub fn get_json(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let rdkit_json_cchar = get_json(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let mol_json_str = CStr::from_ptr(rdkit_json_cchar)
                .to_string_lossy()
                .into_owned();
            free_ptr(rdkit_json_cchar);
            mol_json_str
        }
    }

    /// Gets a fully typed common chem like json object
    pub fn get_commonchem(&self) -> JsonBase {
        let json_repr = self.get_json("");
        let res: JsonBase = serde_json::from_str(&json_repr).expect("Wrong JSON format!?");
        res
    }

    /// Gets the underlying Molecule object of the common chem structure
    pub fn get_JsonMolecule(&self) -> JsonMolecule {
        let json_repr = self.get_json("");
        JsonMolecule::JsonMolFromJson(&json_repr)
    }

    pub fn get_atoms(&self) -> Vec<JsonAtom> {
        let json_mol = self.get_JsonMolecule();
        json_mol.atoms
    }

    pub fn get_numatoms(&self) -> usize {
        let json_mol = self.get_JsonMolecule();
        json_mol.atoms.len()
    }

    pub fn get_bonds(&self) -> Vec<JsonBond> {
        let json_mol = self.get_JsonMolecule();
        json_mol.bonds
    }

    pub fn get_numbonds(&self) -> usize {
        let json_mol = self.get_JsonMolecule();
        json_mol.bonds.len()
    }

    /// Get a 2 dimensional vector with atomic coordinates
    pub fn get_coords(&self) -> Vec<Vec<f32>> {
        let json_mol = self.get_JsonMolecule();
        let conf: &JsonConformer = json_mol.conformers.get(0).unwrap().clone();
        conf.coords.to_owned()
    }

    /// Get the SMILES string from a molecule
    pub fn get_smiles(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let a: *mut c_char = get_smiles(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let can_smiles: String = CStr::from_ptr(a).to_string_lossy().into_owned();
            free_ptr(a);
            can_smiles
        }
    }

    /// get SMARTS
    pub fn get_smarts(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let a: *mut c_char = get_smarts(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let smarts: String = CStr::from_ptr(a).to_string_lossy().into_owned();
            free_ptr(a);
            smarts
        }
    }

    /// get CXSMILES
    pub fn get_cxsmiles(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let a: *mut c_char = get_cxsmiles(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let cxsmiles: String = CStr::from_ptr(a).to_string_lossy().into_owned();
            free_ptr(a);
            cxsmiles
        }
    }

    /// find a substructure match via query molecule
    pub fn get_substruct_match(&self, query: &Molecule, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let a: *mut c_char = get_substruct_match(
                self.pkl_mol,
                *self.pkl_size,
                query.pkl_mol,
                *query.pkl_size,
                json_info.as_ptr(),
            );
            let res: String = CStr::from_ptr(a).to_string_lossy().into_owned();
            free_ptr(a);
            res
        }
    }

    /// find several substructure matches via query molecule
    pub fn get_substruct_matches(&self, query: &Molecule, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let a: *mut c_char = get_substruct_matches(
                self.pkl_mol,
                *self.pkl_size,
                query.pkl_mol,
                *query.pkl_size,
                json_info.as_ptr(),
            );
            let res: String = CStr::from_ptr(a).to_string_lossy().into_owned();
            free_ptr(a);
            res
        }
    }

    /// get svg image
    pub fn get_svg(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let a: *mut c_char = get_svg(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let svg: String = CStr::from_ptr(a).to_string_lossy().into_owned();
            free_ptr(a);
            svg
        }
    }

    /// Normalize  molecule
    pub fn normalize(&mut self, json_info: &str) {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            normalize(
                &mut self.pkl_mol as *mut _,
                self.pkl_size,
                json_info.as_ptr(),
            );
        }
    }

    /// Neutralize charged species
    pub fn neutralize(&mut self, json_info: &str) {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            neutralize(
                &mut self.pkl_mol as *mut _,
                self.pkl_size,
                json_info.as_ptr(),
            );
        }
    }

    /// celanup molecule
    pub fn cleanup(&mut self, json_info: &str) {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            cleanup(
                &mut self.pkl_mol as *mut _,
                self.pkl_size,
                json_info.as_ptr(),
            );
        }
    }

    /// reionize molecule
    pub fn reionize(&mut self, json_info: &str) {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            reionize(
                &mut self.pkl_mol as *mut _,
                self.pkl_size,
                json_info.as_ptr(),
            );
        }
    }

    /// get a the canonical tautomer
    pub fn canonical_tautomer(&mut self, json_info: &str) {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            canonical_tautomer(
                &mut self.pkl_mol as *mut _,
                self.pkl_size,
                json_info.as_ptr(),
            );
        }
    }

    /// get the inchi as a String
    pub fn get_inchi(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let inchi_cchar: *mut c_char =
                get_inchi(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let inchi: String = CStr::from_ptr(inchi_cchar).to_string_lossy().into_owned();
            free_ptr(inchi_cchar);
            inchi
        }
    }

    /// get the inchi key
    pub fn get_inchikey(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let a: *mut c_char = get_inchi(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let b: *mut c_char = get_inchikey_for_inchi(a);
            let inchikey: String = CStr::from_ptr(b).to_string_lossy().into_owned();
            free_ptr(a);
            free_ptr(b);
            inchikey
        }
    }

    /// add the hydrogens
    pub fn add_hs(&mut self) {
        unsafe {
            add_hs(&mut self.pkl_mol as *mut _, self.pkl_size);
        }
    }

    /// remove hydrogens
    pub fn remove_all_hs(&mut self) {
        unsafe {
            remove_all_hs(&mut self.pkl_mol as *mut _, self.pkl_size);
        }
    }

    /// creates 3D coordinates
    pub fn set_3d_coords(&mut self, json_info: &str) {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            set_3d_coords(
                &mut self.pkl_mol as *mut _,
                self.pkl_size,
                json_info.as_ptr(),
            );
        }
    }

    /// Gets a [MDL molfile](https://en.wikipedia.org/wiki/Chemical_table_file) content as a string.
    pub fn get_molblock(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let mblock_cchar: *mut c_char =
                get_molblock(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let res = CStr::from_ptr(mblock_cchar).to_string_lossy().into_owned();
            free_ptr(mblock_cchar);
            res
        }
    }

    /// Gets a v3000 MDL molblock content as a string.
    pub fn get_v3kmolblock(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let mblock_cchar: *mut c_char =
                get_v3kmolblock(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let res = CStr::from_ptr(mblock_cchar).to_string_lossy().into_owned();
            free_ptr(mblock_cchar);
            res
        }
    }

    /// get descriptors as hashmap
    pub fn get_descriptors_as_dict(&self) -> HashMap<String, f32> {
        let desc_string = self.get_descriptors();
        let desc_json: HashMap<String, f32> =
            serde_json::from_str(&desc_string).expect("Wrong JSON format!?");
        desc_json
    }

    /// get descriptors as string
    pub fn get_descriptors(&self) -> String {
        let desc_cchar: *mut c_char = unsafe { get_descriptors(self.pkl_mol, *self.pkl_size) };
        let desc_string: &str = unsafe { CStr::from_ptr(desc_cchar).to_str().unwrap() };
        let res = desc_string.to_owned();
        unsafe { free_ptr(desc_cchar) };
        res
    }

    pub fn get_morgan_fp(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let fp_cchar: *mut c_char =
                get_morgan_fp(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let fp_string: &str = CStr::from_ptr(fp_cchar).to_str().unwrap();
            let res = fp_string.to_owned();
            free_ptr(fp_cchar);
            res
        }
    }

    pub fn get_morgan_fp_as_bytes(&self, json_info: &str) -> Vec<u8> {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let n_bytes: *mut u64 = libc::malloc(mem::size_of::<u64>()) as *mut u64;
            let fp_cchar: *mut c_char =
                get_morgan_fp_as_bytes(self.pkl_mol, *self.pkl_size, n_bytes, json_info.as_ptr());
            let mut fp_bytes: Vec<u8> = Vec::new();
            for pos in 0..*n_bytes {
                let nb: u8 = *fp_cchar.offset(pos as _) as u8;
                fp_bytes.push(nb);
            }
            let res = fp_bytes.to_owned();
            free(n_bytes as *mut c_void);
            free_ptr(fp_cchar);
            res
        }
    }

    pub fn get_rdkit_fp(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let fp_cchar: *mut c_char =
                get_rdkit_fp(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let fp_string: &str = CStr::from_ptr(fp_cchar).to_str().unwrap();
            let res = fp_string.to_owned();
            free_ptr(fp_cchar);
            res
        }
    }

    pub fn get_rdkit_fp_as_bytes(&self, json_info: &str) -> Vec<u8> {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let n_bytes: *mut size_t = libc::malloc(mem::size_of::<u64>()) as *mut size_t;
            let fp_cchar: *mut c_char =
                get_rdkit_fp_as_bytes(self.pkl_mol, *self.pkl_size, n_bytes, json_info.as_ptr());
            let mut fp_bytes: Vec<u8> = Vec::new();
            for pos in 0..*n_bytes {
                let nb: u8 = *fp_cchar.offset(pos as _) as u8;
                fp_bytes.push(nb);
            }
            let res = fp_bytes.to_owned();
            free(n_bytes as *mut c_void);
            free_ptr(fp_cchar);
            res
        }
    }

    pub fn get_pattern_fp(&self, json_info: &str) -> String {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let fp_cchar: *mut c_char =
                get_pattern_fp(self.pkl_mol, *self.pkl_size, json_info.as_ptr());
            let fp_string: &str = CStr::from_ptr(fp_cchar).to_str().unwrap();
            let res = fp_string.to_owned();
            free_ptr(fp_cchar);
            res
        }
    }

    pub fn get_pattern_fp_as_bytes(&self, json_info: &str) -> Vec<u8> {
        let json_info = CString::new(json_info).unwrap();
        unsafe {
            let n_bytes: *mut size_t = libc::malloc(mem::size_of::<u64>()) as *mut size_t;
            let fp_cchar: *mut c_char =
                get_pattern_fp_as_bytes(self.pkl_mol, *self.pkl_size, n_bytes, json_info.as_ptr());
            let mut fp_bytes: Vec<u8> = Vec::new();
            for pos in 0..*n_bytes {
                let nb: u8 = *fp_cchar.offset(pos as _) as u8;
                fp_bytes.push(nb);
            }
            let res = fp_bytes.to_owned();
            free(n_bytes as *mut c_void);
            free_ptr(fp_cchar);
            res
        }
    }

    ///Gets a query molecule from a SMARTS
    pub fn get_qmol(input: &str, json_info: &str) -> Option<Molecule> {
        let input_cstr = CString::new(input).unwrap();
        let json_info = CString::new(json_info).unwrap();
        let pkl_size: *mut size_t = unsafe { libc::malloc(mem::size_of::<u64>()) as *mut size_t };
        let pkl_mol = unsafe { get_qmol(input_cstr.as_ptr(), pkl_size, json_info.as_ptr()) };
        if pkl_mol.is_null() {
            return None;
        }
        return Some(Molecule { pkl_size, pkl_mol });
    }

    /// Gets a parent fragment
    pub fn fragment_parent(&mut self, json_info: &str) -> i16 {
        let json_info = CString::new(json_info).unwrap();
        let result = unsafe {
            fragment_parent(
                &mut self.pkl_mol as *mut _,
                self.pkl_size,
                json_info.as_ptr(),
            )
        };

        result
    }
}

/// read a classical .smi file
pub fn read_smifile(smi_file: &str) -> Vec<Option<Molecule>> {
    let smi_file = read_to_string(smi_file).expect("Could not load file.");
    let mut mol_list: Vec<Option<Molecule>> = Vec::new();
    let smiles_list: Vec<&str> = smi_file.split("\n").collect();
    for (_, s) in smiles_list.iter().enumerate() {
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

/// read a classical .smi file, filter molecules which are none
pub fn read_smifile_unwrap(smi_file: &str) -> Vec<Molecule> {
    let mol_opt_list: Vec<Option<Molecule>> = crate::read_smifile(smi_file);
    let mol_list: Vec<Molecule> = mol_opt_list.into_iter().filter_map(|m| m).collect();
    mol_list
}

/// read a classical .sdf file
pub fn read_sdfile(sd_file: &str) -> Vec<Option<Molecule>> {
    let sd_file = read_to_string(sd_file).expect("Could not load file.");
    let mut mol_list: Vec<Option<Molecule>> = Vec::new();
    let molblock_list: Vec<&str> = sd_file.split("$$$$").collect();
    for (_, s) in molblock_list.iter().enumerate() {
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

/// read a classical .sdf file, filter molecules which are none
pub fn read_sdfile_unwrap(sd_file: &str) -> Vec<Molecule> {
    let mol_opt_list: Vec<Option<Molecule>> = crate::read_sdfile(sd_file);
    let mol_list: Vec<Molecule> = mol_opt_list.into_iter().filter_map(|m| m).collect();
    mol_list
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBase {
    pub commonchem: HashMap<String, i32>,
    pub molecules: Vec<JsonMolecule>,
    pub defaults: RdkitDefaults,
}

/// This implements the commom chem json structure:
/// see also: [https://github.com/CommonChem/CommonChem](https://github.com/CommonChem/CommonChem)
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonMolecule {
    #[serde(default)]
    pub name: String,
    pub atoms: Vec<JsonAtom>,
    pub bonds: Vec<JsonBond>,
    #[serde(default)]
    pub conformers: Vec<JsonConformer>,
    pub extensions: Vec<Extensions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonConformer {
    pub coords: Vec<Vec<f32>>,
    dim: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RdkitDefaults {
    atom: JsonAtom,
    bond: JsonBond,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Extensions {
    name: String,
    formatVersion: i32,
    toolkitVersion: String,
    #[serde(default)]
    aromaticAtoms: Vec<i32>,
    #[serde(default)]
    aromaticBonds: Vec<i32>,
    #[serde(default)]
    atomRings: Vec<Vec<i32>>,
    #[serde(default)]
    cipCodes: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonAtom {
    #[serde(default)]
    chg: i32,
    #[serde(default)]
    impHs: i32,
    #[serde(default)]
    isotope: i32,
    #[serde(default)]
    nRad: i32,
    #[serde(default)]
    stereo: String,
    #[serde(default = "z_default")]
    z: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonBond {
    #[serde(default)]
    atoms: Vec<i32>,
    #[serde(default)]
    bo: i32,
    #[serde(default = "stereo_default")]
    stereo: String,
}

const Z_DEFAULT: i32 = 6;
fn z_default() -> i32 {
    Z_DEFAULT
}

fn stereo_default() -> String {
    String::from("unspecified")
}

impl JsonMolecule {
    ///Create new molecule from smiles, SD file or json
    pub fn new(self, molstring: &str) -> JsonMolecule {
        JsonMolecule::JsonMolFromString(molstring, "")
    }

    pub fn JsonMolFromString(molstring: &str, json_info: &str) -> JsonMolecule {
        let json_str = JSONFromString(molstring, json_info);
        JsonMolecule::JsonMolFromJson(&json_str)
    }

    pub fn JsonMolFromSmiles(smiles: &str, _json_info: &str) -> JsonMolecule {
        JsonMolecule::JsonMolFromString(smiles, "")
    }

    pub fn JsonMolFromJson(json_str: &str) -> JsonMolecule {
        let rdkit_json: JsonBase = serde_json::from_str(&json_str).expect("Wrong JSON format!?");
        let mol = serde_json::to_string(&rdkit_json.molecules[0]).unwrap();
        serde_json::from_str(&mol).expect("Wrong JSON format!?")
    }
}

pub fn JSONFromString(input: &str, json_info: &str) -> String {
    //let (pkl_mol, pkl_size) = Molecule::PklFromString(input, json_info);
    let pkl_mol = Molecule::new(input, json_info).unwrap();
    let mol_json_str = pkl_mol.get_json(json_info);
    mol_json_str.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smiles2descriptors() {
        let orig_smiles = "CCCN";
        let pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        let desc = pkl_mol.get_descriptors_as_dict();
        println!("Descriptors: {:?}", desc);
        let nheavy = desc.get("NumHeavyAtoms").unwrap().round() as i32;
        assert_eq!(nheavy, 4);
        let nrot = desc.get("NumRotatableBonds").unwrap().round() as i32;
        assert_eq!(nrot, 1);
    }

    #[test]
    fn smifile2molecules() {
        let mut mol_list: Vec<Molecule> = read_smifile_unwrap("data/ringtest.smi");
        for (i, mol) in mol_list.iter_mut().enumerate() {
            mol.remove_all_hs();
            println!(
                "Pos:{} INCHIKEY: {} SMILES: {} ",
                i,
                mol.get_inchikey(""),
                mol.get_smiles("")
            )
        }
        assert_eq!(mol_list.len(), 11);
    }
    #[test]
    fn sdfile2molecules() {
        let mut mol_list: Vec<Option<Molecule>> =
            crate::read_sdfile("data/Compound_000000001_000500000.sdf");
        println!("mols: {}", mol_list.len());
        for (i, mol_opt) in mol_list.iter_mut().enumerate() {
            let mol = if let Some(mol) = mol_opt.as_mut() {
                mol
            } else {
                continue;
            };

            mol.remove_all_hs();
            println!(
                "Pos:{} INCHIKEY: {} SMILES: {} NUMATOMS: {} NUMBONDS: {}",
                i,
                mol.get_inchikey(""),
                mol.get_smiles(""),
                mol.get_numatoms(),
                mol.get_numbonds(),
            );
        }
        assert_eq!(mol_list.len(), 9);
    }
    #[test]
    fn sdfile2molecules_win() {
        //tests windows newlines
        let mut mol_list: Vec<Molecule> = read_sdfile_unwrap("data/test_win.sdf");
        for (i, mol) in mol_list.iter_mut().enumerate() {
            mol.remove_all_hs();
            println!(
                "Pos:{} INCHIKEY: {} SMILES: {} ",
                i,
                mol.get_inchikey(""),
                mol.get_smiles(""),
            );
            println!(
                "Pos:{} INCHIKEY: {} SMILES: {} NUMATOMS: {} NUMBONDS: {}",
                i,
                mol.get_inchikey(""),
                mol.get_smiles(""),
                mol.get_numatoms(),
                mol.get_numbonds(),
            )
        }
        assert_eq!(mol_list.len(), 19);
    }
    #[test]
    fn morgan_fp() {
        let smiles = "OCC=CCO";
        let options = "{\"radius\":2,\"nBits\":64}";
        let pkl_mol = Molecule::new(smiles, "").unwrap();
        let fps = pkl_mol.get_morgan_fp(options);
        println!("Fingerprints: {:?}", fps);
        println!("Length: {:?}", fps.len());
        assert_eq!(
            fps,
            "0000000000011000100000000000001000000001000000000000001000010001"
        );
    }

    #[test]
    fn morgan_fp_bytes() {
        let smiles = "OCC=CCO";
        let options = "{\"radius\":2,\"nBits\":64}";
        let pkl_mol = Molecule::new(smiles, "").unwrap();
        let fps = pkl_mol.get_morgan_fp_as_bytes(options);
        for k in fps.iter() {
            println!("value: {:?}", k);
        }
        assert_eq!(fps.len(), 8);
    }

    #[test]
    fn generate3d() {
        let orig_smiles = "CC";
        let mut pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        pkl_mol.set_3d_coords("");
        let coords = pkl_mol.get_coords();
        assert_eq!(coords.len(), 2);
        assert_eq!(coords[0].len(), 3);
    }
    #[test]
    fn smiles_from_smiles_via_pkl() {
        let orig_smiles = "OCCC#CO";
        let pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        println!(
            "SMILES: {} Canonical SMILES: {}",
            orig_smiles,
            pkl_mol.get_smiles("")
        );
        assert_eq!(pkl_mol.get_smiles(""), "OC#CCCO");
    }
    #[test]
    fn inchi_from_smiles_via_pkl() {
        let orig_smiles = "OCCC#CO";
        let pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        println!("inchi:    {}", pkl_mol.get_inchi(""));
        println!("inchikey: {}", pkl_mol.get_inchikey(""));
        assert_eq!(
            pkl_mol.get_inchi(""),
            "InChI=1S/C4H6O2/c5-3-1-2-4-6/h5-6H,1,3H2"
        );
        assert_eq!(pkl_mol.get_inchikey(""), "JSPXPZKDILSYNN-UHFFFAOYSA-N");
    }
    #[test]
    fn molblock_from_smiles_via_pkl() {
        let orig_smiles = "CCO";
        let pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        println!("molblock:*{}*", pkl_mol.get_molblock(""));
        assert_eq!(pkl_mol.get_molblock(""),"\n     RDKit          2D\n\n  3  2  0  0  0  0  0  0  0  0999 V2000\n    0.0000    0.0000    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n    1.2990    0.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n    2.5981   -0.0000    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0\n  1  2  1  0\n  2  3  1  0\nM  END\n");
    }

    #[test]
    fn get_json_via_pkl() {
        let orig_smiles = "OCO";
        let pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        println!("json:    {}", pkl_mol.get_json(""));
        assert_eq!(pkl_mol.get_json(""),"{\"commonchem\":{\"version\":10},\"defaults\":{\"atom\":{\"z\":6,\"impHs\":0,\"chg\":0,\"nRad\":0,\"isotope\":0,\"stereo\":\"unspecified\"},\"bond\":{\"bo\":1,\"stereo\":\"unspecified\"}},\"molecules\":[{\"atoms\":[{\"z\":8,\"impHs\":1},{\"impHs\":2},{\"z\":8,\"impHs\":1}],\"bonds\":[{\"atoms\":[0,1]},{\"atoms\":[1,2]}],\"extensions\":[{\"name\":\"rdkitRepresentation\",\"formatVersion\":2,\"toolkitVersion\":\"2022.03.1\"}]}]}");
    }
    #[test]
    fn get_json_molecule() {
        let orig_smiles = "C#C";
        let pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        let json_mol = pkl_mol.get_JsonMolecule();
        println!("json molecule:    {:?}", json_mol);
        let atoms = json_mol.atoms;
        assert_eq!(atoms.len(), 2);
    }
    #[test]
    fn jsonmolecule_from_smiles() {
        let json_mol = JsonMolecule::JsonMolFromSmiles("CC(C)CCCO", "");
        println!("{:?}", json_mol);
        let bonds = json_mol.bonds;
        assert_eq!(bonds.len(), 6);
    }
    #[test]
    fn json_str_from_smiles() {
        let json_str = JSONFromString("CCCI", "");
        println!("JSON:{}", json_str);
        //let json_mol = json_mol_str.;
        assert_eq!(json_str,"{\"commonchem\":{\"version\":10},\"defaults\":{\"atom\":{\"z\":6,\"impHs\":0,\"chg\":0,\"nRad\":0,\"isotope\":0,\"stereo\":\"unspecified\"},\"bond\":{\"bo\":1,\"stereo\":\"unspecified\"}},\"molecules\":[{\"atoms\":[{\"impHs\":3},{\"impHs\":2},{\"impHs\":2},{\"z\":53}],\"bonds\":[{\"atoms\":[0,1]},{\"atoms\":[1,2]},{\"atoms\":[2,3]}],\"extensions\":[{\"name\":\"rdkitRepresentation\",\"formatVersion\":2,\"toolkitVersion\":\"2022.03.1\"}]}]}");
    }
    #[test]
    fn json_str_from_sdf() {
        let json_str = JSONFromString("\n     RDKit          2D\n\n  7  6  0  0  0  0  0  0  0  0999 V2000\n    0.0000    0.0000    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n    1.2990    0.7500    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0\n    2.5981   -0.0000    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n    3.8971    0.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n    5.1962   -0.0000    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n    6.4952    0.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n    7.7942    1.5000    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0\n  1  2  1  0\n  2  3  1  0\n  3  4  1  0\n  4  5  2  3\n  5  6  1  0\n  6  7  3  0\nM  END\n", "");
        println!("JSON:{}", json_str);
        assert_eq!(json_str,"{\"commonchem\":{\"version\":10},\"defaults\":{\"atom\":{\"z\":6,\"impHs\":0,\"chg\":0,\"nRad\":0,\"isotope\":0,\"stereo\":\"unspecified\"},\"bond\":{\"bo\":1,\"stereo\":\"unspecified\"}},\"molecules\":[{\"name\":\"\",\"atoms\":[{\"impHs\":3},{\"z\":8},{\"impHs\":2},{\"impHs\":1},{\"impHs\":1},{},{\"impHs\":1}],\"bonds\":[{\"atoms\":[0,1]},{\"atoms\":[1,2]},{\"atoms\":[2,3]},{\"bo\":2,\"atoms\":[3,4],\"stereo\":\"either\"},{\"atoms\":[4,5]},{\"bo\":3,\"atoms\":[5,6]}],\"conformers\":[{\"dim\":2,\"coords\":[[0.0,0.0],[1.299,0.75],[2.5981,-0.0],[3.8971,0.75],[5.1962,-0.0],[6.4952,0.75],[7.7942,1.5]]}],\"extensions\":[{\"name\":\"rdkitRepresentation\",\"formatVersion\":2,\"toolkitVersion\":\"2022.03.1\"}]}]}");
    }
    #[test]
    fn neutralize_ion() {
        let orig_smiles = "C(C(=O)[O-])[NH3+]";
        let mut pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        pkl_mol.neutralize("");
        let smiles = pkl_mol.get_smiles("");
        assert_eq!(smiles, "NCC(=O)O");
    }
    #[test]
    fn normalize() {
        let orig_smiles = "CN=N#N";
        let mut pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        pkl_mol.normalize("");
        let smiles = pkl_mol.get_smiles("");
        assert_eq!(smiles, "CN=[N+]=[N-]");
    }
    #[test]
    fn smarts_cxssmiles() {
        let orig_smiles = "CN=N#N";
        let pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        let smarts = pkl_mol.get_smarts("");
        assert_eq!(smarts, "[#6]-[#7]=[#7+]=[#7-]");

        let cx_input = "CO |$C2;O1$| carbon monoxide'";
        let pkl_mol2 = Molecule::new(cx_input, "").unwrap();
        let cxsmiles = pkl_mol2.get_cxsmiles("");
        println!("cxsmiles: {:?}", cxsmiles);
        assert_eq!(cxsmiles, "CO |$C2;O1$|");
    }
    #[test]
    fn find_substructure() {
        let mol = Molecule::new("Cl[C@H](F)C[C@H](F)Cl", "").unwrap();
        let query_mol = Molecule::get_qmol("Cl[C@@H](F)C", "").unwrap();
        let res = mol.get_substruct_match(&query_mol, "");
        assert_eq!(res, "{\"atoms\":[0,1,2,3],\"bonds\":[0,1,2]}");
        let res = mol.get_substruct_matches(&query_mol, "");
        assert_eq!(
            res,
            "[{\"atoms\":[0,1,2,3],\"bonds\":[0,1,2]},{\"atoms\":[6,4,5,3],\"bonds\":[5,4,3]}]"
        );
    }
    #[test]
    fn create_image() {
        let orig_smiles = "CN=N#N";
        let pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        let svg = pkl_mol.get_svg("{\"width\":350,\"height\":300}");
        assert!(svg.contains("width='350px'"));
        assert!(svg.contains("height='300px'"));
        assert!(svg.contains("</svg>"));
    }
    #[test]
    fn test_moblock() {
        let pkl_mol = Molecule::new("CN=N#N", "").unwrap();
        let v3k_molblock = pkl_mol.get_v3kmolblock("");
        let res = Molecule::new(&v3k_molblock, "");
        assert!(res.is_some());
    }
    #[test]
    fn bad_mol() {
        let molblock = "THIOL_12\n     RDKit          3D\n\n 25 25  0  0  0  0  0  0  0  0999 V2000\n   -2.2510   -2.6650   -2.0550 S   0  0  0  0  0  0  0  0  0  0  0  0\n   -3.3040   -2.7120   -2.1100 H   0  0  0  0  0  0  0  0  0  0  0  0\n   -1.7910   -1.5140   -0.7240 C   0  0  0  0  0  0  0  0  0  0  0  0\n   -2.1270   -2.0270    0.1920 H   0  0  0  0  0  0  0  0  0  0  0  0\n   -2.4730   -0.6640   -0.8710 H   0  0  0  0  0  0  0  0  0  0  0  0\n   -0.2780   -0.7500   -0.3280 C   0  0  0  0  0  0  0  0  0  0  0  0\n    0.2420   -1.8480   -0.5140 O   0  0  0  0  0  0  0  0  0  0  0  0\n    0.4860    0.3560   -0.2740 N   0  0  0  0  0  0  0  0  0  0  0  0\n    0.0540    1.2670   -0.1190 H   0  0  0  0  0  0  0  0  0  0  0  0\n    1.9050    0.2450   -0.7390 C   0  0  1  0  0  0  0  0  0  0  0  0\n    1.9190   -0.1360   -1.7740 H   0  0  0  0  0  0  0  0  0  0  0  0\n    2.4830    1.6820   -0.6980 C   0  0  0  0  0  0  0  0  0  0  0  0\n    2.4150    2.0880    0.3240 H   0  0  0  0  0  0  0  0  0  0  0  0\n    3.5420    1.6740   -0.9990 H   0  0  0  0  0  0  0  0  0  0  0  0\n    1.7270    2.5420   -1.6810 C   0  0  0  0  0  0  0  0  0  0  0  0\n    1.7070    2.3180   -2.9770 N   0  0  0  0  0  0  0  0  0  0  0  0\n    2.2060    1.5550   -3.4590 H   0  0  0  0  0  0  0  0  0  0  0  0\n    0.9600    3.1990   -3.5600 C   0  0  0  0  0  0  0  0  0  0  0  0\n    0.7540    3.2500   -4.6280 H   0  0  0  0  0  0  0  0  0  0  0  0\n    0.5040    3.9880   -2.6970 N   0  0  0  0  0  0  0  0  0  0  0  0\n    0.9760    3.6220   -1.3870 C   0  0  0  0  0  0  0  0  0  0  0  0\n    0.7740    4.0890   -0.4240 H   0  0  0  0  0  0  0  0  0  0  0  0\n    2.7730   -0.7080    0.0810 C   0  0  0  0  0  0  0  0  0  0  0  0\n    3.9440   -0.8660   -0.2210 O   0  0  0  0  0  0  0  0  0  0  0  0\n    2.2450   -1.3190    1.1390 O   0  0  0  0  0  0  0  0  0  0  0  0\n  1  2  1  0\n  1  3  1  0\n  3  4  1  0\n  3  5  1  0\n  3  6  1  0\n  6  7  2  0\n  6  8  1  0\n  8  9  1  0\n  8 10  1  0\n 10 11  1  6\n 10 12  1  0\n 10 23  1  0\n 12 13  1  0\n 12 14  1  0\n 12 15  1  0\n 15 16  1  0\n 15 21  2  0\n 16 17  1  0\n 16 18  1  0\n 18 19  1  0\n 18 20  2  0\n 20 21  1  0\n 21 22  1  0\n 23 24  2  0\n 23 25  1  0\nM  CHG  1  25  -1\nM  END\n";
        let mut pkl_mol = Molecule::new(molblock, "").unwrap();
        println!("1: {:?}", pkl_mol);
        pkl_mol.cleanup(""); // this is needed to avoid exception...
                             //pkl_mol2.remove_all_hs();
        println!("2: {:?}", pkl_mol);
        pkl_mol.canonical_tautomer("");
        assert_eq!(pkl_mol.get_smiles(""), "O=C(CS)NC(Cc1c[nH]cn1)C(=O)[O-]");
    }
    #[test]
    fn json_details() {
        let json_args = "{\"removeHs\":false,\"canonical\":false}";
        let pkl_mol = Molecule::new("c1cc(O[H])ccc1", json_args).unwrap();
        println!("{:?}", pkl_mol);
        assert_eq!(pkl_mol.get_smiles(""), "[H]Oc1ccccc1");
    }

    #[test]
    fn test_fragment_parent() {
        let orig_smiles = "CC";
        let mut pkl_mol = Molecule::new(orig_smiles, "").unwrap();
        let result = pkl_mol.fragment_parent("");
        println!("{} => {:?}", result, pkl_mol);
    }
}
