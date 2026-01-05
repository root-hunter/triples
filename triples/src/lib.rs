pub mod berggren;
pub mod euclid;

#[cfg(test)]
mod tests {
    use crate::berggren;

    use super::euclid::gcd;
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

    use super::berggren::triples as berggren_triples;
    use super::euclid::triples as euclid_triples;
    use std::io::Cursor;

    #[test]
    fn test_triples_count() {
        let limit = 100;
        let mut count_euclid = 0;
        let mut count_berggren = 0;
        euclid_triples(limit, &mut count_euclid, Some(&mut Cursor::new(Vec::new())));
        berggren_triples(
            limit,
            &mut count_berggren,
            Some(&mut Cursor::new(Vec::new())),
            berggren::INITIAL_TRIPLE,
        );
        assert_eq!(count_euclid, count_berggren);
    }

    #[test]
    fn test_triples_no_output() {
        let limit = 1000;
        let mut count_euclid = 0;
        let mut count_berggren = 0;
        euclid_triples(limit, &mut count_euclid, None::<&mut Cursor<Vec<u8>>>);
        berggren_triples(limit, &mut count_berggren, None::<&mut Cursor<Vec<u8>>>, berggren::INITIAL_TRIPLE);
        assert_eq!(count_euclid, count_berggren);
    }
}
