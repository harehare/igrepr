use clap::Parser;
use igr::cli::Cli;
use std::process::ExitCode;

fn main() -> ExitCode {
    Cli::parse()
        .run()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            ExitCode::FAILURE
        })
}
