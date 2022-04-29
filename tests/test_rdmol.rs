use rdkit_sys::molecule::Molecule;

#[test]
fn test_rdmol() {
    let _ = Molecule::from_smile("c1ccccc1C(=O)NC").unwrap();
}
