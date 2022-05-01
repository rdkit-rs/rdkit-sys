#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/rdmol.h");

        pub type ROMol = crate::cxx::bridge::rdmol::ffi::ROMol;
        pub type TautomerEnumerator;
        pub type TautomerEnumeratorResult;

        pub fn tautomer_enumerator() -> SharedPtr<TautomerEnumerator>;
        pub fn tautomer_enumerate(
            tautomer_enumerator: SharedPtr<TautomerEnumerator>,
            mol: SharedPtr<ROMol>,
        ) -> SharedPtr<TautomerEnumeratorResult>;
    }
}
