use std::borrow::Borrow;
use std::env;
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const VERSION: &'static str = "2022_03_1";

fn main() {
    env_logger::init();

    eprintln!("Downloading RDKit Release {}", VERSION);
    let path = download_rdkit_release(VERSION).unwrap();

    let expected_source_folder = path
        .parent()
        .unwrap()
        .join(format!("rdkit-Release_{}/", VERSION));

    if let Ok(metadata) = std::fs::metadata(&expected_source_folder) {
        if metadata.is_dir() {
            eprintln!(
                "folder {} already exists",
                expected_source_folder.to_str().unwrap()
            );
        }
    } else {
        run_command_or_fail(
            path.parent().unwrap().to_str().unwrap(),
            "tar",
            &["xzf", &path.file_name().unwrap().to_str().unwrap()],
        );
    }

    eprintln!("Building and linking rdkit statically");
    build_rdkit(VERSION);
}

fn download_rdkit_release(version: &str) -> eyre::Result<PathBuf> {
    let url = format!(
        "https://github.com/rdkit/rdkit/archive/refs/tags/Release_{}.tar.gz",
        version
    );

    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let output_path = crate_root.join(format!("{}.tar.gz", version));

    if let Ok(metadata) = std::fs::metadata(&output_path) {
        if metadata.len() > 0 {
            eprintln!(
                "file {} already exists and has data, skipping the download",
                output_path.display()
            );
            return Ok(output_path);
        }
    }

    let mut local_file = std::fs::File::create(output_path.clone())?;

    let mut response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        eprintln!(
            "could not download {:?}: {:?}",
            response.status(),
            response.text()
        );
        std::process::exit(1);
    }

    let mut buf = vec![0; 1024 * 32];
    let mut _total_bytes_read = 0;
    while let Ok(bytes_read) = response.read(&mut buf) {
        if bytes_read == 0 {
            break;
        }
        _total_bytes_read += bytes_read;
        local_file.write(&buf[0..bytes_read])?;
    }

    Ok(output_path)
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

fn build_rdkit(version: &str) {
    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source_code_path = crate_root.join(format!("rdkit-Release_{}", version));

    let mut config = cmake::Config::new(source_code_path.clone());

    config
        .define("RDK_BUILD_CFFI_LIB", "ON")
        .define("RDK_INSTALL_STATIC_LIBS", "ON")
        .define("RDK_BUILD_INCHI_SUPPORT", "ON")
        .define("RDK_INSTALL_INTREE", "OFF")
        .define("RDK_BUILD_PYTHON_WRAPPERS", "OFF")
        .define("RDK_BUILD_SWIG_JAVA_WRAPPER", "OFF")
        .define("RDK_BUILD_CPP_TESTS", "OFF") // TODO should we turn this one on later?
        .define("RDK_OPTIMIZE_POPCNT", "OFF") // just to skip warnings on the M1
        .env("NUM_JOBS", "4");

    config.very_verbose(true);

    println!("Configuring and compiling rdkit");
    let dst = config.build();

    eprintln!("{}", dst.display());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=rdkitcffi");

    let cffiwrapper_h_path = source_code_path.join("Code/MinimalLib/cffiwrapper.h");
    let rdkit_search_path = dst.join("include/rdkit");
    let clang_rdkit_search = format!("-F{}", rdkit_search_path.display());

    let brew_include_path = "/opt/homebrew/include";
    let brew_search = format!("-F{}", brew_include_path);

    let bindings = bindgen::Builder::default()
        // Search in output path from cmake
        .clang_arg(&clang_rdkit_search)
        .clang_arg(&brew_search)
        // The input header we would like to generate
        // bindings for.
        .header(cffiwrapper_h_path.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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
