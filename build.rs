use cmake::Config;
use std::env;

fn main() {
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "aarch64" => "arm64",
        v @ _ => panic!("Unsupported architecture {}.", v),
    };
    let dst = Config::new("./spdlog-src")
        .define("SPDLOG_BUILD_EXAMPLE", "NO")
        .define("SPDLOG_BUILD_BENCH", "NO")
        .define("SPDLOG_BUILD_TESTS", "NO")
        .define("SPDLOG_INSTALL", "NO")
        .define("CMAKE_SYSTEM_NAME", "iOS")
        .define("CMAKE_OSX_DEPLOYMENT_TARGET", "9.0")
        .define("CMAKE_OSX_ARCHITECTURES", arch)
        .define("CMAKE_IOS_INSTALL_COMBINED", "NO")
        .define("CMAKE_XCODE_ATTRIBUTE_ONLY_ACTIVE_ARCH", "YES")
        .build_target("spdlog")
        .generator("Xcode")
        .target("iphoneos")
        .build();
    //-DCMAKE_XCODE_ATTRIBUTE_CODE_SIGNING_REQUIRED=NO

    let dst = match env::var("PROFILE").unwrap().as_str() {
        "release" => dst.join("build/Release-iphoneos"),
        "debug" => dst.join("build/Debug-iphoneos"),
        _ => panic!("Invalid profile."),
    };
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=spdlog");
}
