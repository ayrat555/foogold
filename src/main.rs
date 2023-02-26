use bip39::Mnemonic;
use bitcoin::util::bip32::DerivationPath;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
use dotenvy::dotenv;
use foogold::AddressGenerator;
use foogold::CombinationChecker;
use foogold::Indexer;
use foogold::MnemonicChecker;
use foogold::MnemonicGenerator;
use foogold::RandomChecker;
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
    RandomChecker(RandomCheckerArgs),
    MnemonicChecker(MnemonicCheckerArgs),
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
struct RandomCheckerArgs {
    #[command(flatten)]
    telegram_opts: TelegramOpts,

    #[command(flatten)]
    database_opts: DatabaseOpts,

    #[arg(long, value_delimiter = ' ', num_args = 1.., env = "DERIVATION_PATHS")]
    derivation_paths: Vec<String>,

    #[arg(long, env = "MNEMONIC_SIZE")]
    mnemonic_size: usize,
}

#[derive(Debug, Args)]
struct MnemonicCheckerArgs {
    #[command(flatten)]
    telegram_opts: TelegramOpts,

    #[command(flatten)]
    database_opts: DatabaseOpts,

    #[arg(long, value_delimiter = ' ', num_args = 1.., env = "DERIVATION_PATHS")]
    derivation_paths: Vec<String>,

    #[arg(long, env = "MNEMONIC")]
    mnemonic: String,
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
        Command::RandomChecker(random_checker_args) => check_random(random_checker_args),
        Command::MnemonicChecker(mnemonic_checker_args) => check_mnemonic(mnemonic_checker_args),
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
    check_combination_size(cli.combination_size);

    let telegram_client = new_telegram_client(cli.telegram_opts);
    let address_generator = new_address_generator(cli.derivation_paths);
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

fn check_random(cli: RandomCheckerArgs) {
    let telegram_client = new_telegram_client(cli.telegram_opts);
    let address_generator = new_address_generator(cli.derivation_paths);
    let repo = new_repo(cli.database_opts);
    let mnemonic_generator = MnemonicGenerator::new(cli.mnemonic_size);

    let checker = RandomChecker::builder()
        .repo(repo)
        .telegram_client(telegram_client)
        .address_generator(address_generator)
        .mnemonic_generator(mnemonic_generator)
        .build();

    if let Err(error) = checker.check() {
        log::error!("Failed to check random mnemonics - {error:?}")
    }
}

fn check_mnemonic(cli: MnemonicCheckerArgs) {
    let telegram_client = new_telegram_client(cli.telegram_opts);
    let address_generator = new_address_generator(cli.derivation_paths);
    let repo = new_repo(cli.database_opts);
    let mnemonic = Mnemonic::parse_normalized(&cli.mnemonic).unwrap();

    let checker = MnemonicChecker::builder()
        .repo(repo)
        .telegram_client(telegram_client)
        .address_generator(address_generator)
        .mnemonic(mnemonic)
        .build();

    if let Err(error) = checker.check() {
        log::error!("Failed to check random mnemonics - {error:?}")
    }
}

fn check_combination_size(combination_size: usize) {
    if !(combination_size == 1 || combination_size == 2) {
        panic!("Supported combination sizes are 1 and 2");
    }
}

fn new_address_generator(paths: Vec<String>) -> AddressGenerator {
    let mut derivation_paths = vec![];

    for raw_path in paths {
        let path = DerivationPath::from_str(&raw_path)
            .unwrap_or_else(|_| panic!("invalid derivation path {raw_path}"));

        derivation_paths.push(path);
    }

    AddressGenerator::new(derivation_paths)
}

fn new_telegram_client(params: TelegramOpts) -> Option<TelegramClient> {
    if params.telegram_token.is_some() {
        let chat_id = params
            .telegram_chat_id
            .expect("Telegram chat id must be present if telegram token is provided");
        let api = Api::new(&params.telegram_token.unwrap());

        let client = TelegramClient::builder().chat_id(chat_id).api(api).build();

        Some(client)
    } else {
        None
    }
}

fn new_repo(params: DatabaseOpts) -> Repo {
    Repo::builder()
        .database_url(params.database_url)
        .pool_size(params.database_pool_size)
        .build()
}
