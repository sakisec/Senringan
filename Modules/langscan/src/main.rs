use std::{env, fs, path::Path};

fn r(p: &str) -> Option<Vec<u8>> { fs::read(Path::new(p)).ok() }
fn h(d: &[u8], s: &str) -> bool { d.windows(s.len()).any(|w| w == s.as_bytes()) }

fn f(p: &str) -> String {
    let d = match r(p) { Some(b) => b, None => return "error".to_string() };

    if h(&d, "rust_begin_unwind") || h(&d, "rust_eh_personality") || h(&d, "libstd-") { return "Rust".to_string(); }
    if h(&d, "Go build ID:") || h(&d, "go.buildid") { return "Go".to_string(); }
    if h(&d, "PK\x03\x04") || h(&d, "META-INF/MANIFEST.MF") { return "Java".to_string(); }
    if h(&d, ".NETFramework") || h(&d, "mscorlib") || h(&d, "Assembly-CSharp") { return "C#/.NET".to_string(); }
    if h(&d, "PYZ") && h(&d, "pyinst") { return "Python".to_string(); }
    if h(&d, "MSVCR") || h(&d, "MSVCP") || h(&d, "libstdc++") || h(&d, "GCC:") || h(&d, "GLIBC") { return "C/C++".to_string(); }
    if h(&d, "Borland") || h(&d, "TPMAIN") { return "Delphi".to_string(); }
    if h(&d, "UPX!") { return "Packed".to_string(); }

    "unknown".to_string()
}

fn main() {
    let a: Vec<String> = env::args().collect();
    if a.len() < 2 { println!("usage: senringan-langscan <bin>"); return; }
    println!("{}", f(&a[1]));
}
