use bip39::Language;
use bip39::Mnemonic;

pub struct MnemonicGenerator {
    number_of_words: usize,
}

impl MnemonicGenerator {
    pub fn new(number_of_words: usize) -> Self {
        MnemonicGenerator { number_of_words }
    }

    pub fn generate(&self) -> String {
        let mut rng = rand::thread_rng();

        Mnemonic::generate_in_with(&mut rng, Language::English, self.number_of_words)
            .unwrap()
            .to_string()
    }
}
