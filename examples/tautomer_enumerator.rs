// use cxx::let_cxx_string;

fn main() {
    // let mol = rdkit_sys::bridge::ffi::smiles_to_mol("c1ccccc1C(=O)NC");

    // let_cxx_string!(smile = "c1ccccc1C(=O)NC");
    // let mol = rdkit_sys::ro_mol_ffi::smiles_to_mol(&smile).unwrap();
    // let tautomer_enumerator =
    // rdkit_sys::mol_standardize_ffi::tautomer_enumerator();
    // let tautomer_enumerator_result =
    // rdkit_sys::mol_standardize_ffi::tautomer_enumerate(
    //     tautomer_enumerator.clone(),
    //     mol.clone(),
    // );
    // let size =
    // rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_size(
    //     tautomer_enumerator_result.clone(),
    // );
    // println!("size: {}", size);
    //
    // let first =
    // rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(
    //     tautomer_enumerator_result.clone(),
    //     0,
    // );
    // let first_smile = rdkit_sys::ro_mol_ffi::mol_to_smiles(first);
    // println!("first smile: {}", first_smile);
    //
    // let second =
    // rdkit_sys::mol_standardize_ffi::tautomer_enumerator_result_tautomers_at(
    //     tautomer_enumerator_result.clone(),
    //     1,
    // );
    // let second_smile = rdkit_sys::ro_mol_ffi::mol_to_smiles(second);
    // println!("second smile: {}", second_smile);
    //
    // let canonical_mol =
    // rdkit_sys::mol_standardize_ffi::tautomer_enumerator_canonicalize(
    //     tautomer_enumerator.clone(),
    //     mol.clone(),
    // );
    // let canonical_smile =
    // rdkit_sys::ro_mol_ffi::mol_to_smiles(canonical_mol);
    // println!("canonical: {}", canonical_smile);
}
