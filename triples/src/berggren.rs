use std::io::Write;

// Info on Berggren's method:
// https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples

pub const INITIAL_TRIPLE: [usize; 3] = [3, 4, 5];

pub const MAT_BERGGREN: [[isize; 9]; 3] = [
    [1, -2, 2, 2, -1, 2, 2, -2, 3],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [-1, 2, 2, -2, 1, 2, -2, 2, 3],
];

pub const MAT_PRICE: [[isize; 9]; 3] = [
    [2, 1, -1, -2, 2, 2, -2, 1, 3],
    [2, 1, 1, 2, -2, 2, 2, -1, 3],
    [2, -1, 1, 2, 2, 2, 2, 1, 3],
];

pub fn triples(limit: usize, count: &mut usize, mut buf: Option<&mut impl Write>, sol: [usize; 3]) {
    if sol[2] > limit {
        return;
    }

    // write to buffered output
    if let Some(out) = buf.as_deref_mut() {
        writeln!(out, "(BERGGREN) {}^2 + {}^2 = {}^2", sol[0], sol[1], sol[2]).unwrap();
    }
    *count += 1;

    for i in 0..3 {
        let mut res = [0isize; 3];
        for j in 0..3 {
            res[j] = MAT_BERGGREN[i][3 * j] * (sol[0] as isize)
                + MAT_BERGGREN[i][3 * j + 1] * (sol[1] as isize)
                + MAT_BERGGREN[i][3 * j + 2] * (sol[2] as isize);
        }
        let new_sol = [res[0] as usize, res[1] as usize, res[2] as usize];
        triples(limit, count, buf.as_deref_mut(), new_sol);
    }

    for i in 1..limit / sol[2] {
        let scaled_sol = [sol[0] * (i + 1), sol[1] * (i + 1), sol[2] * (i + 1)];
        // write to buffered output
        if let Some(out) = buf.as_deref_mut() {
            writeln!(
                out,
                "(BERGGREN) {}^2 + {}^2 = {}^2",
                scaled_sol[0], scaled_sol[1], scaled_sol[2]
            )
            .unwrap();
        }
        *count += 1;
    }
}

pub struct BerggrenIter {
    limit: usize,
    stack: Vec<[usize; 3]>,
    current: Option<[usize; 3]>,
    scale: usize,
    scale_max: usize,
    mat: [[isize; 9]; 3],
}

#[inline]
fn apply(mat: &[isize; 9], t: [usize; 3]) -> [usize; 3] {
    let (a, b, c) = (t[0] as isize, t[1] as isize, t[2] as isize);

    [
        (mat[0] * a + mat[1] * b + mat[2] * c) as usize,
        (mat[3] * a + mat[4] * b + mat[5] * c) as usize,
        (mat[6] * a + mat[7] * b + mat[8] * c) as usize,
    ]
}

impl BerggrenIter {
    pub fn new(limit: usize) -> Self {
        BerggrenIter {
            limit,
            stack: vec![INITIAL_TRIPLE],
            current: None,
            scale: 1,
            scale_max: 0,
            mat: MAT_BERGGREN,
        }
    }

    pub fn from_mat(limit: usize, mat: [[isize; 9]; 3]) -> Self {
        BerggrenIter {
            limit,
            stack: vec![INITIAL_TRIPLE],
            current: None,
            scale: 1,
            scale_max: 0,
            mat,
        }
    }
}

impl Iterator for BerggrenIter {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(t) = self.current {
                if self.scale <= self.scale_max {
                    let k = self.scale;
                    self.scale += 1;
                    return Some((t[0] * k, t[1] * k, t[2] * k));
                }
                self.current = None;
            }

            let t = self.stack.pop()?;
            if t[2] > self.limit {
                continue;
            }

            for i in (0..3).rev() {
                let child = apply(&self.mat[i], t);
                if child[2] <= self.limit {
                    self.stack.push(child);
                }
            }

            self.scale_max = self.limit / t[2];
            self.scale = 1;
            self.current = Some(t);
        }
    }
}