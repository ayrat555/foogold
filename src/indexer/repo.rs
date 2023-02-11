use super::BlockData;
use crate::schema::addresses;
use crate::schema::blocks;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2 as diesel_r2d2;
use diesel::Connection;
use once_cell::sync::OnceCell;
use std::env;
use thiserror::Error;

static POOL: OnceCell<diesel_r2d2::Pool<diesel_r2d2::ConnectionManager<PgConnection>>> =
    OnceCell::new();

#[derive(Insertable)]
#[diesel(table_name = addresses)]
pub struct Address {
    address: String,
}

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Pool error {0:?}")]
    PoolError(#[from] r2d2::Error),
    #[error("Diesel error {0:?}")]
    DieselError(#[from] diesel::result::Error),
}

pub struct Repo {}

impl Repo {
    pub fn insert_block_data(block_data: BlockData) -> Result<(), RepoError> {
        let mut connection = Self::pool().get()?;

        connection.transaction::<(), RepoError, _>(|db_connection| {
            diesel::insert_into(blocks::table)
                .values(blocks::block_number.eq(block_data.block_number as i32))
                .execute(db_connection)?;

            let addresses: Vec<Address> = block_data
                .addresses
                .into_iter()
                .map(|address| Address { address })
                .collect();

            diesel::insert_into(addresses::table)
                .values(addresses)
                .on_conflict(addresses::address)
                .do_nothing()
                .execute(db_connection)?;

            Ok(())
        })
    }

    pub fn block_exists(block_number: i32) -> Result<bool, RepoError> {
        let mut connection = Self::pool().get()?;

        let count = blocks::table
            .filter(blocks::block_number.eq(block_number))
            .count()
            .get_result::<i64>(&mut connection)?;

        Ok(count > 0)
    }

    fn create_connection_pool() -> r2d2::Pool<diesel_r2d2::ConnectionManager<PgConnection>> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool_size = env::var("DB_POOL_SIZE")
            .expect("DB_POOL_SIZE must be set")
            .parse()
            .unwrap();

        let manager = diesel_r2d2::ConnectionManager::<PgConnection>::new(database_url);

        r2d2::Pool::builder()
            .max_size(pool_size)
            .build(manager)
            .unwrap()
    }

    fn pool() -> &'static r2d2::Pool<diesel_r2d2::ConnectionManager<PgConnection>> {
        POOL.get_or_init(Repo::create_connection_pool)
    }
}
