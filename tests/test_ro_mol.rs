#[test]
fn test_ro_mol() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();
    assert!(!romol.is_null());

    rdkit_sys::ro_mol_ffi::update_property_cache(romol, true);
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

#[test]
fn parse_without_sanitize_test() {
    cxx::let_cxx_string!(smile = "N#[N]c1ccc(cc1)N(C)CN(C)(C)(C)");

    let params = rdkit_sys::ro_mol_ffi::new_smiles_parser_params();
    rdkit_sys::ro_mol_ffi::smiles_parser_params_set_sanitize(params.clone(), true);
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol_with_params(&smile, params);

    assert!(romol.is_err());

    let params = rdkit_sys::ro_mol_ffi::new_smiles_parser_params();
    rdkit_sys::ro_mol_ffi::smiles_parser_params_set_sanitize(params.clone(), false);
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol_with_params(&smile, params);

    assert!(romol.is_ok());

    let romol = romol.unwrap();
    let problems = rdkit_sys::ro_mol_ffi::detect_chemistry_problems(romol);
    assert_eq!(problems.len(), 2);

    let types = problems
        .iter()
        .map(|p| rdkit_sys::ro_mol_ffi::mol_sanitize_exception_type(p))
        .collect::<Vec<_>>();
    assert_eq!(&types, &["AtomValenceException", "AtomValenceException"]);

    let atom_idxs = problems
        .iter()
        .map(|p| rdkit_sys::ro_mol_ffi::atom_sanitize_exception_get_atom_idx(p))
        .collect::<Vec<_>>();
    assert_eq!(&atom_idxs, &[1, 11]);

    // assert_eq!(
    //     problems.get(0).unwrap().to_str().unwrap(),
    //     "AtomValenceException"
    // );
    // assert_eq!(
    //     problems.get(1).unwrap().to_str().unwrap(),
    //     "AtomValenceException"
    // );
}
