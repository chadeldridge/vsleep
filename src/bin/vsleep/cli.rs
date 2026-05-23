use clap::{ArgAction, CommandFactory, Parser, value_parser};

static DEFAULT_FILE: &str = "";
static DEFAULT_SPINNER: &str = "aesthetic";
//const DEFAULT_INTERVAL: i64 = 0;

#[derive(Debug, Default, Parser)]
//#[command(disable_help_flag = true, version, about, long_about = None)]
#[command(version, about, long_about = None)]
pub struct Args {
    // Positional Arguments
    /// Sleep duration in seconds
    #[arg(
        value_parser = value_parser!(i64).range(1..),
        required_unless_present = "list"
    )]
    pub duration: Option<i64>,

    // Options
    /// Alternate spinners file to import.
    /// Expected format:
    /// {
    ///   "spinner_name": {
    ///     "frames": [
    ///       "1----",
    ///       "12---",
    ///       "123--",
    ///       "1234-",
    ///       "12345"
    ///     ]
    ///   }
    /// }
    #[arg(short, long, default_value_t = DEFAULT_FILE.to_string())]
    pub file: String,
    /// List spinner names
    #[arg(long, action = ArgAction::SetTrue)]
    pub list: bool,
    //#[arg(short, long, default_value_t = DEFAULT_INTERVAL)]
    //pub interval: i64,
    /// Name of spinner to use. Default: aesthetic
    #[arg(short, long, default_value_t = DEFAULT_SPINNER.to_string())]
    pub spinner: String,
    /// Verbose output: -v shows remaining seconds, -vv also shows the current time
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
    /// Date format string for -vv output (strftime syntax). Defaults to ISO 8601 with numeric offset.
    #[arg(long, default_value_t = String::from("%Y-%m-%d %H:%M:%S %z"))]
    pub date_format: String,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}

#[derive(Debug, Default)]
pub struct Cli {
    pub args: Args,
}

impl Cli {
    pub fn new(args: Args) -> Self {
        Cli { args }
    }

    pub fn print_help() {
        let _ = Args::command().print_help();
    }
}
