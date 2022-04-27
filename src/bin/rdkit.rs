use rdkit_sys::molecule::Molecule;

fn main() {
    let mol = Molecule::from_smile("c1ccccc1C(=O)NC");
}
