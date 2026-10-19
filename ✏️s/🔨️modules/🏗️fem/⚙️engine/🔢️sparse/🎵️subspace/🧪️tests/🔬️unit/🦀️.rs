use semio_s_fem_sparse::{Coo, Csr, NUMERICAL_OWNER_PAGE_BYTES, NumericalCheckpointFault, NumericalOwnerRestoreCursor, NumericalPageCursor, SPARSE_INDEX_SPACE, SPARSE_PAGE_BYTES, SparseColumns, advance_matrix_owner, advance_numerical_page_header, close_vec_owner_step, numerical_page_header, parse_numerical_page, read_checkpoint_u64};
use super::*;
use semio_framework_job::StepBudget;
use super::*;
use super::*;
use semio_s_fem_test_support::close_turn;
use semio_s_fem_test_support::checkpoint_payload;
use semio_s_fem_test_support::close_job;
use semio_s_fem_test_support::close_payload;
use semio_s_fem_test_support::close_writer;
use semio_s_fem_test_support::grant_of;
use semio_s_fem_test_support::seen_of;
use semio_s_fem_test_support::test_operation;
use semio_s_fem_test_support::Seen;
use semio_s_fem_test_support::TEST_GRANT;

#[test]
fn numerical_page_subspace_work_persists_solver_state_without_physical_retirement_cursor() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let operation = test_operation(494);
    let mut sequence = 0;
    let mut observations = Vec::new();
    for field in [11, 23] {
        let mut pages = Vec::new();
        for lane in corpus["workRetirementLanes"].as_array().unwrap() {
            let mut job = SubspaceIterationJob::new(operation, LdltFactor::from_owned_parts(0, Vec::new(), Vec::new()), Csr::from_owned_parts(0, Vec::new(), Vec::new(), Vec::new()), 0, 0, 1);
            job.state.work.close_lane = lane.as_u64().unwrap() as u8;
            let mut retiring = SubspaceWork::empty();
            retiring.close_lane = lane.as_u64().unwrap() as u8;
            job.state.retiring_work = Some(retiring);
            let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            writer.begin_staged_page(&mut context).unwrap();
            let mut cursor = NumericalPageCursor { field, owner: 0, item: 0 };
            for _ in 0..64 {
                test_funded_restore(|context| SubspaceIterationJob::advance_checkpoint_entry(&job.state, &mut cursor, &mut writer, context)).unwrap();
                if cursor.field != field { break; }
            }
            let mut payload = writer.finish().unwrap_or_else(|_| panic!("work control page committed"));
            pages.push(payload.page(0).unwrap().to_vec());
            close_payload(payload);
        }
        let counts: Vec<_> = pages.iter().map(|bytes| {
            let page = parse_numerical_page(bytes, b"FEMSCP1\0").unwrap();
            read_checkpoint_u64(page.bytes, if field == 23 { 8 } else { 0 }).unwrap()
        }).collect();
        observations.push((field, counts, pages[0] == pages[1]));
    }
    let mut bytes = numerical_page_header(b"FEMSCP1\0", 12, NumericalPageCursor { field: 11, owner: 0, item: 0 }).to_vec();
    bytes.extend_from_slice(&13u64.to_le_bytes());
    for index in 0..13 { bytes.extend_from_slice(&(if index == 12 { 11u64 } else { 0 }).to_le_bytes()); }
    let page = parse_numerical_page(&bytes, b"FEMSCP1\0").unwrap();
    let mut work = SubspaceWork::empty();
    work.rhs.data=PagedList::empty();while work.rhs.data.capacity()<4{work.rhs.data.reserve_capacity_one(4,4096).unwrap();}
    let allocated=work.rhs.data.allocated_bytes();let next_backing=work.rhs.data.next_release_allocation_bytes().unwrap();
    let mut cursor = NumericalOwnerRestoreCursor::default();
    let mut entry = 0;
    let mut control = [0; 24];
    let mut fault = None;
    for _ in 0..32 {
        match test_funded_restore(|context| restore_work_entry(&mut work, &page, 11, 1, 1, &mut entry, &mut control, &mut cursor, context)) {
            Ok(true) => break,
            Ok(false) => {}
            Err(error) => { fault = Some(error); break; }
        }
    }
    let first = work.close_step(NUMERICAL_OWNER_PAGE_BYTES);
    let held=work.rhs.data.allocated_bytes();
    work.close_lane = 0;
    let mut released = first.2;
    for _ in 0..64 {
        let step = work.close_step(NUMERICAL_OWNER_PAGE_BYTES);
        released += step.2;
        if step.0 { break; }
    }
    for (field, counts, identical) in observations {
        assert_eq!(counts, vec![corpus["workControlEntries"].as_u64().unwrap(); 2], "field {field} contains only solver control values");
        assert!(identical, "field {field} physical cleanup does not change checkpoint bytes");
    }
    assert_eq!(fault, Some(NumericalCheckpointFault::Truncated));
    assert_eq!(first, (false, 0, next_backing));
    assert_eq!(held, allocated-next_backing);
    assert_eq!(released, allocated);
}



#[test]
fn numerical_page_subspace_restore_close_preserves_every_ungranted_backing() {
    fn physical(state: Option<&SubspaceCheckpoint>) -> usize {state.map_or(0,|state|state.k_factor.allocated_bytes())}
    fn inner_owners(state: Option<&SubspaceCheckpoint>) -> usize {state.map_or(0,|state|usize::from(state.k_factor.columns.entries.capacity()!=0)+usize::from(state.k_factor.d.capacity()!=0))}    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["closeCases"].as_array().unwrap() {
        let operation = test_operation(493);
        let mut factor = LdltFactor { n: 0, columns: SparseColumns::default(), d: PagedList::empty() };
        while factor.columns.entries.capacity()<4{let owner=&mut factor.columns.entries;let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:owner.next_allocation_bytes().unwrap(),maximum_copy_bytes:owner.next_reserve_copy_byte_demand().unwrap(),maximum_depth:owner.next_reserve_depth_demand().unwrap(),..TEST_GRANT};owner.reserve_capacity_one_funded(4,grant).unwrap();}
        while factor.d.capacity()<4{let owner=&mut factor.d;let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:owner.next_allocation_bytes().unwrap(),maximum_copy_bytes:owner.next_reserve_copy_byte_demand().unwrap(),maximum_depth:owner.next_reserve_depth_demand().unwrap(),..TEST_GRANT};owner.reserve_capacity_one_funded(4,grant).unwrap();}
        let next = factor.columns.entries.next_release_allocation_bytes().unwrap();
        let scalar = factor.d.next_release_allocation_bytes().unwrap();        let grant = match row["grant"].as_str().unwrap() { "zero" => 0, "subexact" => next.min(scalar) - 1, "exact" => next.max(scalar), _ => unreachable!() };
        let job = SubspaceIterationJob::new(operation, factor, Csr::from_owned_parts(0, Vec::new(), Vec::new(), Vec::new()), 0, 0, 1);
        let mut restore = SubspaceRestoreCursor::new(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState));
        restore.payload = None;
        restore.state = Some(job.state);
        let before = physical(restore.state.as_ref());
        let first = restore.close_step(grant_of(1, grant));
        let after = physical(restore.state.as_ref());
        let retired = 2 - inner_owners(restore.state.as_ref());
        let reported = match first { semio_framework_job::InteractiveJobCloseStep::Pending { progress } => progress.released_bytes, _ => 0 };
        let mut released = reported;
        let mut exact = before - after == reported && reported <= grant;
        for _ in 0..128 {
            let before = physical(restore.state.as_ref());
            let step = restore.close_step(grant_of(1, NUMERICAL_OWNER_PAGE_BYTES));
            let reported = match step { semio_framework_job::InteractiveJobCloseStep::Pending { progress } => progress.released_bytes, _ => 0 };
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
fn numerical_page_restore_fault_is_sticky_before_later_input_or_allocation() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    for row in corpus["stickyFaultCases"].as_array().unwrap() {
    let expected = match row["expected"].as_str().unwrap() { "Admission" => NumericalCheckpointFault::Admission, "Stale" => NumericalCheckpointFault::Stale, _ => unreachable!() };
    let operation = test_operation(492);
    let mut sequence = 0;
    let mut ldlt = LdltRestoreCursor::new(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState));
    let mut subspace = SubspaceRestoreCursor::new(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState));
    ldlt.fault = Some(expected);
    subspace.fault = Some(expected);
    let mut observations = Vec::new();
    for _ in 0..2 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let left = ldlt.step(&mut context).err();
        let mut sequence_receipt = RetainedCloneProgress::default();
        drop(context);
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let right = subspace.step(&mut context).err();
        observations.push((left, right, ldlt.page_slot, ldlt.page_entry, subspace.page_slot, subspace.page_entry));
    }
    for _ in 0..16 {
        let _ = ldlt.close_step(grant_of(1, NUMERICAL_OWNER_PAGE_BYTES));
        let _ = subspace.close_step(grant_of(1, NUMERICAL_OWNER_PAGE_BYTES));
        if ldlt.terminal_is_empty() && subspace.terminal_is_empty() { break; }
    }
    assert!(ldlt.terminal_is_empty() && subspace.terminal_is_empty());
    for (left, right, left_page, left_entry, right_page, right_entry) in observations {
        assert_eq!(left, Some(expected));
        assert_eq!(right, Some(expected));
        assert_eq!((left_page, left_entry, right_page, right_entry), (0, 0, 0, 0));
    }
    }
}

#[test]
fn numerical_page_matrix_restore_retains_dimensions_and_partial_capacity_across_pages() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["matrixRestoreCases"].as_array().unwrap() {
        let rows = row["rows"].as_u64().unwrap() as usize;
        let cols = row["cols"].as_u64().unwrap() as usize;
        let length = row["length"].as_u64().unwrap() as usize;
        let mut matrix = MatD::zeros(0, 0);
        matrix.rows = rows;
        matrix.cols = cols;
        matrix.data = (0..length).map(|index| index as f64 * 0.25 - 100.0).collect();
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
        let mut write_cursor = NumericalPageCursor { field: 518, owner: 0, item: 0 };
        let operation = test_operation(491);
        let mut sequence = 0;
        let mut write_fault = None;
        let mut completed = false;
        for _ in 0..length * 3 + 64 {
            let result = if writer.staged_page_len().is_none() {
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                writer.begin_staged_page(&mut context).map(|_| false)
            } else {
                match test_funded_restore(|context| advance_numerical_page_header(&mut writer, b"FEMSCP1\0", 12, write_cursor, context)) {
                    Ok(true) => Ok(false),
                    Ok(false) => test_funded_restore(|context| advance_matrix_owner(&mut writer, &matrix, &mut write_cursor, context)),
                    Err(fault) => Err(fault),
                }
            };
            match result {
                Ok(true) => { writer.commit_staged_page().unwrap(); completed = true; break; }
                Ok(false) => {}
                Err(fault) => { write_fault = Some(fault); break; }
            }
        }
        let mut restored = MatD::zeros(0, 0);
        let mut restore_cursor = NumericalOwnerRestoreCursor::default();
        let mut restore_fault = None;
        let mut page_items = Vec::new();
        if completed {
            let mut payload = writer.finish().unwrap_or_else(|_| panic!("matrix writer finishes committed pages"));
            for slot in 0..payload.page_count() {
                let page = parse_numerical_page(payload.page(slot).unwrap(), b"FEMSCP1\0").unwrap();
                page_items.push(page.item);
                let mut entry = 0;
                for _ in 0..length + 64 {
                    match test_funded_restore(|context| restore_matrix_entry(&mut restored, &page, 2048, &mut entry, &mut restore_cursor, context)) {
                        Ok(true) => break,
                        Ok(false) => {}
                        Err(fault) => { restore_fault = Some(fault); break; }
                    }
                }
                if restore_fault.is_some() { break; }
            }
            close_payload(payload);
        } else {
            writer.begin_close();
            close_writer(&mut writer);
            assert!(writer.terminal_is_empty());
        }
        let values = restored.data.clone();
        let dimensions = (restored.rows, restored.cols);
        let capacity = restored.data.capacity();
        let allocated = capacity * size_of::<f64>();
        let mut released = 0;
        for _ in 0..length + 64 {
            match close_vec_owner_step(&mut restored.data, NUMERICAL_OWNER_PAGE_BYTES) {
                Ok(Some((_, bytes))) => released += bytes,
                Ok(None) => break,
                Err(()) => break,
            }
        }
        observations.push((row.clone(), write_fault, restore_fault, page_items, dimensions, capacity, values, matrix.data, allocated, released, restored.data.capacity()));
    }
    for (row, write_fault, restore_fault, pages, dimensions, capacity, values, expected, allocated, released, remaining) in observations {
        assert_eq!(write_fault, None);
        assert_eq!(restore_fault, None);
        assert_eq!(serde_json::json!(pages), row["pageItems"]);
        assert_eq!(dimensions, (row["rows"].as_u64().unwrap() as usize, row["cols"].as_u64().unwrap() as usize));
        assert!(capacity >= dimensions.0 * dimensions.1);
        assert_eq!(values, expected);
        assert_eq!(allocated, released);
        assert_eq!(remaining, 0);
    }
}

#[test]
fn dense_symmetric_eigen_jacobi_matches_known_eigenvalues() {
    let mut a = MatD::zeros(3, 3);
    a.set(0, 0, 3.0);
    a.set(1, 1, 1.0);
    a.set(2, 2, 2.0);
    let (vals, _vecs) = dense_symmetric_eigen_jacobi(&a);
    assert!((vals[0] - 1.0).abs() < 1e-9);
    assert!((vals[1] - 2.0).abs() < 1e-9);
    assert!((vals[2] - 3.0).abs() < 1e-9);
}

#[test]
fn subspace_iteration_matches_diagonal_analytic_case() {
    let n = 10;
    let mut k_coo = Coo::new(n);
    let mut b_coo = Coo::new(n);
    for i in 0..n {
        k_coo.add(i, i, (i + 1) as f64);
        b_coo.add(i, i, 1.0);
    }
    let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
    let b_csr = b_coo.to_csr();
    let pairs = subspace_iteration(&k_factor, &b_csr, n, 4, 30);
    let expected = [1.0, 2.0, 3.0, 4.0];
    for i in 0..4 {
        assert!((pairs.values[i] - expected[i]).abs() / expected[i] < 1e-4, "eigenvalue {} = {} expected {}", i, pairs.values[i], expected[i]);
    }
}

#[test]
fn subspace_iteration_matches_dense_jacobi_on_small_nondiagonal_case() {
    let n = 7;
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 0), (0, 3)];
    let k_coo = Coo::graph_laplacian_plus_identity(n, &edges);
    let dense_k = k_coo.to_dense();
    let (dense_vals, _) = dense_symmetric_eigen_jacobi(&dense_k);

    let mut b_coo = Coo::new(n);
    for i in 0..n {
        b_coo.add(i, i, 1.0);
    }
    let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
    let b_csr = b_coo.to_csr();
    let pairs = subspace_iteration(&k_factor, &b_csr, n, 3, 30);

    for i in 0..3 {
        assert!((pairs.values[i] - dense_vals[i]).abs() / dense_vals[i].abs().max(1e-9) < 1e-3);
    }
}

/// 🕳️ Indefinite and null generalized modes sort behind physical positive modes using the same
/// finite sentinel on every replay, so checkpoints remain valid JSON and ordering is total.
#[test]
fn subspace_iteration_uses_a_deterministic_finite_null_mode_sentinel() {
    let n = 3;
    let mut k_coo = Coo::new(n);
    let mut b_coo = Coo::new(n);
    for (index, stiffness) in [1.0, 2.0, 3.0].into_iter().enumerate() {
        k_coo.add(index, index, stiffness);
    }
    b_coo.add(0, 0, 1.0);
    b_coo.add(2, 2, -1.0);
    let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
    let b_csr = b_coo.to_csr();

    let first = subspace_iteration(&k_factor, &b_csr, n, 3, 30);
    let replay = subspace_iteration(&k_factor, &b_csr, n, 3, 30);

    assert_eq!(first, replay);
    assert_eq!(first.values, vec![1.0, f64::MAX, f64::MAX]);
    assert!(first.values.iter().all(|value| value.is_finite() && *value > 0.0));
    assert!(first.values.windows(2).all(|pair| pair[0] <= pair[1]));
}

#[test]
fn dense_symmetric_eigen_jacobi_handles_zero_size_matrix() {
    let a = MatD::zeros(0, 0);
    let (vals, vecs) = dense_symmetric_eigen_jacobi(&a);
    assert!(vals.is_empty());
    assert_eq!(vecs.rows, 0);
    assert_eq!(vecs.cols, 0);
}

#[test]
fn subspace_factor_cursor_matches_numpy_for_three_nondiagonal_right_hand_sides() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔢️scalar-owners/🔣️.json")).unwrap();
    let case = &fixture["factor"];
    let lower: Vec<Vec<f64>> = serde_json::from_value(case["lower"].clone()).unwrap();
    let rhs: Vec<Vec<f64>> = serde_json::from_value(case["rhs"].clone()).unwrap();
    let expected: Vec<Vec<f64>> = serde_json::from_value(case["expected"].clone()).unwrap();
    let factor = LdltFactor::from_owned_parts(3, (0..3).map(|column| ((column + 1)..3).map(|row| (row as u32, lower[row][column])).collect()).collect(), serde_json::from_value(case["diagonal"].clone()).unwrap());    let mut mass = Coo::new(3);
    for index in 0..3 { mass.add(index, index, 1.0); }
    let mut job = SubspaceIterationJob::new(test_operation(996), factor, mass.to_csr(), 3, 1, 30);
    job.state.work.rhs = MatD::zeros(3, 3).into();job.state.shift=Some(0.0);
    job.state.work.solved = MatD::zeros(3, 3).into();
    for row in 0..3 { for column in 0..3 { job.state.work.rhs.set(row, column, rhs[row][column]); } }
    let pointer = job.state.work.solved.data.backing_ptr(0);
    let capacity = job.state.work.solved.data.capacity();
    job.state.reset_cursor(SubspaceStage::FactorForwardEntry);
    let mut scalar_steps = true;
    let mut retained = true;
    for _ in 0..100 {
        let before = job.state.work.solved.data.clone();
        match job.state.work.stage {
            SubspaceStage::FactorForwardEntry => job.state.advance_factor_forward(),
            SubspaceStage::FactorDiagonalEntry => job.state.advance_factor_diagonal(),
            SubspaceStage::FactorBackwardEntry => job.state.advance_factor_backward(),
            _ => break,
        }
        scalar_steps &= before.iter().zip(&job.state.work.solved.data).filter(|(left, right)| left != right).count() <= 1;
        retained &= job.state.work.solved.data.backing_ptr(0) == pointer && job.state.work.solved.data.capacity() == capacity;
    }
    let terminal = job.state.work.stage == SubspaceStage::ShiftOperatorColumnRow;
    let observed: Vec<Vec<f64>> = (0..3).map(|row| (0..3).map(|column| job.state.work.solved.get(row, column)).collect()).collect();
    close_job(&mut job);
    assert!(InteractiveJob::terminal_is_empty(&job));
    assert!(terminal && scalar_steps && retained);
    assert_eq!(observed, expected);
}

/// 🎶 Modal publication replaces every scalar before exposing terminal convergence.
#[test]
fn subspace_publication_restarts_at_zero_and_commits_convergence_after_the_last_scalar() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔢️scalar-owners/🔣️.json")).unwrap();
    let case = &fixture["publication"];
    let diagonal: Vec<f64> = serde_json::from_value(case["diagonal"].clone()).unwrap();
    let eigenvalues: Vec<f64> = serde_json::from_value(case["eigenvalues"].clone()).unwrap();
    let previous: Vec<f64> = serde_json::from_value(case["previous"].clone()).unwrap();
    let expected:Vec<Vec<f64>>=serde_json::from_value(case["prefixes"].clone()).unwrap();
    let fuels:Vec<u64>=serde_json::from_value(case["fuel"].clone()).unwrap();let commit=case["commitOpportunity"].as_u64().unwrap()as usize;
    for modes in serde_json::from_value::<Vec<usize>>(case["requestedModes"].clone()).unwrap() {
        let mut mass = Coo::new(3);
        for index in 0..3 { mass.add(index, index, 1.0); }
        let operation = test_operation(997 + modes as u64);
        let factor = LdltFactor::from_owned_parts(3, vec![Vec::new();3], diagonal.clone());
        let mut job = SubspaceIterationJob::new(operation, factor, mass.to_csr(), 3, modes, 30);
        job.state.factor_validation_complete = true;
        job.state.x = MatD::zeros(3, 3).into();
        job.state.work.candidate_x = MatD::zeros(3, 3).into();
        job.state.work.theta = eigenvalues.clone().into();
        job.state.final_theta = previous.clone().into();job.state.eigensystem_certified=true;
        job.state.converged_count = modes;
        job.state.reset_cursor(SubspaceStage::ConvergenceMode);
        job.state.work.first = modes;
        let pointer = job.state.final_theta.backing_ptr(0);
        let capacity = job.state.final_theta.capacity();
        let mut sequence = 0;
        let mut observations = Vec::new();
        for fuel in fuels.iter().copied() {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            let yielded = matches!(seen_of(job.step(&mut context)), Seen::Yield);
            observations.push((yielded && context.fuel_remaining() == 0, job.state.final_theta.iter().copied().collect::<Vec<_>>(), job.state.converged, job.state.work.first, job.state.iteration, job.state.final_theta.backing_ptr(0) == pointer && job.state.final_theta.capacity() == capacity, job.terminal_writer.is_none()));
        }
        let solution = job.solution();
        close_job(&mut job);
        assert!(InteractiveJob::terminal_is_empty(&job));
        for (index, (yielded, values, converged, cursor, iteration, retained, no_terminal)) in observations.into_iter().enumerate() {
            assert!(yielded && retained && no_terminal, "publication opportunity {index}, modes {modes}: yielded={yielded}, retained={retained}, no_terminal={no_terminal}");
            assert_eq!(values, expected[index], "publication scalar {index}, modes {modes}");
            assert_eq!(converged, index == commit, "publication convergence {index}, modes {modes}");
            assert_eq!(iteration, usize::from(index == commit));
            if index == 1 || index == 2 { assert_eq!(cursor, 0); }
        }
        assert_eq!(solution.values, eigenvalues[..modes]);
    }
}

fn restore_subspace(operation: Operation, payload: RetainedJobPayload) -> Result<SubspaceIterationJob, NumericalCheckpointFault> {
    let mut restore = SubspaceRestoreCursor::new(operation, payload);
    let mut sequence = 0;
    for _ in 0..400_000 {
        let handoff_due = restore.payload.is_none();
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, RetainedCloneGrant { maximum_items: 1, ..TEST_GRANT }), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let step = restore.step(&mut context);
        assert!(context.retained_progress().copied_items <= 1);
        match step {
            Ok(Some(job)) => { assert!(handoff_due); assert_eq!(context.retained_progress().copied_items, 1); assert_eq!(context.retained_progress().released_bytes, 0); return Ok(job); },
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
fn subspace_job_resume_and_scheduling_are_deterministic() {
    let n = 12;
    let mut k = Coo::new(n);
    let mut b = Coo::new(n);
    for i in 0..n {
        k.add(i, i, (i + 1) as f64);
        b.add(i, i, 1.0);
    }
    let factor = ldlt_factor(&k.to_csc_sym_upper()).expect("factors");
    let mass = b.to_csr();
    let operation = test_operation(105);
    let mut uninterrupted = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 4, 30);
    let mut sequence = 0;
    let expected = loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        if matches!(seen_of(uninterrupted.step(&mut context)), Seen::Complete(_)) {
            break uninterrupted.solution();
        }
    };

    let mut interrupted = SubspaceIterationJob::new(operation, factor, mass, n, 4, 30);
    let checkpoint = loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(interrupted.step(&mut context)) {
            Seen::Checkpoint(pages) => break pages,
            Seen::Yield | Seen::Preview(_) => {}
            outcome => panic!("unexpected subspace checkpoint outcome: {outcome:?}"),
        }
    };
    let mut resumed = restore_subspace(operation, checkpoint_payload(operation, &checkpoint)).expect("subspace retained checkpoint restores");
    close_job(&mut interrupted);
    loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut yielded = StepContext::new(operation.operation, operation.generation, StepBudget::new(0, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        assert!(matches!(seen_of(resumed.step(&mut yielded)), Seen::Yield));
        let mut sequence_receipt = RetainedCloneProgress::default();
        drop(yielded);
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        if matches!(seen_of(resumed.step(&mut context)), Seen::Complete(_)) {
            break;
        }
    }
    assert_eq!(resumed.solution(), expected);
    assert_eq!(resumed.preview().converged_count, 4);
    close_job(&mut uninterrupted);
    close_job(&mut resumed);
}

// #region 🔖️LongTests
/// ⏱️ `long` level: three fuel-1 subspace replays of a full checkpoint/preview cadence take ~40 s in a
/// debug build, past the fundamental profile's 15 s per-test ceiling.
mod long {
    use super::*;

    #[test]
    fn p6h_subspace_cancellation_is_observed_at_every_nested_stage_and_worker_replay_is_exact() {
        let n = 8;
        let mut stiffness = Coo::new(n);
        let mut mass = Coo::new(n);
        for index in 0..n {
            stiffness.add(index, index, (index + 1) as f64);
            mass.add(index, index, 1.0);
        }
        let factor = ldlt_factor(&stiffness.to_csc_sym_upper()).expect("factor");
        let operation = test_operation(122);
        let mass = mass.to_csr();
        let drive = |fuel: u64| {
            let mut replay = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 3, 3);
            let mut replay_sequence = 0;
            for _ in 0..200_000 {
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut replay_sequence, &mut sequence_receipt);
                match seen_of(replay.step(&mut context)) {
                    Seen::Complete(_) => {
                        let solution = replay.solution();
                        close_job(&mut replay);
                        return solution;
                    }
                    Seen::Fault(detail) => panic!("subspace replay fault: {detail:?}"),
                    _ => {}
                }
            }
            panic!("subspace replay did not reach a terminal state")
        };
        let single = drive(1);
        assert_eq!(drive(2), single);
        assert_eq!(drive(4), single);

        let mut validating = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 3, 1);
        let before = validating.state.clone();
        let mut validation_sequence = 0;
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut expired = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut validation_sequence, &mut sequence_receipt);
        assert_eq!(seen_of(validating.step(&mut expired)), Seen::Yield);
        assert!(validating.state == before);
        let mut sequence_receipt = RetainedCloneProgress::default();
        drop(expired);
        let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut validation_sequence, &mut sequence_receipt);
        assert_eq!(seen_of(validating.step(&mut one)), Seen::Yield);
        assert_eq!(validating.state.factor_validation_cursor, 1, "one construction grant validates one factor owner");
        close_job(&mut validating);

        let mut oversized = Vec::<(u32, f64)>::new();
        oversized.try_reserve_exact(NUMERICAL_OWNER_PAGE_BYTES / size_of::<(u32, f64)>() + 1).expect("hostile factor backing");
        assert!(oversized.capacity() * size_of::<(u32, f64)>() > NUMERICAL_OWNER_PAGE_BYTES);
        let mut columns = vec![Vec::new(); n];
        columns[0] = oversized;
        let mut refused_owner = SubspaceIterationJob::new(operation, LdltFactor { n, l_cols: columns, d: vec![1.0; n] }, mass.clone(), n, 3, 1);
        let mut refused_sequence = 0;
        let mut sequence_receipt = RetainedCloneProgress::default();
        drop(one);
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut refused_sequence, &mut sequence_receipt);
        assert!(matches!(seen_of(refused_owner.step(&mut context)), Seen::Fault(_)));
        close_job(&mut refused_owner);

        drop(context);
        for refused_order in [0, SUBSPACE_MAXIMUM_ORDER + 1] {
            let factor = LdltFactor { n: refused_order, l_cols: vec![Vec::new(); refused_order], d: vec![1.0; refused_order] };
            let mass = Csr::from_owned_parts(refused_order, vec![0; refused_order + 1], Vec::new(), Vec::new());
            let mut refused = SubspaceIterationJob::new(operation, factor, mass, refused_order, usize::from(refused_order != 0), 1);
            let mut refused_sequence = 0;
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut refused_sequence, &mut sequence_receipt);
            assert!(matches!(seen_of(refused.step(&mut context)), Seen::Fault(_)));
            close_job(&mut refused);
        }

        let mut publishing = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 3, 1);
        let mut publishing_sequence = 0;
        for _ in 0..200_000 {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut publishing_sequence, &mut sequence_receipt);
            let _ = seen_of(publishing.step(&mut context));
            if publishing.preview_writer.is_some() {
                break;
            }
        }
        assert!(publishing.preview_writer.is_some(), "retained preview page writer becomes interruptible before publication");
        InteractiveJob::begin_close(&mut publishing);
        close_job(&mut publishing);
        assert!(InteractiveJob::terminal_is_empty(&publishing));

        let mut job = SubspaceIterationJob::new(operation, factor, mass, n, 3, 3);
        let mut seen = std::collections::BTreeSet::new();
        let mut sequence = 0;
        for _ in 0..200_000 {
            if seen.len() == 16 {
                break;
            }
            seen.insert(job.state.work.stage as u8);
            job.state.checkpoint_due = true;
            let checkpoint = loop {
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                if let Seen::Checkpoint(pages) = seen_of(job.step(&mut context)) {
                    break pages;
                }
            };
            let mut cancelled = restore_subspace(operation, checkpoint_payload(operation, &checkpoint)).expect("stage retained checkpoint");
            let before = cancelled.state.clone();
            let token = semio_framework_job::root_cancel_token();
            token.cancel_now();
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut cancelled_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
            assert_eq!(seen_of(cancelled.step(&mut cancelled_context)), Seen::Cancelled);
            assert!(cancelled.state == before);
            close_job(&mut cancelled);
            let walked = job.state.work.stage;
            drop(cancelled_context);
            for _ in 0..100_000 {
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                if let Seen::Fault(detail) = seen_of(job.step(&mut context)) {
                    panic!("subspace stage walk fault: {detail:?}");
                }
                if job.state.work.stage != walked {
                    break;
                }
            }
        }
        assert_eq!(seen.len(), 16);

        assert!(matches!(restore_subspace(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState)), Err(NumericalCheckpointFault::Truncated)));
        let wrong_generation = Operation::new(operation.operation, operation.base_revision, semio_framework_job::Generation(operation.generation.0 + 1), operation.seed);
        job.state.checkpoint_due = true;
        let checkpoint = loop {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            if let Seen::Checkpoint(pages) = seen_of(job.step(&mut context)) {
                break pages;
            }
        };
        assert!(matches!(restore_subspace(wrong_generation, checkpoint_payload(operation, &checkpoint)), Err(NumericalCheckpointFault::Stale)));
        job.state.checkpoint_due = true;
        let checkpoint = loop {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            if let Seen::Checkpoint(pages) = seen_of(job.step(&mut context)) {
                break pages;
            }
        };
        let mut interrupted_restore = SubspaceRestoreCursor::new(operation, checkpoint_payload(operation, &checkpoint));
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut restore_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        assert!(matches!(interrupted_restore.step(&mut restore_context), Ok(None)));
        while !interrupted_restore.terminal_is_empty() {
            match interrupted_restore.close_step(grant_of(1, usize::MAX)) {
                Ok(InteractiveJobCloseStep::Pending { progress }) => assert!(progress.copied_items <= 1),
                Ok(InteractiveJobCloseStep::Complete { .. }) => {}
                Ok(InteractiveJobCloseStep::Blocked) | Err(_) => panic!("subspace restore close cannot block"),
            }
        }
        let mut closing = job;
        let mut close_turns = 0;
        loop {
            close_turns += 1;
            match close_turn(&mut closing, grant_of(1, usize::MAX)) {
                Ok(InteractiveJobCloseStep::Complete { .. }) => break,
                Ok(InteractiveJobCloseStep::Pending { progress }) => assert!(progress.copied_items <= 1),
                Ok(InteractiveJobCloseStep::Blocked) | Err(_) => panic!("fixed subspace close cannot block"),
            }
            assert!(close_turns < 200_000);
        }
        assert!(InteractiveJob::terminal_is_empty(&closing));
    }
}
// #endregion 🔖️LongTests

#[test]
fn test_funded_restore<T>(step: impl FnOnce(&mut StepContext<'_>) -> T) -> T {
    let operation = test_operation(9998);
    let mut sequence = 0;
    let mut receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    step(&mut context)
}

#[test]
fn empty_paged_backing<T>(count: usize) -> PagedList<T, SPARSE_INDEX_SPACE> {
    let mut owner = PagedList::empty();
    while owner.capacity() < count { owner.reserve_capacity_one(count, SPARSE_PAGE_BYTES).expect("original fixture backing"); }
    owner
}

#[test]
fn subspace_iteration_returns_only_certified_physical_positive_modes() {
    let n = 3;
    let mut k_coo = Coo::new(n);
    let mut b_coo = Coo::new(n);
    for (index, stiffness) in [1.0, 2.0, 3.0].into_iter().enumerate() {
        k_coo.add(index, index, stiffness);
    }
    b_coo.add(0, 0, 1.0);
    b_coo.add(2, 2, -1.0);
    let k_factor = ldlt_factor(&k_coo.to_csc_sym_upper()).expect("factors");
    let b_csr = b_coo.to_csr();

    let first = subspace_iteration(&k_factor, &b_csr, n, 3, 30);
    let replay = subspace_iteration(&k_factor, &b_csr, n, 3, 30);

    assert_eq!(first, replay);
    assert_eq!(first.values, vec![1.0]);
    assert!(first.values.iter().all(|value| value.is_finite() && *value > 0.0));
    assert!(first.values.windows(2).all(|pair| pair[0] <= pair[1]));
}

#[test]
fn subspace_shape_admission_matches_exact_paged_layout_arithmetic() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🎼️shape/🔣️.json")).unwrap();
    assert_eq!(law["indexBits"], u32::BITS);
    assert_eq!(law["scalarBytes"], size_of::<f64>());
    for row in law["cases"].as_array().unwrap() {
        let order = row["order"].as_u64().unwrap();
        let modes = row["modes"].as_u64().unwrap();
        let actual = usize::try_from(order).ok().zip(usize::try_from(modes).ok()).is_some_and(|(order, modes)| SubspaceIterationJob::admits_shape(order, modes));
        assert_eq!(actual, row[if usize::BITS == 32 { "admitted32" } else { "admitted64" }].as_bool().unwrap(), "order={order}, modes={modes}");
    }
    assert!(!SubspaceIterationJob::admits_shape(usize::MAX, 1));
    assert!(!SubspaceIterationJob::admits_shape(1, usize::MAX));
    println!("[DEBUG] {} logical spectral layouts match {}-bit owner indices and checked arithmetic; multi-page models admit and overflow refuses before construction", law["cases"].as_array().unwrap().len(), usize::BITS);
}
