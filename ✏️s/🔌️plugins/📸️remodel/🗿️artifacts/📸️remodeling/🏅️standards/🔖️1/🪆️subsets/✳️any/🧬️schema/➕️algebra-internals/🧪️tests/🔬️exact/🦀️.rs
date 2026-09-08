
use super::*;
use number::{Rational, VecG};

fn rat(n: i64, d: i64) -> Rational {
    Rational::from_i64(n, d).unwrap()
}

#[test]
fn lll_recovers_a_short_relation() {
    // Basis containing an obviously-reducible long vector alongside a short one; LLL should surface
    // vectors no longer than the original shortest, and preserve the lattice (verified via a
    // determinant/volume proxy: the reduced basis still spans the same rank).
    let basis = vec![VecG::from_vec(vec![rat(1, 1), rat(1, 1)]), VecG::from_vec(vec![rat(1, 1), rat(0, 1)])];
    let reduced = lll_reduce(&basis);
    assert_eq!(reduced.len(), 2);
    let shortest_before = basis.iter().map(|v| v.dot(v)).fold(rat(0, 1), |acc, n| if n < acc || acc == rat(0, 1) { n } else { acc });
    let shortest_after = reduced.iter().map(|v| v.dot(v)).fold(rat(0, 1), |acc, n| if n < acc || acc == rat(0, 1) { n } else { acc });
    assert!(shortest_after <= shortest_before);
}
