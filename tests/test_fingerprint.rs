#[test]
fn test_fingerprint_to_vec() {
    cxx::let_cxx_string!(smile = "c1ccccc1CCCCCCCC");
    let mol = rdkit_sys::ro_mol_ffi::mol_from_smiles(&smile);

    let fingerprint = rdkit_sys::fingerprint_ffi::fingerprint_mol(mol);
    let bytes = rdkit_sys::fingerprint_ffi::explicit_bit_vect_to_bytes_vec(fingerprint);
    let bytes: Vec<u8> = bytes.into_iter().map(|x| *x).collect();
    assert_eq!(
        bytes,
        vec![
            0, 128, 0, 4, 0, 0, 0, 0, 0, 4, 0, 41, 0, 0, 4, 0, 4, 0, 0, 8, 128, 1, 0, 0, 0, 32, 0,
            20, 0, 0, 128, 0
        ]
    )
}
