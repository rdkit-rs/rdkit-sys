#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/rdmol.h");

        pub type TautomerEnumerator;
        pub type TautomerEnumeratorResult;
        pub unsafe fn tautomer_enumerator() -> *mut TautomerEnumerator;
        // pub unsafe fn enumerate_tautomer(mol: SharedPtr<ROMol>) -> *mut
        // TautomerEnumeratorResult;
    }
}
