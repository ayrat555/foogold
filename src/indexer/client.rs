use super::BlockData;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;
use thiserror::Error;
use typed_builder::TypedBuilder;

pub struct RpcClient {
    pub request_agent: ureq::Agent,
    pub url: String,
}

const ADDRESS_KINDS: [AddressKind; 3] = [
    AddressKind::PubKeyHash,
    AddressKind::WitnessV0Keyhash,
    AddressKind::ScriptHash,
];

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Api Error {0:?}")]
    Api(#[from] ureq::Error),
    #[error("IO Error {0:?}")]
    IoError(#[from] std::io::Error),
    #[error("Decode Error {0:?}")]
    DecodeError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, TypedBuilder)]
struct RpcParams<T: serde::ser::Serialize + std::fmt::Debug> {
    #[builder(setter(into))]
    method: String,
    params: T,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Response<T> {
    result: T,
    error: Option<String>,
    id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Block {
    tx: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Transaction {
    vout: Vec<Output>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Output {
    #[serde(rename = "scriptPubKey")]
    script_pub_key: ScriptPubKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct ScriptPubKey {
    address: Option<String>,
    #[serde(rename = "type")]
    kind: AddressKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum AddressKind {
    #[serde(rename = "nonstandard")]
    Nonstandard,
    #[serde(rename = "pubkey")]
    PubKey,
    #[serde(rename = "pubkeyhash")]
    PubKeyHash,
    #[serde(rename = "scripthash")]
    ScriptHash,
    #[serde(rename = "multisig")]
    Multisig,
    #[serde(rename = "nulldata")]
    Nulldata,
    #[serde(rename = "witness_v0_scripthash")]
    WitnessV0Scripthash,
    #[serde(rename = "witness_v0_keyhash")]
    WitnessV0Keyhash,
    #[serde(rename = "witness_v1_taproot")]
    Witnessv1Taproot,
    #[serde(rename = "witness_unknown")]
    WitnessUnknown,
}

impl RpcClient {
    pub fn new(url: String) -> Self {
        let request_agent = ureq::builder().timeout(Duration::from_secs(100)).build();

        RpcClient { url, request_agent }
    }

    pub fn get_block_data_by_block_number(
        &self,
        block_number: u64,
    ) -> Result<BlockData, ClientError> {
        let block_hash = self.get_block_hash(block_number)?;
        let api_block = self.get_block(block_hash)?;
        let block_data = self.api_block_to_block_data(api_block, block_number);

        Ok(block_data)
    }

    fn get_block_hash(&self, block_number: u64) -> Result<String, ClientError> {
        let block_hash = self.request("getblockhash", &[block_number])?;

        Ok(block_hash)
    }

    fn get_block(&self, block_hash: String) -> Result<Block, ClientError> {
        let block = self.request("getblock", &(block_hash, 2))?;

        Ok(block)
    }

    fn request<
        T1: serde::ser::Serialize + std::fmt::Debug,
        T2: serde::de::DeserializeOwned + std::fmt::Debug,
    >(
        &self,
        method: &str,
        params: T1,
    ) -> Result<T2, ClientError> {
        let prepared_request = self
            .request_agent
            .post(&self.url)
            .set("Content-Type", "application/json")
            .set("apikey", "ae3b7595-8b8a-4f7d-aa8c-aa143d5eca48");

        let encoded_params = self.params(method, &params);
        let response = prepared_request.send_string(&encoded_params)?;

        let response: Response<T2> = serde_json::from_reader(response.into_reader())?;

        Ok(response.result)
    }

    fn params<T: serde::ser::Serialize + std::fmt::Debug>(
        &self,
        method: &str,
        params: &T,
    ) -> String {
        let params = RpcParams::builder().method(method).params(params).build();

        serde_json::to_string(&params).unwrap()
    }

    fn api_block_to_block_data(&self, api_block: Block, block_number: u64) -> BlockData {
        let addresses = api_block
            .tx
            .into_iter()
            .flat_map(|tx| tx.vout)
            .map(|vout| vout.script_pub_key)
            .filter(|script_pub_key| ADDRESS_KINDS.contains(&script_pub_key.kind))
            .flat_map(|script_pub_key| script_pub_key.address)
            .unique_by(|address| address.clone())
            .collect::<Vec<String>>();

        BlockData::builder()
            .block_number(block_number)
            .addresses(addresses)
            .build()
    }
}
