use chrono::TimeDelta;
use vsleep::core::{Error, Spinners, TickData, Timer};

mod cli;

const DEFAULT_INTERVAL: i64 = 1;

fn main() {
    let cli = cli::Cli::new(cli::Args::new());
    let verbose = cli.args.verbose;
    let date_format = cli.args.date_format.clone();
    let mut spinners = Spinners::default();

    if !cli.args.file.is_empty() {
        match spinners.import_spinners(cli.args.file) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("error: {err}");
                std::process::exit(1);
            }
        };
    }

    if cli.args.list {
        show_spinner_names(&spinners);
        std::process::exit(0);
    }

    let duration = match cli.args.duration {
        Some(n) => n,
        None => {
            cli::Cli::print_help();
            std::process::exit(1);
        }
    };

    let mut t = Timer::new(
        TimeDelta::seconds(duration),
        TimeDelta::seconds(DEFAULT_INTERVAL),
    );
    let mut s = match spinners.get_spinner(&cli.args.spinner) {
        Some(s) => s,
        None => {
            eprintln!("{}", Error::SpinnerNotFound(cli.args.spinner.clone()));
            std::process::exit(1);
        }
    };

    t.run(|tick| {
        let frame = s.frame();
        println!("{}", format_tick(&frame, tick, verbose, &date_format));
        s.step_frame();
    });
}

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

fn show_spinner_names(s: &Spinners) {
    println!("Spinner Names:");
    for n in s.keys() {
        println!("{n}");
    }
}
