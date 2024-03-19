//@ run-pass
//! Test that has_promoted and get_all_promoted works as expected.

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
use stable_mir::mir::StatementKind::Assign;
use stable_mir::mir::Rvalue::Use;
use stable_mir::mir::Operand::Constant;
use stable_mir::ty::ConstantKind::Allocated;
use std::io::Write;
use std::ops::ControlFlow;

const CRATE_NAME: &str = "input";

/// This function uses the Stable MIR APIs to get information about the test crate.
fn test_promoted() -> ControlFlow<()> {
    let items = stable_mir::all_local_items();

    // One promoted item
    assert!(stable_mir::has_promoted(items[1]));
    let promoted = &stable_mir::get_all_promoted()[&items[1].0];

    
    // Only 1 BB in promoted
    let promoted_body = &promoted[0];
    assert!(promoted_body.blocks.len() == 1);

    let bb = &promoted_body.blocks[0];
    let first_statement = &bb.statements[0];

    // First statement assigns the constant to a place, confirm it is 42
    let Assign(_place, rvalue) = &first_statement.kind else { unreachable!() };
    let Use(operand) = &rvalue else { unreachable!() };
    let Constant(constant) = &operand else { unreachable!() };
    let Allocated(allocation) = &constant.literal.kind() else { unreachable!() };
    assert!(allocation.read_uint().unwrap() == 42);

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
        pub fn main() {{
            const _X:&u32 = &(42);
        }}
        "#
    )?;
    Ok(())
}
