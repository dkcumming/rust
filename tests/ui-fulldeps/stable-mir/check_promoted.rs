//@ run-pass
//! Test that item kind works as expected.

//@ ignore-stage1
//@ ignore-cross-compile
//@ ignore-remote
//@ ignore-windows-gnu mingw has troubles with linking https://github.com/rust-lang/rust/pull/116837
//@ edition: 2021

#![feature(rustc_private)]
#![feature(assert_matches)]
#![feature(control_flow_enum)]

#[macro_use]
extern crate rustc_smir;
extern crate rustc_driver;
extern crate rustc_interface;
extern crate stable_mir;

use rustc_smir::rustc_internal;
// use stable_mir::*;
use std::io::Write;
use std::ops::ControlFlow;

const CRATE_NAME: &str = "input";

/// This function uses the Stable MIR APIs to get information about the test crate.
fn test_promoted() -> ControlFlow<()> {
    let items = stable_mir::all_local_items();
    assert!(items.len() == 1);

    assert!(stable_mir::has_promoted(items[0]));

    ControlFlow::Continue(())
}

/// This test will generate and analyze a dummy crate using the stable mir.
/// For that, it will first write the dummy crate into a file.
/// Then it will create a `StableMir` using custom arguments and then
/// it will run the compiler.
fn main() {
    let path = "item_kind_input.rs";
    generate_input(&path).unwrap();
    let args = vec![
        "rustc".to_string(),
        "-Cpanic=abort".to_string(),
        "--crate-type=lib".to_string(),
        "--crate-name".to_string(),
        CRATE_NAME.to_string(),
        path.to_string(),
    ];
    run!(args, test_promoted).unwrap();
}

fn generate_input(path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    write!(
        file,
        r#"
        pub fn abcd() {{
            println!("HELLO, WORLD!");
        }}
        "#
    )?;
    Ok(())
}
