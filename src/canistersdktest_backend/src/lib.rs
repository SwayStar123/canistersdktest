use std::cell::RefCell;
use std::rc::Rc;

use canister_sdk::{
    ic_exports::ic_cdk::export::candid::{CandidType, Deserialize, Principal},
    ic_storage::{IcStorage, stable::Versioned},
    ic_canister::{generate_idl, update, Canister, MethodType, PreUpdate},
};

#[derive(Default, CandidType, Deserialize, IcStorage)]
pub struct MyCanisterState {
    counter: u64,
}

#[derive(Clone, Canister)]
struct MyCanister {
    #[id]
    principal: Principal,

    #[state]
    state: Rc<RefCell<MyCanisterState>>,
}

impl MyCanister {
    #[query]
    fn get_counter(&self) -> u64 {
        self.state.borrow().counter
    }

    #[update]
    fn add(&self, value: u64) {
        self.state.borrow_mut().counter += value;
    }
}

impl PreUpdate for MyCanister {
    fn pre_update(&self, _method_name: &str, _method_type: MethodType) {
        
    }
}