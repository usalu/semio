//! 🔄️ One subspace iteration forms the shifted operator image `K⁻¹Bx + σx` and its stiffness image `Bx + σKx` like NumPy.
use super::*;
use semio_s_fem_sparse::Coo;
use semio_s_fem_sparse_ldlt::ldlt_factor;
use semio_s_fem_test_support::test_operation;

fn dense(rows: &serde_json::Value) -> Coo {
    let rows = rows.as_array().unwrap();
    let mut coo = Coo::new(rows.len());
    for (i, values) in rows.iter().enumerate() {
        for (j, value) in values.as_array().unwrap().iter().enumerate() {
            let value = value.as_f64().unwrap();
            if value != 0.0 { coo.add(i, j, value); }
        }
    }
    coo
}

#[test]
fn shift_operator_image_matches_the_independent_numpy_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔄️shift-operator/🔣️.json")).unwrap();
    let tolerance = corpus["relativeTolerance"].as_f64().unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let (n, m, shift) = (case["order"].as_u64().unwrap() as usize, case["columns"].as_u64().unwrap() as usize, case["shift"].as_f64().unwrap());
        assert!(m < n, "{name}: a strict subspace takes the shifted iteration");
        let factor = ldlt_factor(&dense(&case["stiffness"]).to_csc_sym_upper()).unwrap();
        let mut state = SubspaceCheckpoint::new(test_operation(7), factor, dense(&case["right"]).to_csr(), n, m, 1000);
        state.m = m;
        state.reserve_mathematical();
        for (i, row) in case["x"].as_array().unwrap().iter().enumerate() {
            for (j, value) in row.as_array().unwrap().iter().enumerate() { state.x.set(i, j, value.as_f64().unwrap()); }
        }
        state.shift = Some(shift);
        state.reset_cursor(SubspaceStage::ApplyOperatorColumnRow);
        let mut steps = 0;
        while state.work.stage != SubspaceStage::OrthogonalizePairElement {
            state.advance_mathematical().unwrap_or_else(|()| panic!("{name}: stage {:?} refused", state.work.stage));
            steps += 1;
            assert!(steps < 200_000, "{name}: the operator image is bounded");
        }
        assert_eq!((state.work.first, state.work.second, state.work.third, state.work.phase, state.work.coefficient), (0, 0, 0, 0, 0.0), "{name}: the orthogonalization cursor is clean");
        for (label, owner, expected) in [("solved", &state.work.solved, &case["solved"]), ("rhs", &state.work.rhs, &case["rhs"])] {
            for (i, row) in expected.as_array().unwrap().iter().enumerate() {
                for (j, value) in row.as_array().unwrap().iter().enumerate() {
                    let value = value.as_f64().unwrap();
                    assert!((owner.get(i, j) - value).abs() <= tolerance * value.abs().max(1.0), "{name} {label}[{i}][{j}]: {} vs {value}", owner.get(i, j));
                }
            }
        }
        println!("[DEBUG] {name}: shifted operator image after {steps} bounded steps equals NumPy");
    }
}
