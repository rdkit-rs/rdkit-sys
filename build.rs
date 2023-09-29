const CPP_VERSION_FLAG: &'static str = "-std=c++20";

fn main() -> miette::Result<()> {
    if std::env::var("DOCS_RS").is_ok() {
        return Err(miette::miette!("weird"));
    }

    // env_logger::init();

    let use_conda = std::env::var("CARGO_FEATURE_DYNAMIC_LINKING_FROM_CONDA").is_ok();

    let mut lib_paths = vec![];
    let mut include_paths = vec![];

    match (std::env::consts::OS, std::env::consts::ARCH, use_conda) {
        (_, _, true) => {
            // prefer the prefix env var, if not, fall back to the base from the CLI
            match std::env::var("CONDA_PREFIX") {
                Ok(prefix) => {
                    include_paths.push(format!("{prefix}/include"));
                    include_paths.push(format!("{prefix}/include/rdkit"));
                    lib_paths.push(format!("{prefix}/lib"));
                }
                Err(_) => {
                    let conda = which::which("conda")
                        .map(|p| p.to_str().unwrap().to_string())
                        .unwrap_or_else(|_| panic!("conda not found"));
                    let mut conda = std::process::Command::new(conda);
                    conda.args(&["info", "--base"]);

                    let output = conda.output().unwrap();
                    let stdout = String::from_utf8(output.stdout).unwrap();
                    let conda_root = stdout.trim().to_string();

                    lib_paths.push(format!("{}/lib", conda_root));
                    include_paths.push(format!("{}/include", conda_root));
                    include_paths.push(format!("{}/include/rdkit", conda_root));
                }
            }
        }
        ("macos", "x86_64", _) => {
            include_paths.push("/usr/local/include".to_string());
            include_paths.push("/usr/local/include/rdkit".to_string());
        }
        ("macos", "aarch64", _) => {
            include_paths.push("/opt/homebrew/include".to_string());
            include_paths.push("/opt/homebrew/include/rdkit".to_string());
            lib_paths.push("/opt/homebrew/lib".to_string())
        }
        ("linux", _, _) => {
            include_paths.push("/usr/local/include".to_string());
            include_paths.push("/usr/local/include/rdkit".to_string());
            include_paths.push("/usr/include".to_string());
            include_paths.push("/usr/include/rdkit".to_string());
        }
        (unsupported_os, unsupported_arch, use_conda) => panic!(
            "sorry, rdkit-sys doesn't support {}/{}/use_conda={} at this time",
            unsupported_os, unsupported_arch, use_conda
        ),
    };

    let rdkit_path = std::path::PathBuf::from("/opt/homebrew/include/rdkit");
    let homebrew_path = std::path::PathBuf::from("/opt/homebrew/include");
    let mut builder = autocxx_build::Builder::new("src/lib.rs", &[&rdkit_path, &homebrew_path])
        .extra_clang_args(&[CPP_VERSION_FLAG])
        .build()
        .unwrap();
    builder.flag(CPP_VERSION_FLAG).compile("rdkit-sys");

    for path in lib_paths {
        println!("cargo:rustc-link-search=native={}", path);
    }
    // println!("cargo:rustc-link-lib=static=c++");

    for lib in &[
        // "Catalogs",
        // "ChemReactions",
        // "ChemTransforms",
        "DataStructs",
        // "Depictor",
        "Descriptors",
        "FileParsers",
        "Fingerprints",
        // "GenericGroups",
        "GraphMol",
        "MolStandardize",
        // "MolTransforms",
        // "PartialCharges",
        "RDGeneral",
        // "RDGeometryLib",
        // "RingDecomposerLib",
        "SmilesParse",
        // "Subgraphs",
        "SubstructMatch",
    ] {
        println!("cargo:rustc-link-lib=dylib=RDKit{}", lib);
    }

    println!("cargo:rustc-link-lib=dylib=boost_serialization");

    Ok(())
}
