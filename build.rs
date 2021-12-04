fn main() {
    println!("cargo:rerun-if-changed=cpplib.cpp");

    cc::Build::new()
        .file("cpplib.cpp")
        .pic(false)
        .flag("-fno-pic")
        .compile("cpplib");

    println!("cargo:rustc-link-lib=stdc++")
}