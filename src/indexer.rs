use thiserror::Error;
use typed_builder::TypedBuilder;

mod client;
mod repo;

pub use client::ClientError;
pub use client::RpcClient;
pub use repo::FoundAddress;
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

#[derive(TypedBuilder)]
pub struct Indexer {
    client: RpcClient,
    repo: Repo,
}

impl Indexer {
    pub fn index_block(&self, block_number: u64) -> Result<(), IndexerError> {
        if !self.repo.block_exists(block_number as i32)? {
            let block_data = self.client.get_block_data_by_block_number(block_number)?;

            self.repo.insert_block_data(&block_data)?;

            log::info!(
                "Block {block_number} indexed, the number of addresses - {}",
                block_data.addresses.len()
            );
        } else {
            log::info!("Block {block_number} already indexed");
        }

        Ok(())
    }
}
