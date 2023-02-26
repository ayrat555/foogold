use super::check_address;
use super::AddressGenerator;
use super::CheckerError;
use super::MnemonicGenerator;
use super::TelegramClient;
use crate::Repo;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct RandomChecker {
    address_generator: AddressGenerator,
    telegram_client: Option<TelegramClient>,
    mnemonic_generator: MnemonicGenerator,
    repo: Repo,
}

impl RandomChecker {
    pub fn check(&self) -> Result<(), CheckerError> {
        loop {
            let mnemonic = self.mnemonic_generator.generate();

            log::info!("Checking mnemonic {mnemonic}");

            let addresses = self.address_generator.generate(mnemonic);

            for address in addresses {
                check_address(&self.repo, &address, &self.telegram_client)?;
            }
        }
    }
}
