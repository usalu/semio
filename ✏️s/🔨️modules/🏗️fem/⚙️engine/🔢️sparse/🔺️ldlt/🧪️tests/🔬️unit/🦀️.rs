use super::*;
use semio_framework_job::StepBudget;
use semio_s_fem_sparse::*;
use semio_s_fem_test_support::{close_turn, checkpoint_payload, close_job, grant_of, payload_from_pages, seen_of, test_operation, Seen, TEST_GRANT};

#[test]
fn numerical_page_restore_close_preserves_ungranted_backing_and_retires_one_owner() {
    fn physical(state: Option<&LdltCheckpoint>) -> usize {
        state.map_or(0, |state| {
            state.row_lists.capacity() * size_of::<Vec<usize>>() + state.row_lists.iter().map(|owner| owner.capacity() * size_of::<usize>()).sum::<usize>()
                + state.l_cols.capacity() * size_of::<Vec<(u32, f64)>>() + state.l_cols.iter().map(|owner| owner.capacity() * size_of::<(u32, f64)>()).sum::<usize>()
        })
    }
    fn inner_owners(state: Option<&LdltCheckpoint>) -> usize {
        state.map_or(0, |state| state.row_lists.iter().filter(|owner| owner.capacity() != 0).count() + state.l_cols.iter().filter(|owner| owner.capacity() != 0).count())
    }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["closeCases"].as_array().unwrap() {
        let operation = test_operation(490);
        let mut job = LdltJob::new(operation, CscSym { n: 0, colptr: Vec::new(), rowind: Vec::new(), vals: Vec::new() }, 1);
        job.state.row_lists = vec![Vec::with_capacity(4)];
        job.state.l_cols = vec![Vec::with_capacity(4)];
        let next = job.state.row_lists[0].capacity() * size_of::<usize>();
        let pair = job.state.l_cols[0].capacity() * size_of::<(u32, f64)>();
        let grant = match row["grant"].as_str().unwrap() { "zero" => 0, "subexact" => next.min(pair) - 1, "exact" => next.max(pair), _ => unreachable!() };
        let mut restore = LdltRestoreCursor::new(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState));
        restore.payload = None;
        restore.state = Some(job.state);
        let before = physical(restore.state.as_ref());
        let first = restore.close_step(grant_of(1, grant));
        let after = physical(restore.state.as_ref());
        let retired = 2 - inner_owners(restore.state.as_ref());
        let reported = match first { Ok(InteractiveJobCloseStep::Pending { progress }) => progress.released_bytes, _ => 0 };
        let mut released = reported;
        let mut exact = before - after == reported && reported <= grant;
        for _ in 0..128 {
            let before = physical(restore.state.as_ref());
            let step = restore.close_step(grant_of(1, NUMERICAL_OWNER_PAGE_BYTES));
            let reported = match step { Ok(InteractiveJobCloseStep::Pending { progress }) => progress.released_bytes, _ => 0 };
            exact &= before - physical(restore.state.as_ref()) == reported;
            released += reported;
            if restore.terminal_is_empty() { break; }
        }
        observations.push((row["id"].clone(), retired, row["expectedRetiredOwners"].as_u64().unwrap() as usize, exact, before, released, restore.terminal_is_empty()));
    }
    for (id, retired, expected, exact, allocated, released, terminal) in observations {
        assert!(terminal, "{id} terminal");
        assert_eq!(retired, expected, "{id} one backing per granted opportunity");
        assert!(exact, "{id} reports only actually retired backing");
        assert_eq!(allocated, released, "{id} conservation");
    }
}

#[test]
fn numerical_page_ldlt_checkpoint_restores_maximum_admitted_matrix() {
    let n = LDLT_MAXIMUM_ORDER;
    let count = n * (n + 1) / 2;
    let mut colptr = Vec::with_capacity(n + 1);
    let mut rowind = Vec::with_capacity(count);
    let mut vals = Vec::with_capacity(count);
    colptr.push(0);
    for column in 0..n {
        for row in column..n {
            rowind.push(row as u32);
            vals.push(vals.len() as f64 * 0.25 - 100.0);
        }
        colptr.push(vals.len() as u32);
    }
    assert_eq!(vals.len(), count);
    let expected = (colptr.clone(), rowind.clone(), vals.clone());
    let operation = test_operation(489);
    let mut sequence = 0;
    let mut job = LdltJob::new(operation, CscSym { n, colptr, rowind, vals }, 1);
    job.state.checkpoint_due = true;
    let mut checkpoint = None;
    let mut write_fault = false;
    for _ in 0..100_000 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(job.step(&mut context)) {
            Seen::Checkpoint(pages) => { checkpoint = Some(pages); break; }
            Seen::Yield | Seen::Preview(_) => {}
            Seen::Fault(_) | Seen::Complete(_) | Seen::Cancelled => { write_fault = true; break; }
        }
    }
    let mut pages = Vec::new();
    let mut restore = checkpoint.map(|checkpoint_pages| {
        let payload = payload_from_pages(operation, JobPayloadStream::CheckpointState, &checkpoint_pages);
        for slot in 0..payload.page_count() {
            let page = parse_numerical_page(payload.page(slot).unwrap(), b"FEMLCP1\0").unwrap();
            if page.field == 3 {
                let overhead = if page.owner == 0 { 8 } else { 0 };
                pages.push(serde_json::json!({ "owner": page.owner, "item": page.item, "bytes": page.bytes.len() + NUMERICAL_CHECKPOINT_HEADER_BYTES, "scalars": (page.bytes.len() - overhead) / 8 }));
            }
        }
        LdltRestoreCursor::new(operation, payload)
    });
    let mut restored = None;
    let mut restore_fault = None;
    if let Some(restore) = restore.as_mut() {
        for _ in 0..100_000 {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            match restore.step(&mut context) {
                Ok(Some(value)) => { restored = Some(value); break; }
                Ok(None) => {}
                Err(fault) => { restore_fault = Some(fault); break; }
            }
        }
    }
    let actual = restored.as_ref().map(|value| (value.state.a.colptr.clone(), value.state.a.rowind.clone(), value.state.a.vals.clone()));
    for _ in 0..10_000 {
        if let Some(restore) = restore.as_mut() { let _ = restore.close_step(grant_of(1, NUMERICAL_OWNER_PAGE_BYTES)); }
        if let Some(restored) = restored.as_mut() { let _ = close_turn(restored, grant_of(1, NUMERICAL_OWNER_PAGE_BYTES)); }
        let _ = close_turn(&mut job, grant_of(1, NUMERICAL_OWNER_PAGE_BYTES));
        if restore.as_ref().is_none_or(LdltRestoreCursor::terminal_is_empty) && restored.as_ref().is_none_or(InteractiveJob::terminal_is_empty) && InteractiveJob::terminal_is_empty(&job) { break; }
    }
    let closed = restore.as_ref().is_none_or(LdltRestoreCursor::terminal_is_empty) && restored.as_ref().is_none_or(InteractiveJob::terminal_is_empty) && InteractiveJob::terminal_is_empty(&job);
    eprintln!("LDLT maximum owner checkpoint pages={pages:?}, write_fault={write_fault}, restore_fault={restore_fault:?}, restored={}, closed={closed}", actual.is_some());
    assert!(closed);
    assert!(!write_fault);
    assert_eq!(pages, vec![serde_json::json!({ "owner": 0, "item": 0, "bytes": NUMERICAL_CHECKPOINT_HEADER_BYTES + 8 + count * 8, "scalars": count })]);
    assert_eq!(restore_fault, None);
    assert_eq!(actual, Some(expected));
}

#[test]
fn ldlt_matches_dense_lu_on_random_spd() {
    let n = 8;
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
    let coo = Coo::graph_laplacian_plus_identity(n, &edges);
    let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
    let x_expected = VecD::from_vec((0..n).map(|i| (i as f64) * 0.5 + 1.0).collect());
    let dense = coo.to_dense();
    let b = dense.mul_vec(&x_expected);
    let x_ldlt = factor.solve(&b);
    let x_lu = dense.lu_solve(&b).expect("dense solvable");
    for i in 0..n {
        assert!((x_ldlt.get(i) - x_expected.get(i)).abs() < 1e-8);
        assert!((x_ldlt.get(i) - x_lu.get(i)).abs() < 1e-8);
    }
}

#[test]
fn ldlt_matches_dense_lu_on_1d_laplacian() {
    let n = 20;
    let mut coo = Coo::new(n);
    for i in 0..n {
        coo.add(i, i, 2.0);
        if i + 1 < n {
            coo.add(i, i + 1, -1.0);
            coo.add(i + 1, i, -1.0);
        }
    }
    let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
    let x_expected = VecD::from_vec((0..n).map(|i| ((i % 5) as f64) - 1.5).collect());
    let dense = coo.to_dense();
    let b = dense.mul_vec(&x_expected);
    let x_ldlt = factor.solve(&b);
    let x_lu = dense.lu_solve(&b).expect("dense solvable");
    for i in 0..n {
        assert!((x_ldlt.get(i) - x_lu.get(i)).abs() < 1e-8);
    }
}

#[test]
fn ldlt_solve_many_matches_solve_per_column() {
    let n = 8;
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
    let coo = Coo::graph_laplacian_plus_identity(n, &edges);
    let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("factors");
    let mut rhs = MatD::zeros(n, 3);
    for r in 0..n {
        rhs.set(r, 0, r as f64 + 1.0);
        rhs.set(r, 1, (n - r) as f64);
        rhs.set(r, 2, if r % 2 == 0 { 1.0 } else { -1.0 });
    }
    let combined = factor.solve_many(&rhs);
    for c in 0..3 {
        let col = VecD::from_vec((0..n).map(|r| rhs.get(r, c)).collect());
        let single = factor.solve(&col);
        for r in 0..n {
            assert!((combined.get(r, c) - single.get(r)).abs() < 1e-12);
        }
    }
}

#[test]
fn ldlt_reports_zero_pivot_on_singular_matrix() {
    let n = 5;
    let edges = [(0, 1), (1, 2), (2, 3)];
    let mut degree = vec![0usize; n];
    for &(u, v) in &edges {
        degree[u] += 1;
        degree[v] += 1;
    }
    let mut coo = Coo::new(n);
    for i in 0..4 {
        coo.add(i, i, degree[i] as f64 + 1.0);
    }
    for &(u, v) in &edges {
        coo.add(u, v, -1.0);
        coo.add(v, u, -1.0);
    }
    match ldlt_factor(&coo.to_csc_sym_upper()) {
        Err(SparseError::ZeroPivot { column }) => assert_eq!(column, 4),
        other => panic!("expected zero pivot error, got {other:?}"),
    }
}

#[test]
fn negative_pivot_count_counts_negative_diagonal_entries() {
    let mut coo = Coo::new(3);
    coo.add(0, 0, 1.0);
    coo.add(1, 1, -2.0);
    coo.add(2, 2, 3.0);
    let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("diagonal matrix factors trivially");
    assert_eq!(factor.negative_pivot_count(), 1);
}

fn restore_ldlt(operation: Operation, payload: RetainedJobPayload) -> Result<LdltJob, NumericalCheckpointFault> {
    let mut restore = LdltRestoreCursor::new(operation, payload);
    let mut sequence = 0;
    for _ in 0..200_000 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match restore.step(&mut context) {
            Ok(Some(job)) => return Ok(job),
            Ok(None) => {}
            Err(fault) => {
                while !restore.terminal_is_empty() {
                    let _ = restore.close_step(grant_of(1, usize::MAX));
                }
                return Err(fault);
            }
        }
    }
    Err(NumericalCheckpointFault::Truncated)
}

#[test]
fn ldlt_job_checkpoint_resume_matches_reference() {
    let n = 30;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let matrix = Coo::graph_laplacian_plus_identity(n, &edges).to_csc_sym_upper();
    let expected = ldlt_factor(&matrix).expect("reference factors");
    let operation = test_operation(104);
    let mut job = LdltJob::new(operation, matrix, 3);
    let mut sequence = 0;
    let checkpoint = loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(job.step(&mut context)) {
            Seen::Checkpoint(pages) => break pages,
            Seen::Yield => {}
            outcome => panic!("unexpected LDLT checkpoint outcome: {outcome:?}"),
        }
    };
    let mut resumed = restore_ldlt(operation, payload_from_pages(operation, JobPayloadStream::CheckpointState, &checkpoint)).expect("retained LDLT checkpoint restores");
    loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(2, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        if matches!(seen_of(resumed.step(&mut context)), Seen::Complete(_)) {
            break;
        }
    }
    assert_eq!(resumed.factor(), Some(expected));
    close_job(&mut job);
    close_job(&mut resumed);
}

#[test]
fn p6h_ldlt_microcursor_max_plus_one_cancel_deadline_stale_replay_and_numerical_parity() {
    let n = 18;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|index| (index, index + 1)).collect();
    let matrix = Coo::graph_laplacian_plus_identity(n, &edges).to_csc_sym_upper();
    let reference = ldlt_factor(&matrix).expect("batch reference factor");
    let operation = test_operation(121);
    let drive = |fuel: u64| {
        let mut job = LdltJob::new(operation, matrix.clone(), 1);
        let mut sequence = 0;
        loop {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            if matches!(seen_of(job.step(&mut context)), Seen::Complete(_)) {
                let factor = job.factor().expect("completed factor owner");
                close_job(&mut job);
                return factor;
            }
        }
    };
    assert_eq!(drive(1), reference);
    assert_eq!(drive(2), reference);
    assert_eq!(drive(4), reference);

    let mut zero_fuel = LdltJob::new(operation, matrix.clone(), 1);
    let before = zero_fuel.state.clone();
    let mut sequence = 0;
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(0, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
    assert_eq!(seen_of(zero_fuel.step(&mut context)), Seen::Yield);
    assert!(zero_fuel.state == before);

    let mut deadline = LdltJob::new(operation, matrix.clone(), 1);
    let before = deadline.state.clone();
    let mut sequence_receipt = RetainedCloneProgress::default();
    drop(context);
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
    assert_eq!(seen_of(deadline.step(&mut context)), Seen::Yield);
    assert!(deadline.state == before);

    let mut stale = LdltJob::new(operation, matrix.clone(), 1);
    let before = stale.state.clone();
    let mut sequence_receipt = RetainedCloneProgress::default();
    drop(context);
    let mut context = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
    assert!(matches!(seen_of(stale.step(&mut context)), Seen::Fault(_)));
    assert!(stale.state == before);

    let mut cancelled = LdltJob::new(operation, matrix, 1);
    let before = cancelled.state.clone();
    let token = semio_framework_job::root_cancel_token();
    token.cancel_now();
    let mut sequence_receipt = RetainedCloneProgress::default();
    drop(context);
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
    assert_eq!(seen_of(cancelled.step(&mut context)), Seen::Cancelled);
    assert!(cancelled.state == before);

    let mut lookup = LdltJob::new(operation, deadline.state.a.clone(), 1);
    let mut observed_lookup = false;
    drop(context);
    for _ in 0..200_000 {
        if lookup.state.cursor.stage == LdltColumnStage::ContributorLookup && lookup.state.cursor.lookup_initialized && lookup.state.cursor.lookup_lower < lookup.state.cursor.lookup_upper {
            let before = lookup.state.clone();
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut expired = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            assert_eq!(seen_of(lookup.step(&mut expired)), Seen::Yield);
            assert!(lookup.state == before);
            let mut sequence_receipt = RetainedCloneProgress::default();
            drop(expired);
            let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            assert_eq!(seen_of(lookup.step(&mut one)), Seen::Yield);
            assert_eq!(lookup.state.cursor.contributor, before.cursor.contributor);
            assert!(lookup.state.cursor.lookup_lower != before.cursor.lookup_lower || lookup.state.cursor.lookup_upper != before.cursor.lookup_upper);
            observed_lookup = true;
            break;
        }
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let _ = seen_of(lookup.step(&mut one));
    }
    assert!(observed_lookup, "adversarial LDLT reaches retained contributor comparison");
    close_job(&mut lookup);

    let refused = CscSym { n: LDLT_MAXIMUM_ORDER + 1, colptr: vec![0; LDLT_MAXIMUM_ORDER + 2], rowind: Vec::new(), vals: Vec::new() };
    let mut maximum_plus_one = LdltJob::new(operation, refused, 1);
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
    assert!(matches!(seen_of(maximum_plus_one.step(&mut context)), Seen::Fault(_)));

    let mut publishing = LdltJob::new(operation, deadline.state.a.clone(), 1);
    drop(context);
    for _ in 0..200_000 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let _ = seen_of(publishing.step(&mut context));
        if publishing.output_writer.is_some() {
            break;
        }
    }
    assert!(publishing.output_writer.is_some(), "retained LDLT result writer becomes interruptible before publication");
    InteractiveJob::begin_close(&mut publishing);
    close_job(&mut publishing);
    assert!(InteractiveJob::terminal_is_empty(&publishing));

    let checkpoint = loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        if let Seen::Checkpoint(pages) = seen_of(zero_fuel.step(&mut context)) {
            break pages;
        }
    };
    let mut roundtrip = restore_ldlt(operation, checkpoint_payload(operation, &checkpoint)).expect("LDLT retained checkpoint roundtrip");
    assert_eq!(roundtrip.state.identity, zero_fuel.state.identity);
    close_job(&mut roundtrip);
    assert!(matches!(restore_ldlt(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState)), Err(NumericalCheckpointFault::Truncated)));
    let wrong_revision = Operation::new(operation.operation, semio_framework_job::RevisionId(operation.base_revision.0 + 1), operation.generation, operation.seed);
    let wrong_seed = Operation::new(operation.operation, operation.base_revision, operation.generation, operation.seed + 1);
    let fresh_checkpoint = |id: u64| {
        let mut source = LdltJob::new(operation, Coo::graph_laplacian_plus_identity(6, &[(0, 1), (1, 2)]).to_csc_sym_upper(), 1);
        let mut local_sequence = id;
        let pages = loop {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut local_sequence, &mut sequence_receipt);
            if let Seen::Checkpoint(pages) = seen_of(source.step(&mut context)) {
                break pages;
            }
        };
        close_job(&mut source);
        pages
    };
    assert!(matches!(restore_ldlt(wrong_revision, checkpoint_payload(operation, &fresh_checkpoint(1))), Err(NumericalCheckpointFault::Stale)));
    assert!(matches!(restore_ldlt(wrong_seed, checkpoint_payload(operation, &fresh_checkpoint(2))), Err(NumericalCheckpointFault::Stale)));
    let mut interrupted_restore = LdltRestoreCursor::new(operation, checkpoint_payload(operation, &fresh_checkpoint(3)));
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut restore_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
    assert!(matches!(interrupted_restore.step(&mut restore_context), Ok(None)));
    while !interrupted_restore.terminal_is_empty() {
        match interrupted_restore.close_step(grant_of(1, usize::MAX)) {
            Ok(InteractiveJobCloseStep::Pending { progress }) => assert!(progress.copied_items <= 1),
            Ok(InteractiveJobCloseStep::Complete { .. }) => {}
            Ok(InteractiveJobCloseStep::Blocked) | Err(_) => panic!("LDLT restore close cannot block"),
        }
    }

    for owner in [&mut deadline, &mut stale, &mut cancelled, &mut maximum_plus_one] {
        close_job(owner);
    }

    let mut closing = zero_fuel;
    let mut close_turns = 0;
    loop {
        close_turns += 1;
        match close_turn(&mut closing, grant_of(1, usize::MAX)) {
            Ok(InteractiveJobCloseStep::Complete { .. }) => break,
            Ok(InteractiveJobCloseStep::Pending { progress }) => assert!(progress.copied_items <= 1),
            Ok(InteractiveJobCloseStep::Blocked) | Err(_) => panic!("fixed LDLT close cannot block"),
        }
        assert!(close_turns < 20_000);
    }
    assert!(InteractiveJob::terminal_is_empty(&closing));
}
