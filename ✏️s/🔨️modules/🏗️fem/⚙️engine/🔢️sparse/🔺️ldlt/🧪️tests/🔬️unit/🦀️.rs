use semio_s_fem_sparse::{Coo, CscSym, NUMERICAL_CHECKPOINT_HEADER_BYTES, NUMERICAL_OWNER_PAGE_BYTES, NumericalCheckpointFault, SPARSE_INDEX_SPACE, SPARSE_PAGE_BYTES, parse_numerical_page};
use super::*;
use semio_s_fem_test_support::OriginalNumericalFixture;

use semio_s_fem_test_support::grant_of;

use semio_s_fem_test_support::seen_of;
use semio_s_fem_test_support::test_operation;
use semio_s_fem_test_support::Seen;
use semio_s_fem_test_support::TEST_GRANT;

#[test]
fn numerical_page_restore_close_preserves_ungranted_backing_and_retires_one_owner() {
    fn physical(state: Option<&LdltCheckpoint>) -> usize {
        state.map_or(0, |state| {
            state.contributors.links.allocated_bytes() + state.columns.entries.allocated_bytes()
        })
    }
    fn inner_owners(state: Option<&LdltCheckpoint>) -> usize {
        state.map_or(0, |state| usize::from(state.contributors.links.capacity() != 0) + usize::from(state.columns.entries.capacity() != 0))
    }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["closeCases"].as_array().unwrap() {
        let operation = test_operation(490);
    let mut original=OriginalNumericalFixture::new(operation);
        let mut job = LdltJob::new(operation, CscSym::from_paged_parts(0, PagedList::empty(), PagedList::empty(), PagedList::empty()), 1);
        job.state.contributors.links = empty_paged_backing(4);
        job.state.columns.entries = empty_paged_backing(4);
        let next = job.state.contributors.links.next_release_allocation_bytes().unwrap();
        let pair = job.state.columns.entries.next_release_allocation_bytes().unwrap();
        let grant = match row["grant"].as_str().unwrap() { "zero" => 0, "subexact" => next.min(pair) - 1, "exact" => next.max(pair), _ => unreachable!() };
        let mut restore = LdltRestoreCursor::new(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState));
        restore.payload = None;
        restore.state = Some(job.state);
        let before = physical(restore.state.as_ref());
        let first = close_restore(&mut original,&mut restore,grant_of(1, grant));
        let after = physical(restore.state.as_ref());
        let retired = 2 - inner_owners(restore.state.as_ref());
        let reported = match first.expect("original cursor close") { semio_framework_job::InteractiveJobCloseStep::Pending { progress } => progress.released_bytes, _ => 0 };
        let mut released = reported;
        let mut exact = before - after == reported && reported <= grant;
        for _ in 0..128 {
            let before = physical(restore.state.as_ref());
            let step = close_restore(&mut original,&mut restore,grant_of(1, NUMERICAL_OWNER_PAGE_BYTES));
            let reported = match step.expect("original cursor close") { semio_framework_job::InteractiveJobCloseStep::Pending { progress } => progress.released_bytes, _ => 0 };
            exact &= before - physical(restore.state.as_ref()) == reported;
            released += reported;
            if restore.terminal_is_empty() { break; }
        }
        observations.push((row["id"].clone(), retired, row["expectedRetiredOwners"].as_u64().unwrap() as usize, exact, before, released, restore.terminal_is_empty()));
        original.finish();
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
    let mut original=OriginalNumericalFixture::new(operation);

    let mut job = LdltJob::new(operation, CscSym::from_owned_parts(n,colptr,rowind,vals), 1);
    job.state.checkpoint_due = true;
    let mut checkpoint = None;
    let mut write_fault = false;
    for _ in 0..100_000 {

        match step_job(&mut original,&mut job,1,u64::MAX) {
            Seen::Checkpoint(pages) => { checkpoint = Some(pages); break; }
            Seen::Yield | Seen::Preview(_) => {}
            Seen::Fault(_) | Seen::Complete(_) | Seen::Cancelled => { write_fault = true; break; }
        }
    }
    let mut pages = Vec::new();
    let mut restore = checkpoint.map(|checkpoint_pages| {
        let payload = original.payload_from_pages(JobPayloadStream::CheckpointState,&checkpoint_pages);
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

            match step_restore(&mut original,restore,1,u64::MAX) {
                Ok(Some(value)) => { restored = Some(value); break; }
                Ok(None) => {}
                Err(fault) => { restore_fault = Some(fault); break; }
            }
        }
    }
    let actual = restored.as_ref().map(|value| (value.state.a.colptr.iter().copied().collect::<Vec<_>>(),value.state.a.rowind.iter().copied().collect::<Vec<_>>(),value.state.a.vals.iter().copied().collect::<Vec<_>>()));
    for _ in 0..10_000 {
        if let Some(restore) = restore.as_mut() { let _ = close_restore(&mut original,restore,grant_of(1, NUMERICAL_OWNER_PAGE_BYTES)); }
        if let Some(restored) = restored.as_mut() { let _ = original.close_turn(restored,grant_of(1, NUMERICAL_OWNER_PAGE_BYTES)); }
        let _ = original.close_turn(&mut job,grant_of(1, NUMERICAL_OWNER_PAGE_BYTES));
        if restore.as_ref().is_none_or(LdltRestoreCursor::terminal_is_empty) && restored.as_ref().is_none_or(InteractiveJob::terminal_is_empty) && InteractiveJob::terminal_is_empty(&job) { break; }
    }
    let closed = restore.as_ref().is_none_or(LdltRestoreCursor::terminal_is_empty) && restored.as_ref().is_none_or(InteractiveJob::terminal_is_empty) && InteractiveJob::terminal_is_empty(&job);
    eprintln!("[DEBUG] LDLT maximum owner checkpoint pages={pages:?}, write_fault={write_fault}, restore_fault={restore_fault:?}, restored={}, closed={closed}", actual.is_some());
    assert!(closed);
    assert!(!write_fault);
    let mut expected_pages=Vec::new();let mut item=0;let mut owner=0;
    while item<count{let overhead=usize::from(owner==0)*8;let scalars=((semio_framework_job::JOB_PAYLOAD_PAGE_BYTES-NUMERICAL_CHECKPOINT_HEADER_BYTES-overhead)/8).min(count-item);expected_pages.push(serde_json::json!({"owner":owner,"item":item,"bytes":NUMERICAL_CHECKPOINT_HEADER_BYTES+overhead+scalars*8,"scalars":scalars}));item+=scalars;owner+=1;}
    assert_eq!(pages,expected_pages);
    assert_eq!(restore_fault, None);
    assert_eq!(actual, Some(expected));
    original.finish();
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

fn restore_ldlt(original:&mut OriginalNumericalFixture,operation: Operation, payload: RetainedJobPayload) -> Result<LdltJob, NumericalCheckpointFault> {
    let mut restore = LdltRestoreCursor::new(operation, payload);

    for _ in 0..200_000 {
        let handoff_due = restore.payload.is_none();

        let step = step_restore(original,&mut restore,1,u64::MAX);
        assert!(original.receipt().copied_items <= 1);
        match step {
            Ok(Some(job)) => { assert!(handoff_due); assert_eq!(original.receipt().copied_items, 1); assert_eq!(original.receipt().released_bytes, 0); return Ok(job); },
            Ok(None) => {}
            Err(fault) => {
                while !restore.terminal_is_empty() {
                    let _ = close_restore(original,&mut restore,grant_of(1, usize::MAX));
                }
                return Err(fault);
            }
        }
    }
    while !restore.terminal_is_empty(){close_restore(original,&mut restore,grant_of(1,NUMERICAL_OWNER_PAGE_BYTES)).unwrap();}
    Err(NumericalCheckpointFault::Truncated)
}

#[test]
fn ldlt_job_checkpoint_resume_matches_reference() {
    let n = 30;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let matrix = Coo::graph_laplacian_plus_identity(n, &edges).to_csc_sym_upper();
    let expected = ldlt_factor(&matrix).expect("reference factors");
    let operation = test_operation(104);
    let mut original=OriginalNumericalFixture::new(operation);
    let mut job = LdltJob::new(operation, matrix, 3);

    let checkpoint = loop {

        match step_job(&mut original,&mut job,1,u64::MAX) {
            Seen::Checkpoint(pages) => break pages,
            Seen::Yield => {}
            outcome => panic!("unexpected LDLT checkpoint outcome: {outcome:?}"),
        }
    };
    let mut resumed = restore_pages(&mut original,operation,&checkpoint).expect("retained LDLT checkpoint restores");
    loop {

        if matches!(step_job(&mut original,&mut resumed,2,u64::MAX), Seen::Complete(_)) {
            break;
        }
    }
    assert_eq!(resumed.factor(), Some(expected));
    original.close(&mut job);
    original.close(&mut resumed);
    original.finish();
}

#[test]
fn p6h_ldlt_microcursor_max_plus_one_cancel_deadline_stale_replay_and_numerical_parity() {
    let n = 18;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|index| (index, index + 1)).collect();
    let matrix = Coo::graph_laplacian_plus_identity(n, &edges).to_csc_sym_upper();
    let reference = ldlt_factor(&matrix).expect("batch reference factor");
    let operation = test_operation(121);
    let mut original=OriginalNumericalFixture::new(operation);
    let mut drive = |fuel: u64| {
        let mut job = LdltJob::new(operation, matrix.clone(), 1);

        loop {

            if matches!(step_job(&mut original,&mut job,fuel,u64::MAX), Seen::Complete(_)) {
                let factor = job.factor().expect("completed factor owner");
                original.close(&mut job);
                return factor;
            }
        }
    };
    assert_eq!(drive(1), reference);
    assert_eq!(drive(2), reference);
    assert_eq!(drive(4), reference);

    let mut zero_fuel = LdltJob::new(operation, matrix.clone(), 1);
    let before = zero_fuel.state.clone();

    assert_eq!(step_job(&mut original,&mut zero_fuel,0,u64::MAX), Seen::Yield);
    assert!(zero_fuel.state == before);

    let mut deadline = LdltJob::new(operation, matrix.clone(), 1);
    let before = deadline.state.clone();

    assert_eq!(step_job(&mut original,&mut deadline,1,0), Seen::Yield);
    assert!(deadline.state == before);

    let mut stale = LdltJob::new(operation, matrix.clone(), 1);
    let before = stale.state.clone();
    let mut stale_original=OriginalNumericalFixture::new(Operation::new(operation.operation,operation.base_revision,semio_framework_job::Generation(operation.generation.0+1),operation.seed));

    assert!(matches!(step_job(&mut stale_original,&mut stale,1,u64::MAX), Seen::Fault(_)));
    assert!(stale.state == before);

    let mut cancelled = LdltJob::new(operation, matrix, 1);
    let before = cancelled.state.clone();
    let mut cancelled_original=OriginalNumericalFixture::new(operation);cancelled_original.cancel();

    assert_eq!(step_job(&mut cancelled_original,&mut cancelled,1,u64::MAX), Seen::Cancelled);
    assert!(cancelled.state == before);

    let mut lookup = LdltJob::new(operation, deadline.state.a.clone(), 1);
    let mut observed_lookup = false;
    for _ in 0..200_000 {
        if lookup.state.cursor.stage == LdltColumnStage::ContributorLookup && lookup.state.cursor.lookup_initialized && lookup.state.cursor.lookup_lower < lookup.state.cursor.lookup_upper {
            let before = lookup.state.clone();

            assert_eq!(step_job(&mut original,&mut lookup,1,0), Seen::Yield);
            assert!(lookup.state == before);

            assert_eq!(step_job(&mut original,&mut lookup,1,u64::MAX), Seen::Yield);
            assert_eq!(lookup.state.cursor.contributor, before.cursor.contributor);
            assert!(lookup.state.cursor.lookup_lower != before.cursor.lookup_lower || lookup.state.cursor.lookup_upper != before.cursor.lookup_upper);
            observed_lookup = true;
            break;
        }

        let _ = step_job(&mut original,&mut lookup,1,u64::MAX);
    }
    assert!(observed_lookup, "adversarial LDLT reaches retained contributor comparison");
    original.close(&mut lookup);

    let refused = CscSym::from_owned_parts(3, vec![0, 0], Vec::new(), Vec::new());
    let mut maximum_plus_one = LdltJob::new(operation, refused, 1);

    assert!(matches!(step_job(&mut original,&mut maximum_plus_one,1,u64::MAX), Seen::Fault(_)));

    let mut publishing = LdltJob::new(operation, deadline.state.a.clone(), 1);
    for _ in 0..200_000 {

        let _ = step_job(&mut original,&mut publishing,1,u64::MAX);
        if publishing.output_writer.is_some() {
            break;
        }
    }
    assert!(publishing.output_writer.is_some(), "retained LDLT result writer becomes interruptible before publication");
    InteractiveJob::begin_close(&mut publishing);
    original.close(&mut publishing);
    assert!(InteractiveJob::terminal_is_empty(&publishing));

    let checkpoint = loop {

        if let Seen::Checkpoint(pages) = step_job(&mut original,&mut zero_fuel,1,u64::MAX) {
            break pages;
        }
    };
    let mut roundtrip = restore_pages(&mut original,operation,&checkpoint).expect("LDLT retained checkpoint roundtrip");
    assert_eq!(roundtrip.state.identity, zero_fuel.state.identity);
    original.close(&mut roundtrip);
    assert!(matches!(restore_ldlt(&mut original,operation,RetainedJobPayload::empty(JobPayloadStream::CheckpointState)), Err(NumericalCheckpointFault::Truncated)));
    let wrong_revision = Operation::new(operation.operation, semio_framework_job::RevisionId(operation.base_revision.0 + 1), operation.generation, operation.seed);
    let wrong_seed = Operation::new(operation.operation, operation.base_revision, operation.generation, operation.seed + 1);
    fn fresh_checkpoint(original:&mut OriginalNumericalFixture,operation:Operation)->Vec<Vec<u8>> {
        let mut source = LdltJob::new(operation, Coo::graph_laplacian_plus_identity(6, &[(0, 1), (1, 2)]).to_csc_sym_upper(), 1);

        let pages = loop {

            if let Seen::Checkpoint(pages) = step_job(original,&mut source,1,u64::MAX) {
                break pages;
            }
        };
        original.close(&mut source);
        pages
    }
    let pages=fresh_checkpoint(&mut original,operation);assert!(matches!(restore_pages(&mut original,wrong_revision,&pages),Err(NumericalCheckpointFault::Stale)));
    let pages=fresh_checkpoint(&mut original,operation);assert!(matches!(restore_pages(&mut original,wrong_seed,&pages),Err(NumericalCheckpointFault::Stale)));
    let pages=fresh_checkpoint(&mut original,operation);let mut interrupted_restore=LdltRestoreCursor::new(operation,original.payload_from_pages(JobPayloadStream::CheckpointState,&pages));

    assert!(matches!(step_restore(&mut original,&mut interrupted_restore,1,u64::MAX), Ok(None)));
    while !interrupted_restore.terminal_is_empty() {
        match close_restore(&mut original,&mut interrupted_restore,grant_of(1, usize::MAX)).expect("original interrupted cursor close") {
            semio_framework_job::InteractiveJobCloseStep::Pending { progress } => assert!(progress.copied_items <= 1),
            semio_framework_job::InteractiveJobCloseStep::Complete { .. } => {}
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("LDLT restore close cannot block"),
        }
    }

    for owner in [&mut deadline, &mut maximum_plus_one] {
        original.close(owner);
    }

    let mut closing = zero_fuel;
    let mut close_turns = 0;
    loop {
        close_turns += 1;
        match original.close_turn(&mut closing,grant_of(1,usize::MAX)).expect("original fixed LDLT close") {
            semio_framework_job::InteractiveJobCloseStep::Complete { .. } => break,
            semio_framework_job::InteractiveJobCloseStep::Pending { progress } => assert!(progress.copied_items <= 1),
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("fixed LDLT close cannot block"),
        }
        assert!(close_turns < 20_000);
    }
    assert!(InteractiveJob::terminal_is_empty(&closing));
    stale_original.close(&mut stale);cancelled_original.close(&mut cancelled);stale_original.finish();cancelled_original.finish();
    original.finish();
}

#[test]
fn numerical_page_ldlt_checkpoint_restores_matrix_across_original_pages() {
    let n = 65;
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
    let mut original=OriginalNumericalFixture::new(operation);

    let mut job = LdltJob::new(operation, CscSym::from_owned_parts(n, colptr, rowind, vals), 1);
    job.state.checkpoint_due = true;
    let mut checkpoint = None;
    let mut write_fault = false;
    for _ in 0..100_000 {

        match step_job(&mut original,&mut job,1,u64::MAX) {
            Seen::Checkpoint(pages) => { checkpoint = Some(pages); break; }
            Seen::Yield | Seen::Preview(_) => {}
            Seen::Fault(_) | Seen::Complete(_) | Seen::Cancelled => { write_fault = true; break; }
        }
    }
    let mut pages = Vec::new();
    let mut restore = checkpoint.map(|checkpoint_pages| {
        let payload = original.payload_from_pages(JobPayloadStream::CheckpointState,&checkpoint_pages);
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

            match step_restore(&mut original,restore,1,u64::MAX) {
                Ok(Some(value)) => { restored = Some(value); break; }
                Ok(None) => {}
                Err(fault) => { restore_fault = Some(fault); break; }
            }
        }
    }
    let actual = restored.as_ref().map(|value| (value.state.a.colptr.iter().copied().collect::<Vec<_>>(), value.state.a.rowind.iter().copied().collect::<Vec<_>>(), value.state.a.vals.iter().copied().collect::<Vec<_>>()));
    for _ in 0..10_000 {
        if let Some(restore) = restore.as_mut() { let _ = close_restore(&mut original,restore,grant_of(1, NUMERICAL_OWNER_PAGE_BYTES)); }
        if let Some(restored) = restored.as_mut() { let _ = original.close_turn(restored,grant_of(1,NUMERICAL_OWNER_PAGE_BYTES)); }
        let _ = original.close_turn(&mut job,grant_of(1,NUMERICAL_OWNER_PAGE_BYTES));
        if restore.as_ref().is_none_or(LdltRestoreCursor::terminal_is_empty) && restored.as_ref().is_none_or(InteractiveJob::terminal_is_empty) && InteractiveJob::terminal_is_empty(&job) { break; }
    }
    let closed = restore.as_ref().is_none_or(LdltRestoreCursor::terminal_is_empty) && restored.as_ref().is_none_or(InteractiveJob::terminal_is_empty) && InteractiveJob::terminal_is_empty(&job);
    eprintln!("[DEBUG] LDLT paged owner checkpoint pages={pages:?}, write_fault={write_fault}, restore_fault={restore_fault:?}, restored={}, closed={closed}", actual.is_some());
    assert!(closed);
    assert!(!write_fault);
    let first = (semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - NUMERICAL_CHECKPOINT_HEADER_BYTES - 8) / 8;
    assert_eq!(pages, vec![serde_json::json!({ "owner": 0, "item": 0, "bytes": NUMERICAL_CHECKPOINT_HEADER_BYTES + 8 + first * 8, "scalars": first }), serde_json::json!({ "owner": 1, "item": first, "bytes": NUMERICAL_CHECKPOINT_HEADER_BYTES + (count - first) * 8, "scalars": count - first })]);
    assert_eq!(restore_fault, None);
    assert_eq!(actual, Some(expected));
    original.finish();
}

fn empty_paged_backing<T>(count: usize) -> PagedList<T, SPARSE_INDEX_SPACE> {
    let mut owner = PagedList::empty();
    while owner.capacity() < count { owner.reserve_capacity_one(count, SPARSE_PAGE_BYTES).expect("original fixture backing"); }
    owner
}

fn step_job(original:&mut OriginalNumericalFixture,job:&mut impl InteractiveJob,fuel:u64,deadline:u64)->Seen{original.turn(TEST_GRANT,fuel,deadline,|cx|Ok(seen_of(job.step(cx)))).expect("same original LDLT job step")}
fn step_restore(original:&mut OriginalNumericalFixture,cursor:&mut LdltRestoreCursor,fuel:u64,deadline:u64)->Result<Option<LdltJob>,NumericalCheckpointFault>{original.turn(RetainedCloneGrant{maximum_items:1,..TEST_GRANT},fuel,deadline,|cx|Ok(cursor.step(cx))).expect("same original LDLT restore turn")}
fn close_restore(original:&mut OriginalNumericalFixture,cursor:&mut LdltRestoreCursor,grant:RetainedCloneGrant)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{original.turn(grant,1,u64::MAX,|cx|cx.with_original_retirement(|cx|cursor.close_step(cx)))}
fn restore_pages(original:&mut OriginalNumericalFixture,operation:Operation,pages:&[Vec<u8>])->Result<LdltJob,NumericalCheckpointFault>{let payload=original.payload_from_pages(JobPayloadStream::CheckpointState,pages);restore_ldlt(original,operation,payload)}
