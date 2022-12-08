fn main() {
    #[cfg(target_arch = "x86_64")]
    x86_64_asm();
}

fn x86_64_asm() {
    let asm_files = &["src/x86_64/syscall.s"];
    nasm_rs::compile_library("br_x86_64.a", asm_files).unwrap();
}
