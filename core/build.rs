use std::{collections::HashSet, env, fs::File, io::Write, process::Command};

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};

const IGNORE_MACROS: [&str; 5] = [
    "FP_INFINITE",
    "FP_NAN",
    "FP_NORMAL",
    "FP_SUBNORMAL",
    "FP_ZERO",
];

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        if self.0.contains(name) {
            MacroParsingBehavior::Ignore
        } else {
            MacroParsingBehavior::Default
        }
    }
}

impl IgnoreMacros {
    fn new() -> Self {
        Self(IGNORE_MACROS.into_iter().map(|s| s.to_owned()).collect())
    }
}

struct BindingFiles {
    dir: String,
    files: Vec<String>,
}
impl BindingFiles {
    fn new<'a>(dir: &str, files: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            dir: dir.to_owned(),
            files: files.into_iter().map(|s| s.to_owned()).collect(),
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search=native=/opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasi");
    println!("cargo:rustc-link-search=native=/opt/wasi-sdk/lib/clang/17/lib/wasi/");
    println!("cargo:rustc-link-lib=static=clang_rt.builtins-wasm32");

    let cwd = env::current_dir().unwrap();
    let ffmpeg_dir = cwd.join("./ffmpeg");
    let libs = [
        BindingFiles::new("avformat", ["avformat"]),
        BindingFiles::new("avutil", ["avutil", "random_seed", "pixdesc"]),
        BindingFiles::new("avcodec", ["avcodec"]),
    ];
    for lib in libs.iter() {
        let lib_dir = ffmpeg_dir.join(format!("lib{}", &lib.dir));
        println!(
            "cargo:rerun-if-changed={}",
            lib_dir.join(format!("lib{}.a", &lib.dir)).display()
        );
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static={}", &lib.dir);
    }

    let mut configure = Command::new("./configure");
    configure
        .args([
            "--disable-swresample",
            "--disable-swscale",
            "--disable-doc",
            "--disable-iconv",
            "--disable-autodetect",
            "--disable-debug",
            "--disable-runtime-cpudetect",
            "--disable-programs",
            "--disable-network",
            "--disable-pthreads",
            "--disable-asm",
            "--enable-cross-compile",
            "--enable-lto",
            "--enable-protocol=file",
            "--arch=x86_32",
            "--target-os=none",
            "--pkg-config-flags=\"--static\"",
            "--cc=/opt/wasi-sdk/bin/clang",
            "--cxx=/opt/wasi-sdk/bin/clang++",
            "--objcc=/opt/wasi-sdk/bin/clang",
            "--dep-cc=/opt/wasi-sdk/bin/clang",
            "--ar=/opt/wasi-sdk/bin/ar",
            "--ranlib=/opt/wasi-sdk/bin/ranlib",
            "--nm=/opt/wasi-sdk/bin/nm",
            "--extra-cflags=\"-msimd128 -D_WASI_EMULATED_SIGNAL -lwasi-emulated-signal -D_WASI_EMULATED_PROCESS_CLOCKS -lwasi-emulated-process-clocks\"",
        ])
        .current_dir(ffmpeg_dir.clone());
    let output = configure.output().expect("failed to execute configure");
    if output.status.code() != Some(0) {
        println!("configure command: {:?}", configure);
        println!("configure status: {}", output.status);
        println!(
            "configure stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        println!(
            "configure stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("configure failed");
    }

    let mut make = Command::new("make");
    make.current_dir(ffmpeg_dir.clone()).arg("-j");
    let output = make.output().expect("failed to execute make");
    if output.status.code() != Some(0) {
        println!("make command: {:?}", make);
        println!("make status: {}", output.status);
        println!("make stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("make stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("make failed");
    }

    let mut bindings_file = File::create(cwd.join("src/bindings.rs")).unwrap();
    bindings_file
        .write_all("#![allow(warnings)]\n".as_bytes())
        .unwrap();

    for lib in libs.iter() {
        let lib_dir = ffmpeg_dir.join(format!("lib{}", &lib.dir));

        let binding_builder = {
            let mut builder = bindgen::builder();
            for file in &lib.files {
                let header_path = lib_dir.join(format!("{}.h", file));
                builder = builder.header(header_path.to_str().unwrap());
            }

            builder
        };

        let binding = binding_builder
            .clang_arg("--target=x86_64-unknown-linux-gnu")
            // .clang_arg("--target=wasm32-wasi")
            .clang_arg("-I./ffmpeg")
            .clang_arg("-D_WASI_EMULATED_SIGNAL")
            .clang_arg("-D_WASI_EMULATED_PROCESS_CLOCKS")
            .parse_callbacks(Box::new(IgnoreMacros::new()))
            // .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = cwd.join(format!("src/bindings/{}.rs", lib.dir));
        binding
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");

        bindings_file
            .write_all(format!("pub mod {};\n", lib.dir).as_bytes())
            .unwrap();
    }
}
