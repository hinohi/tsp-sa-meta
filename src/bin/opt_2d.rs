use std::{
    fs::File,
    io::{BufWriter, Write},
};

use clap::Parser;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use tsp_sa_meta::{metropolis, DistType, PowSchedule, Schedule, Tour, TownDistance};

#[derive(Debug, Parser)]
struct Args {
    /// random number's seed
    #[clap(short, long)]
    seed: u128,
    /// the number of town
    #[clap(short, long)]
    towns: usize,
    /// box size
    #[clap(short = 'l', long)]
    size: f64,
    /// distance definition
    #[clap(short, long, default_value_t = DistType::L2)]
    dist: DistType,
    /// the start time, max temperature
    #[clap(short = 'M', long)]
    temp_max_factor: f64,
    /// the end time, min temperature
    #[clap(short = 'm', long)]
    temp_min_factor: f64,
    /// 1 / (1 + x^e)
    #[clap(short, long)]
    exponent: f64,
    /// iter count
    #[clap(short = 'c', long, default_value_t = 10000)]
    iter_count: u64,
    /// dump path for debug
    #[clap(short = 'o', long)]
    debug_out: Option<String>,
}

fn main() {
    let args: Args = Args::parse();
    let mut random = Mcg128Xsl64::new(args.seed);
    let town = TownDistance::from_rng(2, args.towns, args.size, args.dist, &mut random);
    let mut tour = Tour::with_random(&town, &mut random);

    let iter_count = args.iter_count * args.towns as u64;
    let avg_dist = town.avg_dist();
    let mut mc = PowSchedule::new(
        avg_dist * args.temp_max_factor,
        avg_dist * args.temp_min_factor,
        args.exponent,
    );
    mc.set_max_iteration(iter_count);
    let mut total_dist = tour.get_total_dist();
    let mut best = total_dist;
    for _ in 0..iter_count {
        let a = random.gen_range(0..args.towns);
        let b = random.gen_range(0..args.towns);
        let delta = tour.try_2opt(a, b);
        let t = mc.get_temperature();
        if metropolis(&mut random, t, delta) {
            tour.do_2opt(a, b);
            total_dist += delta;
        }
        if total_dist < best {
            total_dist = tour.get_total_dist();
            best = total_dist;
        }
    }
    println!("{}", best);
    if let Some(debug) = args.debug_out {
        let mut f = BufWriter::new(File::create(debug).unwrap());
        let path = tour.get_path();
        for i in 1..args.towns {
            let start = &town.towns[path[i - 1]];
            let end = &town.towns[path[i]];
            writeln!(
                f,
                "{} {} {} {}",
                start[0],
                start[1],
                end[0] - start[0],
                end[1] - start[1]
            )
            .unwrap();
        }
        let start = &town.towns[path[args.towns - 1]];
        let end = &town.towns[path[0]];
        writeln!(
            f,
            "{} {} {} {}",
            start[0],
            start[1],
            end[0] - start[0],
            end[1] - start[1]
        )
        .unwrap();
    }
}
