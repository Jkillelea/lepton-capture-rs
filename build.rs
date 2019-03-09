fn main() {
    println!("cargo:rustc-link-search=./lepton-sdk-rs-sys/lepton-sdk-fork/Debug/");
    println!("cargo:rustc-link-lib=LEPTON_SDK");
}
