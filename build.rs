use cmake::Config;
use std::env;

fn main() {
    let is_apple = env::var("CARGO_CFG_TARGET_FAMILY")
        .ok()
        .and_then(|x| Some(x == "apple"))
        .unwrap_or(false);
    assert!(!is_apple, "Only apple platform is supported.");
    let is_ios =
        env::var("CARGO_CFG_TARGET_OS").unwrap_or("Unknown".to_string()) == "ios".to_string();
    let arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "aarch64" => "arm64",
        "x86_64" => "x86_64",
        v @ _ => panic!(
            "Unsupported architecture `{}`, only `aarch64-apple-ios` is supported.",
            v
        ),
    };
    let mut config = Config::new("./source");
    config
        .define("SPDLOG_BUILD_EXAMPLE", "NO")
        .define("SPDLOG_BUILD_BENCH", "NO")
        .define("SPDLOG_BUILD_TESTS", "NO")
        .define("SPDLOG_INSTALL", "YES")
        .define("CMAKE_OSX_ARCHITECTURES", arch)
        .define("CMAKE_XCODE_ATTRIBUTE_ONLY_ACTIVE_ARCH", "YES")
        .build_target("spdlog")
        .generator("Xcode");
    let dst = if is_ios {
        config
            .define("CMAKE_SYSTEM_NAME", "iOS")
            .define("CMAKE_OSX_DEPLOYMENT_TARGET", "9.0")
            .define("CMAKE_IOS_INSTALL_COMBINED", "NO")
            .target("iphoneos")
            .build()
    } else {
        config.build()
    };

    let mut is_spdlogd = false;
    let debug = env::var("DEBUG").unwrap_or("false".to_string()).as_str() == "true";
    let outdir = match env::var("OPT_LEVEL").unwrap_or("0".to_string()).as_str() {
        "0" => {
            assert!(debug);
            is_spdlogd = true;
            "build/Debug"
        }
        "1" | "2" | "3" => {
            if debug {
                "build/RelWithDebInfo"
            } else {
                "build/Release"
            }
        }
        "s" | "z" => "MinSizeRel",
        _ => panic!("Invalid profile."),
    };
    let dst = if is_ios {
        dst.join(format!("{}-iphoneos", outdir))
    } else {
        dst.join(outdir)
    };
    println!("cargo:rustc-link-search=native={}", dst.display());
    if is_spdlogd {
        println!("cargo:rustc-link-lib=static=spdlogd");
    } else {
        println!("cargo:rustc-link-lib=static=spdlog");
    }
}
