use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cw721::{ContractInfoResponse, Expiration};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    /// The owner of the newly minter NFT
    pub owner: CanonicalAddr,
    /// approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Identifies the asset to which this NFT represents
    pub name: String,

    //Add monster level
    pub level: u64,
    /// Describes the asset to which this NFT represents
    pub description: String,
    /// A URI pointing to an image representing the asset
    pub image: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransactionInfo {
    pub user_id: CanonicalAddr,
    pub ai_provider_id: String,
    pub ai_service_id: String,
    pub input_data: String,
    pub ai_output_data: String,
    pub experts_output_data: String,
    pub status: u8
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: CanonicalAddr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

pub const CONTRACT_INFO: Item<ContractInfoResponse> = Item::new(b"nft_info");
pub const MINTER: Item<CanonicalAddr> = Item::new(b"minter");
pub const TOKEN_COUNT: Item<u64> = Item::new(b"num_tokens");

// pub const TOKENS: Map<&str, TokenInfo> = Map::new(b"tokens");
pub const OPERATORS: Map<(&[u8], &[u8]), Expiration> = Map::new(b"operators");

pub fn num_tokens<S: Storage>(storage: &S) -> StdResult<u64> {
    Ok(TOKEN_COUNT.may_load(storage)?.unwrap_or_default())
}

pub fn increment_tokens<S: Storage>(storage: &mut S) -> StdResult<u64> {
    let val = num_tokens(storage)? + 1;
    TOKEN_COUNT.save(storage, &val)?;
    Ok(val)
}

pub struct TokenIndexes<'a, S: Storage> {
    pub owner: MultiIndex<'a, S, TokenInfo>,
}

impl<'a, S: Storage> IndexList<S, TokenInfo> for TokenIndexes<'a, S> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<S, TokenInfo>> + '_> {
        let v: Vec<&dyn Index<S, TokenInfo>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub struct TransactionIndexes<'a, S: Storage> {
    pub input_data: MultiIndex<'a, S, TransactionInfo>,
}

impl<'a, S: Storage> IndexList<S, TransactionInfo> for TransactionIndexes<'a, S> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<S, TransactionInfo>> + '_> {
        let v: Vec<&dyn Index<S, TransactionInfo>> = vec![&self.input_data];
        Box::new(v.into_iter())
    }
}

pub fn tokens<'a, S: Storage>() -> IndexedMap<'a, &'a str, TokenInfo, S, TokenIndexes<'a, S>> {
    let indexes = TokenIndexes {
        owner: MultiIndex::new(|d| d.owner.to_vec(), b"tokens", b"tokens__owner"),
    };
    IndexedMap::new(b"tokens", indexes)
}


pub fn transactions<'a, S: Storage>() -> IndexedMap<'a, &'a str, TransactionInfo, S, TransactionIndexes<'a, S>> {
    let indexes = TransactionIndexes {
        input_data: MultiIndex::new(|d| d.input_data.as_bytes().to_vec(), b"transactions", b"transactions user_id"),
    };
    IndexedMap::new(b"transactions", indexes)
}
