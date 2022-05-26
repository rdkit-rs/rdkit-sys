#[test]
fn test_ro_mol() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile);
    assert!(romol.is_ok());
}

#[test]
fn bad_mol_test() {
    cxx::let_cxx_string!(smile = "F(C)(C)(C)(C)(C)");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile);

    if let Err(e) = romol {
        assert_eq!(
            e.what(),
            "Explicit valence for atom # 0 F, 5, is greater than permitted"
        )
    } else {
        panic!("expected err variant")
    }
}
