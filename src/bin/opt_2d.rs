use std::{
    fs::File,
    io::{BufWriter, Write},
};

use clap::Parser;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use tsp_sa_meta::{DistType, MetropolisPow, Tour, TownDistance, Transition};

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
    temp_max: f64,
    /// the end time, min temperature
    #[clap(short = 'm', long)]
    temp_min: f64,
    /// 1 / (1 + x^e)
    #[clap(short, long)]
    exponent: f64,
    /// swap or 2opt
    #[clap(short = 'w', long, default_value_t = 0.0)]
    swap_rate: f64,
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
    let mut town_pos = Vec::with_capacity(args.towns);
    for _ in 0..args.towns {
        town_pos.push([
            random.gen_range(0.0..args.size),
            random.gen_range(0.0..args.size),
        ]);
    }
    let town = TownDistance::new(&town_pos, args.dist);
    let mut tour = Tour::with_random(&town, &mut random);

    let iter_count = args.iter_count * args.towns as u64;
    let mut mc = MetropolisPow::new(args.temp_max, args.temp_min, args.exponent);
    mc.set_max_iteration(iter_count);
    let mut total_dist = tour.get_total_dist();
    let mut best = total_dist;
    for _ in 0..iter_count {
        let a = random.gen_range(0..args.towns);
        let b = random.gen_range(0..args.towns);
        if random.gen_bool(args.swap_rate) {
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
    if let Some(debug) = args.debug_out {
        let mut f = BufWriter::new(File::create(debug).unwrap());
        let path = tour.get_path();
        for i in 1..args.towns {
            let start = town_pos[path[i - 1]];
            let end = town_pos[path[i]];
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
        let start = town_pos[path[args.towns - 1]];
        let end = town_pos[path[0]];
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
