use rand::{seq::SliceRandom, Rng};

use crate::town::TownDistance;
use crate::utils::order_ab;

pub struct Tour<'a> {
    town: &'a TownDistance,
    path: Vec<usize>,
    total_dist: f64,
}

impl<'a> Tour<'a> {
    pub fn new(town: &'a TownDistance, path: Vec<usize>) -> Tour<'a> {
        assert_eq!(town.len(), path.len());
        let mut total_dist = 0.0;
        for i in 1..path.len() {
            total_dist += town.dist(path[i - 1], path[i]);
        }
        total_dist += town.dist(path[path.len() - 1], path[0]);
        Tour {
            town,
            path,
            total_dist,
        }
    }
    pub fn with_random<R: Rng>(town: &'a TownDistance, r: &mut R) -> Tour<'a> {
        assert!(town.len() >= 1);
        let mut path = (0..town.len()).collect::<Vec<_>>();
        path.shuffle(r);
        Self::new(town, path)
    }

    pub fn total_dist(&self) -> f64 {
        self.total_dist
    }

    pub fn try_2opt(&self, a: usize, b: usize) -> f64 {
        let (a, b) = order_ab(a, b);
        let before = self
            .town
            .dist(self.path[a], self.path[(a + 1) % self.path.len()])
            + self
                .town
                .dist(self.path[b], self.path[(b + 1) % self.path.len()]);
        let after = self.town.dist(self.path[a], self.path[b])
            + self.town.dist(
                self.path[(a + 1) % self.path.len()],
                self.path[(b + 1) % self.path.len()],
            );
        after - before
    }

    pub fn do_2opt(&mut self, a: usize, b: usize, delta: f64) {
        debug_assert_eq!(delta, self.try_2opt(a, b));
        let n = self.path.len();
        let (mut a, mut b) = order_ab(a, b);
        if (b - a) * 2 <= n {
            a += 1;
            while a < b {
                self.path.swap(a, b);
                a += 1;
                b -= 1;
            }
        } else {
            a += n;
            b += 1;
            while b < a {
                self.path.swap(a % n, b % n);
                a -= 1;
                b += 1;
            }
        }
        self.total_dist += delta;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::town::TownDistance;

    #[test]
    fn test_2opt() {
        let town_pos = [[0.0], [1.0], [2.0], [3.0], [4.0]];
        let town = TownDistance::with_l2(&town_pos);
        let mut tour = Tour::new(&town, vec![0, 1, 2, 3, 4]);
        assert_eq!(tour.total_dist(), 8.0);
        assert_eq!(tour.try_2opt(0, 1), 0.0);
        assert_eq!(tour.try_2opt(0, 2), 2.0);
        assert_eq!(tour.try_2opt(0, 3), 4.0);
        assert_eq!(tour.try_2opt(2, 0), 2.0);
        assert_eq!(tour.try_2opt(0, 4), 0.0);
        assert_eq!(tour.try_2opt(1, 4), 0.0);
        tour.do_2opt(0, 2, 2.0);
        assert_eq!(tour.path, vec![0, 2, 1, 3, 4]);
        tour.do_2opt(4, 1, tour.try_2opt(1, 4));
        assert_eq!(tour.path, vec![2, 0, 1, 3, 4]);
    }
}
