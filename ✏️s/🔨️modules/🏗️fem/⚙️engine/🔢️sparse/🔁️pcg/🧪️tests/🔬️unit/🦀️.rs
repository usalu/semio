use semio_s_fem_sparse::{Coo, Csr, MountedScalarSlots, NUMERICAL_OWNER_PAGE_BYTES, NumericalCheckpointFault, parse_numerical_page, read_checkpoint_u64};
use super::*;
use semio_framework_job::StepBudget;
use super::*;
use semio_s_fem_test_support::close_turn;
use semio_s_fem_test_support::accounted_close;
use semio_s_fem_test_support::checkpoint_payload;
use semio_s_fem_test_support::close_job;
use semio_s_fem_test_support::close_payload;
use semio_s_fem_test_support::close_writer;
use semio_s_fem_test_support::grant_of;
use semio_s_fem_test_support::pages_bytes;
use semio_s_fem_test_support::payload_from_pages;
use semio_s_fem_test_support::seen_of;
use semio_s_fem_test_support::test_operation;
use semio_s_fem_test_support::tuple_of;
use semio_s_fem_test_support::Seen;
use semio_s_fem_test_support::TEST_GRANT;

#[test]
fn mathematical_pcg_and_original_native_job_share_neutral_outcomes() {
    use semio_s_fem_test_support::OriginalNumericalFixture;
    use semio_framework_job::{admit_original_construction, OriginalConstructionDemand};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/❄️cold/🔣️.json")).unwrap();
    for (index, sample) in corpus["cases"].as_array().unwrap().iter().enumerate() {
        let values = |key: &str| VecD::from_vec(sample[key].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect());
        let n = sample["matrix"].as_array().unwrap().len();
        let mut coo = Coo::new(n);
        for (row, cells) in sample["matrix"].as_array().unwrap().iter().enumerate() {
            for (column, value) in cells.as_array().unwrap().iter().enumerate() { if value.as_f64().unwrap() != 0.0 { coo.add(row, column, value.as_f64().unwrap()); } }
        }
        let a = coo.to_csr();
        let rhs = values("rhs");
        let mut mathematical = values("initial");
        let tolerance = sample["tolerance"].as_f64().unwrap();
        let maximum = sample["maximumIterations"].as_u64().unwrap() as usize;
        let expected = pcg(&a, &rhs, &mut mathematical, tolerance, maximum);
        assert_eq!(expected.iterations, sample["iterations"].as_u64().unwrap() as usize);
        assert_eq!(expected.converged, sample["converged"].as_bool().unwrap());
        for (i, value) in sample["solution"].as_array().unwrap().iter().enumerate() { assert!((mathematical.get(i) - value.as_f64().unwrap()).abs() < 1e-10); }
        let operation = test_operation(180_000 + index as u64);
        let mut original = OriginalNumericalFixture::new(operation);
        let mut inputs = Some((a, rhs, values("initial")));
        let mut candidate = None;
        let capacity = 5 * n * size_of::<f64>();
        let demand = OriginalConstructionDemand { items: 1, copy_bytes: size_of::<PcgJob>(), capacity_bytes: capacity, release_bytes: 0, depth: 1 };
        assert!(original.turn(TEST_GRANT, 1, u64::MAX, |cx| admit_original_construction(cx, &mut candidate, demand, |_| {
            let (a, b, x) = inputs.take().unwrap();
            let job = PcgJob::new(operation, a, b, x, tolerance, maximum, 4);
            Ok((job, RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<PcgJob>(), retained_capacity_bytes: capacity, ..Default::default() }))
        })).expect("actual original PCG construction"));
        let mut job = candidate.unwrap();
        let mut complete = false;
        for _ in 0..200_000 {
            let end = original.turn(TEST_GRANT, 4, u64::MAX, |cx| semio_s_fem_sparse::numerical_batch_step(&mut job, cx)).expect("same original PCG turn");
            match end { semio_s_fem_sparse::NumericalBatchEnd::Complete => { complete = true; break; }, semio_s_fem_sparse::NumericalBatchEnd::Continue => {}, other => panic!("unexpected PCG outcome: {other:?}") }
        }
        assert!(complete);
        let (solution, actual) = job.solution();
        assert_eq!(actual, expected);
        for i in 0..n { assert_eq!(solution.get(i), mathematical.get(i)); }
        original.close(&mut job);
        original.finish();
    }
}

#[test]
fn pcg_matches_ldlt_and_dense_lu() {
    let n = 8;
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
    let coo = Coo::graph_laplacian_plus_identity(n, &edges);
    let dense = coo.to_dense();
    let x_expected = VecD::from_vec((0..n).map(|i| (i as f64) * 0.3 - 1.0).collect());
    let b = dense.mul_vec(&x_expected);
    let csr = coo.to_csr();
    let mut x0 = VecD::zeros(n);
    let stats = pcg(&csr, &b, &mut x0, 1e-10, 500);
    assert!(stats.converged);
    let lu = dense.lu_solve(&b).expect("dense solvable");
    for i in 0..n {
        assert!((x0.get(i) - lu.get(i)).abs() < 1e-6);
    }
}

#[test]
fn pcg_converges_immediately_when_initial_guess_is_already_exact() {
    let mut coo = Coo::new(3);
    coo.add(0, 0, 2.0);
    coo.add(1, 1, 3.0);
    coo.add(2, 2, 4.0);
    let csr = coo.to_csr();
    let mut x0 = VecD::from_vec(vec![1.0, 2.0, 3.0]);
    let b = csr.mul_vec(&x0);
    let stats = pcg(&csr, &b, &mut x0, 1e-8, 100);
    assert_eq!(stats.iterations, 0);
    assert!(stats.converged);
}

/// ⏱️ `pcg` with `max_iter: 0` never enters its iteration loop and reports `converged: false`.
#[test]
fn pcg_reports_not_converged_when_max_iter_is_zero() {
    let mut coo = Coo::new(3);
    coo.add(0, 0, 2.0);
    coo.add(1, 1, 3.0);
    coo.add(2, 2, 4.0);
    let csr = coo.to_csr();
    let b = VecD::from_vec(vec![1.0, 1.0, 1.0]);
    let mut x0 = VecD::zeros(3);
    let stats = pcg(&csr, &b, &mut x0, 1e-12, 0);
    assert_eq!(stats.iterations, 0);
    assert!(!stats.converged);
}

/// ⏱️ `pcg` against an all-zero operator has zero search-direction curvature (`pᵀAp = 0`) on its
/// very first step, hitting the early `break` guard against dividing by zero — reported as
/// `converged: false` after exactly 1 iteration.
#[test]
fn pcg_breaks_on_zero_curvature_direction() {
    let coo = Coo::new(3); // no entries added: A is the zero operator
    let csr = coo.to_csr();
    let b = VecD::from_vec(vec![1.0, 1.0, 1.0]);
    let mut x0 = VecD::zeros(3);
    let stats = pcg(&csr, &b, &mut x0, 1e-12, 50);
    assert_eq!(stats.iterations, 1);
    assert!(!stats.converged);
}

#[test]
fn pcg_job_construction_checks_order_before_backing_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/⛽️publication-grant/🔣️.json")).unwrap();
    let ceiling = fixture["construction"]["backingCeilingBytes"].as_u64().unwrap() as usize;
    let mut observations = Vec::new();
    for (index, case) in fixture["construction"]["cases"].as_array().unwrap().iter().enumerate() {
        let order = case["order"].as_u64().unwrap() as usize;
        let mut coo = Coo::new(order);
        for row in 0..order { coo.add(row, row, 1.0); }
        let mut construction = PcgJobConstruction::new(test_operation(1_020 + index as u64), coo.to_csr());
        let result = construction.step_one();
        let allocated = construction.b.0.capacity() * size_of::<f64>();
        let initialized = construction.b.len();
        let mut closed = false;
        for _ in 0..20_000 {
            let step = tuple_of(construction.close_step(grant_of(1, ceiling.max(allocated))).expect("REASON"));
            if step.0 { closed = true; break; }
        }
        observations.push((case.clone(), result, allocated, initialized, closed));
    }
    for (case, result, allocated, initialized, closed) in observations {
        assert!(closed);
        assert_eq!(initialized, 0);
        if case["rejected"].as_bool().unwrap() {
            assert_eq!(result, Err(b"pcg-construction-owner-page-capacity" as &'static [u8]));
            assert_eq!(allocated, 0, "refused order must never allocate its oversized backing");
        } else {
            assert_eq!(result, Ok(false));
            assert!(allocated >= case["requestedBytes"].as_u64().unwrap() as usize && allocated <= ceiling);
        }
    }
}

/// 🧮 Retained vector and mounted RHS construction preserve the independent NumPy norm.
#[test]
fn pcg_job_construction_uses_actual_rhs_norm() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️pcg-wire/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in fixture["rhsNorm"]["cases"].as_array().unwrap() {
        let rhs: Vec<f64> = row["rhs"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect();
        for mounted in [false, true] {
            let mut matrix = Coo::new(rhs.len());
            for index in 0..rhs.len() { matrix.add(index, index, 1.0); }
            let operation = test_operation(1_380);
            let mut construction = if mounted {
                let mut input = MountedScalarSlots::new();
                while !input.admit_one(rhs.len()).unwrap() {}
                for value in &rhs { input.push(*value).unwrap(); }
                PcgJobConstruction::new_with_mounted_rhs(operation, matrix.to_csr(), input).unwrap_or_else(|_| panic!("matching mounted RHS rejected"))
            } else {
                PcgJobConstruction::new_with_rhs(operation, matrix.to_csr(), VecD::from_vec(rhs.clone())).unwrap_or_else(|_| panic!("matching vector RHS rejected"))
            };
            let mut result = None;
            let mut turns = 0;
            for turn in 1..20_000 {
                if construction.step_one().unwrap() {
                    result = construction.take_complete();
                    turns = turn;
                    break;
                }
            }
            let observed = result.as_ref().map(|job| (job.state.b_norm, job.state.b.0.clone()));
            let solved = result.map(|job| drive_pcg_job(job, operation));
            let mut closed = false;
            for _ in 0..20_000 {
                if tuple_of(construction.close_step(grant_of(1, PCG_SCALAR_BACKING_BYTES)).expect("REASON")).0 { closed = true; break; }
            }
            observations.push((row.clone(), mounted, rhs.clone(), observed, solved, turns, closed));
        }
    }
    for (row, mounted, rhs, observed, solved, turns, closed) in observations {
        assert!(closed);
        let (actual, actual_rhs) = observed.expect("retained constructor completed");
        let expected = row["norm"].as_f64().unwrap().max(1e-300);
        assert_eq!(actual_rhs, rhs);
        assert!((actual - expected).abs() <= expected * 1e-14, "RHS {} mounted={mounted}: {actual} vs {expected}", row["id"]);
        assert!(turns >= rhs.len() * 7, "each owned RHS scalar participates in its own admitted work");
        let (solution, stats) = solved.expect("retained solve completed");
        assert!(stats.converged);
        for (index, value) in rhs.iter().enumerate() {
            assert!((solution.0[index] - value).abs() <= value.abs().max(1e-300) * 1e-14, "RHS {} mounted={mounted}, scalar={index}", row["id"]);
        }
    }
}


/// 🚫 Invalid RHS values retain their first constructor fault and close every backing owner.
#[test]
fn pcg_job_construction_rejects_nonfinite_rhs_without_losing_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️pcg-wire/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in fixture["rhsNorm"]["invalid"].as_array().unwrap() {
        let rhs: Vec<f64> = row["rhsBits"].as_array().unwrap().iter().map(|bits| f64::from_bits(u64::from_str_radix(bits.as_str().unwrap(), 16).unwrap())).collect();
        for mounted in [false, true] {
            let mut matrix = Coo::new(rhs.len());
            for index in 0..rhs.len() { matrix.add(index, index, 1.0); }
            let mut construction = if mounted {
                let mut input = MountedScalarSlots::new();
                while !input.admit_one(rhs.len()).unwrap() {}
                for value in &rhs { input.push(*value).unwrap(); }
                PcgJobConstruction::new_with_mounted_rhs(test_operation(1_381), matrix.to_csr(), input).unwrap_or_else(|_| panic!("matching mounted RHS rejected before retained validation"))
            } else {
                PcgJobConstruction::new_with_rhs(test_operation(1_381), matrix.to_csr(), VecD::from_vec(rhs.clone())).unwrap_or_else(|_| panic!("matching vector RHS rejected before retained validation"))
            };
            let mut failure = None;
            for _ in 0..100 {
                match construction.step_one() {
                    Err(fault) => { failure = Some(fault); break; }
                    Ok(true) => break,
                    Ok(false) => {}
                }
            }
            let signature = |owner: &PcgJobConstruction| (owner.stage, owner.cursor, owner.rhs_norm.to_bits(), owner.b.0.as_ptr() as usize, owner.b.0.capacity(), owner.matrix.is_some(), owner.mounted_b.is_some());
            let before = signature(&construction);
            let repeated = [construction.step_one(), construction.step_one()];
            let stable = signature(&construction) == before;
            let zeroes_unallocated = [&construction.x, &construction.diag, &construction.r, &construction.z, &construction.p, &construction.ap].iter().all(|owner| owner.0.capacity() == 0);
            let rhs_bytes = construction.b.0.capacity() * size_of::<f64>();
            let mut closed = false;
            let mut released = 0;
            for _ in 0..20_000 {
                let step = tuple_of(construction.close_step(grant_of(1, PCG_SCALAR_BACKING_BYTES)).expect("REASON"));
                assert!(step.2 <= PCG_SCALAR_BACKING_BYTES);
                released += step.2;
                if step.0 { closed = true; break; }
            }
            let empty = construction.matrix.is_none() && construction.mounted_b.is_none() && construction.complete.is_none()
                && [&construction.b, &construction.x, &construction.diag, &construction.r, &construction.z, &construction.p, &construction.ap].iter().all(|owner| owner.0.capacity() == 0);
            eprintln!("PCG invalid RHS id={}, mounted={mounted}, fault={failure:?}, stable={stable}, rhsBytes={rhs_bytes}, released={released}, closed={closed}, empty={empty}", row["id"]);
            observations.push((row.clone(), failure, repeated, stable, zeroes_unallocated, rhs_bytes, released, closed, empty));
        }
    }
    for (row, failure, repeated, stable, zeroes_unallocated, rhs_bytes, released, closed, empty) in observations {
        let expected = row["fault"].as_str().unwrap().as_bytes();
        assert_eq!(failure, Some(expected));
        assert!(repeated.iter().all(|result| result.as_ref().err().is_some_and(|fault| *fault == expected)));
        assert!(stable && zeroes_unallocated && closed && empty, "invalid RHS {}", row["id"]);
        assert!(rhs_bytes > 0 && released >= rhs_bytes);
    }
}

/// 📜 PCG publication uses canonical retained numerical pages and derives output scalars in place.
#[test]
fn pcg_job_publication_matches_canonical_wire_pages() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️pcg-wire/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let operation = test_operation(fixture["operation"].as_u64().unwrap());
        let mut matrix = Coo::new(2);
        matrix.add(0, 0, 2.0);
        matrix.add(1, 1, 4.0);
        let mut job = PcgJob::new(operation, matrix.to_csr(), VecD::from_vec(vec![1.0, 2.0]), VecD::from_vec(vec![-0.25, 0.5]), 1e-12, 20, 1);
        let kind = case["id"].as_str().unwrap();
        job.state.stage = match case["stage"].as_u64().unwrap() { 0 => PcgStage::InitializeDiagonal, 4 => PcgStage::IterationSpmv, 8 => PcgStage::Complete, _ => unreachable!() };
        job.state.converged = case["converged"].as_bool().unwrap();
        job.state.coarse_published = case["coarsePublished"].as_bool().unwrap();
        job.state.checkpoint_due = kind == "checkpoint";
        job.state.preview_due = kind == "preview";
        let mut sequence = 0;
        let mut pages = None;
        let mut turns = 0;
        for turn in 1..10_000 {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            let lent = match seen_of(job.step(&mut context)) {
                Seen::Checkpoint(lent) | Seen::Preview(lent) | Seen::Complete(lent) => Some(lent),
                Seen::Fault(_) | Seen::Cancelled => break,
                Seen::Yield => None,
            };
            if let Some(lent) = lent {
                pages = Some(lent.iter().map(|page| page.iter().map(|byte| format!("{byte:02x}")).collect::<String>()).collect::<Vec<_>>());
                turns = turn;
                break;
            }
        }
        let mut delivered_again = false;
        if kind == "complete" {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            delivered_again = matches!(seen_of(job.step(&mut context)), Seen::Complete(_));
        }
        close_job(&mut job);
        let closed = InteractiveJob::terminal_is_empty(&job);
        eprintln!("PCG canonical {kind}: turns={turns}, pages={}, delivered_again={delivered_again}, closed={closed}", pages.as_ref().map_or(0, Vec::len));
        observations.push((case.clone(), pages, turns, delivered_again, closed));
    }
    for (case, pages, turns, delivered_again, closed) in observations {
        assert!(closed);
        assert!(!delivered_again, "terminal publication transfers exactly once");
        assert!(turns > 1, "a payload cannot be materialized in one opportunity");
        let expected: Vec<String> = case["fields"].as_array().unwrap().iter().map(|field| field["hex"].as_str().unwrap().to_owned()).collect();
        assert_eq!(pages, Some(expected), "{}", case["id"]);
    }
}

/// 🪪 A PCG checkpoint belongs to one exact operation, revision, generation and seed.
#[test]
fn pcg_job_checkpoint_rejects_foreign_operation_identity() {
    let operation = test_operation(1_048);
    let mut matrix = Coo::new(1);
    matrix.add(0, 0, 2.0);
    let mut job = PcgJob::new(operation, matrix.to_csr(), VecD::from_vec(vec![1.0]), VecD::zeros(1), 1e-12, 20, 1);
    let identities = [
        Operation::new(semio_framework_job::OperationId(operation.operation.0 + 1), operation.base_revision, operation.generation, operation.seed),
        Operation::new(operation.operation, semio_framework_job::RevisionId(operation.base_revision.0 + 1), operation.generation, operation.seed),
        Operation::new(operation.operation, operation.base_revision, semio_framework_job::Generation(operation.generation.0 + 1), operation.seed),
        Operation::new(operation.operation, operation.base_revision, operation.generation, operation.seed + 1),
    ];
    let mut rejected = Vec::new();
    for identity in identities {
        match restore_pcg(identity, pcg_checkpoint_payload(&job)) {
            Ok(mut resumed) => {
                close_job(&mut resumed);
                assert!(InteractiveJob::terminal_is_empty(&resumed));
                rejected.push(false);
            }
            Err(_) => rejected.push(true),
        }
    }
    close_job(&mut job);
    assert!(InteractiveJob::terminal_is_empty(&job));
    assert_eq!(rejected, vec![true; 4]);
}

/// 📣️ PCG publication consumes its own grant and preserves pending control state on refusal.
#[test]
fn pcg_job_publication_grants_preserve_pending_state_and_work_cursor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/⛽️publication-grant/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for (index, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let operation = test_operation(980 + index as u64);
        let mut matrix = Coo::new(1);
        matrix.add(0, 0, 2.0);
        let mut job = PcgJob::new(operation, matrix.to_csr(), VecD::from_vec(vec![1.0]), VecD::zeros(1), 1e-12, 20, 1);
        let kind = case["kind"].as_str().unwrap();
        job.state.checkpoint_due = kind == "checkpoint";
        job.state.preview_due = kind == "preview";
        if kind == "complete" { job.state.stage = PcgStage::Complete; }
        let mut sequence = 0;
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(case["fuel"].as_u64().unwrap(), case["deadline"].as_u64().unwrap(), TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let outcome = match seen_of(job.step(&mut context)) {
            Seen::Yield => "yield",
            Seen::Preview(_) => "preview",
            Seen::Checkpoint(_) => "checkpoint",
            Seen::Complete(_) => "complete",
            Seen::Fault(_) => "fault",
            Seen::Cancelled => "cancelled",
        };
        let pending = match kind { "checkpoint" => job.state.checkpoint_due, "preview" => job.state.preview_due, _ => job.state.stage == PcgStage::Complete };
        let mut observed = serde_json::json!({ "pending": pending, "cursor": job.state.entry_cursor, "fuelRemaining": context.fuel_remaining(), "outcome": outcome });
        let mut closed = false;
        for _ in 0..1_024 {
            let (terminal, items, bytes) = tuple_of(close_turn(&mut job, grant_of(1, NUMERICAL_OWNER_PAGE_BYTES)).expect("REASON"));
            assert!(items <= 1 && bytes <= NUMERICAL_OWNER_PAGE_BYTES);
            if terminal { closed = true; break; }
        }
        assert!(closed, "publication fixture closes its exact matrix and scalar owners");
        observed["terminalEmpty"] = serde_json::json!(InteractiveJob::terminal_is_empty(&job));
        observations.push((observed, case["expected"].clone()));
    }
    for (index, (observed, expected)) in observations.into_iter().enumerate() {
        assert_eq!(observed, expected, "publication fixture {index}");
    }
}

/// 🧭️ Each admitted initial PCG scalar fills the already owned search direction.
#[test]
fn pcg_job_initial_precondition_preserves_admitted_direction_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔢️scalar-owners/🔣️.json")).unwrap();
    let case = &fixture["precondition"];
    let diagonal: Vec<f64> = serde_json::from_value(case["diagonal"].clone()).unwrap();
    let residual: Vec<f64> = serde_json::from_value(case["residual"].clone()).unwrap();
    let expected: Vec<Vec<f64>> = serde_json::from_value(case["steps"].clone()).unwrap();
    let mut matrix = Coo::new(3);
    for (index, value) in diagonal.iter().copied().enumerate() { matrix.add(index, index, value); }
    let operation = test_operation(995);
    let mut job = PcgJob::new(operation, matrix.to_csr(), VecD::from_vec(residual.clone()), VecD::zeros(3), 1e-12, 20, 1);
    job.state.diag = VecD::from_vec(diagonal);
    job.state.r = VecD::from_vec(residual);
    job.state.stage = PcgStage::InitialPrecondition;
    let pointer = job.state.p.0.as_ptr();
    let capacity = job.state.p.0.capacity();
    let mut sequence = 0;
    let mut observed = Vec::new();
    for _ in 0..3 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        assert!(matches!(seen_of(job.step(&mut context)), Seen::Yield));
        assert_eq!(context.fuel_remaining(), 0);
        observed.push((job.state.p.0.clone(), job.state.p.0.as_ptr() == pointer && job.state.p.0.capacity() == capacity));
    }
    close_job(&mut job);
    assert!(InteractiveJob::terminal_is_empty(&job));
    for (index, (direction, retained)) in observed.into_iter().enumerate() {
        assert_eq!(direction, expected[index], "precondition scalar {index}");
        assert_eq!(retained, case["retainsBacking"].as_bool().unwrap(), "direction backing {index}");
    }
}

#[test]
fn pcg_job_publication_every_cut_cancels_without_losing_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️pcg-wire/🔣️.json")).unwrap();
    let mut failures = Vec::new();
    let mut cuts = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let turns = 2 + case["fields"].as_array().unwrap().iter().map(|field| 4 + field["values"].as_array().unwrap().len()).sum::<usize>();
        let kind = case["id"].as_str().unwrap();
        for cut in 0..turns {
            let operation = test_operation(1_040);
            let mut matrix = Coo::new(2);
            matrix.add(0, 0, 2.0);
            matrix.add(1, 1, 4.0);
            let mut job = PcgJob::new(operation, matrix.to_csr(), VecD::from_vec(vec![1.0, 2.0]), VecD::from_vec(vec![-0.25, 0.5]), 1e-12, 20, 1);
            job.state.stage = match kind { "checkpoint" => PcgStage::InitializeDiagonal, "preview" => PcgStage::IterationSpmv, _ => PcgStage::Complete };
            job.state.checkpoint_due = kind == "checkpoint";
            job.state.preview_due = kind == "preview";
            job.state.coarse_published = kind != "checkpoint";
            job.state.converged = kind == "complete";
            let mut sequence = 0;
            let mut yielded = true;
            for _ in 0..cut {
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                yielded &= matches!(seen_of(job.step(&mut context)), Seen::Yield);
            }
            let snapshot = |job: &PcgJob| (
                job.state.checkpoint_control(operation), job.state.checkpoint_due, job.state.preview_due, job.terminal_published,
                job.publication.as_ref().map(|publication| (publication.cursor, publication.writer.page_count(), publication.writer.staged_page_len(), publication.complete)),
                pcg_physical_bytes(job),
            );
            let before = snapshot(&job);
            for (fuel, deadline) in [(0, u64::MAX), (1, 0)] {
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, deadline, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
                yielded &= matches!(seen_of(job.step(&mut context)), Seen::Yield) && snapshot(&job) == before;
            }
            let token = semio_framework_job::root_cancel_token();
            token.cancel_now();
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
            let cancelled = matches!(seen_of(job.step(&mut context)), Seen::Cancelled) && snapshot(&job) == before;
            let zero_items = matches!(close_turn(&mut job, grant_of(0, usize::MAX)), Ok(InteractiveJobCloseStep::Pending { progress }) if progress == RetainedCloneProgress::default()) && snapshot(&job) == before;
            let has_pages = job.publication.as_ref().is_some_and(|publication| publication.writer.page_count() != 0 || publication.writer.staged_page_len().is_some());
            let subexact = if has_pages || job.desk.lent.is_some() {
                matches!(close_turn(&mut job, grant_of(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - 1)), Ok(InteractiveJobCloseStep::Pending { progress }) if progress.copied_items <= 1 && progress.released_bytes < semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) && snapshot(&job) == before
            } else {
                true
            };
            job.begin_close();
            let (released, exact, bounded, complete) = accounted_close(&mut job, |job| close_turn(job, grant_of(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)).expect("REASON"), pcg_physical_bytes);
            let terminal = InteractiveJob::terminal_is_empty(&job) && pcg_physical_bytes(&job) == 0;
            if !(yielded && cancelled && zero_items && subexact && bounded && exact && complete && terminal && released == before.5) {
                failures.push(format!("{kind}/{cut}: yield={yielded}, cancel={cancelled}, zero={zero_items}, subexact={subexact}, bounded={bounded}, exact={exact}, complete={complete}, terminal={terminal}, physical={}/{released}", before.5));
            }
            cuts += 1;
        }
    }
    assert_eq!(cuts, 173);
    assert!(failures.is_empty(), "{failures:?}");
}

/// ⏸️ Every PCG restore cut retains its exact input and candidate until cancellation cleanup.
#[test]
fn pcg_job_restore_every_cut_cancels_without_advancing_candidate() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️pcg-wire/🔣️.json")).unwrap();
    let pages: Vec<Vec<u8>> = fixture["cases"][0]["fields"].as_array().unwrap().iter().map(|field| {
        field["hex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
    }).collect();
    let operation = test_operation(1_040);
    let mut baseline = PcgRestoreCursor::new(operation, checkpoint_payload(operation, &pages));
    let mut sequence = 0;
    let mut turns = 0;
    let mut fields = 0u16;
    for turn in 1..10_000 {
        if baseline.expected_field < 11 { fields |= 1 << baseline.expected_field; }
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match baseline.step(&mut context) {
            Ok(Some(mut job)) => { close_pcg_job(&mut job); turns = turn; break; }
            Ok(None) => {}
            Err(_) => break,
        }
    }
    while !baseline.terminal_is_empty() { let _ = baseline.close_step(grant_of(1, usize::MAX)); }
    assert!(turns > 0 && fields == 0x7ff);
    let retained = |cursor: &PcgRestoreCursor| cursor.payload.as_ref().map_or(0, |payload| payload.page_count() * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) + cursor.job.as_ref().map_or(0, pcg_physical_bytes);
    let snapshot = |cursor: &PcgRestoreCursor| (cursor.page_slot, cursor.page_entry, cursor.expected_field, format!("{:?}", cursor.owner_cursor), retained(cursor));
    let mut failures = Vec::new();
    for cut in 0..turns {
        let mut restore = PcgRestoreCursor::new(operation, checkpoint_payload(operation, &pages));
        let mut pending = true;
        for _ in 0..cut {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            match restore.step(&mut context) {
                Ok(None) => {}
                Ok(Some(mut job)) => { close_pcg_job(&mut job); pending = false; }
                Err(_) => pending = false,
            }
        }
        let before = snapshot(&restore);
        for (fuel, deadline) in [(0, u64::MAX), (1, 0)] {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, deadline, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            match restore.step(&mut context) {
                Ok(None) => {}
                Ok(Some(mut job)) => { close_pcg_job(&mut job); pending = false; }
                Err(_) => pending = false,
            }
            pending &= snapshot(&restore) == before;
        }
        let token = semio_framework_job::root_cancel_token();
        token.cancel_now();
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
        let cancelled = match restore.step(&mut context) {
            Err(NumericalCheckpointFault::Cancelled) => true,
            Ok(Some(mut job)) => { close_pcg_job(&mut job); false }
            _ => false,
        };
        let mut sequence_receipt = RetainedCloneProgress::default();
        drop(context);
        let mut later = StepContext::new(semio_framework_job::OperationId(operation.operation.0 + 1), operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let sticky = restore.step(&mut later).err() == Some(NumericalCheckpointFault::Cancelled) && snapshot(&restore) == before;
        let zero = matches!(restore.close_step(grant_of(0, usize::MAX)), Ok(InteractiveJobCloseStep::Pending { progress }) if progress == RetainedCloneProgress::default()) && snapshot(&restore) == before;
        let held = matches!(restore.close_step(grant_of(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - 1)), Ok(InteractiveJobCloseStep::Pending { progress }) if progress.copied_items <= 1 && progress.released_bytes < semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) && retained(&restore) == before.4;
        let (released, exact, bounded, complete) = accounted_close(&mut restore, |restore| restore.close_step(grant_of(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)).expect("REASON"), |restore| retained(restore));
        let terminal = restore.terminal_is_empty() && retained(&restore) == 0;
        if !(pending && cancelled && sticky && zero && held && bounded && exact && complete && terminal && released == before.4) {
            failures.push(format!("cut={cut}, pending={pending}, cancel={cancelled}, sticky={sticky}, zero={zero}, held={held}, bounded={bounded}, exact={exact}, complete={complete}, terminal={terminal}, physical={}/{released}", before.4));
        }
    }
    eprintln!("PCG restore cancellation: {turns} cuts, fields={fields:#x}, failures={}", failures.len());
    assert!(failures.is_empty(), "{failures:?}");
}

/// 🧱 PCG restores CSR continuation pages and solves the independent dense rank-one fixture.
#[test]
fn pcg_job_checkpoint_dense_csr_continuations_restore_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️pcg-wire/🔣️.json")).unwrap();
    let dense = &fixture["denseRestore"];
    let n = dense["order"].as_u64().unwrap() as usize;
    let mut matrix = Coo::new(n);
    for row in 0..n {
        for column in 0..n {
            matrix.add(row, column, if row == column { dense["diagonal"].as_f64().unwrap() } else { dense["offDiagonal"].as_f64().unwrap() });
        }
    }
    let operation = test_operation(1_051);
    let mut job = PcgJob::new(operation, matrix.to_csr(), VecD::from_vec((1..=n).map(|value| value as f64).collect()), VecD::zeros(n), 1e-12, 20, 1);
    let payload = pcg_checkpoint_payload(&job);
    let pages: Vec<Vec<u8>> = (0..payload.page_count()).map(|index| payload.page(index).unwrap().to_vec()).collect();
    let coordinates: Vec<(u16, usize)> = pages.iter().map(|bytes| { let page = parse_numerical_page(bytes, b"FEMPCP1\0").unwrap(); (page.field, page.item) }).collect();
    let resumed = restore_pcg(operation, payload).expect("dense paged checkpoint restores");
    let restored_pages = pcg_checkpoint_pages(&resumed);
    close_pcg_job(&mut job);
    let (actual, stats) = drive_pcg_job(resumed, operation);
    let expected: Vec<f64> = serde_json::from_value(dense["expected"].clone()).unwrap();
    assert_eq!(pages, restored_pages);
    assert_eq!(coordinates.iter().filter(|(field, _)| *field == 2).map(|(_, item)| *item).collect::<Vec<_>>(), serde_json::from_value::<Vec<usize>>(dense["indexPageItems"].clone()).unwrap());
    assert_eq!(coordinates.iter().filter(|(field, _)| *field == 3).map(|(_, item)| *item).collect::<Vec<_>>(), serde_json::from_value::<Vec<usize>>(dense["valuePageItems"].clone()).unwrap());
    assert!(stats.converged);
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.0.iter().zip(expected) { assert!((actual - expected).abs() < 1e-12); }
}

/// 🫙 An empty PCG checkpoint owns only its CSR zero pointer and restores a finite terminal state.
#[test]
fn pcg_job_checkpoint_empty_restore_closes_all_owners() {
    let operation = test_operation(1_052);
    let mut job = PcgJob::new(operation, Coo::new(0).to_csr(), VecD::zeros(0), VecD::zeros(0), 1e-12, 20, 1);
    let payload = pcg_checkpoint_payload(&job);
    let resumed = restore_pcg(operation, payload).expect("empty checkpoint restores");
    let input_bytes = pcg_physical_bytes(&job);
    let restored_bytes = pcg_physical_bytes(&resumed);
    close_pcg_job(&mut job);
    let (solution, stats) = drive_pcg_job(resumed, operation);
    assert_eq!(input_bytes, restored_bytes);
    assert_eq!(solution.len(), 0);
    assert!(stats.converged && stats.residual_norm == 0.0);
}

fn pcg_physical_bytes(job: &PcgJob) -> usize {
    let state = &job.state;
    let vectors = [&state.b.0, &state.x.0, &state.diag.0, &state.r.0, &state.z.0, &state.p.0, &state.ap.0];
    state.a.physical_backing_bytes().into_iter().sum::<usize>() + vectors.into_iter().map(|owner| owner.capacity() * size_of::<f64>()).sum::<usize>()
        + job.publication.as_ref().map_or(0, |publication| (publication.writer.page_count() + usize::from(publication.writer.staged_page_len().is_some())) * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)
        + job.desk.lent.as_ref().map_or(0, |payload| payload.page_count() * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)
}

/// 🧯 Every malformed PCG owner retains its first fault and physically drains the exact candidate.
#[test]
fn pcg_job_restore_rejects_malformed_owners_and_closes_exact_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️pcg-wire/🔣️.json")).unwrap();
    let pages: Vec<Vec<u8>> = fixture["cases"][0]["fields"].as_array().unwrap().iter().map(|field| {
        field["hex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
    }).collect();
    let operation = test_operation(1_040);
    let mut observations = Vec::new();
    for case in fixture["faultCases"].as_array().unwrap() {
        let mut altered = pages.clone();
        let page = case["page"].as_u64().unwrap() as usize;
        match case["action"].as_str().unwrap() {
            "write" => {
                let value = case["value"].as_str().unwrap().parse::<u64>().unwrap().to_le_bytes();
                let offset = case["offset"].as_u64().unwrap() as usize;
                let width = case["width"].as_u64().unwrap() as usize;
                altered[page][offset..offset + width].copy_from_slice(&value[..width]);
            }
            "truncate" => altered[page].truncate(case["length"].as_u64().unwrap() as usize),
            "remove" => { altered.remove(page); }
            "append" => altered.push(altered[page].clone()),
            _ => unreachable!(),
        }
        let mut restore = PcgRestoreCursor::new(operation, checkpoint_payload(operation, &altered));
        let mut sequence = 0;
        let mut error = None;
        for _ in 0..10_000 {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
            match restore.step(&mut context) {
                Ok(None) => {}
                Ok(Some(mut job)) => { close_pcg_job(&mut job); break; }
                Err(fault) => { error = Some(fault); break; }
            }
        }
        let retained = |cursor: &PcgRestoreCursor| cursor.payload.as_ref().map_or(0, |payload| payload.page_count() * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) + cursor.job.as_ref().map_or(0, pcg_physical_bytes);
        let before = (restore.page_slot, restore.page_entry, restore.expected_field, retained(&restore));
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut later = StepContext::new(semio_framework_job::OperationId(operation.operation.0 + 1), operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let sticky = restore.step(&mut later).err() == error;
        let unchanged = before == (restore.page_slot, restore.page_entry, restore.expected_field, retained(&restore));
        let zero_items = matches!(restore.close_step(grant_of(0, usize::MAX)), Ok(InteractiveJobCloseStep::Pending { progress }) if progress == RetainedCloneProgress::default()) && retained(&restore) == before.3;
        let subexact = if restore.payload.as_ref().is_some_and(|payload| payload.page_count() != 0) {
            matches!(restore.close_step(grant_of(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - 1)), Ok(InteractiveJobCloseStep::Pending { progress }) if progress.copied_items <= 1 && progress.released_bytes < semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) && retained(&restore) == before.3
        } else { true };
        let (released, exact, bounded, complete) = accounted_close(&mut restore, |restore| restore.close_step(grant_of(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)).expect("REASON"), |restore| retained(restore));
        let terminal = restore.terminal_is_empty() && retained(&restore) == 0;
        observations.push((case.clone(), error, sticky, unchanged, zero_items, subexact, bounded && exact && complete, terminal, before.3, released));
    }
    for (case, error, sticky, unchanged, zero_items, subexact, bounded, terminal, retained, released) in observations {
        assert!(sticky && unchanged && zero_items && subexact && bounded && terminal, "{}", case["id"]);
        assert_eq!(error.map(|fault| format!("{fault:?}")), Some(case["fault"].as_str().unwrap().to_owned()), "{}", case["id"]);
        assert_eq!(retained, released, "{}", case["id"]);
    }
}

fn close_pcg_job(job: &mut PcgJob) {
    close_job(job);
}

fn pcg_checkpoint_payload(job: &PcgJob) -> RetainedJobPayload {
    let mut publication = PcgPublication::new(PcgPublicationKind::Checkpoint);
    let mut sequence = 0;
    for _ in 0..2_000_000 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(job.operation.operation, job.operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        if publication.writer.staged_page_len().is_none() {
            publication.writer.begin_staged_page(&mut context).unwrap();
        } else if test_funded_restore(|context| publication.advance(job.operation, &job.state, context)).unwrap() {
            return publication.writer.finish().unwrap();
        }
    }
    close_writer(&mut publication.writer);
    panic!("PCG fixture checkpoint completes");
}

fn pcg_checkpoint_pages(job: &PcgJob) -> Vec<Vec<u8>> {
    let payload = pcg_checkpoint_payload(job);
    let pages = (0..payload.page_count()).map(|index| payload.page(index).unwrap().to_vec()).collect();
    close_payload(payload);
    pages
}

fn restore_pcg(operation: Operation, payload: RetainedJobPayload) -> Result<PcgJob, NumericalCheckpointFault> {
    let mut restore = PcgRestoreCursor::new(operation, payload);
    let mut sequence = 0;
    let mut fault = NumericalCheckpointFault::Truncated;
    for _ in 0..2_000_000 {
        let handoff_due = restore.payload.is_none();
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, RetainedCloneGrant { maximum_items: 1, ..TEST_GRANT }), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        let step = restore.step(&mut context);
        assert!(context.retained_progress().copied_items <= 1);
        match step {
            Ok(Some(job)) => { assert!(handoff_due); assert_eq!(context.retained_progress().copied_items, 1); assert_eq!(context.retained_progress().released_bytes, 0); assert!(restore.terminal_is_empty()); return Ok(job); }
            Ok(None) => {}
            Err(error) => { fault = error; break; }
        }
    }
    while !restore.terminal_is_empty() { let _ = restore.close_step(grant_of(1, usize::MAX)); }
    Err(fault)
}

fn drive_pcg_job(mut job: PcgJob, operation: Operation) -> (VecD, PcgStats) {
    let mut sequence = 0;
    loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(job.step(&mut context)) {
            Seen::Complete(_) => {
                let (solution, stats) = job.solution();
                let solution = solution.clone();
                close_pcg_job(&mut job);
                return (solution, stats);
            }
            Seen::Fault(detail) => panic!("pcg fault: {}", String::from_utf8_lossy(&pages_bytes(&detail))),
            Seen::Cancelled => panic!("pcg unexpectedly cancelled"),
            Seen::Preview(_) | Seen::Checkpoint(_) | Seen::Yield => {}
        }
    }
}

#[test]
fn pcg_job_is_batch_deterministic_and_matches_reference() {
    let n = 24;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let csr = Coo::graph_laplacian_plus_identity(n, &edges).to_csr();
    let b = VecD::from_vec((0..n).map(|i| (i + 1) as f64).collect());
    let operation = test_operation(101);
    let (one, one_stats) = drive_pcg_job(PcgJob::new(operation, csr.clone(), b.clone(), VecD::zeros(n), 1e-11, 200, 1), operation);
    let (wide, wide_stats) = drive_pcg_job(PcgJob::new(operation, csr.clone(), b.clone(), VecD::zeros(n), 1e-11, 200, 97), operation);
    let mut reference = VecD::zeros(n);
    let reference_stats = pcg(&csr, &b, &mut reference, 1e-11, 200);
    assert_eq!(one, wide);
    assert_eq!(one_stats, wide_stats);
    assert_eq!(one, reference);
    assert_eq!(one_stats, reference_stats);
}

#[test]
fn pcg_job_checkpoint_resume_is_exact() {
    let n = 24;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let csr = Coo::graph_laplacian_plus_identity(n, &edges).to_csr();
    let b = VecD::from_vec((0..n).map(|i| (i + 1) as f64).collect());
    let operation = test_operation(102);
    let expected = drive_pcg_job(PcgJob::new(operation, csr.clone(), b.clone(), VecD::zeros(n), 1e-12, 200, 7), operation);
    let mut job = PcgJob::new(operation, csr, b, VecD::zeros(n), 1e-12, 200, 7);
    let mut sequence = 0;
    let checkpoint = loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(job.step(&mut context)) {
            Seen::Checkpoint(pages) => break pages,
            Seen::Preview(_) | Seen::Yield => {}
            Seen::Complete(_) => panic!("PCG completes before the required checkpoint"),
            Seen::Fault(detail) => panic!("PCG checkpoint fault: {}", String::from_utf8_lossy(&pages_bytes(&detail))),
            Seen::Cancelled => panic!("PCG checkpoint unexpectedly cancelled"),
        }
    };
    let expected_pages = checkpoint.clone();
    let resumed = restore_pcg(operation, payload_from_pages(operation, JobPayloadStream::CheckpointState, &checkpoint)).expect("pcg checkpoint restores");
    let actual_pages = pcg_checkpoint_pages(&resumed);
    close_pcg_job(&mut job);
    let actual = drive_pcg_job(resumed, operation);
    assert_eq!(actual_pages, expected_pages);
    assert_eq!(actual, expected);
}

#[test]
fn pcg_job_publishes_coarse_preview_before_final_tolerance() {
    let n = 40;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let csr = Coo::graph_laplacian_plus_identity(n, &edges).to_csr();
    let b = VecD::from_vec((0..n).map(|i| (i + 1) as f64).collect());
    let operation = test_operation(107);
    let mut job = PcgJob::new(operation, csr, b, VecD::zeros(n), 1e-12, 200, 512);
    let mut sequence = 0;
    loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
        match seen_of(job.step(&mut context)) {
            Seen::Preview(pages) => {
                let page = parse_numerical_page(&pages[0], b"FEMPCG1\0").unwrap();
                let quality = read_checkpoint_u64(page.bytes, 8 + 6 * 8).unwrap();
                let residual_norm = f64::from_bits(read_checkpoint_u64(page.bytes, 8 + 8 * 8).unwrap());
                close_pcg_job(&mut job);
                assert_eq!(quality, 1);
                assert!(residual_norm < 1e-3);
                assert!(residual_norm >= 1e-12);
                break;
            }
            Seen::Complete(_) => panic!("pcg reached final tolerance before publishing coarse quality"),
            Seen::Fault(detail) => panic!("PCG preview fault: {}", String::from_utf8_lossy(&pages_bytes(&detail))),
            Seen::Cancelled => panic!("PCG preview unexpectedly cancelled"),
            Seen::Checkpoint(_) | Seen::Yield => {}
        }
    }
}

#[test]
fn solver_jobs_reject_stale_and_cancelled_steps_without_mutation() {
    let mut coo = Coo::new(8);
    for i in 0..8 {
        coo.add(i, i, 2.0 + i as f64);
    }
    let csr = coo.to_csr();
    let operation = test_operation(103);
    let mut stale = PcgJob::new(operation, csr.clone(), VecD::from_vec(vec![1.0; 8]), VecD::zeros(8), 1e-9, 20, 8);
    let before = pcg_checkpoint_pages(&stale);
    let mut sequence = 0;
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), StepBudget::new(100, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
    assert!(matches!(seen_of(stale.step(&mut context)), Seen::Fault(_)));
    assert_eq!(pcg_checkpoint_pages(&stale), before);

    let mut cancelled = PcgJob::new(operation, csr, VecD::from_vec(vec![1.0; 8]), VecD::zeros(8), 1e-9, 20, 8);
    let before = pcg_checkpoint_pages(&cancelled);
    let token = semio_framework_job::root_cancel_token();
    token.cancel_now();
    let mut sequence_receipt = RetainedCloneProgress::default();
    drop(context);
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(100, u64::MAX, TEST_GRANT), token, || Some(0), &mut sequence, &mut sequence_receipt);
    assert_eq!(seen_of(cancelled.step(&mut context)), Seen::Cancelled);
    assert_eq!(pcg_checkpoint_pages(&cancelled), before);
    close_pcg_job(&mut stale);
    close_pcg_job(&mut cancelled);
}

#[test]
fn adversarial_solver_steps_stay_below_eight_milliseconds() {
    let n = 20_000;
    let mut coo = Coo::new(n);
    for i in 0..n {
        coo.add(i, i, 2.0);
    }
    let operation = test_operation(106);
    let mut pcg = PcgJob::new(operation, coo.to_csr(), VecD::from_vec(vec![1.0; n]), VecD::zeros(n), 1e-9, 20, 1);
    let mut sequence = 0;
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
    let started = std::time::Instant::now();
    let _ = pcg.step(&mut context);
    assert!(started.elapsed() < std::time::Duration::from_millis(8));
}

#[test]
fn pcg_construction_initializes_one_scalar_per_opportunity_and_closes_interruptibly() {
    let matrix = Csr::from_owned_parts(3, vec![0, 1, 2, 3], vec![0, 1, 2], vec![2.0, 3.0, 4.0]);
    let mut construction = PcgJobConstruction::new(test_operation(107), matrix);
    let mut opportunities = 0;
    while !construction.step_one().expect("fixed PCG construction") {
        opportunities += 1;
        assert!(opportunities < 128);
    }
    assert!(opportunities > 18, "six retained vectors cannot be initialized in one constructor turn");
    let mut job = construction.take_complete().expect("terminal construction transfers once");
    assert_eq!(job.state.a.n, 3);
    assert!(construction.take_complete().is_none());
    close_pcg_job(&mut job);
    assert!(tuple_of(construction.close_step(grant_of(1, PCG_SCALAR_BACKING_BYTES)).expect("REASON")).0);

    let matrix = Csr::from_owned_parts(3, vec![0, 1, 2, 3], vec![0, 1, 2], vec![2.0, 3.0, 4.0]);
    let mut interrupted = PcgJobConstruction::new(test_operation(108), matrix);
    assert!(!interrupted.step_one().expect("one reservation"));
    let before = interrupted.matrix.as_ref().expect("matrix retained").vals.len();
    let (terminal, _, _) = tuple_of(interrupted.close_step(grant_of(1, 4_096)).expect("REASON"));
    assert!(!terminal);
    assert_eq!(interrupted.matrix.as_ref().expect("matrix shell retained").vals.len() + 1, before);
    let mut closed = false;
    for _ in 0..128 {
        if tuple_of(interrupted.close_step(grant_of(1, PCG_SCALAR_BACKING_BYTES)).expect("REASON")).0 { closed = true; break; }
    }
    assert!(closed);
}

fn test_funded_restore<T>(step: impl FnOnce(&mut StepContext<'_>) -> T) -> T {
    let operation = test_operation(9998);
    let mut sequence = 0;
    let mut receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    step(&mut context)
}
