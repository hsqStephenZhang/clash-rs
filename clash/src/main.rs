#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

extern crate clash_lib as clash;

use clap::Parser;
use clash::TokioRuntime;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

#[derive(Parser)]
#[clap(author, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, value_name = "DIRECTORY")]
    directory: Option<PathBuf>,

    #[clap(
        short,
        long,
        visible_short_aliases = ['f'], // -f is used by clash, it is a compatibility option
        value_parser,
        value_name = "FILE",
        default_value = "config.yaml",
        help = "Specify configuration file"
    )]
    config: PathBuf,
    #[clap(
        short = 't',
        long,
        value_parser,
        default_value = "false",
        help = "Test configuration and exit"
    )]
    test_config: bool,
    #[clap(
        short,
        long,
        visible_short_aliases = ['V'],
        value_parser,
        default_value = "false",
        help = "Print clash-rs version and exit"
    )]
    version: bool,
    #[clap(short, long, help = "Additinally log to file")]
    log_file: Option<String>,
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let cli = Cli::parse();

    if cli.version {
        println!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CLASH_VERSION_OVERRIDE")
        );
        exit(0)
    }

    let file = cli
        .directory
        .as_ref()
        .unwrap_or(&std::env::current_dir().unwrap())
        .join(cli.config)
        .to_string_lossy()
        .to_string();

    if !Path::new(&file).exists() {
        // TODO: offer a internal default config, to compatible with clash
        // behavior
        panic!("config file not found: {}", file);
    }
    if cli.test_config {
        match clash::Config::File(file.clone()).try_parse() {
            Ok(_) => {
                println!("configuration file {} test is successful", file);
                exit(0);
            }
            Err(e) => {
                eprintln!("configuration file {} test failed: {}", file, e);
                exit(1);
            }
        }
    }

    match clash::start(clash::Options {
        config: clash::Config::File(file),
        cwd: cli.directory.map(|x| x.to_string_lossy().to_string()),
        rt: Some(TokioRuntime::MultiThread),
        log_file: cli.log_file,
    }) {
        Ok(_) => {}
        Err(_) => {
            exit(1);
        }
    }
}
