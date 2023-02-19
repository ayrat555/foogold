use bitcoin::util::bip32::DerivationPath;
use clap::Parser;
use dotenvy::dotenv;
use foogold::AddressGenerator;
use foogold::CombinationChecker;
use foogold::Indexer;
use foogold::Repo;
use foogold::RpcClient;
use foogold::TelegramClient;
use frankenstein::Api;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(name = "Fool's Gold")]
#[command(author = "Airat Badykov <ayratin555@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "A tool for trying your luck with random bitcoin mnemonics", long_about = None)]
struct Cli {
    #[arg(short, long, env = "DATABASE_URL")]
    database_url: String,

    #[arg(short, long, default_value_t = 10, env = "DATABASE_POOL_SIZE")]
    database_pool_size: u32,

    #[arg(short, long, env = "NODE_URL")]
    node_url: String,

    #[arg(short, long, env = "NODE_REQUEST_HEADER_NAME")]
    node_request_header_name: Option<String>,

    #[arg(short, long, env = "NODE_REQUEST_HEADER_VALUE")]
    node_request_header_value: Option<String>,

    #[arg(short, long, env = "SYNC_START_BLOCK")]
    sync_start_block: u64,

    #[arg(short, long, env = "SYNC_END_BLOCK")]
    sync_end_block: u64,
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let api = Api::new("1276618370:AAGx5YhNQvUG4eUcQXN-OB_a09ZzYl6uh6o");
    let client = TelegramClient::builder()
        .chat_id(275808073)
        .api(api)
        .build();
    let address_generator = AddressGenerator::new(vec![
        DerivationPath::from_str("m/44'/0'/0'/0/0").unwrap(),
        DerivationPath::from_str("m/44'/0'/0'/0/1").unwrap(),
        DerivationPath::from_str("m/49'/0'/0'/0/0").unwrap(),
        DerivationPath::from_str("m/49'/0'/0'/0/1").unwrap(),
        DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap(),
        DerivationPath::from_str("m/84'/0'/0'/0/1").unwrap(),
    ]);

    let repo = Repo::builder()
        .database_url("postgres://postgres:postgres@localhost:5432/foogold".to_string())
        .pool_size(10)
        .build();

    let checker = CombinationChecker::builder()
        .repo(repo)
        .telegram_client(client)
        .address_generator(address_generator)
        .mnemonic_size(12)
        .combination(2)
        .build();

    checker.check().unwrap()

    // let cli = Cli::parse();

    // index_blocks(cli);
}

fn index_blocks(cli: Cli) {
    let header = match (cli.node_request_header_name, cli.node_request_header_value) {
        (Some(key), Some(value)) => Some((key, value)),
        _ => None,
    };

    let client = RpcClient::builder()
        .url(cli.node_url)
        .header(header)
        .build();

    let repo = Repo::builder()
        .database_url(cli.database_url)
        .pool_size(cli.database_pool_size)
        .build();

    let indexer = Indexer::builder().client(client).repo(repo).build();

    for i in cli.sync_start_block..cli.sync_end_block {
        if let Err(error) = indexer.index_block(i) {
            log::error!("Failed to index the block {i} - {error:?}")
        }
    }
}

// examples

// fn generate_10_mnemonics() -> Vec<String> {
//     let mnemonic_generator = MnemonicGenerator::new(12);
//     let mut vec = vec![];

//     for _ in 1..10 {
//         let mnemonic = mnemonic_generator.generate();

//         vec.push(mnemonic.to_string());
//     }

//     vec
// }

// fn fetch_block_data() {
//     let client = RpcClient::new(
//         "https://spinode-kong-proxy.payments-dev.testenv.io/d12f525b-880b-4d6e-ae52-003668c92f08"
//             .to_string(),
//     );

//     eprintln!("{:?}", client.get_block_data_by_block_number(100000));
// }

// fn generate_addresses() {
//     let mnemonic_generator = MnemonicGenerator::new(12);
//     let address_generator = AddressGenerator::new(vec![
//         DerivationPath::from_str("m/44'/0'/0'/0/0").unwrap(),
//         DerivationPath::from_str("m/44'/0'/0'/0/1").unwrap(),
//         DerivationPath::from_str("m/49'/0'/0'/0/0").unwrap(),
//         DerivationPath::from_str("m/49'/0'/0'/0/1").unwrap(),
//         DerivationPath::from_str("m/84'/0'/0'/0/0").unwrap(),
//         DerivationPath::from_str("m/84'/0'/0'/0/1").unwrap(),
//     ]);

//     let mnemonic = mnemonic_generator.generate();

//     eprintln!("mnemonic {}", mnemonic);

//     for address in address_generator.generate(mnemonic) {
//         eprintln!("{} {}", address.address, address.derivation_path);
//     }

//     eprintln!(
//         "{:?}",
//         address_generator.generate(mnemonic_generator.generate())
//     );
// }
