#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");

        pub type ROMol;
        pub type ExplicitBitVect = crate::fingerprint_ffi::ExplicitBitVect;

        pub fn mol_from_smiles(smi: &CxxString) -> SharedPtr<ROMol>;
        pub fn mol_to_smiles(mol: SharedPtr<ROMol>) -> String;
    }
}
