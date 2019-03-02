use crate::utils::order_ab;

pub struct TownDistance {
    num: usize,
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
    let mut s = 0.0;
    for (aa, bb) in a.iter().zip(b.iter()) {
        let t = aa - bb;
        s += t * t;
    }
    s.sqrt()
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

impl TownDistance {
    pub fn len(&self) -> usize {
        self.num
    }
    pub fn dist(&self, a: usize, b: usize) -> f64 {
        let (a, b) = order_ab(a, b);
        self.distance[b * (b + 1) / 2 + a]
    }

    pub fn with_l1<T>(towns: &[T]) -> TownDistance
    where
        T: AsRef<[f64]>,
    {
        let mut distance = Vec::with_capacity(towns.len() * (towns.len() + 1) / 2);
        for (i, a) in towns.iter().enumerate() {
            for b in towns.iter().take(i + 1) {
                distance.push(l1(a.as_ref(), b.as_ref()));
            }
        }
        TownDistance {
            num: towns.len(),
            distance,
        }
    }

    pub fn with_l2<T>(towns: &[T]) -> TownDistance
    where
        T: AsRef<[f64]>,
    {
        let mut distance = Vec::with_capacity(towns.len() * (towns.len() + 1) / 2);
        for (i, a) in towns.iter().enumerate() {
            for b in towns.iter().take(i + 1) {
                distance.push(l2(a.as_ref(), b.as_ref()));
            }
        }
        TownDistance {
            num: towns.len(),
            distance,
        }
    }

    pub fn with_l2_sq<T>(towns: &[T]) -> TownDistance
    where
        T: AsRef<[f64]>,
    {
        let mut distance = Vec::with_capacity(towns.len() * (towns.len() + 1) / 2);
        for (i, a) in towns.iter().enumerate() {
            for b in towns.iter().take(i + 1) {
                distance.push(l2_sq(a.as_ref(), b.as_ref()));
            }
        }
        TownDistance {
            num: towns.len(),
            distance,
        }
    }
    pub fn with_l_inf<T>(towns: &[T]) -> TownDistance
    where
        T: AsRef<[f64]>,
    {
        let mut distance = Vec::with_capacity(towns.len() * (towns.len() + 1) / 2);
        for (i, a) in towns.iter().enumerate() {
            for b in towns.iter().take(i + 1) {
                distance.push(l_inf(a.as_ref(), b.as_ref()));
            }
        }
        TownDistance {
            num: towns.len(),
            distance,
        }
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
        let towns = vec![[0.0, 0.0], [0.0, 3.0], [4.0, 0.0], [3.0, 4.0]];
        let cost = [
            [0.0, 3.0, 4.0, 5.0],
            [3.0, 0.0, 5.0, 10f64.sqrt()],
            [4.0, 5.0, 0.0, 17f64.sqrt()],
            [5.0, 10f64.sqrt(), 17f64.sqrt(), 0.0],
        ];
        let dist = TownDistance::with_l2(&towns);
        assert_eq!(dist.len(), towns.len());
        for i in 0..towns.len() {
            for j in 0..towns.len() {
                assert_eq!(dist.dist(i, j), cost[i][j]);
            }
        }
    }
}
