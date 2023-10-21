#[test]
fn test_atoms() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let romol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();

    let num_atoms = rdkit_sys::ro_mol_ffi::get_num_atoms(romol.clone(), true);

    for idx in 0..num_atoms {
        println!("grabbing idx {}", idx);
        let _atom = rdkit_sys::ro_mol_ffi::get_atom_with_idx(romol.clone(), idx);
    }
}
