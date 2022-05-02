#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/mol_standardize.h");

        pub type ROMol = crate::ro_mol::ffi::ROMol;
        pub type RWMol = crate::rw_mol::ffi::RWMol;

        pub type TautomerEnumerator;
        pub type TautomerEnumeratorResult;
        pub type CleanupParameters;

        pub fn tautomer_enumerator() -> SharedPtr<TautomerEnumerator>;

        pub fn tautomer_enumerate(
            tautomer_enumerator: SharedPtr<TautomerEnumerator>,
            mol: SharedPtr<ROMol>,
        ) -> SharedPtr<TautomerEnumeratorResult>;

        pub fn tautomer_enumerator_pick_canonical(
            tautomer_enumerator: SharedPtr<TautomerEnumerator>,
            tautomer_enumerator_result: SharedPtr<TautomerEnumeratorResult>,
        ) -> SharedPtr<ROMol>;

        pub fn default_cleanup_parameters() -> SharedPtr<CleanupParameters>;

        pub fn fragment_parent(
            rw_mol: SharedPtr<RWMol>,
            cleanup_params: SharedPtr<CleanupParameters>,
            skip_standardize: bool,
        ) -> SharedPtr<RWMol>;

    }
}
