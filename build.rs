fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    env_logger::init();

    let library_root = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "x86_64") => "/usr/local",
        ("macos", "aarch64") => "/opt/homebrew",
        ("linux", _) => "/usr",
        (unsupported_os, unsupported_arch) => panic!(
            "sorry, rdkit-sys doesn't support {} on {} at this time",
            unsupported_os, unsupported_arch
        ),
    };

    let brew_lib_path = format!("{}/lib", library_root);
    let include = format!("{}/include", library_root);
    let rdkit_include = format!("{}/include/rdkit", library_root);

    let dir = std::fs::read_dir("src/bridge").unwrap();
    let rust_files = dir
        .into_iter()
        .filter_map(|p| match p {
            Ok(p) => {
                if p.metadata().unwrap().is_file() {
                    Some(p.path())
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .filter(|p| !p.ends_with("mod.rs"))
        .collect::<Vec<_>>();

    let mut cc_paths = vec![];

    let wrapper_root = std::path::PathBuf::from("wrapper");
    for file in &rust_files {
        let file_name = file.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        let base_name = &file_name[0..file_name.len() - 3];

        let cc_path = wrapper_root.join("src").join(format!("{}.cc", base_name));
        let meta = std::fs::metadata(&cc_path).unwrap();
        if !meta.is_file() {
            panic!("{} must exist", cc_path.display())
        }
        cc_paths.push(cc_path);

        let h_path = wrapper_root
            .join("include")
            .join(format!("{}.h", base_name));
        let meta = std::fs::metadata(&h_path).unwrap();
        if !meta.is_file() {
            panic!("{} must exist", h_path.display())
        }
    }

    cxx_build::bridges(rust_files)
        .files(cc_paths)
        .include(include)
        .include(rdkit_include)
        .include(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .flag("-std=c++14")
        .warnings(false)
        // rdkit has warnings that blow up our build. we could enumerate all those warnings and tell
        // the compiler to allow them... .warnings_into_errors(true)
        .compile("rdkit");

    println!("cargo:rustc-link-search=native={}", brew_lib_path);
    // println!("cargo:rustc-link-lib=static=c++");

    for lib in &[
        "Catalogs",
        "ChemReactions",
        "ChemTransforms",
        "DataStructs",
        "Descriptors",
        "FileParsers",
        "Fingerprints",
        "GenericGroups",
        "GraphMol",
        "MolStandardize",
        "RDGeneral",
        "RDGeometryLib",
        "RingDecomposerLib",
        "SmilesParse",
        "Subgraphs",
        "SubstructMatch",
    ] {
        println!("cargo:rustc-link-lib=static=RDKit{}_static", lib);
    }
    println!("cargo:rustc-link-lib=static=boost_serialization");
}
