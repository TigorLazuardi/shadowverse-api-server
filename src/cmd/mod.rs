use clap::{crate_authors, crate_version, Clap};
mod fetch;
mod http;

#[derive(Clap)]
enum SubCommand {
    #[clap(version = crate_version!(), author = crate_authors!(), about = "serves http server")]
    HTTP,
    #[clap(version = crate_version!(), author = crate_authors!(), about = "serves http server")]
    Fetch,
}

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!())]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// starts argument parsing
pub fn execute() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::HTTP => http::run(),
        SubCommand::Fetch => fetch::run(),
    };
}
