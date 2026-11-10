use super::*;
use semio_s_fem_payload_turns::{NumericalBytesPublication, numerical_begin_page, numerical_commit_page, numerical_post, numerical_writer_demand};
use semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;

fn released_bytes(outcome: semio_framework_value_persistent::list::PagedListReleaseOutcome) -> usize {
    match outcome { semio_framework_value_persistent::list::PagedListReleaseOutcome::Ready(progress) => progress.released_allocation_bytes, semio_framework_value_persistent::list::PagedListReleaseOutcome::Denied => 0 }
}
use semio_framework_job::StepBudget;
use semio_s_fem_test_support::{OriginalNumericalFixture, TEST_GRANT, close_grant, observe_original_numerical_allocations};

#[test]
fn numerical_restore_funds_each_original_contributor_frontier_separately() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/⛽️restore-funding/🔣️.json")).unwrap();
    let count = fixture["count"].as_u64().unwrap() as usize;
    let mut bytes = numerical_page_header(b"FEMLCP1\0", 11, NumericalPageCursor { field: 9, owner: 0, item: 0 }).to_vec();
    bytes.extend_from_slice(&(count as u64).to_le_bytes());
    for index in 0..count {
        bytes.extend_from_slice(&(fixture["columns"][index].as_u64().unwrap() as u32).to_le_bytes());
        bytes.extend_from_slice(&fixture["next"][index].as_u64().unwrap().to_le_bytes());
    }
    let page = parse_numerical_page(&bytes, b"FEMLCP1\0").unwrap();
    let mut owner: PagedList<RowContributor, SPARSE_INDEX_SPACE> = PagedList::empty();
    let mut cursor = NumericalOwnerRestoreCursor::default();
    let mut entry = 0;
    let operation = test_operation(9999);
    let mut sequence = 0;
    let mut births = 0;
    let mut complete = false;
    for turn in 0..128 {
        let demand = numerical_restore_owner_demand(&owner, &page, &cursor, entry, false).unwrap();
        let exact = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        for axis in fixture["deniedAxes"].as_array().unwrap() {
            let mut denied = exact;
            match axis.as_str().unwrap() {
                "items" => denied.maximum_items = 0,
                "copyBytes" if demand.copy_bytes != 0 => denied.maximum_copy_bytes -= 1,
                "capacityBytes" if demand.capacity_bytes != 0 => denied.maximum_capacity_bytes -= 1,
                "depth" => denied.maximum_depth -= 1,
                _ => continue,
            }
            let before = (owner.len(), owner.capacity(), owner.allocated_bytes(), owner.backing_ptr(0), entry, cursor.next_item, cursor.page_open);
            let mut receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, denied), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
            let (step, physical) = semio_s_fem_test_support::observe_original_numerical_allocations(|| restore_contributor_entry(&mut owner, &page, count, &mut entry, &mut cursor, &mut context));
            assert!(!step.unwrap());
            assert_eq!(physical, (0, 0, 0));
            assert_eq!(context.retained_progress(), RetainedCloneProgress::default());
            assert_eq!((owner.len(), owner.capacity(), owner.allocated_bytes(), owner.backing_ptr(0), entry, cursor.next_item, cursor.page_open), before);
        }
        let before = (owner.len(), owner.allocated_bytes());
        let mut receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, exact), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
        let (step, physical) = semio_s_fem_test_support::observe_original_numerical_allocations(|| restore_contributor_entry(&mut owner, &page, count, &mut entry, &mut cursor, &mut context));
        complete = step.unwrap();
        let actual = context.retained_progress();
        assert_eq!(actual.copied_items, 1);
        assert_eq!(actual.copied_bytes, demand.copy_bytes);
        assert_eq!(actual.retained_capacity_bytes, owner.allocated_bytes() - before.1);
        assert_eq!(actual.retained_capacity_bytes, demand.capacity_bytes);
        assert_eq!(physical, (actual.retained_capacity_bytes, actual.released_bytes, actual.retained_capacity_bytes));
        assert!(actual.fits(exact));
        if demand.capacity_bytes != 0 { births += 1; assert_eq!(owner.len(), before.0); }
        if complete { break; }
    }
    assert!(complete && births > 0);
    assert_eq!(owner.len(), count);
    for index in 0..count {
        let value = owner.get(index).unwrap();
        assert_eq!(value.column, fixture["columns"][index].as_u64().unwrap() as u32);
        assert_eq!(value.next, fixture["next"][index].as_u64().unwrap());
    }
    while !owner.terminal_is_empty() {
        if owner.pop().is_none() { semio_s_fem_test_support::granted(TEST_GRANT, |effect| owner.release_empty_page(effect)).result.unwrap(); }
    }
}

fn test_funded_restore<T>(step: impl FnOnce(&mut StepContext<'_>) -> T) -> T {
    let operation = test_operation(9998);
    let mut sequence = 0;
    let mut receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
    step(&mut context)
}

fn original_writer(original: &mut OriginalNumericalFixture) -> Option<Box<RetainedPayloadBuilder>> {
    original.open_writer(JobPayloadStream::CheckpointState)
}

fn original_finish(original: &mut OriginalNumericalFixture, writer: Option<Box<RetainedPayloadBuilder>>) -> RetainedJobPayload {
    original.finish_writer(writer)
}

fn original_begin(original: &mut OriginalNumericalFixture, writer: &mut Option<Box<RetainedPayloadBuilder>>) {
    original.until_done(|cx| numerical_begin_page(writer.as_deref_mut().unwrap(), cx));
}

fn original_commit(original: &mut OriginalNumericalFixture, writer: &mut Option<Box<RetainedPayloadBuilder>>) {
    original.until_done(|cx| numerical_commit_page(writer.as_deref_mut().unwrap(), cx));
}

fn original_step<T>(original: &mut OriginalNumericalFixture, step: impl FnOnce(&mut StepContext<'_>) -> T) -> T {
    original.turn(TEST_GRANT, 1, u64::MAX, |cx| Ok(step(cx))).expect("original writer turn")
}

fn empty_paged_backing<T>(count: usize) -> PagedList<T, SPARSE_INDEX_SPACE> {
    let mut owner = PagedList::empty();
    while owner.capacity() < count { semio_s_fem_test_support::granted(TEST_GRANT, |effect| owner.reserve_capacity_one(count, effect)).result.expect("original fixture backing"); }
    owner
}

#[test]
fn numerical_page_modal_complete_close_retains_empty_capacitated_matrix() {
    fn physical(owner: &SymmetricPencilConstruction) -> usize {
        owner.complete.as_ref().map_or(0, |(matrix, _)| matrix.allocated_bytes())
    }
    fn inner_owners(owner: &SymmetricPencilConstruction) -> usize {
        owner.complete.as_ref().map_or(0, |(matrix, _)| usize::from(matrix.vals.capacity() != 0) + usize::from(matrix.rowind.capacity() != 0) + usize::from(matrix.colptr.capacity() != 0))
    }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["closeCases"].as_array().unwrap() {
        let empty = || Csr::from_owned_parts(0, Vec::new(), Vec::new(), Vec::new());
        let mut owner = SymmetricPencilConstruction::new(empty(), VecD::from_vec(Vec::new()));
        owner.matrix = None;
        owner.mass = None;
        owner.complete = Some((CscSym::from_paged_parts(0, empty_paged_backing(4), empty_paged_backing(4), empty_paged_backing(4)), empty()));
        let next = owner.complete.as_ref().unwrap().0.vals.next_release_allocation_bytes().unwrap();
        let grant = match row["grant"].as_str().unwrap() { "zero" => 0, "subexact" => next - 1, "exact" => next, _ => unreachable!() };
        let before = physical(&owner);
        let first = owner.close_step(grant);
        let retired = 3 - inner_owners(&owner);
        let mut released = first.2;
        let mut exact = before - physical(&owner) == first.2 && first.2 <= grant;
        let mut terminal = first.0;
        for _ in 0..128 {
            let before = physical(&owner);
            let step = owner.close_step(NUMERICAL_OWNER_PAGE_BYTES);
            exact &= before - physical(&owner) == step.2;
            released += step.2;
            if step.0 { terminal = true; break; }
        }
        observations.push((row["id"].clone(), retired, row["expectedRetiredOwners"].as_u64().unwrap() as usize, exact, before, released, terminal));
    }
    for (id, retired, expected, exact, allocated, released, terminal) in observations {
        assert!(terminal, "{id} terminal");
        assert_eq!(retired, expected, "{id} one backing per granted opportunity");
        assert!(exact, "{id} reports only actually retired backing");
        assert_eq!(allocated, released, "{id} conservation");
    }
}

#[test]
fn numerical_page_restore_rejects_hostile_coordinates_without_changing_target_backing() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["restoreFaultCases"].as_array().unwrap() {
        let mut owner: PagedList<f64, 8192> = PagedList::default();
        let mut cursor = NumericalOwnerRestoreCursor::default();
        if row["continuation"].as_bool().unwrap() {
            let mut bytes = numerical_page_header(b"FEMLCP1\0", 11, NumericalPageCursor { field: 3, owner: 0, item: 0 }).to_vec();
            bytes.extend_from_slice(&2044u64.to_le_bytes());
            for index in 0..2043 { bytes.extend_from_slice(&(index as f64).to_bits().to_le_bytes()); }
            let page = parse_numerical_page(&bytes, b"FEMLCP1\0").unwrap();
            let mut entry = 0;
            for _ in 0..4096 {
                if test_funded_restore(|context| restore_paged_f64_entry(&mut owner, &page, 8192, &mut entry, &mut cursor, context)).unwrap() { break; }
            }
            assert_eq!(owner.len(), 2043);
        }
        let before = (owner.len(), owner.allocated_bytes(), owner.get(0).copied(), cursor.next_item);
        let mut bytes = numerical_page_header(b"FEMLCP1\0", 11, NumericalPageCursor { field: 3, owner: row["owner"].as_u64().unwrap() as usize, item: row["item"].as_u64().unwrap() as usize }).to_vec();
        let body_bytes = row["bodyBytes"].as_u64().unwrap() as usize;
        bytes.resize(NUMERICAL_CHECKPOINT_HEADER_BYTES + body_bytes, 0);
        if row["owner"] == 0 && body_bytes >= 8 { bytes[32..40].copy_from_slice(&row["length"].as_u64().unwrap().to_le_bytes()); }
        let page = parse_numerical_page(&bytes, b"FEMLCP1\0").unwrap();
        let result = test_funded_restore(|context| restore_paged_f64_entry(&mut owner, &page, 8192, &mut 0, &mut cursor, context));
        let held = before == (owner.len(), owner.allocated_bytes(), owner.get(0).copied(), cursor.next_item);
        let allocated = owner.allocated_bytes();
        let mut released = 0;
        for _ in 0..10_000 {
            if owner.pop().is_some() { continue; }
            if owner.terminal_is_empty() { break; }
            released += semio_s_fem_test_support::granted(TEST_GRANT, |effect| owner.release_empty_page(effect)).result.map(released_bytes).unwrap();
        }
        let closed = owner.terminal_is_empty();
        let fault = result.err().map(|fault| format!("{fault:?}"));
        observations.push((row["id"].clone(), fault, row["fault"].as_str().unwrap().to_owned(), held, closed, allocated, released));
    }
    assert_eq!(read_checkpoint_u16(&[], usize::MAX), Err(NumericalCheckpointFault::Truncated));
    assert_eq!(read_checkpoint_u32(&[], usize::MAX), Err(NumericalCheckpointFault::Truncated));
    assert_eq!(read_checkpoint_u64(&[], usize::MAX), Err(NumericalCheckpointFault::Truncated));
    for (id, fault, expected, held, closed, allocated, released) in observations {
        assert_eq!(fault.as_deref(), Some(expected.as_str()), "{id}");
        assert!(held && closed, "{id} retains then closes exact owner");
        assert_eq!(allocated, released, "{id}");
    }
}

#[test]
fn numerical_page_restore_accepts_every_declared_paged_scalar_continuation() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["cases"].as_array().unwrap() {
        let count = row["count"].as_u64().unwrap() as usize;
        let mut owner: PagedList<f64, 8192> = PagedList::default();
        let mut fault = None;
        let mut complete_pages = 0;
        let mut owner_cursor = NumericalOwnerRestoreCursor::default();
        for page in row["pages"].as_array().unwrap() {
            let item = page["item"].as_u64().unwrap() as usize;
            let cursor = NumericalPageCursor { field: 3, owner: page["owner"].as_u64().unwrap() as usize, item };
            let mut bytes = numerical_page_header(b"FEMLCP1\0", 11, cursor).to_vec();
            if cursor.owner == 0 { bytes.extend_from_slice(&(count as u64).to_le_bytes()); }
            for index in item..item + page["scalars"].as_u64().unwrap() as usize { bytes.extend_from_slice(&(index as f64 * 0.25 - 100.0).to_bits().to_le_bytes()); }
            let view = parse_numerical_page(&bytes, b"FEMLCP1\0").unwrap();
            let mut entry = 0;
            for _ in 0..count + 1024 {
                match test_funded_restore(|context| restore_paged_f64_entry(&mut owner, &view, 8192, &mut entry, &mut owner_cursor, context)) {
                    Ok(true) => { complete_pages += 1; break; }
                    Ok(false) => {}
                    Err(error) => { fault = Some(error); break; }
                }
            }
            if fault.is_some() { break; }
        }
        let actual: Vec<f64> = (0..owner.len()).map(|index| *owner.get(index).unwrap()).collect();
        let allocated = owner.allocated_bytes();
        let mut released = 0;
        for _ in 0..count + 1024 {
            if owner.pop().is_some() { continue; }
            match semio_s_fem_test_support::granted(TEST_GRANT, |effect| owner.release_empty_page(effect)).result {
                Ok(step) => released += released_bytes(step),
                Err(_) => break,
            }
            if owner.terminal_is_empty() { break; }
        }
        let closed = owner.terminal_is_empty();
        observations.push((row["id"].clone(), fault, complete_pages, row["pages"].as_array().unwrap().len(), actual, count, closed, allocated, released));
    }
    for (id, fault, pages, expected_pages, actual, count, closed, allocated, released) in observations {
        assert!(closed, "{id} closes");
        assert_eq!(allocated, released, "{id} physical close");
        assert_eq!(fault, None, "{id} accepts continuation");
        assert_eq!(pages, expected_pages, "{id} consumes every page");
        assert_eq!(actual, (0..count).map(|index| index as f64 * 0.25 - 100.0).collect::<Vec<_>>(), "{id} scalar reconstruction");
    }
}

#[test]
fn numerical_page_scalar_owner_continues_before_writing_past_exact_backing() {
    use semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    assert_eq!(corpus["pageBytes"].as_u64().unwrap() as usize, JOB_PAYLOAD_PAGE_BYTES);
    for row in corpus["cases"].as_array().unwrap() {
        let count = row["count"].as_u64().unwrap() as usize;
        let values: Vec<f64> = (0..count).map(|index| index as f64 * 0.25 - 100.0).collect();
        let operation = test_operation(480 + count as u64);
        let mut original = OriginalNumericalFixture::new(operation);
        let mut writer = original_writer(&mut original);
        let mut cursor = NumericalPageCursor::new();
        let mut complete = false;
        let mut fault = None;
        for _ in 0..count * 3 + 32 {
            let result = if writer.as_deref().unwrap().staged_page_len().is_none() {
                original_begin(&mut original, &mut writer);
                Ok(false)
            } else {
                match original_step(&mut original, |context| advance_numerical_page_header(writer.as_deref_mut().unwrap(), b"FEMPGT1\0", 41, cursor, context)) {
                    Ok(true) => Ok(false),
                    Ok(false) => original_step(&mut original, |context| advance_f64_owner(writer.as_deref_mut().unwrap(), &values, &mut cursor, context)),
                    Err(error) => Err(error),
                }
            };
            match result {
                Ok(true) => {
                    original_commit(&mut original, &mut writer);
                    complete = true;
                    break;
                }
                Ok(false) => {}
                Err(error) => { fault = Some(format!("{error:?}")); break; }
            }
        }
        let mut pages = Vec::new();
        let mut decoded = Vec::new();
        let mut released = 0;
        if complete {
            {
                {
                    let mut payload = original_finish(&mut original, writer.take());
                    for index in 0..payload.page_count() {
                        let bytes = payload.page(index).unwrap();
                        let owner = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
                        let item = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
                        let start = if owner == 0 { 40 } else { 32 };
                        let scalars = (bytes.len() - start) / 8;
                        pages.push(serde_json::json!({ "owner": owner, "item": item, "bytes": bytes.len(), "scalars": scalars }));
                        for bytes in bytes[start..].chunks_exact(8) { decoded.push(f64::from_bits(u64::from_le_bytes(bytes.try_into().unwrap()))); }
                    }
                    released += payload.page_count() * JOB_PAYLOAD_PAGE_BYTES;
                    original.close_payload(&mut payload);
                    assert!(payload.terminal_is_empty());
                }
            }
        }
        original.close_writer(&mut writer);
        assert!(writer.is_none());
        original.finish();
        eprintln!("numerical page {} complete={complete}, fault={fault:?}, cursor={cursor:?}, pages={pages:?}, released={released}", row["id"]);
        observations.push((row["id"].clone(), complete, pages, row["pages"].clone(), decoded, values, released));
    }
    for (id, complete, pages, expected, decoded, values, released) in observations {
        assert!(complete, "{id} completes across page boundaries");
        assert_eq!(serde_json::Value::Array(pages.clone()), expected, "{id}");
        assert_eq!(decoded, values, "{id} preserves every scalar");
        assert_eq!(released, pages.len() * JOB_PAYLOAD_PAGE_BYTES, "{id} releases actual backing");
    }
}

#[test]
fn numerical_page_all_owner_entries_preserve_cursor_until_their_width_fits() {
    use semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;

    fn advance(original: &mut OriginalNumericalFixture, kind: &str, writer: &mut RetainedPayloadBuilder, cursor: &mut NumericalPageCursor, matrix: &MatD, integers: &PagedList<u32, 1>, scalars: &PagedList<f64, 1>) -> Result<bool, ValueError> {
        match kind {
            "length" => original_step(original, |context| advance_owner_length(writer, 2, cursor, context)),
            "u32" => original_step(original, |context| advance_u32_owner(writer, &[7], cursor, context)),
            "usize" => original_step(original, |context| advance_u64_owner(writer, &[7], cursor, context)),
            "u64" => original_step(original, |context| advance_u64_values(writer, &[7], cursor, context)),
            "f64" => original_step(original, |context| advance_f64_owner(writer, &[1.5], cursor, context)),
            "paged-u32" => original_step(original, |context| advance_paged_u32_owner(writer, integers, cursor, context)),
            "paged-f64" => original_step(original, |context| advance_paged_f64_owner(writer, scalars, cursor, context)),
            "pair" => original_step(original, |context| advance_pair_owner(writer, &[(7, 1.5)], cursor, context)),
            "matrix-shape" | "matrix-scalar" => original_step(original, |context| advance_matrix_owner(writer, matrix, cursor, context)),
            _ => unreachable!(),
        }
    }

    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
    let mut matrix = MatD::zeros(1, 1);
    matrix.data[0] = 1.5;
    let mut integers = PagedList::<u32, 1>::empty();
    let mut scalars = PagedList::<f64, 1>::empty();
    integers.reserve_full().unwrap();
    scalars.reserve_full().unwrap();
    integers.push_reserved(7).unwrap();
    scalars.push_reserved(1.5).unwrap();
    let mut observations = Vec::new();
    for (index, row) in corpus["boundaryCases"].as_array().unwrap().iter().enumerate() {
        let kind = row["kind"].as_str().unwrap();
        let width = row["width"].as_u64().unwrap() as usize;
        let remaining = row["remaining"].as_u64().unwrap() as usize;
        let operation = test_operation(5_000 + index as u64);
        let mut original = OriginalNumericalFixture::new(operation);
        let mut writer = original_writer(&mut original);
        original_begin(&mut original, &mut writer);
        let filler = vec![0u8; JOB_PAYLOAD_PAGE_BYTES - remaining];
        if !filler.is_empty() { original.until_done(|cx| { let turn = writer.as_deref_mut().unwrap().write_staged_source_funded(cx, filler.as_slice()); numerical_post(cx, turn) }); }
        let mut cursor = NumericalPageCursor { field: 0, owner: usize::from(kind != "length" && kind != "matrix-shape"), item: 0 };
        let before = cursor;
        let mut fault = advance(&mut original, kind, writer.as_deref_mut().unwrap(), &mut cursor, &matrix, &integers, &scalars).err();
        let committed = writer.as_deref().unwrap().staged_page_len().is_none();
        let held = !committed || cursor == before;
        if committed && fault.is_none() {
            original_begin(&mut original, &mut writer);
            original_step(&mut original, |context| advance_numerical_page_header(writer.as_deref_mut().unwrap(), b"FEMPGT1\0", 42, cursor, context)).unwrap();
            fault = advance(&mut original, kind, writer.as_deref_mut().unwrap(), &mut cursor, &matrix, &integers, &scalars).err();
        }
        original_commit(&mut original, &mut writer);
        let mut payload = original_finish(&mut original, writer);
        let page_count = payload.page_count();
        let last = payload.page(page_count - 1).unwrap();
        let entry = last[last.len() - width..].to_vec();
        let actual = serde_json::json!({ "committed": committed, "owner": cursor.owner, "item": cursor.item, "entry": entry });
        original.close_payload(&mut payload);
        original.finish();
        observations.push((format!("{kind}/{remaining}"), actual, row["expected"].clone(), fault.is_none(), held, payload.terminal_is_empty()));
    }
    integers.pop();
    scalars.pop();
    while integers.allocated_bytes() != 0 { semio_s_fem_test_support::granted(TEST_GRANT, |effect| integers.release_empty_page(effect)).result.unwrap(); }
    while scalars.allocated_bytes() != 0 { semio_s_fem_test_support::granted(TEST_GRANT, |effect| scalars.release_empty_page(effect)).result.unwrap(); }
    for (id, actual, expected, valid, held, released) in observations {
        assert!(valid && held && released, "{id} preserves pending input and exact backing");
        assert_eq!(actual, expected, "{id}");
    }
}

#[test]
fn rcm_reduces_bandwidth_on_scattered_path_graph() {
    let shuffle = [9usize, 0, 8, 1, 7, 2, 6, 3, 5, 4];
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); 10];
    let mut edges = Vec::new();
    for i in 0..9 {
        let (u, v) = (shuffle[i], shuffle[i + 1]);
        adjacency[u].push(v);
        adjacency[v].push(u);
        edges.push((u, v));
    }
    let bandwidth = |index_of: &dyn Fn(usize) -> usize| -> usize { edges.iter().map(|&(u, v)| (index_of(u) as i64 - index_of(v) as i64).unsigned_abs() as usize).max().unwrap() };
    let before = bandwidth(&|x| x);
    let perm = rcm_order(&adjacency);
    let mut new_index = vec![0usize; 10];
    for (new_idx, &old_idx) in perm.iter().enumerate() {
        new_index[old_idx] = new_idx;
    }
    let after = bandwidth(&|x| new_index[x]);
    assert!(after <= before);
}

#[test]
fn csc_sym_get_and_to_csr_full_match_dense() {
    let mut coo = Coo::new(3);
    coo.add(0, 0, 4.0);
    coo.add(1, 1, 5.0);
    coo.add(2, 2, 6.0);
    coo.add(0, 1, 2.0);
    coo.add(1, 0, 2.0);
    coo.add(1, 2, 3.0);
    coo.add(2, 1, 3.0);
    let dense = coo.to_dense();
    let csc = coo.to_csc_sym_upper();

    for r in 0..3 {
        for c in 0..3 {
            assert!((csc.get(r, c) - dense.get(r, c)).abs() < 1e-12, "get({r},{c}) = {} vs dense {}", csc.get(r, c), dense.get(r, c));
        }
    }
    assert_eq!(csc.get(0, 2), 0.0, "no (0,2) entry was ever added");

    let full = csc.to_csr_full();
    let x = VecD::from_vec(vec![1.0, 2.0, 3.0]);
    let expected = dense.mul_vec(&x);
    let actual = full.mul_vec(&x);
    for i in 0..3 {
        assert!((actual.get(i) - expected.get(i)).abs() < 1e-9, "mul_vec[{i}] = {} vs {}", actual.get(i), expected.get(i));
    }
}

fn test_operation(id: u64) -> Operation {
    Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(7), semio_framework_job::Generation(3), 11)
}

fn publication_bytes() -> Vec<u8> {
    (0..2 * JOB_PAYLOAD_PAGE_BYTES + 321).map(|index| (index * 31 % 251) as u8).collect()
}

fn publish_turn(original: &mut OriginalNumericalFixture, publication: &mut NumericalBytesPublication, lent: &mut Option<RetainedJobPayload>, grant: RetainedCloneGrant) -> bool {
    original.turn(grant, 1, u64::MAX, |cx| publication.advance(lent, cx).map_err(Into::into)).expect("same original publication turn")
}

fn payload_content(payload: &RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|index| payload.page(index).unwrap().to_vec()).collect()
}

/// 📦️ A prepared byte string publishes as one payload whose pages concatenate back to the exact bytes.
#[test]
fn bytes_publication_publishes_every_byte_across_pages() {
    let bytes = publication_bytes();
    let mut original = OriginalNumericalFixture::new(test_operation(7_001));
    let mut publication = NumericalBytesPublication::new(JobPayloadStream::CommitOutput, bytes.clone());
    let mut lent = None;
    let mut turns = 0;
    while !publish_turn(&mut original, &mut publication, &mut lent, TEST_GRANT) {
        turns += 1;
        assert!(turns < 10_000, "publication never finished");
    }
    let mut payload = lent.take().expect("the finished publication lends its payload");
    assert!(payload.page_count() >= 3);
    assert_eq!(payload_content(&payload), bytes);
    assert!(publication.close_demand().is_none(), "a finished publication leaves no builder behind");
    original.close_payload(&mut payload);
    original.finish();
}

/// ✂️ A cancelled publication retires its half-built builder through funded close turns, releasing every page, ledger and the box.
#[test]
fn bytes_publication_cancelled_mid_publication_closes_with_funded_turns() {
    let mut original = OriginalNumericalFixture::new(test_operation(7_002));
    let mut publication = NumericalBytesPublication::new(JobPayloadStream::CommitOutput, publication_bytes());
    let mut lent = None;
    for _ in 0..40 {
        assert!(!publish_turn(&mut original, &mut publication, &mut lent, TEST_GRANT));
    }
    assert!(lent.is_none() && publication.close_demand().is_some(), "cancelled mid-publication with an owned half-built builder");
    let mut released = 0;
    for _ in 0..10_000 {
        let Some(demand) = publication.close_demand() else { break };
        let demand = demand.expect("a half-built builder quotes its close demand");
        let step = original.turn(close_grant(demand), 1, u64::MAX, |cx| { let grant = cx.retained_grant(); Ok(semio_s_fem_test_support::granted(grant, |effect| publication.close_step(effect))) }).expect("same original close turn");
        released += step.expect("a half-built builder closes").expect("funded close turn").progress().released_bytes;
    }
    assert!(publication.close_demand().is_none(), "close ends with the box released");
    assert!(released > 0);
    original.finish();
}

/// 🚫️ A refused push (unfunded turn) changes nothing and costs nothing; the same publication resumes and finishes exactly.
#[test]
fn bytes_publication_resumes_after_a_refused_push() {
    let bytes = publication_bytes();
    let mut original = OriginalNumericalFixture::new(test_operation(7_003));
    let mut publication = NumericalBytesPublication::new(JobPayloadStream::CommitOutput, bytes.clone());
    let mut lent = None;
    for _ in 0..8 {
        assert!(!publish_turn(&mut original, &mut publication, &mut lent, TEST_GRANT));
    }
    let refused = RetainedCloneGrant { maximum_items: 0, ..TEST_GRANT };
    for _ in 0..4 {
        let (done, heap) = observe_original_numerical_allocations(|| publish_turn(&mut original, &mut publication, &mut lent, refused));
        assert!(!done);
        assert_eq!((heap.0, heap.1), (0, 0), "a refused push neither allocates nor releases");
    }
    let mut turns = 0;
    while !publish_turn(&mut original, &mut publication, &mut lent, TEST_GRANT) {
        turns += 1;
        assert!(turns < 10_000, "publication never resumed");
    }
    let mut payload = lent.take().expect("the resumed publication lends its payload");
    assert_eq!(payload_content(&payload), bytes);
    original.close_payload(&mut payload);
    original.finish();
}
