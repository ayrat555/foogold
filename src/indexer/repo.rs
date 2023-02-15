use super::BlockData;
use crate::schema::addresses;
use crate::schema::blocks;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2 as diesel_r2d2;
use diesel::Connection;
use once_cell::sync::OnceCell;
use thiserror::Error;
use typed_builder::TypedBuilder;

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

#[derive(TypedBuilder)]
pub struct Repo {
    database_url: String,
    pool_size: u32,
}

impl Repo {
    pub fn insert_block_data(&self, block_data: &BlockData) -> Result<(), RepoError> {
        let mut connection = self.pool().get()?;

        connection.transaction::<(), RepoError, _>(|db_connection| {
            diesel::insert_into(blocks::table)
                .values(blocks::block_number.eq(block_data.block_number as i32))
                .execute(db_connection)?;

            let addresses: Vec<Address> = block_data
                .addresses
                .clone()
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

    pub fn block_exists(&self, block_number: i32) -> Result<bool, RepoError> {
        let mut connection = self.pool().get()?;

        let count = blocks::table
            .filter(blocks::block_number.eq(block_number))
            .count()
            .get_result::<i64>(&mut connection)?;

        Ok(count > 0)
    }

    fn create_connection_pool(&self) -> r2d2::Pool<diesel_r2d2::ConnectionManager<PgConnection>> {
        let manager =
            diesel_r2d2::ConnectionManager::<PgConnection>::new(self.database_url.clone());

        r2d2::Pool::builder()
            .max_size(self.pool_size)
            .build(manager)
            .unwrap()
    }

    fn pool(&self) -> &'static r2d2::Pool<diesel_r2d2::ConnectionManager<PgConnection>> {
        POOL.get_or_init(|| self.create_connection_pool())
    }
}
