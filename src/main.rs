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

#[inline]
fn print_duration(duration: &Duration) {
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() / 60) % 60;
    let seconds = duration.as_secs() % 60;
    let millis = (duration.as_millis() % 1000) / 100;
    print!("\r{:02}:{:02}:{:02}.{}", hours, minutes, seconds, millis);
}


fn main() {
    let args = Args::parse();
    let start = Instant::now();
    loop {
        let dt = Instant::now() - start;
        if dt.as_secs_f64() > args.seconds {
            break;
        }
        let remaining = Duration::from_secs_f64(args.seconds) - dt;
        print_duration(&remaining);
        std::io::stdout().flush().unwrap();

        let max_sleep_seconds: f64 = 0.1;
        let sleep_time = Duration::from_secs_f64(max_sleep_seconds.min(remaining.as_secs_f64()));
        std::thread::sleep(sleep_time);
    }
    print_duration(&Duration::from_secs(0));
    println!(" \x1b[3mDone counting down from {:.2} seconds\x1b[0m", args.seconds);
    std::io::stdout().flush().unwrap();
}
