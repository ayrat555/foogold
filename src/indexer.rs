use thiserror::Error;
use typed_builder::TypedBuilder;

mod client;
mod repo;

pub use client::ClientError;
pub use client::RpcClient;
pub use repo::Repo;
pub use repo::RepoError;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Repo Error {0:?}")]
    RepoError(#[from] RepoError),
    #[error("Client Error {0:?}")]
    ClientError(#[from] ClientError),
}

#[derive(TypedBuilder, Debug)]
pub struct BlockData {
    block_number: u64,
    addresses: Vec<String>,
}

pub struct Indexer {
    client: RpcClient,
}

impl Indexer {
    pub fn new(node_url: String) -> Self {
        let client = RpcClient::new(node_url.to_string());

        Self { client }
    }

    pub fn index_block(&self, block_number: u64) -> Result<(), IndexerError> {
        if !Repo::block_exists(block_number as i32)? {
            let block_data = self.client.get_block_data_by_block_number(block_number)?;

            Repo::insert_block_data(block_data)?;
        }

        Ok(())
    }
}
