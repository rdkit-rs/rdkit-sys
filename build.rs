use std::borrow::Borrow;
// use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // if env::var("CARGO_FEATURE_DYNAMIC_LINKING").is_err() {
    //     eprintln!("must enable CARGO_FEATURE_DYNAMIC_LINKING");
    //     std::process::exit(1)
    // }

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

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=rdkitcffi");
}
