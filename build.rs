fn main() {
    env_logger::init();

    let brew_lib_path = "/opt/homebrew/lib";

    cxx_build::bridge("src/cxx/bridge/rdmol.rs") // returns a cc::Build
        .file("wrapper/src/rdmol.cc")
        .include("/opt/homebrew/include/rdkit")
        .include("/opt/homebrew/include")
        .include(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .flag_if_supported("-std=c++14")
        .warnings(false)
        .compile("rdkit");

    println!("cargo:rustc-link-search=native={}", brew_lib_path);
    // println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=static=RDKitGraphMol_static");
    println!("cargo:rustc-link-lib=static=RDKitSmilesParse_static");
    println!("cargo:rustc-link-lib=static=RDKitRDGeneral_static");
    println!("cargo:rustc-link-lib=static=RDKitRDGeometryLib_static");
}
