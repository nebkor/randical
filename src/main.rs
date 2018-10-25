extern crate clap;
extern crate rand;

use std::process::exit;

use clap::{App, Arg, ArgMatches};

use rand::prelude::*;

const VERSION: &str = "1";
const DEFAULT_NUM_VALS: usize = 1;

fn get_args() -> ArgMatches<'static> {
    App::new("Radical Random Value Generator")
        .version(VERSION)
        .arg(
            Arg::with_name("NUM_VALS")
                .short("n")
                .long("num-vals")
                .takes_value(true)
                .help(&format!(
                    "Number of random values to print out. Defaults to {}.",
                    DEFAULT_NUM_VALS
                )),
        )
        .arg(
            Arg::with_name("TYPE")
                .short("t")
                .long("type")
                .takes_value(true)
                .help("Type of random value to print. Defaults to 'bool', with true represented as '1', and false as '0'.\nPossible values accepted are 'b'ool, 'f'loat64, 'u'nsigned64, and 's'igned64"),
        )
        .arg(
            Arg::with_name("EXIT")
                .short("e")
                .long("exit")
                .takes_value(false)
                .help("Randomly exit with either status 0, like /bin/true, or status 1, like /bin/false. Technically compatible with all other options, but doing so could obscure potential errors. Sets default number of values to print out to 0.")
        )
        .get_matches()
}

fn print_bool(b: &mut ThreadRng) {
    let t = if b.gen() { 1 } else { 0 };
    println!("{}", t);
}

fn get_generator(args: &ArgMatches) -> Box<dyn FnMut() -> ()> {
    let mut rng = thread_rng();

    match args.value_of("TYPE").unwrap_or("b") {
        "f" => Box::new(move || println!("{}", rng.gen::<f64>())),
        "u" => Box::new(move || println!("{}", rng.gen::<u64>())),
        "s" => Box::new(move || println!("{}", rng.gen::<i64>())),
        _ => Box::new(move || print_bool(&mut rng)),
    }
}

fn main() {
    let args = get_args();

    let do_exit: bool = args.is_present("EXIT");

    let num_vals: usize = if args.is_present("NUM_VALS") {
        args.value_of("NUM_VALS")
            .unwrap()
            .parse()
            .unwrap_or(DEFAULT_NUM_VALS)
    } else if do_exit {
        0
    } else {
        DEFAULT_NUM_VALS
    };

    let mut gen = get_generator(&args);

    for _ in 0..num_vals {
        gen();
    }

    if do_exit && rand::random() {
        exit(1)
    }
    exit(0)
}
