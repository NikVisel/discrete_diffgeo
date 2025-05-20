use std::env;
use std::panic;
use std::path::PathBuf;
use bindgen;
use pkg_config;
#[cfg(windows)]
use vcpkg;

fn main() {
    // Only run when USD feature is enabled
    if env::var("CARGO_FEATURE_USD").is_err() {
        return;
    }

    #[cfg(not(windows))]
    {
        // Unix-like: pkg-config
        let lib = pkg_config::Config::new()
            .atleast_version("23.0")
            .probe("usd")
            .expect("Could not find Pixar USD via pkg-config");
        // Link libraries
        for path in &lib.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for libname in &lib.libs {
            println!("cargo:rustc-link-lib=dylib={}", libname);
        }
        generate_bindings(&lib.include_paths[0]);
    }

    #[cfg(windows)]
    {
        // Windows: try vcpkg, fallback to pkg-config
        let maybe = panic::catch_unwind(|| vcpkg::Config::new().find_package("usd")).ok().and_then(Result::ok);
        if let Some(lib) = maybe {
            // Link via cargo metadata
            for meta in &lib.cargo_metadata {
                println!("{}", meta);
            }
            generate_bindings(&lib.include_paths[0]);
        } else {
            let lib = pkg_config::Config::new()
                .atleast_version("23.0")
                .probe("usd")
                .expect("Could not find Pixar USD via pkg-config or vcpkg");
            for path in &lib.link_paths {
                println!("cargo:rustc-link-search=native={}", path.display());
            }
            for libname in &lib.libs {
                println!("cargo:rustc-link-lib=dylib={}", libname);
            }
            generate_bindings(&lib.include_paths[0]);
        }
    }
}

fn generate_bindings(include: &std::path::Path) {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include.display()))
        .allowlist_function("Usd.*")
        .allowlist_function("pxr.*")
        .allowlist_type("Usd.*")
        .allowlist_type("pxr.*")
        .generate()
        .expect("Unable to generate USD bindings");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("usd_bindings.rs"))
        .expect("Couldn't write USD bindings!");
}
