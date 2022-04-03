use std::{fmt, str::FromStr};

use rand::Rng;

use crate::utils::order_ab;

pub struct TownDistance {
    pub towns: Vec<Vec<f64>>,
    distance: Vec<f64>,
}

fn l1(a: &[f64], b: &[f64]) -> f64 {
    let mut s = 0.0;
    for (aa, bb) in a.iter().zip(b.iter()) {
        s += (aa - bb).abs();
    }
    s
}

fn l2(a: &[f64], b: &[f64]) -> f64 {
    l2_sq(a, b).sqrt()
}

fn l2_sq(a: &[f64], b: &[f64]) -> f64 {
    let mut s = 0.0;
    for (aa, bb) in a.iter().zip(b.iter()) {
        let t = aa - bb;
        s += t * t;
    }
    s
}

fn l_inf(a: &[f64], b: &[f64]) -> f64 {
    let mut s = 0.0;
    for (aa, bb) in a.iter().zip(b.iter()) {
        let t = (aa - bb).abs();
        if s < t {
            s = t;
        }
    }
    s
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DistType {
    L1,
    L2,
    L2Sq,
    LInf,
}

impl fmt::Display for DistType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DistType::*;
        f.write_str(match self {
            L1 => "L1",
            L2 => "L2",
            L2Sq => "Squared L2",
            LInf => "L infinity",
        })
    }
}

impl FromStr for DistType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        match s.as_ref() {
            "l1" => Ok(DistType::L1),
            "l2" => Ok(DistType::L2),
            "l2sq" | "l2_sq" => Ok(DistType::L2Sq),
            "linf" | "l_inf" => Ok(DistType::LInf),
            _ => Err(format!("unsupported type: {}", s)),
        }
    }
}

impl TownDistance {
    pub fn len(&self) -> usize {
        self.towns.len()
    }

    pub fn dist(&self, a: usize, b: usize) -> f64 {
        let (a, b) = order_ab(a, b);
        self.distance[b * (b + 1) / 2 + a]
    }

    pub fn new(towns: Vec<Vec<f64>>, dist_type: DistType) -> TownDistance {
        let mut distance = Vec::with_capacity(towns.len() * (towns.len() + 1) / 2);
        for (i, a) in towns.iter().enumerate() {
            let a = a.as_ref();
            for b in towns.iter().take(i + 1) {
                let b = b.as_ref();
                distance.push(match dist_type {
                    DistType::L1 => l1(a, b),
                    DistType::L2 => l2(a, b),
                    DistType::L2Sq => l2_sq(a, b),
                    DistType::LInf => l_inf(a, b),
                });
            }
        }
        TownDistance { towns, distance }
    }

    pub fn from_rng<R: Rng>(
        dim: usize,
        towns: usize,
        box_size: f64,
        dist_type: DistType,
        rng: &mut R,
    ) -> TownDistance {
        let mut t = Vec::with_capacity(towns);
        for _ in 0..towns {
            t.push(
                (0..dim)
                    .map(|_| rng.gen_range(0.0..box_size))
                    .collect::<Vec<f64>>(),
            );
        }
        TownDistance::new(t, dist_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l1() {
        assert_eq!(l1(&[0.0], &[0.0]), 0.0);
        assert_eq!(l1(&[1.0], &[-1.5]), 2.5);
        assert_eq!(l1(&[1.0, -1.0], &[1.0, 1.0]), 2.0);
        assert_eq!(l1(&[0.0, 3.0, 0.0], &[0.0, 0.0, -4.0]), 7.0);
    }

    #[test]
    fn test_l2() {
        assert_eq!(l2(&[0.0], &[0.0]), 0.0);
        assert_eq!(l2(&[1.0], &[-1.5]), 2.5);
        assert_eq!(l2(&[1.0, -1.0], &[1.0, 1.0]), 2.0);
        assert_eq!(l2(&[0.0, 3.0, 0.0], &[0.0, 0.0, -4.0]), 5.0);
    }

    #[test]
    fn test_l2_sq() {
        assert_eq!(l2_sq(&[0.0], &[0.0]), 0.0);
        assert_eq!(l2_sq(&[1.0], &[-1.5]), 2.5 * 2.5);
        assert_eq!(l2_sq(&[1.0, -1.0], &[1.0, 1.0]), 4.0);
        assert_eq!(l2_sq(&[0.0, 3.0, 0.0], &[0.0, 0.0, -4.0]), 25.0);
    }

    #[test]
    fn test_l_inf() {
        assert_eq!(l_inf(&[0.0], &[0.0]), 0.0);
        assert_eq!(l_inf(&[1.0], &[-1.5]), 2.5);
        assert_eq!(l_inf(&[1.0, -1.0], &[1.0, 1.0]), 2.0);
        assert_eq!(l_inf(&[0.0, 3.0, 0.0], &[0.0, 0.0, -4.0]), 4.0);
    }

    #[test]
    fn town_distance_l2() {
        let towns = vec![
            vec![0.0, 0.0],
            vec![0.0, 3.0],
            vec![4.0, 0.0],
            vec![3.0, 4.0],
        ];
        let cost = [
            [0.0, 3.0, 4.0, 5.0],
            [3.0, 0.0, 5.0, 10f64.sqrt()],
            [4.0, 5.0, 0.0, 17f64.sqrt()],
            [5.0, 10f64.sqrt(), 17f64.sqrt(), 0.0],
        ];
        let dist = TownDistance::new(towns.clone(), DistType::L2);
        assert_eq!(dist.len(), towns.len());
        for i in 0..towns.len() {
            for j in 0..towns.len() {
                assert_eq!(dist.dist(i, j), cost[i][j]);
            }
        }
    }
}
