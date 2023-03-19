#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/inchi.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn mol_to_inchi(mol: SharedPtr<ROMol>) -> String;
        pub fn inchi_to_mol(inchi: &CxxString) -> Result<SharedPtr<ROMol>>;
    }
}
