use libc::c_void;

use crate::cxx::bridge::rdmol::ffi as rdmol_ffi;

pub struct Fingerprint {
    pub(crate) ptr: *mut rdmol_ffi::ExplicitBitVect,
}

impl Drop for Fingerprint {
    fn drop(&mut self) {
        unsafe { libc::free(self.ptr as *mut c_void) }
    }
}

impl Clone for Fingerprint {
    fn clone(&self) -> Self {
        Fingerprint {
            ptr: unsafe { rdmol_ffi::copy_explicit_bit_vect(self.ptr) },
        }
    }
}

impl Fingerprint {
    pub fn or(&self, other: &Fingerprint) -> Fingerprint {
        let clone = self.clone();
        unsafe { rdmol_ffi::fingerprint_or(clone.ptr, other.ptr) }
        clone
    }

    pub fn and(&self, other: &Fingerprint) -> Fingerprint {
        let clone = self.clone();
        unsafe { rdmol_ffi::fingerprint_and(clone.ptr, other.ptr) }
        clone
    }

    pub fn tanimoto_distance(&self, other: &Fingerprint) -> f32 {
        let and = self.and(other);
        let or = self.or(other);

        let and_ones = unsafe { rdmol_ffi::get_num_on_bits(and.ptr) };
        let or_ones = unsafe { rdmol_ffi::get_num_on_bits(or.ptr) };

        and_ones as f32 / or_ones as f32
    }
}
