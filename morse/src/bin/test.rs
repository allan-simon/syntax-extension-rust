#![feature(phase)]
#[phase(plugin, link)]
extern crate morse;

fn main() {
    println!(
        "oh a int {}",
        morse! {.-_.-_-...}
    );
}
