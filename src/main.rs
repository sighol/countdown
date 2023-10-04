use std::{
    io::Write,
    time::{Duration, Instant},
};

use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Args {
    #[arg()]
    seconds: f64,
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();
    loop {
        let dt = Instant::now() - start;
        if dt.as_secs_f64() > args.seconds {
            break;
        }
        let reamining = Duration::from_secs_f64(args.seconds) - dt;
        let hours = reamining.as_secs() / 3600;
        let minutes = (reamining.as_secs() / 60) % 60;
        let seconds = reamining.as_secs() % 60;
        let millis = (reamining.as_millis() % 1000) / 10 * 10;
        print!("\r{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis,);
        std::io::stdout().flush().unwrap();

        let max_sleep_seconds: f64 = 0.08;
        let sleep_time = Duration::from_secs_f64(max_sleep_seconds.min(reamining.as_secs_f64()));
        std::thread::sleep(sleep_time);
    }
    println!("\rDone sleeping for {} seconds", args.seconds);
    std::io::stdout().flush().unwrap();
}
