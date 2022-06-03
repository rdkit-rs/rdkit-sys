#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/ro_mol.h");
        include!("wrapper/include/fingerprint.h");

        pub type ROMol = crate::ro_mol_ffi::ROMol;
        pub type ExplicitBitVect;
        pub fn fingerprint_mol(mol: SharedPtr<ROMol>) -> SharedPtr<ExplicitBitVect>;

        pub fn copy_explicit_bit_vect(
            fingerprint: SharedPtr<ExplicitBitVect>,
        ) -> SharedPtr<ExplicitBitVect>;

        pub fn fingerprint_or(left: SharedPtr<ExplicitBitVect>, right: SharedPtr<ExplicitBitVect>);
        pub fn fingerprint_and(left: SharedPtr<ExplicitBitVect>, right: SharedPtr<ExplicitBitVect>);
        pub fn get_num_on_bits(bitvect: SharedPtr<ExplicitBitVect>) -> u32;

        pub fn explicit_bit_vect_to_bytes_vec(
            bitvect: SharedPtr<ExplicitBitVect>,
        ) -> UniquePtr<CxxVector<u64>>;
    }
}
