use std::env;
use std::f64;
use std::process;

use getopts::Options;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use tsp_sa_meta::DistType;
use tsp_sa_meta::{MetropolisPow, Tour, TownDistance, Transition};

#[derive(Debug)]
struct Args {
    seed: u128,

    towns: usize,
    size: f64,
    dist: DistType,

    temp_max: f64,
    temp_min: f64,
    exponent: f64,
    swap_rate: f64,
    iter_count: u64,
}

fn print_usage(program: &str, opts: &Options) -> ! {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn parse_args() -> Args {
    let args: Vec<_> = env::args().collect();
    let mut opt = Options::new();
    opt.optflag("h", "help", "print this help menu");
    opt.optopt("s", "seed", "random number's seed(required)", "SEED");
    opt.optopt("t", "towns", "the number of town(required)", "TOWNS");
    opt.optopt("l", "size", "box size(required)", "SIZE");
    opt.optopt("d", "dist", "distance definition(default L2)", "DIST");
    opt.optopt("M", "temp-max", "max temperature(required)", "T0");
    opt.optopt("m", "temp-min", "min temperature(required)", "T1");
    opt.optopt("e", "exponent", "1 / (1 + x^e) (required)", "EXP");
    opt.optopt("w", "swap-rate", "swap or 2opt (default 0.0)", "SWAP");
    opt.optopt("c", "count", "iter count (default 10^4)", "COUNT");
    let m = opt
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    if m.opt_present("h") {
        print_usage(&args[0], &opt);
    }
    if !m.free.is_empty() {
        print_usage(&args[0], &opt);
    }
    Args {
        seed: m
            .opt_str("seed")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<u128>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        towns: m
            .opt_str("towns")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<usize>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        size: m
            .opt_str("size")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        dist: m
            .opt_str("dist")
            .unwrap_or_else(|| "L2".to_string())
            .parse::<DistType>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        temp_max: m
            .opt_str("temp-max")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        temp_min: m
            .opt_str("temp-min")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        exponent: m
            .opt_str("exponent")
            .unwrap_or_else(|| print_usage(&args[0], &opt))
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        swap_rate: m
            .opt_str("swap-rate")
            .unwrap_or_else(|| "0.0".to_string())
            .parse::<f64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
        iter_count: m
            .opt_str("count")
            .unwrap_or_else(|| "10000".to_string())
            .parse::<u64>()
            .unwrap_or_else(|f| panic!(f.to_string())),
    }
}

fn main() {
    let args = parse_args();
    let mut random = Mcg128Xsl64::new(args.seed);
    let mut town_pos = Vec::with_capacity(args.towns);
    for _ in 0..args.towns {
        town_pos.push([
            random.gen_range(0.0, args.size),
            random.gen_range(0.0, args.size),
        ]);
    }
    let town = TownDistance::new(&town_pos, args.dist);
    let mut tour = Tour::with_random(&town, &mut random);

    let iter_count = args.iter_count * args.towns as u64;
    let mut mc = MetropolisPow::new(args.temp_max, args.temp_min, args.exponent);
    mc.set_max_iteration(iter_count);
    let mut total_dist = tour.get_total_dist();
    let mut best = total_dist;
    println!("{}", best);
    for _ in 0..iter_count {
        let a = random.gen_range(0, args.towns);
        let b = random.gen_range(0, args.towns);
        if random.gen_range(0.0, 1.0) < args.swap_rate {
            let delta = tour.try_swap(a, b);
            if mc.trans(&mut random, delta) {
                tour.do_swap(a, b);
                total_dist += delta;
            }
        } else {
            let delta = tour.try_2opt(a, b);
            if mc.trans(&mut random, delta) {
                tour.do_2opt(a, b);
                total_dist += delta;
            }
        }
        if total_dist < best {
            total_dist = tour.get_total_dist();
            best = total_dist;
        }
    }
    println!("{}", best);
}
