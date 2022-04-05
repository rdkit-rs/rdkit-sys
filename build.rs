use std::borrow::Borrow;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    env_logger::init();

    // if !Path::new("rdkit/license.txt").exists() {
    eprintln!("Setting up submodules");
    run_command_or_fail(".", "git", &["submodule", "update", "--init"]);
    run_command_or_fail(".", "mkdir", &["-p", "rdkit/build"]);
    eprintln!("Building and linking rdkit statically");
    build_rdkit();
    // }
}

fn run_command_or_fail<P, S>(dir: &str, cmd: P, args: &[S])
where
    P: AsRef<Path>,
    S: Borrow<str> + AsRef<OsStr>,
{
    let cmd = cmd.as_ref();
    let cmd = if cmd.components().count() > 1 && cmd.is_relative() {
        // If `cmd` is a relative path (and not a bare command that should be
        // looked up in PATH), absolutize it relative to `dir`, as otherwise the
        // behavior of std::process::Command is undefined.
        // https://github.com/rust-lang/rust/issues/37868
        PathBuf::from(dir)
            .join(cmd)
            .canonicalize()
            .expect("canonicalization failed")
    } else {
        PathBuf::from(cmd)
    };
    eprintln!(
        "Running command: \"{} {}\" in dir: {}",
        cmd.display(),
        args.join(" "),
        dir
    );
    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => (),
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}

fn build_rdkit() {
    let mut config = cmake::Config::new("rdkit");

    config
        .define("RDK_BUILD_CFFI_LIB", "ON")
        .define("RDK_INSTALL_STATIC_LIBS", "ON")
        .define("RDK_BUILD_INCHI_SUPPORT", "ON")
        .define("RDK_INSTALL_INTREE", "OFF")
        .define("RDK_BUILD_PYTHON_WRAPPERS", "OFF")
        .define("RDK_BUILD_SWIG_JAVA_WRAPPER", "OFF")
        .define("RDK_BUILD_CPP_TESTS", "OFF") // TODO should we turn this one on later?
        .define("RDK_OPTIMIZE_POPCNT", "OFF") // just to skip warnings on the M1
    ;

    config.very_verbose(true);

    println!("Configuring and compiling rdkit");
    let dst = config.build();

    eprintln!("{}", dst.display());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=rdkitcffi");

    let crate_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cffiwrapper_h_path = format!("{}/rdkit/Code/MinimalLib/cffiwrapper.h", crate_root);
    let rdkit_search_path = dst.join("include/rdkit");
    let clang_rdkit_search = format!("-F{}", rdkit_search_path.display());

    let brew_include_path = "/opt/homebrew/include";
    let brew_search = format!("-F{}", brew_include_path);
    // eprintln!("{}", rdkit_path);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Search in output path from cmake
        .clang_arg(&clang_rdkit_search)
        .clang_arg(&brew_search)
        // The input header we would like to generate
        // bindings for.
        .header(&cffiwrapper_h_path)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // All the allows, stolen from chrissly's codebase
        .allowlist_function("version")
        .allowlist_function("enable_logging")
        .allowlist_function("disable_logging")
        .allowlist_function("get_smiles")
        .allowlist_function("get_mol")
        .allowlist_function("get_inchikey_for_inchi")
        .allowlist_function("get_inchi")
        .allowlist_function("get_molblock")
        .allowlist_function("get_v3kmolblock")
        .allowlist_function("get_json")
        .allowlist_function("canonical_tautomer")
        .allowlist_function("get_descriptors")
        .allowlist_function("add_hs")
        .allowlist_function("set_3d_coords")
        .allowlist_function("set_2d_coords")
        .allowlist_function("get_svg")
        .allowlist_function("remove_all_hs")
        .allowlist_function("get_substruct_matches")
        .allowlist_function("get_substruct_match")
        .allowlist_function("get_cxsmiles")
        .allowlist_function("get_smarts")
        .allowlist_function("get_qmol")
        .allowlist_function("cleanup")
        .allowlist_function("neutralize")
        .allowlist_function("reionize")
        .allowlist_function("normalize")
        .allowlist_function("get_morgan_fp")
        .allowlist_function("get_morgan_fp_as_bytes")
        .allowlist_function("get_rdkit_fp")
        .allowlist_function("get_rdkit_fp_as_bytes")
        .allowlist_function("get_pattern_fp")
        .allowlist_function("get_pattern_fp_as_bytes")
        .allowlist_function("free")
        .allowlist_function("free_ptr")
        .allowlist_var("size_t")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
