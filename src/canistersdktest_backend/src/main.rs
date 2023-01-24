use crate::lib::MyCanister;

pub mod lib;

fn main() {
    let canister_e_idl = MyCanister::idl();
    let idl =
        candid::bindings::candid::compile(&canister_e_idl.env.env, &Some(canister_e_idl.actor));

    println!("{}", idl);
}