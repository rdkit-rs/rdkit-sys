use rdkit_sys::substruct_match_ffi::*;

#[test]
fn test_substruct_match() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::mol_from_smiles( &smile);

    cxx::let_cxx_string!(smile = "C");
    let query = rdkit_sys::ro_mol_ffi::mol_from_smiles(&smile);

    let match_params = new_substruct_match_parameters();

    let hits = substruct_match_as_bool(mol, query, match_params);

    assert_eq!(hits, true);
}