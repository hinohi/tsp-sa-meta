use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use rand_pcg::Mcg128Xsl64;
use tsp_sa_meta::{DistType, MetropolisPow, Tour, TownDistance, Transition};

fn do_mc(town: &TownDistance, rng: &Mcg128Xsl64) {
    let mut rng = rng.clone();
    let mut tour = Tour::with_random(&town, &mut rng);
    let iter_count = 10000;
    let mut mc = MetropolisPow::new(10.0, 0.01, 1.0);
    let mut total_dist = tour.get_total_dist();
    let mut best = total_dist;
    mc.set_max_iteration(iter_count);
    for _ in 0..iter_count {
        let a = rng.gen_range(0..town.len());
        let b = rng.gen_range(0..town.len());
        let delta = tour.try_2opt(a, b);
        if mc.trans(&mut rng, delta) {
            tour.do_2opt(a, b);
            total_dist += delta;
        }
        if total_dist < best {
            total_dist = tour.get_total_dist();
            best = total_dist;
        }
    }
}

fn towns200(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    let town = TownDistance::from_rng(2, 200, 100.0, DistType::L2, &mut rng);
    c.bench_function("towns200", |b| b.iter(|| do_mc(&town, &rng)));
}

fn towns500(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    let town = TownDistance::from_rng(2, 500, 100.0, DistType::L2, &mut rng);
    c.bench_function("towns500", |b| b.iter(|| do_mc(&town, &rng)));
}

fn towns1000(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    let town = TownDistance::from_rng(2, 1000, 100.0, DistType::L2, &mut rng);
    c.bench_function("towns1000", |b| b.iter(|| do_mc(&town, &rng)));
}

fn towns2000(c: &mut Criterion) {
    let mut rng = Mcg128Xsl64::new(1);
    let town = TownDistance::from_rng(2, 2000, 100.0, DistType::L2, &mut rng);
    c.bench_function("towns2000", |b| b.iter(|| do_mc(&town, &rng)));
}

criterion_group!(benches, towns200, towns500, towns1000, towns2000);
criterion_main!(benches);
