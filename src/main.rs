use bitcoin::util::bip32::DerivationPath;
use dotenvy::dotenv;
use foogold::AddressGenerator;
use foogold::Indexer;
use foogold::MnemonicGenerator;
use foogold::RpcClient;
use std::str::FromStr;

fn main() {
    dotenv().ok();

    let indexer = Indexer::new(
        "https://btc.getblock.io/1c662d3b-4c7e-447c-8cc0-1c3c9be5b5f7/mainnet/".to_string(),
    );

    indexer.index_block(100_000);
}

// examples

fn generate_10_mnemonics() -> Vec<String> {
    let mnemonic_generator = MnemonicGenerator::new(12);
    let mut vec = vec![];

    for _ in 1..10 {
        let mnemonic = mnemonic_generator.generate();

        vec.push(mnemonic.to_string());
    }

    vec
}

fn fetch_block_data() {
    let client = RpcClient::new(
        "https://btc.getblock.io/1c662d3b-4c7e-447c-8cc0-1c3c9be5b5f7/mainnet/".to_string(),
    );

    eprintln!("{:?}", client.get_block_data_by_block_number(100000));
}

fn generate_addresses() {
    let mnemonic_generator = MnemonicGenerator::new(12);
    let address_generator = AddressGenerator::new(vec![
        DerivationPath::from_str("m/44'/0'/0'/0/0").unwrap(),
        DerivationPath::from_str("m/44'/0'/0'/0/1").unwrap(),
        DerivationPath::from_str("m/49'/0'/0'/0/0").unwrap(),
        DerivationPath::from_str("m/49'/0'/0'/0/1").unwrap(),
        DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap(),
        DerivationPath::from_str("m/84'/0'/0'/0/1").unwrap(),
    ]);

    let mnemonic = mnemonic_generator.generate();

    eprintln!("mnemonic {}", mnemonic);

    for address in address_generator.generate(mnemonic) {
        eprintln!("{} {}", address.address, address.derivation_path);
    }

    eprintln!(
        "{:?}",
        address_generator.generate(mnemonic_generator.generate())
    );
}
