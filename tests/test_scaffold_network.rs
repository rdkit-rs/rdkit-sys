use rdkit_sys::scaffold_network_ffi::*;

#[test]
fn test_scaffold_network() {
    default_scaffold_network_params();
    let params = new_scaffold_network_params(vec![]);

    set_include_generic_scaffolds(params, true);

    let _scaffold_network = default_scaffold_network();
}
