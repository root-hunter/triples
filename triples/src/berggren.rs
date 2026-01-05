use std::io::Write;

const MAT: [[isize; 9]; 3] = [
    [1, -2, 2, 2, -1, 2, 2, -2, 3],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [-1, 2, 2, -2, 1, 2, -2, 2, 3],
];

pub const INITIAL_TRIPLE: [usize; 3] = [3, 4, 5];

pub fn triples(
    limit: usize,
    count: &mut usize,
    mut buf: Option<&mut impl Write>,
    sol: [usize; 3],
) {
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
            res[j] = MAT[i][3 * j] * (sol[0] as isize)
                + MAT[i][3 * j + 1] * (sol[1] as isize)
                + MAT[i][3 * j + 2] * (sol[2] as isize);
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
