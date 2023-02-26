use crate::indexer::RepoError;
use crate::FoundAddress;
use crate::Repo;
use std::thread;
use std::time;
use thiserror::Error;

mod addresses;
mod combination_checker;
mod mnemonic;
mod random_checker;
mod telegram_client;

pub use addresses::Address;
pub use addresses::AddressGenerator;
pub use combination_checker::CombinationChecker;
pub use mnemonic::MnemonicGenerator;
pub use random_checker::RandomChecker;
pub use telegram_client::TelegramClient;

#[derive(Error, Debug)]
pub enum CheckerError {
    #[error("Repo Error {0:?}")]
    RepoError(#[from] RepoError),

    #[error("Telegram Error {0:?}")]
    TelegramError(#[from] frankenstein::Error),
}

pub fn check_address(
    repo: &Repo,
    address: &Address,
    telegram_client: &Option<TelegramClient>,
) -> Result<(), CheckerError> {
    if repo.address_exists(&address.address)? {
        let found_address = FoundAddress::builder()
            .address(address.address.clone())
            .mnemonic(address.mnemonic.to_string())
            .derivation_path(address.derivation_path.to_string())
            .build();

        repo.insert_found_address(found_address)?;

        log::info!("Found address #{address:?}");

        if let Some(telegram_client) = telegram_client {
            telegram_client
                .send_notification(format!("Found a new address {}", address.address))?;
        }

        let two_secs = time::Duration::from_millis(2_000);

        thread::sleep(two_secs);
    }

    Ok(())
}
