use bitcoincore_rpc::Auth;
use bitcoincore_rpc::Client;
use bitcoincore_rpc::Error;
use bitcoincore_rpc::RpcApi;
use diesel::pg::PgConnection;
use diesel::r2d2;
use once_cell::sync::OnceCell;
use std::env;
use typed_builder::TypedBuilder;

mod client;

pub use client::RpcClient;

static POOL: OnceCell<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>> = OnceCell::new();

struct Repo {}

impl Repo {
    pub fn create_connection_pool() -> r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool_size = env::var("DB_POOL_SIZE")
            .expect("DB_POOL_SIZE must be set")
            .parse()
            .unwrap();

        let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::builder()
            .max_size(pool_size)
            .build(manager)
            .unwrap()
    }

    pub fn pool() -> &'static r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
        POOL.get_or_init(Repo::create_connection_pool)
    }
}

#[derive(TypedBuilder, Debug)]
pub struct BlockData {
    block_number: u64,
    addresses: Vec<String>,
}

struct BlockIndexer {
    client: Client,
}

impl BlockIndexer {
    pub fn new() -> BlockIndexer {
        let node_url = env::var("NODE_URL").expect("NODE_URL must be set");
        let client = Client::new(&node_url, Auth::None).unwrap();

        BlockIndexer { client }
    }

    pub fn index_block(&self, block_number: u64) -> Result<(), Error> {
        let block_hash = self.client.get_block_hash(block_number)?;
        let block = self.client.get_block(&block_hash)?;

        Ok(())
    }

    // fn api_block_to_block_data(&self, api_block: Block, block_number: u64) -> BlockData {}
}
