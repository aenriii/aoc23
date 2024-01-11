#![allow(incomplete_features)]
#![feature(unsized_locals, unsized_fn_params)]

mod days;
pub type Fn_ = dyn Fn() -> ();
pub type FnPtr = *const Fn_;
fn main() {
    let stopwatch = std::time::Instant::now();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut options = getargs::Options::new(args.iter().map(|s| s.as_str()));
    let mut args = Args {
        time: false,
        day: 0,
        part: 0,
        verbosity_level: 0,
    };
    while let Ok(Some(arg)) = options.next_arg() {
        use getargs::Arg::*;
        match arg {
            Positional(arg) => {
                let mut b = arg.bytes();
                let mut day = 0;
                let mut part = 0;
                while let Some(byte) = b.next() {
                    if byte == b':' {
                        break;
                    }
                    day = (day * 10) + (byte - b'0') as u8;
                }
                while let Some(byte) = b.next() {
                    part = part * 10 + (byte - b'0') as u8;
                }
                args.day = day;
                args.part = part;
            },
            Short('t') | Long("time") => {
                args.time = true;
            },
            Short('v') | Long("verbose") => {
                args.verbosity_level += 1;
            },
            Short('q') | Long("quiet") => {
                args.verbosity_level -= 1;
            },
            Short('h') | Long("help") => {
                println!("Usage: {} [options] [day:part]", std::env::args().next().unwrap());
                println!("Options:");
                println!("  -t, --time     Print the time taken to run the solution");
                println!("  -v, --verbose  Increase verbosity level");
                println!("  -q, --quiet    Decrease verbosity level");
                println!("  -h, --help     Print this help message");
                std::process::exit(0);
            },
            _ => {
                println!("Unknown argument: {}", arg);
                std::process::exit(1);
            },
        }
    }
    if args.time {
        println!("Parsed args in {}μs", stopwatch.elapsed().as_micros());
    }
    let function = unsafe { *days::FUNCTIONS.add(((args.day << 3) | args.part-1) as usize) };
    if function as *const () as usize == 0 {
        println!("No solution found for day {} part {}", args.day, args.part);
        if args.time {
            println!("Ran in {}μs", stopwatch.elapsed().as_micros());
        }
        std::process::exit(1);
    }
    let stopwatch = std::time::Instant::now();
    (unsafe { &*function })();
    if args.time {
        println!("Ran in {}μs", stopwatch.elapsed().as_micros());
    }

}

#[repr(packed)]
#[derive(Debug)]
struct Args {
    time: bool,
    day: u8,
    part: u8,
    verbosity_level: u8,
}