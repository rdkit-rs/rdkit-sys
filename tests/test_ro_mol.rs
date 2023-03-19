#[test]
fn test_ro_mol() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();
    assert!(!romol.is_null());
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
    assert_eq!(
        problems.get(0).unwrap().to_str().unwrap(),
        "AtomValenceException"
    );
    assert_eq!(
        problems.get(1).unwrap().to_str().unwrap(),
        "AtomValenceException"
    );
}

#[test]
fn test_mol_to_inchi() {
    cxx::let_cxx_string!(smile = "C");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();
    let inchi = rdkit_sys::ro_mol_ffi::mol_to_inchi(romol);
    assert_eq!(inchi, "InChI=1S/CH4/h1H4");
}

#[test]
fn test_good_inchi_to_mol() {
    cxx::let_cxx_string!(inchi = "InChI=1S/C2H6/c1-2/h1-2H3");
    let romol = rdkit_sys::ro_mol_ffi::inchi_to_mol(&inchi).unwrap();
    assert!(!romol.is_null());
    assert_eq!(rdkit_sys::ro_mol_ffi::mol_to_inchi(romol.clone()), "InChI=1S/C2H6/c1-2/h1-2H3");
    assert_eq!(rdkit_sys::ro_mol_ffi::mol_to_smiles(romol), "CC");
}

#[test]
fn test_bad_inchi_to_mol() {
    cxx::let_cxx_string!(bad_inchi = "asd");
    let romol = rdkit_sys::ro_mol_ffi::inchi_to_mol(&bad_inchi);
    assert!(romol.is_ok());
    assert!(romol.unwrap().is_null());
}
