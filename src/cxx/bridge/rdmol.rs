#[cxx::bridge(namespace = "RDKit")]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrapper/include/rdmol.h");

        type ROMol;
        // pub type RWMol;
        //
        fn mol_from_smiles(smi: &CxxString) -> SharedPtr<ROMol>;
        //
        // fn to_smiles(mol: SharedPtr<ROMol>) -> UniquePtr<CxxString>;
        //
        // fn num_atoms(mol: SharedPtr<ROMol>) -> u32;
        //
        // fn to_svg(mol: SharedPtr<ROMol>) -> UniquePtr<CxxString>;
    }
}
