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

fn countdown(seconds: f64) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut stdout = std::io::stdout().lock();
    loop {
        let dt = Instant::now() - start;
        if dt.as_secs_f64() > seconds {
            break;
        }
        let remaining = Duration::from_secs_f64(seconds) - dt;
        print_duration(&mut stdout, &remaining);
        stdout.flush()?;

        let max_sleep_seconds: f64 = 0.1;
        let sleep_time = Duration::from_secs_f64(max_sleep_seconds.min(remaining.as_secs_f64()));
        std::thread::sleep(sleep_time);
    }
    print_duration(&mut stdout, &Duration::from_secs(0));
    writeln!(
        &mut stdout,
        " \x1b[3mDone counting down from {:.2} seconds\x1b[0m",
        seconds
    )?;

    Ok(())
}

#[inline]
fn print_duration(f: &mut std::io::StdoutLock, duration: &Duration) {
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() / 60) % 60;
    let seconds = duration.as_secs() % 60;
    let millis = (duration.as_millis() % 1000) / 100;
    write!(f, "\r{:02}:{:02}:{:02}.{}", hours, minutes, seconds, millis).unwrap();
}

fn main() {
    let args = Args::parse();
    countdown(args.seconds).unwrap();
}
