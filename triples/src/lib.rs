pub mod berggren;
pub mod euclid;
pub mod utils;

pub trait TripleGenerator: Iterator<Item = (usize, usize, usize)> {}

impl<T> TripleGenerator for T where T: Iterator<Item = (usize, usize, usize)> {}

pub enum Method {
    Euclid,
    Berggren,
}

pub enum TripleIter {
    Euclid(euclid::EuclidIter),
    Berggren(berggren::BerggrenIter),
}

impl TripleIter {
    pub fn new(limit: usize, method: Method) -> Self {
        match method {
            Method::Euclid => TripleIter::Euclid(euclid::EuclidIter::new(limit)),
            Method::Berggren => TripleIter::Berggren(berggren::BerggrenIter::new(limit)),
        }
    }
}

impl Iterator for TripleIter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TripleIter::Euclid(it) => it.next(),
            TripleIter::Berggren(it) => it.next(),
        }
    }
}

pub fn generate(
    limit: usize,
    method: Method,
    count: &mut usize,
    buf: Option<&mut impl std::io::Write>,
) {
    match method {
        Method::Euclid => euclid::generate(limit, count, buf),
        Method::Berggren => berggren::generate(limit, count, buf, berggren::INITIAL_TRIPLE),
    }
}

pub fn triples(limit: usize, method: Method) -> impl TripleGenerator {
    TripleIter::new(limit, method)
}

#[cfg(test)]
mod tests {
    use crate::{berggren, euclid};

    use super::utils::gcd;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(270, 192), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(3421, 1234), 1);
    }

    use super::berggren::generate as berggren_triples;
    use super::euclid::generate as euclid_triples;
    use std::io::Cursor;

    const LIMITS: [usize; 15] = [
        10, 50, 100, 500, 1_000,
        2_000, 5_000, 10_000, 20_000,
        50_000, 75_000, 100_000,
        200_000, 500_000, 1_000_000
    ];

    #[test]
    fn test_triples_count() {
        for &limit in &LIMITS {
            let mut count_euclid = 0;
            let mut count_berggren = 0;
            euclid_triples(limit, &mut count_euclid, None::<&mut Cursor<Vec<u8>>>);
            berggren_triples(
                limit,
                &mut count_berggren,
                None::<&mut Cursor<Vec<u8>>>,
                berggren::INITIAL_TRIPLE,
            );
            assert_eq!(count_euclid, count_berggren);
        }
    }

    #[test]
    fn test_triples_no_output() {
        for &limit in &LIMITS {
            let mut count_euclid = 0;
            let mut count_berggren = 0;
            euclid_triples(limit, &mut count_euclid, None::<&mut Cursor<Vec<u8>>>);
            berggren_triples(
                limit,
                &mut count_berggren,
                None::<&mut Cursor<Vec<u8>>>,
                berggren::INITIAL_TRIPLE,
            );
            assert_eq!(count_euclid, count_berggren);
        }
    }

    #[test]
    fn test_euclid_iter() {
        for &limit in &LIMITS {
            let mut count = 0;
            for (_a, _b, _c) in euclid::EuclidIter::new(limit) {
                count += 1;
            }
            let mut expected_count = 0;
            euclid_triples(limit, &mut expected_count, None::<&mut Cursor<Vec<u8>>>);
            assert_eq!(count, expected_count);
        }
    }

    #[test]
    fn test_berggren_iter() {
        for &limit in &LIMITS {
            let mut count = 0;
            for (_a, _b, _c) in berggren::BerggrenIter::new(limit) {
                count += 1;
            }
            let mut expected_count = 0;
            berggren_triples(
                limit,
                &mut expected_count,
                None::<&mut Cursor<Vec<u8>>>,
                berggren::INITIAL_TRIPLE,
            );
            assert_eq!(count, expected_count);
        }
    }

    #[test]
    fn test_berggren_iter_from_mat() {
        let mat = berggren::MAT_PRICE;
        for &limit in &LIMITS {
            let mut count = 0;
            for (_a, _b, _c) in berggren::BerggrenIter::from_mat(limit, mat) {
                count += 1;
            }
            let mut expected_count = 0;
            berggren_triples(
                limit,
                &mut expected_count,
                None::<&mut Cursor<Vec<u8>>>,
                berggren::INITIAL_TRIPLE,
            );
            assert_eq!(count, expected_count);
        }
    }

    #[test]
    fn test_iterators_agree() {
        for &limit in &LIMITS {
            let mut euclid_count = 0;
            for (_a, _b, _c) in euclid::EuclidIter::new(limit) {
                euclid_count += 1;
            }

            let mut berggren_count = 0;
            for (_a, _b, _c) in berggren::BerggrenIter::new(limit) {
                berggren_count += 1;
            }

            assert_eq!(euclid_count, berggren_count);
        }
    }

    #[test]
    fn test_triple_iter_enum() {
        for &limit in &LIMITS {
            let mut euclid_count = 0;
            let mut berggren_count = 0;
            let mut euclid_iter = super::TripleIter::new(limit, super::Method::Euclid);
            let mut berggren_iter = super::TripleIter::new(limit, super::Method::Berggren);
            while let Some((_a, _b, _c)) = euclid_iter.next() {
                euclid_count += 1;
            }
            while let Some((_a, _b, _c)) = berggren_iter.next() {
                berggren_count += 1;
            }
            assert_eq!(euclid_count, berggren_count);
        }
    }

    #[test]
    fn test_triples_function() {
        for &limit in &LIMITS {
            let mut euclid_count = 0;
            let mut berggren_count = 0;
            let mut euclid_iter = super::triples(limit, super::Method::Euclid);
            let mut berggren_iter = super::triples(limit, super::Method::Berggren);
            while let Some((_a, _b, _c)) = euclid_iter.next() {
                euclid_count += 1;
            }
            while let Some((_a, _b, _c)) = berggren_iter.next() {
                berggren_count += 1;
            }
            assert_eq!(euclid_count, berggren_count);
        }
    }

    #[test]
    fn test_generate_function() {
        for &limit in &LIMITS {
            let mut euclid_count = 0;
            let mut berggren_count = 0;
            super::generate(
                limit,
                super::Method::Euclid,
                &mut euclid_count,
                None::<&mut Cursor<Vec<u8>>>,
            );
            super::generate(
                limit,
                super::Method::Berggren,
                &mut berggren_count,
                None::<&mut Cursor<Vec<u8>>>,
            );
            assert_eq!(euclid_count, berggren_count);
        }
    }
}
