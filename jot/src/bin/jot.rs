// Copied and slightly modified from [simeg/eureka](https://github.com/simeg/eureka)

#[macro_use]
extern crate clap;
extern crate pretty_env_logger;
extern crate termcolor;

use std::io;

use anyhow::anyhow;
use clap::{
    Arg,
    ArgMatches,
    Command,
};
use jot::{
    config_manager::ConfigManager,
    error::pretend_this_is_main,
    git::Git,
    printer::Printer,
    program_access::ProgramAccess,
    reader::Reader,
    Jot,
    JotOptions,
};
use log::error;

const ARG_CLEAR_CONFIG: &str = "clear-config";
const ARG_VIEW: &str = "view";

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    pretend_this_is_main().unwrap();

    let cli_flags: ArgMatches = Command::new("jot")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Jot down any one liner without leaving the terminal")
        .arg(
            Arg::new(ARG_CLEAR_CONFIG)
                .long(ARG_CLEAR_CONFIG)
                .help("Clear your saved configuration"),
        )
        .arg(
            Arg::new(ARG_VIEW)
                .long(ARG_VIEW)
                .short(ARG_VIEW.chars().next().unwrap())
                .help("View jots with your $PAGER env variable. If unset use less"),
        )
        .get_matches();

    // dbg!(&cli_flags);

    let stdio = io::stdin(); // Constructs a new handle to the standard input of the current process.
    let input = stdio.lock(); // Locks this handle to the standard input stream, returning a readable guard.

    let output = termcolor::StandardStream::stdout(termcolor::ColorChoice::Always);

    let mut jot = Jot::new(
        ConfigManager::default(),
        Printer::new(output),
        Reader::new(input),
        Git::default(),
        ProgramAccess::default(),
    );

    let opts = JotOptions {
        clear_config: cli_flags.contains_id(ARG_CLEAR_CONFIG),
        view: cli_flags.contains_id(ARG_VIEW),
    };

    // dbg!(&opts);

    match jot.run(opts) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("{}", e);
            Err(anyhow!("{}", e))
        }
    }
}
