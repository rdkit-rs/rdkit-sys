#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/rw_mol.h");

        pub type RWMol;
        pub type ROMol = crate::ro_mol_ffi::ROMol;

        pub fn rw_mol_from_ro_mol(
            mol: SharedPtr<ROMol>,
            quick_copy: bool,
            conf_id: i32,
        ) -> SharedPtr<RWMol>;
    }
}
