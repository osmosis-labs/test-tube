extern crate core;

use std::{env, path::PathBuf, process::Command};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let prebuilt_lib_dir = manifest_dir.join("libosmosistesttube").join("artifacts");

    let lib_name = "osmosistesttube";

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let header = if std::env::var("DOCS_RS").is_ok() {
        manifest_dir
            .join("libosmosistesttube")
            .join("artifacts")
            .join("libosmosistesttube.docrs.h")
    } else {
        out_dir.join(format!("lib{}.h", lib_name))
    };
    // rerun when go code is updated
    println!("cargo:rerun-if-changed=./libosmosistesttube");

    let lib_filename = if cfg!(target_os = "macos") {
        format!("lib{}.{}", lib_name, "dylib")
    } else if cfg!(target_os = "linux") {
        format!("lib{}.{}", lib_name, "so")
    } else if cfg!(target_os = "windows") {
        // untested
        format!("{}.{}", lib_name, "dll")
    } else {
        panic!("Unsupported architecture");
    };

    let lib_filename = lib_filename.as_str();

    if env::var("PREBUILD_LIB") == Ok("1".to_string()) {
        build_libosmosistesttube(prebuilt_lib_dir.join(lib_filename));
    }

    // We only build if the file doesn't exist OR if the ENV variable is not set
    let out_dir_lib_path = out_dir.join(lib_filename);

    // TODO: more robust check for rebuilding lib, this can cause confusion in dev mode
    if std::fs::metadata(&out_dir_lib_path).is_err()
        || env::var("OSMOSIS_TUBE_DEV") == Ok("1".to_string())
    {
        build_libosmosistesttube(out_dir_lib_path);
    }

    // copy built lib to target dir if debug build
    if env::var("PROFILE").unwrap() == "debug" {
        let target_dir = out_dir.join("..").join("..").join("..").join("deps");

        // for each file with pattern `libosmosistesttube.*`, copy to target dir
        for entry in std::fs::read_dir(out_dir.clone()).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("libosmosistesttube") {
                    let target_path = target_dir.join(file_name);
                    std::fs::copy(path, target_path).unwrap();
                }
            }
        }
    }

    // define lib name
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );

    // disable linking if docrs
    if std::env::var("DOCS_RS").is_err() {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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

fn build_libosmosistesttube(out: PathBuf) {
    // skip if doc_rs build
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let tidy_status = Command::new("go")
        .current_dir(manifest_dir.join("libosmosistesttube"))
        .arg("mod")
        .arg("tidy")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if !tidy_status.success() {
        panic!("failed to run 'go mod tidy'");
    }

    let exit_status = Command::new("go")
        .current_dir(manifest_dir.join("libosmosistesttube"))
        .arg("build")
        .arg("-buildmode=c-shared")
        .arg("-ldflags")
        .arg("-w")
        .arg("-o")
        .arg(out)
        .arg("main.go")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if !exit_status.success() {
        panic!("failed to build go code");
    }
}
