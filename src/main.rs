use foogold::MnemonicGenerator;

fn main() {
    let mnemonic_generator = MnemonicGenerator::new(12);

    for _ in 1..10 {
        let mnemonic = mnemonic_generator.generate();

        eprintln!("{mnemonic}");
    }
}
