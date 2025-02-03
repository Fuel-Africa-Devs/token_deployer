use fuels::{prelude::*, types::ContractId};
use std::str::FromStr;

use fuels::types::{AssetId, Identity};
// to create test script
// cargo generate --init fuellabs/sway templates/sway-test-rs --name token_deployer  --force

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/token_deployer-abi.json"
));

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await
    .unwrap();
    let wallet = wallets.pop().unwrap();

    let id = Contract::load_from(
        "./out/debug/token_deployer.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);

    (instance, id.into())
}

#[tokio::test]
async fn can_get_contract_id() {
    let (instance, id) = get_contract_instance().await;
    let hex_str = "0x0000000000000000000000000000000000000000000000000000000000000000";
    let asset_id = AssetId::from_str(hex_str).unwrap();
    println!("{:?}, and id {:?}", asset_id, id);
    instance
        .methods()
        .save_token(asset_id.clone(), id)
        .call()
        .await
        .unwrap();
    let user_info = instance
        .methods()
        .get_token_info(asset_id)
        .call()
        .await
        .unwrap();
    println!("{:?}", user_info.value.owner);
    let owner = user_info.value.owner;
    let b256_address: [u8; 32] = [
        0x09, 0xc0, 0xb2, 0xd1, 0xa4, 0x86, 0xc4, 0x38, 0xa8, 0x7b, 0xcb, 0xa6, 0xb4, 0x6a, 0x7a,
        0x1a, 0x23, 0xf3, 0x89, 0x7c, 0xc8, 0x3a, 0x94, 0x52, 0x1a, 0x96, 0xda, 0x5c, 0x23, 0xbc,
        0x58, 0xdb,
    ]; // randomcar
    match owner {
        Identity::Address(address) => {
            assert_ne!(address, Address::from(b256_address));
        }
        Identity::ContractId(_) => todo!(),
    };

    // check deployment time....
    assert!(user_info.value.deployement_date > 0);

    // check length...
    let num_of_deployments = instance
        .methods()
        .get_number_of_deployed_tokens()
        .call()
        .await
        .unwrap();
    assert!(num_of_deployments.value > 0);

    // Now you have an instance of your contract you can use to test each function
}
// cargo test -- --nocapture to see println!
