use crate::indexer::RepoError;
use crate::FoundAddress;
use crate::Repo;
use bip39::Language;
use bip39::Mnemonic;
use itertools::Itertools;
use thiserror::Error;
use typed_builder::TypedBuilder;

mod addresses;
mod mnemonic;
mod telegram_client;

pub use addresses::AddressGenerator;
pub use mnemonic::MnemonicGenerator;
pub use telegram_client::TelegramClient;

#[derive(TypedBuilder)]
pub struct CombinationChecker {
    combination: usize,
    mnemonic_size: usize,
    address_generator: AddressGenerator,
    telegram_client: TelegramClient,
    repo: Repo,
}

#[derive(Error, Debug)]
pub enum CheckerError {
    #[error("Repo Error {0:?}")]
    RepoError(#[from] RepoError),

    #[error("Telegram Error {0:?}")]
    TelegramError(#[from] frankenstein::Error),
}

impl CombinationChecker {
    pub fn check(&self) -> Result<(), CheckerError> {
        for mnemonic in self.mnemonics() {
            let addresses = self.address_generator.generate(mnemonic);

            for address in addresses {
                if self.repo.address_exists(&address.address)? {
                    let found_address = FoundAddress::builder()
                        .address(address.address.clone())
                        .mnemonic(address.mnemonic.to_string())
                        .derivation_path(address.derivation_path.to_string())
                        .build();

                    self.repo.insert_found_address(found_address)?;
                    self.telegram_client
                        .send_notification(format!("Found a new address {}", address.address))?;
                }
            }
        }

        Ok(())
    }

    fn mnemonics(&self) -> Vec<Mnemonic> {
        let dup_times = self.mnemonic_size / self.combination;

        Language::English
            .all_words()
            .map(String::from)
            .into_iter()
            .combinations(self.combination)
            .map(|combination| {
                let mut result = vec![];
                for _i in 0..dup_times {
                    result.extend(combination.clone());
                }

                let str_mnemonic = result.join(" ");

                let mnemonic = Mnemonic::parse_normalized(&str_mnemonic).unwrap();

                mnemonic
            })
            .collect::<Vec<Mnemonic>>()
    }
}
