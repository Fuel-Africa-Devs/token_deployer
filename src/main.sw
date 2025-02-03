contract;
pub mod data_structures;
pub mod events;
pub mod interface;
use standards::src20::{SetDecimalsEvent, SetNameEvent, SetSymbolEvent, SRC20, TotalSupplyEvent};
use ::events::TokenDeployed;
use ::interface::{Deployer, TokenInfo};
use ::data_structures::user_info::Owner;
use std::{auth::msg_sender, block::height, context::msg_amount, hash::Hash, string::String};
use std::storage::storage_vec::*;
storage {
    token_owner_info: StorageMap<AssetId, Owner> = StorageMap {},
    tokens_deployed: StorageVec<ContractId> = StorageVec {},
}
impl Deployer
 for Contract {
    #[storage(write)]
    fn save_token(asset_id: AssetId, contract_id: ContractId) {
        let caller = msg_sender().unwrap();
        let time_stamp = height().as_u64();
        let owner = Owner::new(caller, time_stamp);
        storage.tokens_deployed.push(contract_id);
        storage.token_owner_info.insert(asset_id, owner);
        log(TokenDeployed {
            time: time_stamp,
            owner: caller,
        })
    }
}
impl TokenInfo
 for Contract {
    #[storage(read)]
    fn get_token_info(assetId: AssetId) -> Owner {
        storage.token_owner_info.get(assetId).try_read().unwrap()
    }
    #[storage(read)]
    fn get_number_of_deployed_tokens() -> u64 {
        storage.tokens_deployed.len()
    }
}
// #[test]
//  use super::*;
//     use std::contract_id::ContractId;
// fn test_save_token() {

// }
