fn main() -> miette::Result<()> {
    
    println!("cargo:rustc-link-lib=dylib=ida");
    println!("cargo:rustc-link-search=native=D:\\rust\\cxx\\ida_cxx\\lib");

    let path = std::path::PathBuf::from("src/include"); // include path
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path]).build()?;
        // This assumes all your C++ bindings are in lib.rs
    b.flag_if_supported("-std=c++11")
     .compile("autocxx-demo"); // arbitrary library name, pick anything

     
    println!("cargo:rerun-if-changed=src/lib.rs");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}