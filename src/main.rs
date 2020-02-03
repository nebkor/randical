use std::process::exit;

use clap::{App, Arg, ArgMatches};

use rand::prelude::*;

const VERSION: &str = "1.61";

const DEFAULT_NUM_VALS: usize = 1;

const BUEL_TRUE: &str = "Here.";
const BUEL_FALSE: &str = "Um, he's sick. My best friend's sister's boyfriend's brother's girlfriend heard from this guy who knows this kid who's going with the girl who saw Ferris pass out at 31 Flavors last night. I guess it's pretty serious.";

const BULE_TRUE: &str = "true";
const BULE_FALSE: &str = "false";

fn get_args() -> ArgMatches<'static> {
    App::new("Radical Random Value Generator")
        .version(VERSION)
        .about("Generates arbitrary numbers of uniformly distributed random values.")
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
                .help("Type of random value to print. Defaults to 'bool'.\nPossible values are 'b'ool, 'f'loat64, 'u'nsigned64, and 's'igned64"),
        )
        .arg(
            Arg::with_name("BULE")
                .long("bule")
                .takes_value(false)
                .conflicts_with("TYPE")
                .conflicts_with("BUEL")
                .help(&format!("Prints either '{}' or '{}', with equal probability. Not compatible with `-t` or `--buel`.",
                               BULE_TRUE, BULE_FALSE))
        )
        .arg(
            Arg::with_name("BUEL")
                .long("buel")
                .takes_value(false)
                .conflicts_with("TYPE")
                .conflicts_with("BULE")
                .help(&format!("Prints either '{}' or '{}', with equal probability. Not compatible with `-t` or `--bule`.",
                               BUEL_TRUE, BUEL_FALSE))
        )

        .arg(
            Arg::with_name("EXIT")
                .short("e")
                .long("exit")
                .takes_value(false)
                .help("With equal probability, exit with either status 0, like /bin/true, or status 1, like /bin/false. Technically compatible with all other options, but exit status will have no relation to any generated output. Sets default number of values to print to 0.")
        )
        .get_matches()
}

fn print_bool<'a>(b: &'a mut ThreadRng, args: &'a ArgMatches) {
    let t: &str;
    match b.gen() {
        true => {
            if args.is_present("BULE") {
                t = BULE_TRUE;
            } else if args.is_present("BUEL") {
                t = BUEL_TRUE;
            } else {
                t = "Radical!";
            }
        }
        false => {
            if args.is_present("BULE") {
                t = BULE_FALSE;
            } else if args.is_present("BUEL") {
                t = BUEL_FALSE;
            } else {
                t = "Bogus.";
            }
        }
    }
    println!("{}", t);
}

fn get_generator<'a>(args: &'a ArgMatches) -> Box<(dyn FnMut() + 'a)> {
    let mut rng = thread_rng();

    match args.value_of("TYPE").unwrap_or("b") {
        "f" => Box::new(move || println!("{}", rng.gen::<f64>())),
        "u" => Box::new(move || println!("{}", rng.gen::<u64>())),
        "s" => Box::new(move || println!("{}", rng.gen::<i64>())),
        "b" => Box::new(move || print_bool(&mut rng, args)),
        _ => panic!(),
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
