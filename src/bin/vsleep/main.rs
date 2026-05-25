use std::time::Duration;

use vsleep::core::{Error, Spinners, TickData, Timer};

mod cli;

const DEFAULT_INTERVAL: u64 = 1;

fn main() {
    // Setup basic variables.
    let cli = cli::Cli::new(cli::Args::new());
    let verbose = cli.args.verbose;
    let date_format = cli.args.date_format.clone();
    let mut spinners = Spinners::default();

    // Import spinners from provide file if a spinner file was provided.
    if !cli.args.file.is_empty() {
        match spinners.import_spinners(cli.args.file) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("error: {err}");
                std::process::exit(1);
            }
        };
    }

    // If the --list option was passed, list all spinner names and exit.
    if cli.args.list {
        show_spinner_names(&spinners);
        std::process::exit(0);
    }

    // Get the duration passed by the user. Duration is an option to allow us to have aruguements
    // that do not require a duration to be passed, like --list.
    let duration = match cli.args.duration {
        Some(n) => n,
        None => {
            cli::Cli::print_help();
            std::process::exit(1);
        }
    };

    // Create the timer.
    let mut t = Timer::new(
        Duration::from_secs(duration as u64),
        Duration::from_secs(DEFAULT_INTERVAL),
    );

    // Get the spinner to be used.
    let mut s = match spinners.get_spinner(&cli.args.spinner) {
        Some(s) => s,
        None => {
            eprintln!("{}", Error::SpinnerNotFound(cli.args.spinner.clone()));
            std::process::exit(1);
        }
    };

    // Run the timer and wait for return. Run will do all the work by executing
    // the closure each tick.
    t.run(|tick| {
        let frame = s.frame();
        println!("{}", format_tick(&frame, tick, verbose, &date_format));
        s.step_frame();
    });
}

// Format output for each tick.
fn format_tick(frame: &str, tick: &TickData, verbose: u8, date_format: &str) -> String {
    match verbose {
        0 => frame.to_string(),
        1 => format!("{} {}", frame, tick.remaining),
        _ => format!(
            "{} {} {}",
            tick.now.format(date_format),
            frame,
            tick.remaining
        ),
    }
}

// List all spinner names.
fn show_spinner_names(s: &Spinners) {
    println!("Spinner Names:");
    for n in s.keys() {
        println!("{n}");
    }
}
