use bip39::Mnemonic;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::All;
use bitcoin::util::address::Address as BitcoinAddress;
use bitcoin::util::bip32::DerivationPath;
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::util::key::PublicKey;
use bitcoin::util::key::Secp256k1;
use typed_builder::TypedBuilder;

pub struct AddressGenerator {
    derivation_paths: Vec<DerivationPath>,
    secp256k1: Secp256k1<All>,
}

#[derive(TypedBuilder, Debug)]
pub struct Address {
    mnemonic: String,
    derivation_path: DerivationPath,
    address: String,
}

impl AddressGenerator {
    pub fn new(derivation_paths: Vec<DerivationPath>) -> AddressGenerator {
        let secp256k1 = Secp256k1::new();
        Self {
            derivation_paths,
            secp256k1,
        }
    }

    pub fn generate(&self, mnemonic: Mnemonic) -> Vec<Address> {
        let seed = mnemonic.to_seed_normalized("");
        let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).unwrap();

        let mut addresses: Vec<Address> = vec![];

        for path in &self.derivation_paths {
            let child = master_key.derive_priv(&self.secp256k1, path).unwrap();
            let public_key = ExtendedPubKey::from_priv(&self.secp256k1, &child).public_key;

            let addr =
                BitcoinAddress::p2wpkh(&PublicKey::new(public_key), Network::Bitcoin).unwrap();

            let address = Address::builder()
                .mnemonic(mnemonic.to_string())
                .derivation_path(path.clone())
                .address(addr.to_string())
                .build();

            addresses.push(address);
        }

        addresses
    }
}
