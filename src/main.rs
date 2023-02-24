use bitcoin::util::bip32::DerivationPath;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
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
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Index(IndexerArgs),
    CombinationChecker(CombinationCheckerArgs),
}

#[derive(Debug, Args)]
struct IndexerArgs {
    #[command(flatten)]
    database_opts: DatabaseOpts,

    #[arg(long, env = "NODE_URL")]
    node_url: String,

    #[arg(long, env = "NODE_REQUEST_HEADER_NAME")]
    node_request_header_name: Option<String>,

    #[arg(long, env = "NODE_REQUEST_HEADER_VALUE")]
    node_request_header_value: Option<String>,

    #[arg(long, env = "SYNC_START_BLOCK")]
    sync_start_block: u64,

    #[arg(long, env = "SYNC_END_BLOCK")]
    sync_end_block: u64,
}

#[derive(Debug, Args, Clone)]
struct DatabaseOpts {
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    #[arg(long, default_value_t = 10, env = "DATABASE_POOL_SIZE")]
    database_pool_size: u32,
}

#[derive(Debug, Args)]
struct CombinationCheckerArgs {
    #[command(flatten)]
    telegram_opts: TelegramOpts,

    #[command(flatten)]
    database_opts: DatabaseOpts,

    #[arg(long, value_delimiter = ' ', num_args = 1.., env = "DERIVATION_PATHS")]
    derivation_paths: Vec<String>,

    #[arg(long, env = "MNEMONIC_SIZE")]
    mnemonic_size: usize,

    #[arg(long, env = "COMBINATION_SIZE")]
    combination_size: usize,
}

#[derive(Debug, Args)]
struct TelegramOpts {
    #[arg(long, env = "TELEGRAM_API_TOKEN")]
    telegram_token: Option<String>,

    #[arg(long, env = "TELEGRAM_CHAT_ID")]
    telegram_chat_id: Option<i64>,
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Command::Index(indexer_args) => index_blocks(indexer_args),
        Command::CombinationChecker(combination_checker_args) => {
            check_combinations(combination_checker_args)
        }
    }
}

fn index_blocks(cli: IndexerArgs) {
    let header = match (cli.node_request_header_name, cli.node_request_header_value) {
        (Some(key), Some(value)) => Some((key, value)),
        _ => None,
    };

    let client = RpcClient::builder()
        .url(cli.node_url)
        .header(header)
        .build();

    let repo = new_repo(cli.database_opts);
    let indexer = Indexer::builder().client(client).repo(repo).build();

    for i in cli.sync_start_block..cli.sync_end_block {
        if let Err(error) = indexer.index_block(i) {
            log::error!("Failed to index the block {i} - {error:?}")
        }
    }
}

fn check_combinations(cli: CombinationCheckerArgs) {
    let telegram_client = if cli.telegram_opts.telegram_token.is_some() {
        let chat_id = cli
            .telegram_opts
            .telegram_chat_id
            .expect("Telegram chat id must be present if telegram token is provided");
        let api = Api::new(&cli.telegram_opts.telegram_token.unwrap());

        let client = TelegramClient::builder().chat_id(chat_id).api(api).build();

        Some(client)
    } else {
        None
    };

    let mut derivation_paths = vec![];

    for raw_path in cli.derivation_paths {
        let path = DerivationPath::from_str(&raw_path)
            .expect(&format!("invalid derivation path {raw_path}"));

        derivation_paths.push(path);
    }

    let address_generator = AddressGenerator::new(derivation_paths);
    let repo = new_repo(cli.database_opts);

    let checker = CombinationChecker::builder()
        .repo(repo)
        .telegram_client(telegram_client)
        .address_generator(address_generator)
        .mnemonic_size(cli.mnemonic_size)
        .combination(cli.combination_size)
        .build();

    if let Err(error) = checker.check() {
        log::error!("Failed to check combinations - {error:?}")
    }
}

fn new_repo(params: DatabaseOpts) -> Repo {
    Repo::builder()
        .database_url(params.database_url)
        .pool_size(params.database_pool_size)
        .build()
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
