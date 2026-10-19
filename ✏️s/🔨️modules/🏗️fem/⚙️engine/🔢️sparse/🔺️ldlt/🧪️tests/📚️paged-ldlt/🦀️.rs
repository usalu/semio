use super::*;
use semio_s_fem_sparse::{Coo, NumericalBatchEnd, numerical_batch_step, numerical_batch_close, numerical_close_demand, numerical_close_original, numerical_ladder_turn};
use semio_s_fem_test_support::TEST_GRANT;
use semio_s_fem_test_support::{OriginalNumericalFixture, close_grant};

#[test]
fn paged_ldlt_crosses_original_owner_and_checkpoint_boundaries() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📚️paged-ldlt/🔣️.json")).unwrap();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let n = case["order"].as_u64().unwrap() as usize;
        let mut coo = Coo::new(n);
        for row in 0..n {
            coo.add(row, row, case["diagonal"].as_f64().unwrap());
            if row + 1 < n { coo.add(row, row + 1, case["offDiagonal"].as_f64().unwrap()); }
        }
        let operation = Operation::new(semio_framework_job::OperationId(n as u64 + 80100), semio_framework_job::RevisionId(0), semio_framework_job::Generation(1), 0);
        let mut job = LdltJob::new(operation, coo.to_csc_sym_upper(), 1);
        let mut original = OriginalNumericalFixture::new(operation);
        let mut complete = false;
        for _ in 0..20_000_000 {
            match original.turn(TEST_GRANT, 64, u64::MAX, |cx| numerical_batch_step(&mut job, cx)).expect("same original native batch") {
                NumericalBatchEnd::Complete => { complete = true; break; },
                NumericalBatchEnd::Continue => {},
                _ => panic!("{} factorization refused original paged input", case["id"]),
            }
        }
        assert!(complete, "{} paged factor completes", case["id"]);
        let mut transferred = None;
        let admitted = original.turn(TEST_GRANT, 1, u64::MAX, |cx| {
            if let Some((factor, progress)) = job.take_factor(cx.retained_grant()) {
                transferred = Some(factor);
                cx.consume_retained(progress)?;
                Ok(true)
            } else { Ok(false) }
        }).expect("original factor handoff receipt");
        assert!(admitted);
        let mut factor = transferred.expect("original factor transfer");
        assert_eq!(factor.order(), n);
        let solution = factor.solve(&VecD::from_vec(vec![1.0; n]));
        for sample in case["samples"].as_array().unwrap() {
            let row = sample["row"].as_u64().unwrap() as usize;
            assert!((solution.get(row) - sample["solution"].as_f64().unwrap()).abs() <= tolerance);
        }
        for column in 0..n {
            assert!(factor.diagonal(column).unwrap() > 0.0);
            assert_eq!(factor.column_entries_len(column), Some(usize::from(column + 1 < n)));
            if column + 1 < n { assert_eq!(factor.column_entry(column, 0).unwrap().0 as usize, column + 1); }
        }
        while !job.terminal_is_empty() {
            let grant = close_grant(numerical_close_demand(&job).unwrap());
            original.turn(grant, 1, u64::MAX, |cx| cx.with_original_retirement(|cx| numerical_batch_close(&mut job, cx))).expect("same original native batch close");
        }
        let mut closed = false;
        for _ in 0..100_000 {
            if factor.terminal_is_empty() { closed = true; break; }
            let demand = factor.retirement_demands().unwrap();
            original.turn(close_grant(demand), 1, u64::MAX, |cx| cx.with_original_retirement(|cx| numerical_close_original(cx, &mut factor, demand, |factor, grant| Ok(numerical_ladder_turn(factor.close_step(grant.maximum_release_bytes))), |factor| factor.terminal_is_empty()))).expect("same original factor close");
        }
        assert!(closed && factor.terminal_is_empty());
        original.finish();
        println!("[DEBUG] paged LDLT order{n} matches independent sparse LU and keeps original equation/column correspondence");
    }
}
