use bitcoin::util::bip32::DerivationPath;
use foogold::AddressGenerator;
use foogold::MnemonicGenerator;
use foogold::RpcClient;
use std::str::FromStr;

fn main() {
    // let mut buf: Vec<AlignedType> = Vec::new();
    // buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    // let secp256k1 = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let client = RpcClient::new(
        "https://btc.getblock.io/1c662d3b-4c7e-447c-8cc0-1c3c9be5b5f7/mainnet/".to_string(),
    );

    eprintln!("{:?}", client.get_block_data_by_block_number(776002));
}

// example
fn generate_10_mnemonics() -> Vec<String> {
    let mnemonic_generator = MnemonicGenerator::new(12);
    let mut vec = vec![];

    for _ in 1..10 {
        let mnemonic = mnemonic_generator.generate();

        vec.push(mnemonic.to_string());
    }

    vec
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
