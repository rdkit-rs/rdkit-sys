fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    env_logger::init();

    let use_conda = std::env::var("CARGO_FEATURE_DYNAMIC_LINKING_FROM_CONDA").is_ok();

    let library_root = match (std::env::consts::OS, std::env::consts::ARCH, use_conda) {
        (_, _, true) => {
            // prefer the prefix env var, if not, fall back to the base from the CLI
            match std::env::var("CONDA_PREFIX") {
                Ok(prefix) => prefix.to_string(),
                Err(_) => {
                    let conda = which::which("conda")
                        .map(|p| p.to_str().unwrap().to_string())
                        .unwrap_or_else(|_| panic!("conda not found"));
                    let mut conda = std::process::Command::new(conda);
                    conda.args(&["info", "--base"]);

                    let output = conda.output().unwrap();
                    let stdout = String::from_utf8(output.stdout).unwrap();
                    stdout.trim().to_string()
                }
            }
        }
        ("macos", "x86_64", _) => "/usr/local".to_string(),
        ("macos", "aarch64", _) => "/opt/homebrew".to_string(),
        ("linux", _, _) => "/usr".to_string(),
        (unsupported_os, unsupported_arch, use_conda) => panic!(
            "sorry, rdkit-sys doesn't support {}/{}/use_conda={} at this time",
            unsupported_os, unsupported_arch, use_conda
        ),
    };

    let brew_lib_path = format!("{}/lib", library_root);
    let include = format!("{}/include", library_root);
    let rdkit_include = format!("{}/include/rdkit", library_root);

    panic!("{} / {} / {}", brew_lib_path, include, rdkit_include);

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
        if use_conda {
            println!("cargo:rustc-link-lib=dylib=RDKit{}", lib);
        } else {
            println!("cargo:rustc-link-lib=static=RDKit{}_static", lib);
        }
    }

    if use_conda {
        println!("cargo:rustc-link-lib=dylib=boost_serialization");
    } else {
        println!("cargo:rustc-link-lib=static=boost_serialization");
    }
}
