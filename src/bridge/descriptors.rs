#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/descriptors.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type Properties;

        pub fn new_properties() -> SharedPtr<Properties>;
        pub fn get_property_names(
            properties: SharedPtr<Properties>,
        ) -> UniquePtr<CxxVector<CxxString>>;
        pub fn compute_properties(
            properties: SharedPtr<Properties>,
            mol: SharedPtr<ROMol>,
        ) -> UniquePtr<CxxVector<f64>>;
        pub fn calc_mol_formula(mol: SharedPtr<ROMol>) -> String;
        pub fn symmetrize_sssr(mol: SharedPtr<ROMol>) -> u16;
        pub fn mol_exact_mw(mol: SharedPtr<ROMol>) -> u32;
    }
}
