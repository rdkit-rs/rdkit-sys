#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/rdmol.h");

        pub type ROMol;
        pub fn mol_from_smiles(smi: &CxxString) -> SharedPtr<ROMol>;
        pub fn mol_to_smiles(mol: SharedPtr<ROMol>) -> String;
        pub fn fingerprint_mol(mol: SharedPtr<ROMol>) -> *mut ExplicitBitVect;

        pub type ExplicitBitVect;
        pub unsafe fn copy_explicit_bit_vect(
            fingerprint: *mut ExplicitBitVect,
        ) -> *mut ExplicitBitVect;

        pub unsafe fn fingerprint_or(left: *mut ExplicitBitVect, right: *mut ExplicitBitVect);
        pub unsafe fn fingerprint_and(left: *mut ExplicitBitVect, right: *mut ExplicitBitVect);
        pub unsafe fn get_num_on_bits(bitvect: *mut ExplicitBitVect) -> u32;

        pub type TautomerEnumerator;
        pub type TautomerEnumeratorResult;
        pub unsafe fn tautomer_enumerator() -> *mut TautomerEnumerator;
        // pub unsafe fn enumerate_tautomer(mol: SharedPtr<ROMol>) -> *mut
        // TautomerEnumeratorResult;
    }
}
