#![allow(unused)]

use std::io::stdin;
// use clevis::{DecryptConfig, EncryptConfig, EncryptSource};
use std::io::{BufRead, Read};

use clap::{Args, Parser, Subcommand};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Encrypt stdin data and print it to stdout
    Encrypt(EncryptArgs),
    /// Decrypt stdin data and print it to stdout
    Decrypt { name: Option<String> },
}

#[derive(Debug, Args)]
struct EncryptArgs {
    /// The base URL of the Tang server
    #[arg(short, long)]
    url: String,
    /// The thumbprint of a trusted signing key
    #[arg(short, long)]
    thumbprint: Option<String>,
    /// A filename containing a trusted advertisement
    #[arg(short, long)]
    adv: Option<String>,
    /// A trusted advertisement (raw JSON)
    #[arg(long)]
    adv_file: Option<String>,
    /// Skip the advertisement check
    #[arg(short = 'y', long, default_value_t = false)]
    skip_trust: bool,
}

fn main() {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(|| Box::new(std::io::stderr()))
                .with_target(true)
                .with_span_events(FmtSpan::CLOSE),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(v) => run_encryption(v),
        Commands::Decrypt { name: _ } => run_decryption(),
    }
}

fn run_encryption(args: EncryptArgs) {
    let thp = args.thumbprint.unwrap_or_default();
    // let process = EncryptConfig {
    //     thp,
    //     source: EncryptSource::Server(args.url),
    // };

    // let mut buf: Vec<u8> = Vec::new();
    // stdin().lock().read_to_end(&mut buf).unwrap();

    // let res = process.encrypt(&buf);

    // if let Err(err) = res {
    //     eprintln!("error: {err:?}");
    // }
}

fn run_decryption() {
    let mut buf: Vec<u8> = Vec::new();
    stdin().lock().read_to_end(&mut buf).unwrap();
    // stdin().lock().read_until(b'.', &mut buf).unwrap();
    // buf.pop(); // remove trailing '.'
    // DecryptConfig::from_b64_jwe(&buf).unwrap();
}
