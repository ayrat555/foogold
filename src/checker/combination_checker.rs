use super::check_address;
use super::AddressGenerator;
use super::CheckerError;
use super::TelegramClient;
use crate::Repo;
use bip39::Language;
use bip39::Mnemonic;
use itertools::Itertools;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct CombinationChecker {
    combination: usize,
    mnemonic_size: usize,
    address_generator: AddressGenerator,
    telegram_client: Option<TelegramClient>,
    repo: Repo,
}

impl CombinationChecker {
    pub fn check(&self) -> Result<(), CheckerError> {
        for mnemonic in self.mnemonics() {
            log::info!("Checking mnemonic {mnemonic}");

            let addresses = self.address_generator.generate(mnemonic);

            for address in addresses {
                check_address(&self.repo, &address, &self.telegram_client)?;
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
                Mnemonic::parse_normalized(&str_mnemonic).unwrap()
            })
            .collect::<Vec<Mnemonic>>()
    }
}
