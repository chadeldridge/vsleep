use std::thread;
use std::time::Duration;
use vsleep::spinner::Spinners;

mod cli;

fn main() {
    let cli = cli::Cli::new(cli::Args::new());
    let interval = Duration::from_secs(1);
    let mut spinners = Spinners::default();

    if !cli.args.file.is_empty() {
        match spinners.import_spinners(cli.args.file) {
            Ok(_) => {}
            Err(err) => {
                println!("error loading spinners file: {}", err);
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

    let mut s = match spinners.get_spinner(&cli.args.spinner) {
        Some(s) => s,
        None => {
            println!("error: spinner not found");
            std::process::exit(1);
        }
    };

    //println!("using {}", s.get_name());
    for i in 1..=duration {
        thread::sleep(interval);

        let frame = s.get_frame();
        thread::spawn(move || {
            print_status(i, frame);
        });

        s.step_frame();
    }
}

fn show_spinner_names(s: &Spinners) {
    println!("Spinner Names:");
    for n in s.keys() {
        println!("{n}");
    }
}

fn print_status(count: u64, frame: String) {
    println!("{} {}", frame, count);
}
