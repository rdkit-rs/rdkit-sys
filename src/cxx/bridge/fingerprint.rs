#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/rdmol.h");

        pub type ExplicitBitVect;
        pub unsafe fn copy_explicit_bit_vect(
            fingerprint: *mut ExplicitBitVect,
        ) -> *mut ExplicitBitVect;

        pub unsafe fn fingerprint_or(left: *mut ExplicitBitVect, right: *mut ExplicitBitVect);
        pub unsafe fn fingerprint_and(left: *mut ExplicitBitVect, right: *mut ExplicitBitVect);
        pub unsafe fn get_num_on_bits(bitvect: *mut ExplicitBitVect) -> u32;
    }
}
