use std::cell::RefCell;
use std::rc::Rc;

use ic_canister::{
    generate_exports, generate_idl, query, state_getter, update, Idl, MethodType, Canister, PreUpdate
};
use ic_storage::{IcStorage, stable::Versioned};
use ic_exports::ic_cdk::export::candid::{CandidType, Deserialize, Principal};

#[derive(Default, CandidType, Deserialize, IcStorage)]
pub struct MyCanisterState {
    counter: u64,
}

impl Versioned for MyCanisterState {
    type Previous = ();

    fn upgrade((): ()) -> Self {
        Self::default()
    }
}

#[derive(Clone, Canister)]
pub struct MyCanister {
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

    pub fn idl() -> Idl {
        generate_idl!()
    }
}

impl PreUpdate for MyCanister {
    fn pre_update(&self, _method_name: &str, _method_type: MethodType) {
        
    }
}