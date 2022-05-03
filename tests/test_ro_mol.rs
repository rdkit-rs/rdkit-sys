#[test]
fn test_ro_mol() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let _ = rdkit_sys::ro_mol_ffi::mol_from_smiles(&smile);
}