use goblin::Object;
use std::{env, fs, path::Path};

fn r(p: &str) -> Option<Vec<u8>> {
    fs::read(Path::new(p)).ok()
}
fn h(d: &[u8], s: &str) -> bool {
    d.windows(s.len()).any(|w| w == s.as_bytes())
}
fn p(pe: &goblin::pe::PE, d: &[u8]) -> Option<&'static str> {
    if pe.header.optional_header.and_then(|o| o.data_directories.get_clr_runtime_header().ok()).is_some() {
        return Some("C#/.NET");
    }
    if pe.imports.iter().any(|i| {
        let n = i.name.to_lowercase();
        n.contains("python") || n.contains("pyi") || n.contains("pyth")
    }) {
        return Some("Python");
    }
    if pe.imports.iter().any(|i| {
        let n = i.name.to_lowercase();
        n.contains("msvcrt") || n.contains("vcruntime") || n.contains("msvc")
    }) {
        return Some("C/C++");
    }
    if pe.imports.iter().any(|i| i.name.to_lowercase().contains("go")) {
        return Some("Go");
    }
    if h(d, "UPX!") {
        return Some("Packed");
    }
    None
}
fn pe(d: &[u8]) -> Option<&'static str> {
    if let Ok(Object::PE(pe)) = Object::parse(d) {
        if let Some(g) = p(&pe, d) {
            return Some(g);
        }
        if h(d, "Go build ID:") || h(d, "go.buildid") {
            return Some("Go");
        }
        if h(d, "rust_begin_unwind") || h(d, "rust_eh_personality") || h(d, "rustc_demangle") || h(d, "libstd-") {
            return Some("Rust");
        }
        if h(d, "MSVCR") || h(d, "MSVCP") {
            return Some("C/C++");
        }
        if h(d, "libstdc++") || h(d, "GCC:") || h(d, "GLIBC") {
            return Some("C/C++");
        }
        if h(d, "Borland") || h(d, "TPMAIN") {
            return Some("Delphi");
        }
        if h(d, "PK\x03\x04") || h(d, "META-INF/MANIFEST.MF") {
            return Some("Java");
        }
        None
    } else {
        None
    }
}
fn elf(d: &[u8]) -> Option<&'static str> {
    if let Ok(Object::Elf(e)) = Object::parse(d) {
        if h(d, "Go build ID:") || h(d, "go.buildid") {
            return Some("Go");
        }
        if h(d, "rust_begin_unwind") || h(d, "rust_eh_personality") || h(d, "libstd-") {
            return Some("Rust");
        }
        if e.dynlibs.iter().any(|s| s.to_lowercase().contains("python")) {
            return Some("Python");
        }
        if e.dynlibs.iter().any(|s| s.to_lowercase().contains("stdc++")) {
            return Some("C++");
        }
        if h(d, "PK\x03\x04") || h(d, "META-INF/MANIFEST.MF") {
            return Some("Java");
        }
        None
    } else {
        None
    }
}
fn mac(d: &[u8]) -> Option<&'static str> {
    if let Ok(Object::Mach(_m)) = Object::parse(d) {
        if h(d, "rust_begin_unwind") || h(d, "libstd-") {
            return Some("Rust");
        }
        if h(d, "Go build ID:") || h(d, "go.buildid") {
            return Some("Go");
        }
        if h(d, "Python") {
            return Some("Python");
        }
        None
    } else {
        None
    }
}
fn f(p: &str) -> String {
    let d = match r(p) {
        Some(b) => b,
        None => return "error".to_string(),
    };
    if let Some(l) = pe(&d) {
        return l.to_string();
    }
    if let Some(l) = elf(&d) {
        return l.to_string();
    }
    if let Some(l) = mac(&d) {
        return l.to_string();
    }
    if h(&d, "Go build ID:") || h(&d, "go.buildid") {
        return "Go".to_string();
    }
    if h(&d, "rust_begin_unwind") || h(&d, "rust_eh_personality") || h(&d, "libstd-") {
        return "Rust".to_string();
    }
    if h(&d, "PK\x03\x04") || h(&d, "META-INF/MANIFEST.MF") {
        return "Java".to_string();
    }
    if h(&d, ".NETFramework") || h(&d, "mscorlib") || h(&d, "Assembly-CSharp") {
        return "C#/.NET".to_string();
    }
    if h(&d, "PYZ") || (h(&d, "python") && h(&d, "pyinst")) {
        return "Python".to_string();
    }
    "unknown".to_string()
}
fn main() {
    let a: Vec<String> = env::args().collect();
    if a.len() < 2 {
        println!("usage: langdetect <bin>");
        return;
    }
    println!("{}", f(&a[1]));
}
