fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return
    }

    env_logger::init();

    let library_root = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "x86_64") => "/usr/local",
        ("macos", "aarch64") => "/opt/homebrew",
        ("linux", _) => "beep-beep",
        (unsupported_os, unsupported_arch) => panic!("sorry, rdkit-sys doesn't support {} on {} at this time", unsupported_os, unsupported_arch)
    };

    let brew_lib_path = format!("{}/lib", library_root);
    let include = format!("{}/include", library_root);
    let rdkit_include = format!("{}/include/rdkit", library_root);

    let bridges = ["ro_mol", "rw_mol", "fingerprint", "mol_standardize"];
    let bridge_rust = bridges.iter().map(|x| format!("src/bridge/{}.rs", x));
    let wrappers_cxx = bridges.iter().map(|w| format!("wrapper/src/{}.cc", w));

    cxx_build::bridges(bridge_rust)
        .files(wrappers_cxx)
        .include(include)
        .include(rdkit_include)
        .include(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .flag("-std=c++14")
        .warnings(false)
        .compile("rdkit");

    println!("cargo:rustc-link-search=native={}", brew_lib_path);
    // println!("cargo:rustc-link-lib=dylib=c++");

    for lib in &[
        "GraphMol",
        "Fingerprints",
        "SmilesParse",
        "RDGeneral",
        "RDGeometryLib",
        "Subgraphs",
        "DataStructs",
        "MolStandardize",
    ] {
        println!("cargo:rustc-link-lib=static=RDKit{}_static", lib);
    }
}
