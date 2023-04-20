use candid::{Principal, Encode};

use ic_cdk::api::management_canister::main::*;

async fn upgrade_canister_cb() {
    ic_cdk::println!("upgrading");
    
    // get wasm
    let wasm = b"\x00asm\x01\x00\x00\x00".to_vec();
    // let wasm = include_bytes!("../../target/wasm32-unknown-unknown/release/child.wasm").to_vec();
    
    // upgrade code
    let id = ic_cdk::id();
    let install_args = InstallCodeArgument { mode: CanisterInstallMode::Upgrade, canister_id: id, wasm_module: wasm, arg: Encode!().unwrap(), };
    let _ = ic_cdk::api::call::call::<_, ()>(Principal::management_canister(), "install_code", (install_args,),).await.unwrap();
    // let _ = ic_cdk::api::call::call::<_, ()>(Principal::management_canister(), "raw_rand", (),).await.unwrap();

    ic_cdk::println!("upgraded");
}

// dfx canister call aaaaa-aa update_settings '(record { canister_id = principal "'$(dfx canister id child)'"; settings = record { controllers = opt vec { principal "'$(dfx canister id child)'"; principal "'$(dfx identity get-principal)'"; }; }; })'
// dfx canister call child upgrade_canister
#[ic_cdk_macros::update]
async fn upgrade_canister() {

    upgrade_canister_cb().await;
}
