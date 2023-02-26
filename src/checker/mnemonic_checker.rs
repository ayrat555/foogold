use super::check_address;
use super::AddressGenerator;
use super::CheckerError;
use super::TelegramClient;
use crate::Repo;
use bip39::Mnemonic;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct MnemonicChecker {
    address_generator: AddressGenerator,
    telegram_client: Option<TelegramClient>,
    repo: Repo,
    mnemonic: Mnemonic,
}

impl MnemonicChecker {
    pub fn check(&self) -> Result<(), CheckerError> {
        log::info!("Checking mnemonic {}", self.mnemonic);

        let addresses = self.address_generator.generate(self.mnemonic.clone());

        for address in addresses {
            check_address(&self.repo, &address, &self.telegram_client)?;
        }

        Ok(())
    }
}
