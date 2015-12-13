#![feature(lang_items)]
#![no_std]

extern crate rlibc;
extern crate vga;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern "C" fn kmain() {
    let color = vga::DEFAULT_COLOR;
    let hello = "Hello from Rust world!";

    vga::clear_console();
    vga::kprintfln(hello, color);
    vga::kprintfln(hello, color);
}
