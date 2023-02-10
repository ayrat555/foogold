use foogold::MnemonicGenerator;

fn main() {
    // let mut buf: Vec<AlignedType> = Vec::new();
    // buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
    // let secp256k1 = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

    let mnemonic_generator = MnemonicGenerator::new(12);

    for _ in 1..10 {
        let mnemonic = mnemonic_generator.generate();

        eprintln!("{mnemonic}, {:?}", mnemonic.to_seed_normalized(""));
    }
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
