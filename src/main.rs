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
        "https://spinode-kong-proxy.payments-dev.testenv.io/6dd03d56-0da0-4385-8c52-3e5a5efdf2f0"
            .to_string(),
    );

    for i in 450000..500000 {
        indexer.index_block(i).unwrap();

        eprintln!("Indexed block {i}")
    }
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
        "https://spinode-kong-proxy.payments-dev.testenv.io/d12f525b-880b-4d6e-ae52-003668c92f08"
            .to_string(),
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
