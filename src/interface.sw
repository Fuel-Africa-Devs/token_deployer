library;
use standards::src20::{SetDecimalsEvent, SetNameEvent, SetSymbolEvent, SRC20, TotalSupplyEvent};
use ::data_structures::user_info::Owner;
use std::string::String;
abi Deployer {
    #[storage(write)]
    fn save_token(asset_id: AssetId, contract_id: ContractId);
}
abi TokenInfo
 {
    #[storage(read)]
    fn get_token_info(assetId: AssetId) -> Owner;
    #[storage(read)]
    fn get_number_of_deployed_tokens() -> u64;
}
