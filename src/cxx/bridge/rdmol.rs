#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/rdmol.h");

        pub type ROMol;
        pub type ExplicitBitVect = crate::cxx::bridge::fingerprint::ffi::ExplicitBitVect;

        pub fn mol_from_smiles(smi: &CxxString) -> SharedPtr<ROMol>;
        pub fn mol_to_smiles(mol: SharedPtr<ROMol>) -> String;
        pub fn fingerprint_mol(mol: SharedPtr<ROMol>) -> SharedPtr<ExplicitBitVect>;
    }
}
