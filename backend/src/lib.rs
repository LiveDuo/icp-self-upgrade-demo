use candid::{Principal, Encode};

use ic_cdk::api::call::CallResult;

use std::time::Duration;

use ic_cdk::api::management_canister::main::*;

async fn upgrade_canister_cb() {
    ic_cdk::println!("Child: Self upgrading...");

    // upgrade code
    let id = ic_cdk::id();
    let wasm = b"\x00asm\x01\x00\x00\x00".to_vec();
    let install_args = InstallCodeArgument { mode: CanisterInstallMode::Upgrade, canister_id: id, wasm_module: wasm, arg: Encode!().unwrap(), };
    let result: CallResult<()> = ic_cdk::api::call::call(Principal::management_canister(), "install_code", (install_args,),).await;
    result.unwrap();
}

// dfx canister call child upgrade_canister
#[ic_cdk_macros::update]
async fn upgrade_canister() {

    let _id = ic_cdk::timer::set_timer(Duration::from_millis(0), || ic_cdk::spawn(upgrade_canister_cb()));
}
