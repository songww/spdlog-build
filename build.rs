use cmake::Config;
use std::env;

fn main() {
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "aarch64" => "arm64",
        v @ _ => panic!(
            "Unsupported architecture `{}`, only `aarch64-apple-ios` is supported.",
            v
        ),
    };
    let dst = Config::new("./source")
        .define("SPDLOG_BUILD_EXAMPLE", "NO")
        .define("SPDLOG_BUILD_BENCH", "NO")
        .define("SPDLOG_BUILD_TESTS", "NO")
        .define("SPDLOG_INSTALL", "YES")
        .define("CMAKE_SYSTEM_NAME", "iOS")
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", "9.0")
        .define("CMAKE_OSX_ARCHITECTURES", arch)
        .define("CMAKE_IOS_INSTALL_COMBINED", "NO")
        .define("CMAKE_XCODE_ATTRIBUTE_ONLY_ACTIVE_ARCH", "YES")
        .build_target("spdlog")
        .generator("Xcode")
        .target("iphoneos")
        .build();

    let debug = env::var("DEBUG").unwrap_or("false".to_string()).as_str() == "true";
    let dst = match env::var("OPT_LEVEL").unwrap_or("0".to_string()).as_str() {
        "0" => {
            assert!(debug);
            dst.join("build/Debug-iphoneos")
        }
        "1" | "2" | "3" => {
            if debug {
                dst.join("build/RelWithDebInfo-iphoneos")
            } else {
                dst.join("build/Release-iphoneos")
            }
        }
        "s" | "z" => dst.join("MinSizeRel-iphoneos"),
        _ => panic!("Invalid profile."),
    };
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=spdlog");
}
