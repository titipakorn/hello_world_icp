use candid::{Principal};
use ic_cdk::{query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Vec<Principal>>;
type ProfileStore = BTreeMap<Principal,Vec<String>>;

thread_local! {
static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
static ID_STORE: RefCell<IdStore> = RefCell::default();
}

#[query(name = "submitted_names")]
fn submitted_names() -> Vec<String> {
    let id = ic_cdk::api::caller();
    let submitted_name = PROFILE_STORE.with(|profile_store| {
    profile_store
    .borrow()
    .get(&id)
    .cloned().unwrap_or_default()
    });
    submitted_name.iter().map(|name| format!("Hello, {}!", name)).collect::<Vec<String>>()
}

#[update]
fn greet(name: String) -> String {
let principal_id = ic_cdk::api::caller();
ID_STORE.with(|id_store| {
    let mut id_store = id_store.borrow_mut();
    id_store.entry(name.clone())
        .or_insert_with(Vec::new)
        .push(principal_id);
});

PROFILE_STORE.with(|profile_store| {
    let mut profile_store = profile_store.borrow_mut();
    profile_store.entry(principal_id)
        .or_insert_with(Vec::new)
        .push(name.clone());
});
format!("Hello, {}!", name)
}