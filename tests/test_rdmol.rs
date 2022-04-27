use rdkit_sys::molecule::Molecule;

#[test]
fn test_rdmol() {
    let mol = Molecule::from_smile("c1ccccc1C(=O)NC");
}
