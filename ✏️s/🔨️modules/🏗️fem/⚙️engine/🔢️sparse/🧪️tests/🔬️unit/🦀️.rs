use super::*;
use crate::engine_test_vectors::payload_bytes;

#[test]
fn numerical_page_subspace_work_persists_solver_state_without_physical_retirement_cursor() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
    let operation = test_operation(494);
    let mut sequence = 0;
    let mut observations = Vec::new();
    for field in [522, 534] {
        let mut pages = Vec::new();
        for lane in corpus["workRetirementLanes"].as_array().unwrap() {
            let mut job = SubspaceIterationJob::new(operation, LdltFactor { n: 0, l_cols: Vec::new(), d: Vec::new() }, Csr::from_owned_parts(0, Vec::new(), Vec::new(), Vec::new()), 0, 0, 1);
            job.state.work.close_lane = lane.as_u64().unwrap() as u8;
            let mut retiring = SubspaceWork::empty();
            retiring.close_lane = lane.as_u64().unwrap() as u8;
            job.state.retiring_work = Some(retiring);
            let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            writer.begin_staged_page(&mut context).unwrap();
            let mut cursor = NumericalPageCursor { field, owner: 0, item: 0 };
            for _ in 0..64 {
                SubspaceIterationJob::advance_checkpoint_entry(&job.state, &mut cursor, &mut writer).unwrap();
                if cursor.field != field { break; }
            }
            let mut payload = writer.finish().unwrap_or_else(|_| panic!("work control page committed"));
            pages.push(payload.page(0).unwrap().to_vec());
            for _ in 0..16 {
                let _ = payload.close_step(1, NUMERICAL_OWNER_PAGE_BYTES);
                if payload.terminal_is_empty() { break; }
            }
            assert!(payload.terminal_is_empty());
        }
        let counts: Vec<_> = pages.iter().map(|bytes| {
            let page = parse_numerical_page(bytes, b"FEMSCP1\0").unwrap();
            read_checkpoint_u64(page.bytes, if field == 534 { 8 } else { 0 }).unwrap()
        }).collect();
        eprintln!("[DEBUG] numerical work physical cursor field={field}, counts={counts:?}, same_bytes={}", pages[0] == pages[1]);
        observations.push((field, counts, pages[0] == pages[1]));
    }
    let mut bytes = numerical_page_header(b"FEMSCP1\0", 12, NumericalPageCursor { field: 522, owner: 0, item: 0 }).to_vec();
    bytes.extend_from_slice(&13u64.to_le_bytes());
    for index in 0..13 { bytes.extend_from_slice(&(if index == 12 { 11u64 } else { 0 }).to_le_bytes()); }
    let page = parse_numerical_page(&bytes, b"FEMSCP1\0").unwrap();
    let mut work = SubspaceWork::empty();
    work.rhs.data = Vec::with_capacity(4);
    let allocated = work.rhs.data.capacity() * size_of::<f64>();
    let mut cursor = NumericalOwnerRestoreCursor::default();
    let mut entry = 0;
    let mut control = [0; 24];
    let mut fault = None;
    for _ in 0..32 {
        match restore_work_entry(&mut work, &page, 522, 1, 1, &mut entry, &mut control, &mut cursor) {
            Ok(true) => break,
            Ok(false) => {}
            Err(error) => { fault = Some(error); break; }
        }
    }
    let first = work.close_step(NUMERICAL_OWNER_PAGE_BYTES);
    let held = work.rhs.data.capacity() * size_of::<f64>();
    work.close_lane = 0;
    let mut released = first.2;
    for _ in 0..64 {
        let step = work.close_step(NUMERICAL_OWNER_PAGE_BYTES);
        released += step.2;
        if step.0 { break; }
    }
    eprintln!("[DEBUG] numerical work rejected physical cursor fault={fault:?}, first={first:?}, held={held}, allocated={allocated}, released={released}");
    for (field, counts, identical) in observations {
        assert_eq!(counts, vec![corpus["workControlEntries"].as_u64().unwrap(); 2], "field {field} contains only solver control values");
        assert!(identical, "field {field} physical cleanup does not change checkpoint bytes");
    }
    assert_eq!(fault, Some(NumericalCheckpointFault::Truncated));
    assert_eq!(first, (false, 1, allocated));
    assert_eq!(held, 0);
    assert_eq!(released, allocated);
}

#[test]
fn numerical_page_modal_complete_close_retains_empty_capacitated_matrix() {
    fn physical(owner: &ModalInputConstruction) -> usize {
        owner.complete.as_ref().map_or(0, |(matrix, _)| matrix.vals.capacity() * size_of::<f64>() + (matrix.rowind.capacity() + matrix.colptr.capacity()) * size_of::<u32>())
    }
    fn inner_owners(owner: &ModalInputConstruction) -> usize {
        owner.complete.as_ref().map_or(0, |(matrix, _)| usize::from(matrix.vals.capacity() != 0) + usize::from(matrix.rowind.capacity() != 0) + usize::from(matrix.colptr.capacity() != 0))
    }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["closeCases"].as_array().unwrap() {
        let empty = || Csr::from_owned_parts(0, Vec::new(), Vec::new(), Vec::new());
        let mut owner = ModalInputConstruction::new(empty(), VecD::from_vec(Vec::new()));
        owner.matrix = None;
        owner.mass = None;
        owner.complete = Some((CscSym { n: 0, colptr: Vec::with_capacity(4), rowind: Vec::with_capacity(4), vals: Vec::with_capacity(4) }, empty()));
        let next = owner.complete.as_ref().unwrap().0.vals.capacity() * size_of::<f64>();
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
        eprintln!("[DEBUG] numerical modal complete close {} grant={grant}, before={before}, first={first:?}, retired={retired}, exact={exact}, released={released}, terminal={terminal}", row["id"]);
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
fn numerical_page_subspace_restore_close_preserves_every_ungranted_backing() {
    fn physical(state: Option<&SubspaceCheckpoint>) -> usize {
        state.map_or(0, |state| state.k_factor.l_cols.capacity() * size_of::<Vec<(u32, f64)>>()
            + state.k_factor.l_cols.iter().map(|owner| owner.capacity() * size_of::<(u32, f64)>()).sum::<usize>()
            + state.k_factor.d.capacity() * size_of::<f64>())
    }
    fn inner_owners(state: Option<&SubspaceCheckpoint>) -> usize {
        state.map_or(0, |state| state.k_factor.l_cols.iter().filter(|owner| owner.capacity() != 0).count() + usize::from(state.k_factor.d.capacity() != 0))
    }
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["closeCases"].as_array().unwrap() {
        let operation = test_operation(493);
        let factor = LdltFactor { n: 0, l_cols: vec![Vec::with_capacity(4)], d: Vec::with_capacity(4) };
        let next = factor.l_cols[0].capacity() * size_of::<(u32, f64)>();
        let scalar = factor.d.capacity() * size_of::<f64>();
        let grant = match row["grant"].as_str().unwrap() { "zero" => 0, "subexact" => next.min(scalar) - 1, "exact" => next.max(scalar), _ => unreachable!() };
        let job = SubspaceIterationJob::new(operation, factor, Csr::from_owned_parts(0, Vec::new(), Vec::new(), Vec::new()), 0, 0, 1);
        let mut restore = SubspaceRestoreCursor::new(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState));
        restore.payload = None;
        restore.state = Some(job.state);
        let before = physical(restore.state.as_ref());
        let first = restore.close_step(1, grant);
        let after = physical(restore.state.as_ref());
        let retired = 2 - inner_owners(restore.state.as_ref());
        let reported = match first { semio_framework_job::InteractiveJobCloseStep::Pending { released_bytes, .. } => released_bytes, _ => 0 };
        let mut released = reported;
        let mut exact = before - after == reported && reported <= grant;
        for _ in 0..128 {
            let before = physical(restore.state.as_ref());
            let step = restore.close_step(1, NUMERICAL_OWNER_PAGE_BYTES);
            let reported = match step { semio_framework_job::InteractiveJobCloseStep::Pending { released_bytes, .. } => released_bytes, _ => 0 };
            exact &= before - physical(restore.state.as_ref()) == reported;
            released += reported;
            if restore.terminal_is_empty() { break; }
        }
        eprintln!("[DEBUG] numerical subspace restore close {} grant={grant}, before={before}, first={first:?}, retired={retired}, exact={exact}, released={released}, terminal={}", row["id"], restore.terminal_is_empty());
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
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
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
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        let left = ldlt.step(&mut context).err();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        let right = subspace.step(&mut context).err();
        observations.push((left, right, ldlt.page_slot, ldlt.page_entry, subspace.page_slot, subspace.page_entry));
    }
    for _ in 0..16 {
        let _ = ldlt.close_step(1, NUMERICAL_OWNER_PAGE_BYTES);
        let _ = subspace.close_step(1, NUMERICAL_OWNER_PAGE_BYTES);
        if ldlt.terminal_is_empty() && subspace.terminal_is_empty() { break; }
    }
    eprintln!("[DEBUG] numerical sticky restore faults observations={observations:?}, ldlt_closed={}, subspace_closed={}", ldlt.terminal_is_empty(), subspace.terminal_is_empty());
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
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
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
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
                writer.begin_staged_page(&mut context).map(|_| false)
            } else {
                match advance_numerical_page_header(&mut writer, b"FEMSCP1\0", 12, write_cursor) {
                    Ok(true) => Ok(false),
                    Ok(false) => advance_matrix_owner(&mut writer, &matrix, &mut write_cursor),
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
                    match restore_matrix_entry(&mut restored, &page, 2048, &mut entry, &mut restore_cursor) {
                        Ok(true) => break,
                        Ok(false) => {}
                        Err(fault) => { restore_fault = Some(fault); break; }
                    }
                }
                if restore_fault.is_some() { break; }
            }
            for _ in 0..64 { if matches!(payload.close_step(1, NUMERICAL_OWNER_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) { break; } }
            assert!(payload.terminal_is_empty());
        } else {
            writer.begin_close();
            for _ in 0..64 { if matches!(writer.close_step(1, NUMERICAL_OWNER_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) { break; } }
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
        eprintln!("[DEBUG] numerical matrix {} write={write_fault:?}, restore={restore_fault:?}, dimensions={dimensions:?}, capacity={capacity}, pages={page_items:?}, allocated={allocated}, released={released}", row["id"]);
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
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
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
        let first = restore.close_step(1, grant);
        let after = physical(restore.state.as_ref());
        let retired = 2 - inner_owners(restore.state.as_ref());
        let reported = match first { semio_framework_job::InteractiveJobCloseStep::Pending { released_bytes, .. } => released_bytes, _ => 0 };
        let mut released = reported;
        let mut exact = before - after == reported && reported <= grant;
        for _ in 0..128 {
            let before = physical(restore.state.as_ref());
            let step = restore.close_step(1, NUMERICAL_OWNER_PAGE_BYTES);
            let reported = match step { semio_framework_job::InteractiveJobCloseStep::Pending { released_bytes, .. } => released_bytes, _ => 0 };
            exact &= before - physical(restore.state.as_ref()) == reported;
            released += reported;
            if restore.terminal_is_empty() { break; }
        }
        eprintln!("[DEBUG] numerical restore close {} grant={grant}, before={before}, first={first:?}, retired={retired}, exact={exact}, released={released}, terminal={}", row["id"], restore.terminal_is_empty());
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
fn numerical_page_restore_rejects_hostile_coordinates_without_changing_target_backing() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
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
                if restore_paged_f64_entry(&mut owner, &page, 8192, &mut entry, &mut cursor).unwrap() { break; }
            }
            assert_eq!(owner.len(), 2043);
        }
        let before = (owner.len(), owner.allocated_bytes(), owner.get(0).copied(), cursor.next_item);
        let mut bytes = numerical_page_header(b"FEMLCP1\0", 11, NumericalPageCursor { field: 3, owner: row["owner"].as_u64().unwrap() as usize, item: row["item"].as_u64().unwrap() as usize }).to_vec();
        let body_bytes = row["bodyBytes"].as_u64().unwrap() as usize;
        bytes.resize(NUMERICAL_CHECKPOINT_HEADER_BYTES + body_bytes, 0);
        if row["owner"] == 0 && body_bytes >= 8 { bytes[32..40].copy_from_slice(&row["length"].as_u64().unwrap().to_le_bytes()); }
        let page = parse_numerical_page(&bytes, b"FEMLCP1\0").unwrap();
        let result = restore_paged_f64_entry(&mut owner, &page, 8192, &mut 0, &mut cursor);
        let held = before == (owner.len(), owner.allocated_bytes(), owner.get(0).copied(), cursor.next_item);
        let allocated = owner.allocated_bytes();
        let mut released = 0;
        for _ in 0..10_000 {
            if owner.pop().is_some() { continue; }
            if owner.terminal_is_empty() { break; }
            released += owner.release_empty_page(SPARSE_PAGE_BYTES).unwrap().released_allocation_bytes;
        }
        let closed = owner.terminal_is_empty();
        let fault = result.err().map(|fault| format!("{fault:?}"));
        eprintln!("[DEBUG] numerical hostile {} fault={fault:?}, held={held}, allocated={allocated}, released={released}, closed={closed}", row["id"]);
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
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
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
                match restore_paged_f64_entry(&mut owner, &view, 8192, &mut entry, &mut owner_cursor) {
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
            match owner.release_empty_page(SPARSE_PAGE_BYTES) {
                Ok(step) => released += step.released_allocation_bytes,
                Err(_) => break,
            }
            if owner.terminal_is_empty() { break; }
        }
        let closed = owner.terminal_is_empty();
        eprintln!("[DEBUG] numerical restore {} complete_pages={complete_pages}, fault={fault:?}, scalars={}, allocated={allocated}, released={released}, closed={closed}", row["id"], actual.len());
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
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match job.step(&mut context) {
            StepOutcome::CheckpointReady(value) => { checkpoint = Some(value.state); break; }
            StepOutcome::Yield => {}
            StepOutcome::Fault(fault) => { close_payload(fault.detail); write_fault = true; break; }
            StepOutcome::PreviewReady(payload) => close_payload(payload),
            StepOutcome::Complete(candidate) => { close_payload(candidate.state); close_payload(candidate.output); write_fault = true; break; }
            _ => { write_fault = true; break; }
        }
    }
    let mut pages = Vec::new();
    let mut restore = checkpoint.map(|payload| {
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
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            match restore.step(&mut context) {
                Ok(Some(value)) => { restored = Some(value); break; }
                Ok(None) => {}
                Err(fault) => { restore_fault = Some(fault); break; }
            }
        }
    }
    let actual = restored.as_ref().map(|value| (value.state.a.colptr.clone(), value.state.a.rowind.clone(), value.state.a.vals.clone()));
    for _ in 0..10_000 {
        if let Some(restore) = restore.as_mut() { let _ = restore.close_step(1, NUMERICAL_OWNER_PAGE_BYTES); }
        if let Some(restored) = restored.as_mut() { let _ = InteractiveJob::close_step(restored, 1, NUMERICAL_OWNER_PAGE_BYTES); }
        let _ = InteractiveJob::close_step(&mut job, 1, NUMERICAL_OWNER_PAGE_BYTES);
        if restore.as_ref().is_none_or(LdltRestoreCursor::terminal_is_empty) && restored.as_ref().is_none_or(InteractiveJob::terminal_is_empty) && InteractiveJob::terminal_is_empty(&job) { break; }
    }
    let closed = restore.as_ref().is_none_or(LdltRestoreCursor::terminal_is_empty) && restored.as_ref().is_none_or(InteractiveJob::terminal_is_empty) && InteractiveJob::terminal_is_empty(&job);
    eprintln!("[DEBUG] LDLT maximum owner checkpoint pages={pages:?}, write_fault={write_fault}, restore_fault={restore_fault:?}, restored={}, closed={closed}", actual.is_some());
    assert!(closed);
    assert!(!write_fault);
    assert_eq!(pages, vec![serde_json::json!({ "owner": 0, "item": 0, "bytes": NUMERICAL_CHECKPOINT_HEADER_BYTES + 8 + count * 8, "scalars": count })]);
    assert_eq!(restore_fault, None);
    assert_eq!(actual, Some(expected));
}


#[test]
fn numerical_page_scalar_owner_continues_before_writing_past_exact_backing() {
    use semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    assert_eq!(corpus["pageBytes"].as_u64().unwrap() as usize, JOB_PAYLOAD_PAGE_BYTES);
    for row in corpus["cases"].as_array().unwrap() {
        let count = row["count"].as_u64().unwrap() as usize;
        let values: Vec<f64> = (0..count).map(|index| index as f64 * 0.25 - 100.0).collect();
        let operation = test_operation(480 + count as u64);
        let mut sequence = 0;
        let mut writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState));
        let mut cursor = NumericalPageCursor::new();
        let mut complete = false;
        let mut fault = None;
        for _ in 0..count * 3 + 32 {
            let writer = writer.as_mut().unwrap();
            let result = if writer.staged_page_len().is_none() {
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
                writer.begin_staged_page(&mut context).map(|_| false)
            } else {
                match advance_numerical_page_header(writer, b"FEMPGT1\0", 41, cursor) {
                    Ok(true) => Ok(false),
                    Ok(false) => advance_f64_owner(writer, &values, &mut cursor),
                    Err(error) => Err(error),
                }
            };
            match result {
                Ok(true) => {
                    match writer.commit_staged_page() {
                        Ok(()) => complete = true,
                        Err(error) => fault = Some(error),
                    }
                    break;
                }
                Ok(false) => {}
                Err(error) => { fault = Some(error); break; }
            }
        }
        let mut pages = Vec::new();
        let mut decoded = Vec::new();
        let mut released = 0;
        if complete {
            match writer.take().unwrap().finish() {
                Ok(mut payload) => {
                    for index in 0..payload.page_count() {
                        let bytes = payload.page(index).unwrap();
                        let owner = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
                        let item = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
                        let start = if owner == 0 { 40 } else { 32 };
                        let scalars = (bytes.len() - start) / 8;
                        pages.push(serde_json::json!({ "owner": owner, "item": item, "bytes": bytes.len(), "scalars": scalars }));
                        for bytes in bytes[start..].chunks_exact(8) { decoded.push(f64::from_bits(u64::from_le_bytes(bytes.try_into().unwrap()))); }
                    }
                    for _ in 0..32 {
                        match payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES) {
                            semio_framework_job::JobPayloadCloseStep::Pending { released_bytes, .. } => released += released_bytes,
                            semio_framework_job::JobPayloadCloseStep::Complete => break,
                        }
                    }
                    assert!(payload.terminal_is_empty());
                }
                Err(returned) => { writer = Some(returned); complete = false; }
            }
        }
        if let Some(writer) = writer.as_mut() {
            writer.begin_close();
            for _ in 0..32 {
                match writer.close_step(1, JOB_PAYLOAD_PAGE_BYTES) {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_bytes, .. } => released += released_bytes,
                    semio_framework_job::JobPayloadCloseStep::Complete => break,
                }
            }
            assert!(writer.terminal_is_empty());
        }
        eprintln!("[DEBUG] numerical page {} complete={complete}, fault={fault:?}, cursor={cursor:?}, pages={pages:?}, released={released}", row["id"]);
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
    use semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;

    fn advance(kind: &str, writer: &mut RetainedJobPayloadWriter, cursor: &mut NumericalPageCursor, matrix: &MatD, integers: &PagedList<u32, 1>, scalars: &PagedList<f64, 1>) -> Result<bool, JobPayloadAdmissionFault> {
        match kind {
            "length" => advance_owner_length(writer, 2, cursor),
            "u32" => advance_u32_owner(writer, &[7], cursor),
            "usize" => advance_u64_owner(writer, &[7], cursor),
            "u64" => advance_u64_values(writer, &[7], cursor),
            "f64" => advance_f64_owner(writer, &[1.5], cursor),
            "paged-u32" => advance_paged_u32_owner(writer, integers, cursor),
            "paged-f64" => advance_paged_f64_owner(writer, scalars, cursor),
            "pair" => advance_pair_owner(writer, &[(7, 1.5)], cursor),
            "matrix-shape" | "matrix-scalar" => advance_matrix_owner(writer, matrix, cursor),
            _ => unreachable!(),
        }
    }

    let corpus: serde_json::Value = serde_json::from_str(include_str!("../📦️numerical-pages/🧫️fixtures/🔣️.json")).unwrap();
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
        let mut sequence = 0;
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
        writer.begin_staged_page(&mut context).unwrap();
        writer.write_staged(&[0; JOB_PAYLOAD_PAGE_BYTES][..JOB_PAYLOAD_PAGE_BYTES - remaining]).unwrap();
        let mut cursor = NumericalPageCursor { field: 0, owner: usize::from(kind != "length" && kind != "matrix-shape"), item: 0 };
        let before = cursor;
        let mut fault = advance(kind, &mut writer, &mut cursor, &matrix, &integers, &scalars).err();
        let committed = writer.staged_page_len().is_none();
        let held = !committed || cursor == before;
        if committed && fault.is_none() {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
            writer.begin_staged_page(&mut context).unwrap();
            advance_numerical_page_header(&mut writer, b"FEMPGT1\0", 42, cursor).unwrap();
            fault = advance(kind, &mut writer, &mut cursor, &matrix, &integers, &scalars).err();
        }
        writer.commit_staged_page().unwrap();
        let mut payload = writer.finish().unwrap();
        let page_count = payload.page_count();
        let last = payload.page(page_count - 1).unwrap();
        let entry = last[last.len() - width..].to_vec();
        let actual = serde_json::json!({ "committed": committed, "owner": cursor.owner, "item": cursor.item, "entry": entry });
        let mut released = 0;
        for _ in 0..32 {
            match payload.close_step(1, JOB_PAYLOAD_PAGE_BYTES) {
                semio_framework_job::JobPayloadCloseStep::Pending { released_bytes, .. } => released += released_bytes,
                semio_framework_job::JobPayloadCloseStep::Complete => break,
            }
        }
        assert!(payload.terminal_is_empty());
        eprintln!("[DEBUG] numerical entry {kind}/{remaining} fault={fault:?}, committed={committed}, held={held}, cursor={cursor:?}, released={released}");
        observations.push((format!("{kind}/{remaining}"), actual, row["expected"].clone(), fault.is_none(), held, released == page_count * JOB_PAYLOAD_PAGE_BYTES));
    }
    integers.pop();
    scalars.pop();
    while integers.allocated_bytes() != 0 { integers.release_empty_page(JOB_PAYLOAD_PAGE_BYTES).unwrap(); }
    while scalars.allocated_bytes() != 0 { scalars.release_empty_page(JOB_PAYLOAD_PAGE_BYTES).unwrap(); }
    for (id, actual, expected, valid, held, released) in observations {
        assert!(valid && held && released, "{id} preserves pending input and exact backing");
        assert_eq!(actual, expected, "{id}");
    }
}

fn graph_laplacian_plus_identity(n: usize, edges: &[(usize, usize)]) -> Coo {
    let mut degree = vec![0usize; n];
    for &(u, v) in edges {
        degree[u] += 1;
        degree[v] += 1;
    }
    let mut coo = Coo::new(n);
    for i in 0..n {
        coo.add(i, i, degree[i] as f64 + 1.0);
    }
    for &(u, v) in edges {
        coo.add(u, v, -1.0);
        coo.add(v, u, -1.0);
    }
    coo
}

#[test]
fn ldlt_matches_dense_lu_on_random_spd() {
    let n = 8;
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
    let coo = graph_laplacian_plus_identity(n, &edges);
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
    let coo = graph_laplacian_plus_identity(n, &edges);
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
fn pcg_matches_ldlt_and_dense_lu() {
    let n = 8;
    let edges = [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 0), (0, 4), (2, 6)];
    let coo = graph_laplacian_plus_identity(n, &edges);
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
    let k_coo = graph_laplacian_plus_identity(n, &edges);
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

/// 🔍️ `CscSym::get` reads back every entry of a symmetric matrix (both `row<=col` and `row>col`
/// orderings resolve to the same stored upper-triangle slot) and returns `0.0` for an absent entry;
/// `to_csr_full` mirrors the SAME matrix into a full (both triangles materialized) `Csr`.
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

/// 🔢️ `negative_pivot_count` counts `D[j] < 0` — a diagonal (already-factored-trivially) indefinite
/// matrix with one negative entry must report exactly one negative pivot.
#[test]
fn negative_pivot_count_counts_negative_diagonal_entries() {
    let mut coo = Coo::new(3);
    coo.add(0, 0, 1.0);
    coo.add(1, 1, -2.0);
    coo.add(2, 2, 3.0);
    let factor = ldlt_factor(&coo.to_csc_sym_upper()).expect("diagonal matrix factors trivially");
    assert_eq!(factor.negative_pivot_count(), 1);
}

/// ⏱️ `pcg` returns immediately (zero iterations, `converged: true`) when the initial guess `x0`
/// already satisfies the residual tolerance.
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

/// 🎯️ `dense_symmetric_eigen_jacobi` on a 0x0 matrix returns empty eigenvalues/eigenvectors instead
/// of looping — the degenerate size `subspace_iteration`'s own `.max(1)` guard against normally
/// avoids, but the helper itself must still handle directly.
#[test]
fn dense_symmetric_eigen_jacobi_handles_zero_size_matrix() {
    let a = MatD::zeros(0, 0);
    let (vals, vecs) = dense_symmetric_eigen_jacobi(&a);
    assert!(vals.is_empty());
    assert_eq!(vecs.rows, 0);
    assert_eq!(vecs.cols, 0);
}

fn test_operation(id: u64) -> Operation {
    Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(7), semio_framework_job::Generation(3), 11)
}

/// 📣️ PCG publication consumes its own grant and preserves pending control state on refusal.
#[test]
fn pcg_job_publication_grants_preserve_pending_state_and_work_cursor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/⛽️publication-grant/🔣️.json")).unwrap();
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
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(case["fuel"].as_u64().unwrap(), case["deadline"].as_u64().unwrap()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        let outcome = match job.step(&mut context) {
            StepOutcome::Yield => "yield",
            StepOutcome::PreviewReady(payload) => { close_payload(payload); "preview" }
            StepOutcome::CheckpointReady(checkpoint) => { close_payload(checkpoint.state); "checkpoint" }
            StepOutcome::Complete(candidate) => { close_payload(candidate.state); close_payload(candidate.output); "complete" }
            StepOutcome::Fault(fault) => { close_payload(fault.detail); "fault" }
            StepOutcome::Cancelled => "cancelled",
        };
        let pending = match kind { "checkpoint" => job.state.checkpoint_due, "preview" => job.state.preview_due, _ => job.state.stage == PcgStage::Complete };
        let mut observed = serde_json::json!({ "pending": pending, "cursor": job.state.entry_cursor, "fuelRemaining": context.fuel_remaining(), "outcome": outcome });
        let mut closed = false;
        for _ in 0..1_024 {
            let (terminal, items, bytes) = job.close_step(NUMERICAL_OWNER_PAGE_BYTES);
            assert!(items <= 1 && bytes <= NUMERICAL_OWNER_PAGE_BYTES);
            if terminal { closed = true; break; }
        }
        assert!(closed, "publication fixture closes its exact matrix and scalar owners");
        observed["terminalEmpty"] = serde_json::json!(InteractiveJob::terminal_is_empty(&job));
        assert_eq!(observed, case["expected"], "publication fixture {index}");
    }
}

/// 🧭️ Each admitted initial PCG scalar fills the already owned search direction.
#[test]
fn pcg_job_initial_precondition_preserves_admitted_direction_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️scalar-owners/🔣️.json")).unwrap();
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
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        assert!(matches!(job.step(&mut context), StepOutcome::Yield));
        assert_eq!(context.fuel_remaining(), 0);
        observed.push((job.state.p.0.clone(), job.state.p.0.as_ptr() == pointer && job.state.p.0.capacity() == capacity));
    }
    for _ in 0..1_024 { if job.close_step(NUMERICAL_OWNER_PAGE_BYTES).0 { break; } }
    assert!(InteractiveJob::terminal_is_empty(&job));
    for (index, (direction, retained)) in observed.into_iter().enumerate() {
        assert_eq!(direction, expected[index], "precondition scalar {index}");
        assert_eq!(retained, case["retainsBacking"].as_bool().unwrap(), "direction backing {index}");
    }
    eprintln!("[DEBUG] PCG initialized three NumPy scalars in the original admitted direction backing");
}

/// 🪜️ Retained LDLT substitution visits every factor column before diagonal scaling.
#[test]
fn subspace_factor_cursor_matches_numpy_for_three_nondiagonal_right_hand_sides() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️scalar-owners/🔣️.json")).unwrap();
    let case = &fixture["factor"];
    let lower: Vec<Vec<f64>> = serde_json::from_value(case["lower"].clone()).unwrap();
    let rhs: Vec<Vec<f64>> = serde_json::from_value(case["rhs"].clone()).unwrap();
    let expected: Vec<Vec<f64>> = serde_json::from_value(case["expected"].clone()).unwrap();
    let factor = LdltFactor {
        n: 3,
        l_cols: (0..3).map(|column| ((column + 1)..3).map(|row| (row as u32, lower[row][column])).collect()).collect(),
        d: serde_json::from_value(case["diagonal"].clone()).unwrap(),
    };
    let mut mass = Coo::new(3);
    for index in 0..3 { mass.add(index, index, 1.0); }
    let mut job = SubspaceIterationJob::new(test_operation(996), factor, mass.to_csr(), 3, 1, 30);
    job.state.work.rhs = MatD::zeros(3, 3);
    job.state.work.solved = MatD::zeros(3, 3);
    for row in 0..3 { for column in 0..3 { job.state.work.rhs.set(row, column, rhs[row][column]); } }
    let pointer = job.state.work.solved.data.as_ptr();
    let capacity = job.state.work.solved.data.capacity();
    job.reset_cursor(SubspaceStage::FactorForwardEntry);
    let mut scalar_steps = true;
    let mut retained = true;
    for _ in 0..100 {
        let before = job.state.work.solved.data.clone();
        match job.state.work.stage {
            SubspaceStage::FactorForwardEntry => job.advance_factor_forward(),
            SubspaceStage::FactorDiagonalEntry => job.advance_factor_diagonal(),
            SubspaceStage::FactorBackwardEntry => job.advance_factor_backward(),
            _ => break,
        }
        scalar_steps &= before.iter().zip(&job.state.work.solved.data).filter(|(left, right)| left != right).count() <= 1;
        retained &= job.state.work.solved.data.as_ptr() == pointer && job.state.work.solved.data.capacity() == capacity;
    }
    let terminal = job.state.work.stage == SubspaceStage::OrthogonalizePairElement;
    let observed: Vec<Vec<f64>> = (0..3).map(|row| (0..3).map(|column| job.state.work.solved.get(row, column)).collect()).collect();
    for _ in 0..1_024 {
        if matches!(InteractiveJob::close_step(&mut job, 1, NUMERICAL_OWNER_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) { break; }
    }
    assert!(InteractiveJob::terminal_is_empty(&job));
    assert!(terminal && scalar_steps && retained);
    assert_eq!(observed, expected);
    eprintln!("[DEBUG] Subspace factor cursor matches three independent NumPy solves with one retained scalar per transition");
}

/// 🎶 Modal publication replaces every scalar before exposing terminal convergence.
#[test]
fn subspace_publication_restarts_at_zero_and_commits_convergence_after_the_last_scalar() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️scalar-owners/🔣️.json")).unwrap();
    let case = &fixture["publication"];
    let diagonal: Vec<f64> = serde_json::from_value(case["diagonal"].clone()).unwrap();
    let eigenvalues: Vec<f64> = serde_json::from_value(case["eigenvalues"].clone()).unwrap();
    let previous: Vec<f64> = serde_json::from_value(case["previous"].clone()).unwrap();
    let expected: Vec<Vec<f64>> = serde_json::from_value(case["steps"].clone()).unwrap();
    for modes in serde_json::from_value::<Vec<usize>>(case["requestedModes"].clone()).unwrap() {
        let mut mass = Coo::new(3);
        for index in 0..3 { mass.add(index, index, 1.0); }
        let operation = test_operation(997 + modes as u64);
        let factor = LdltFactor { n: 3, l_cols: vec![Vec::new(), Vec::new(), Vec::new()], d: diagonal.clone() };
        let mut job = SubspaceIterationJob::new(operation, factor, mass.to_csr(), 3, modes, 30);
        job.state.factor_validation_complete = true;
        job.state.x = MatD::zeros(3, 3);
        job.state.work.candidate_x = MatD::zeros(3, 3);
        job.state.work.theta = eigenvalues.clone();
        job.state.final_theta = previous.clone();
        job.state.converged_count = modes;
        job.reset_cursor(SubspaceStage::ConvergenceMode);
        job.state.work.first = modes;
        let pointer = job.state.final_theta.as_ptr();
        let capacity = job.state.final_theta.capacity();
        let mut sequence = 0;
        let mut observations = Vec::new();
        for fuel in [0, 1, 0, 1, 1, 1, 1, 1] {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            let yielded = matches!(job.step(&mut context), StepOutcome::Yield);
            observations.push((yielded && context.fuel_remaining() == 0, job.state.final_theta.clone(), job.state.converged, job.state.work.first, job.state.iteration, job.state.final_theta.as_ptr() == pointer && job.state.final_theta.capacity() == capacity, job.terminal_writer.is_none()));
        }
        let solution = job.solution();
        for _ in 0..1_024 {
            if matches!(InteractiveJob::close_step(&mut job, 1, NUMERICAL_OWNER_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) { break; }
        }
        assert!(InteractiveJob::terminal_is_empty(&job));
        for (index, (yielded, values, converged, cursor, iteration, retained, no_terminal)) in observations.into_iter().enumerate() {
            assert!(yielded && retained && no_terminal, "publication opportunity {index}, modes {modes}: yielded={yielded}, retained={retained}, no_terminal={no_terminal}");
            assert_eq!(values, if index < 3 { &previous } else { &expected[(index - 3).min(2)] }.clone(), "publication scalar {index}, modes {modes}");
            assert_eq!(converged, index == 7, "publication convergence {index}, modes {modes}");
            assert_eq!(iteration, usize::from(index == 7));
            if index == 1 || index == 2 { assert_eq!(cursor, 0); }
        }
        assert_eq!(solution.values, eigenvalues[..modes]);
    }
    eprintln!("[DEBUG] Subspace publication retains its backing and publishes all NumPy eigenvalues before terminal convergence");
}

fn drive_pcg_job(mut job: PcgJob, operation: Operation) -> (VecD, PcgStats) {
    let mut sequence = 0;
    loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match job.step(&mut context) {
            StepOutcome::Complete(candidate) => {
                close_payload(candidate.state);
                close_payload(candidate.output);
                let (solution, stats) = job.solution();
                return (solution.clone(), stats);
            }
            StepOutcome::PreviewReady(payload) => close_payload(payload),
            StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
            StepOutcome::Fault(fault) => panic!("pcg fault: {}", String::from_utf8_lossy(&payload_bytes(fault.detail))),
            StepOutcome::Cancelled => panic!("pcg unexpectedly cancelled"),
            _ => {}
        }
    }
}

fn restore_ldlt(operation: Operation, payload: RetainedJobPayload) -> Result<LdltJob, NumericalCheckpointFault> {
    let mut restore = LdltRestoreCursor::new(operation, payload);
    let mut sequence = 0;
    for _ in 0..200_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match restore.step(&mut context) {
            Ok(Some(job)) => return Ok(job),
            Ok(None) => {}
            Err(fault) => {
                while !restore.terminal_is_empty() {
                    let _ = restore.close_step(1, usize::MAX);
                }
                return Err(fault);
            }
        }
    }
    Err(NumericalCheckpointFault::Truncated)
}

fn restore_subspace(operation: Operation, payload: RetainedJobPayload) -> Result<SubspaceIterationJob, NumericalCheckpointFault> {
    let mut restore = SubspaceRestoreCursor::new(operation, payload);
    let mut sequence = 0;
    for _ in 0..400_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match restore.step(&mut context) {
            Ok(Some(job)) => return Ok(job),
            Ok(None) => {}
            Err(fault) => {
                while !restore.terminal_is_empty() {
                    let _ = restore.close_step(1, usize::MAX);
                }
                return Err(fault);
            }
        }
    }
    Err(NumericalCheckpointFault::Truncated)
}

fn close_payload(mut payload: RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

#[test]
fn pcg_job_is_batch_deterministic_and_matches_reference() {
    let n = 24;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let csr = graph_laplacian_plus_identity(n, &edges).to_csr();
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
    let csr = graph_laplacian_plus_identity(n, &edges).to_csr();
    let b = VecD::from_vec((0..n).map(|i| (i + 1) as f64).collect());
    let operation = test_operation(102);
    let expected = drive_pcg_job(PcgJob::new(operation, csr.clone(), b.clone(), VecD::zeros(n), 1e-12, 200, 7), operation);
    let mut job = PcgJob::new(operation, csr, b, VecD::zeros(n), 1e-12, 200, 7);
    let mut sequence = 0;
    let checkpoint = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match job.step(&mut context) {
            StepOutcome::CheckpointReady(checkpoint) => break payload_bytes(checkpoint.state),
            StepOutcome::PreviewReady(payload) => close_payload(payload),
            StepOutcome::Complete(candidate) => { close_payload(candidate.state); close_payload(candidate.output); panic!("PCG completes before the required checkpoint"); }
            StepOutcome::Fault(fault) => panic!("PCG checkpoint fault: {}", String::from_utf8_lossy(&payload_bytes(fault.detail))),
            StepOutcome::Cancelled => panic!("PCG checkpoint unexpectedly cancelled"),
            StepOutcome::Yield => {}
        }
    };
    let resumed = PcgJob::from_checkpoint(operation, &checkpoint).expect("pcg checkpoint restores");
    assert_eq!(resumed.checkpoint_bytes(), checkpoint);
    assert_eq!(drive_pcg_job(resumed, operation), expected);
}

#[test]
fn pcg_job_publishes_coarse_preview_before_final_tolerance() {
    let n = 40;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let csr = graph_laplacian_plus_identity(n, &edges).to_csr();
    let b = VecD::from_vec((0..n).map(|i| (i + 1) as f64).collect());
    let operation = test_operation(107);
    let mut job = PcgJob::new(operation, csr, b, VecD::zeros(n), 1e-12, 200, 512);
    let mut sequence = 0;
    loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(u64::MAX, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match job.step(&mut context) {
            StepOutcome::PreviewReady(bytes) => {
                let preview: PcgPreview = decode_value(&payload_bytes(bytes)).expect("pcg preview decodes");
                assert_eq!(preview.quality, PcgQuality::Coarse);
                assert!(preview.residual_norm < 1e-3);
                assert!(preview.residual_norm >= 1e-12);
                break;
            }
            StepOutcome::Complete(candidate) => { close_payload(candidate.state); close_payload(candidate.output); panic!("pcg reached final tolerance before publishing coarse quality"); }
            StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
            StepOutcome::Fault(fault) => panic!("PCG preview fault: {}", String::from_utf8_lossy(&payload_bytes(fault.detail))),
            StepOutcome::Cancelled => panic!("PCG preview unexpectedly cancelled"),
            StepOutcome::Yield => {}
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
    let before = stale.checkpoint_bytes();
    let mut sequence = 0;
    let mut context = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), StepBudget::new(100, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(stale.step(&mut context), StepOutcome::Fault(_)));
    assert_eq!(stale.checkpoint_bytes(), before);

    let mut cancelled = PcgJob::new(operation, csr, VecD::from_vec(vec![1.0; 8]), VecD::zeros(8), 1e-9, 20, 8);
    let before = cancelled.checkpoint_bytes();
    let token = semio_framework_job::root_cancel_token();
    semio_framework_async::block_on(token.cancel());
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(100, u64::MAX), token, || Some(0), &mut sequence);
    assert_eq!(cancelled.step(&mut context), StepOutcome::Cancelled);
    assert_eq!(cancelled.checkpoint_bytes(), before);
}

#[test]
fn ldlt_job_checkpoint_resume_matches_reference() {
    let n = 30;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    let matrix = graph_laplacian_plus_identity(n, &edges).to_csc_sym_upper();
    let expected = ldlt_factor(&matrix).expect("reference factors");
    let operation = test_operation(104);
    let mut job = LdltJob::new(operation, matrix, 3);
    let mut sequence = 0;
    let checkpoint = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match job.step(&mut context) {
            StepOutcome::CheckpointReady(checkpoint) => break checkpoint.state,
            StepOutcome::Yield => {}
            outcome => panic!("unexpected LDLT checkpoint outcome: {outcome:?}"),
        }
    };
    let mut resumed = restore_ldlt(operation, checkpoint).expect("retained LDLT checkpoint restores");
    loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(2, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match resumed.step(&mut context) {
            StepOutcome::Complete(candidate) => {
                close_payload(candidate.state);
                close_payload(candidate.output);
                break;
            }
            StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
            _ => {}
        }
    }
    assert_eq!(resumed.factor(), Some(expected));
    while !InteractiveJob::terminal_is_empty(&job) {
        let _ = InteractiveJob::close_step(&mut job, 1, usize::MAX);
    }
    while !InteractiveJob::terminal_is_empty(&resumed) {
        let _ = InteractiveJob::close_step(&mut resumed, 1, usize::MAX);
    }
}

#[test]
fn p6h_ldlt_microcursor_max_plus_one_cancel_deadline_stale_replay_and_numerical_parity() {
    let n = 18;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|index| (index, index + 1)).collect();
    let matrix = graph_laplacian_plus_identity(n, &edges).to_csc_sym_upper();
    let reference = ldlt_factor(&matrix).expect("batch reference factor");
    let operation = test_operation(121);
    let drive = |fuel: u64| {
        let mut job = LdltJob::new(operation, matrix.clone(), 1);
        let mut sequence = 0;
        loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            match job.step(&mut context) {
                StepOutcome::Complete(candidate) => {
                    close_payload(candidate.state);
                    close_payload(candidate.output);
                    let factor = job.factor().expect("completed factor owner");
                    while !InteractiveJob::terminal_is_empty(&job) {
                        let _ = InteractiveJob::close_step(&mut job, 1, usize::MAX);
                    }
                    return factor;
                }
                StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
                StepOutcome::PreviewReady(preview) => close_payload(preview),
                _ => {}
            }
        }
    };
    assert_eq!(drive(1), reference);
    assert_eq!(drive(2), reference);
    assert_eq!(drive(4), reference);

    let mut zero_fuel = LdltJob::new(operation, matrix.clone(), 1);
    let before = zero_fuel.state.clone();
    let mut sequence = 0;
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(0, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert_eq!(zero_fuel.step(&mut context), StepOutcome::Yield);
    assert!(zero_fuel.state == before);

    let mut deadline = LdltJob::new(operation, matrix.clone(), 1);
    let before = deadline.state.clone();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert_eq!(deadline.step(&mut context), StepOutcome::Yield);
    assert!(deadline.state == before);

    let mut stale = LdltJob::new(operation, matrix.clone(), 1);
    let before = stale.state.clone();
    let mut context = StepContext::new(operation.operation, semio_framework_job::Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(stale.step(&mut context), StepOutcome::Fault(_)));
    assert!(stale.state == before);

    let mut cancelled = LdltJob::new(operation, matrix, 1);
    let before = cancelled.state.clone();
    let token = semio_framework_job::root_cancel_token();
    semio_framework_async::block_on(token.cancel());
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || Some(0), &mut sequence);
    assert_eq!(cancelled.step(&mut context), StepOutcome::Cancelled);
    assert!(cancelled.state == before);

    let mut lookup = LdltJob::new(operation, deadline.state.a.clone(), 1);
    let mut observed_lookup = false;
    for _ in 0..200_000 {
        if lookup.state.cursor.stage == LdltColumnStage::ContributorLookup && lookup.state.cursor.lookup_initialized && lookup.state.cursor.lookup_lower < lookup.state.cursor.lookup_upper {
            let before = lookup.state.clone();
            let mut expired = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            assert_eq!(lookup.step(&mut expired), StepOutcome::Yield);
            assert!(lookup.state == before);
            let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            assert_eq!(lookup.step(&mut one), StepOutcome::Yield);
            assert_eq!(lookup.state.cursor.contributor, before.cursor.contributor);
            assert!(lookup.state.cursor.lookup_lower != before.cursor.lookup_lower || lookup.state.cursor.lookup_upper != before.cursor.lookup_upper);
            observed_lookup = true;
            break;
        }
        let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = lookup.step(&mut one) {
            close_payload(checkpoint.state);
        }
    }
    assert!(observed_lookup, "adversarial LDLT reaches retained contributor comparison");
    while !InteractiveJob::terminal_is_empty(&lookup) {
        let _ = InteractiveJob::close_step(&mut lookup, 1, usize::MAX);
    }

    let refused = CscSym { n: LDLT_MAXIMUM_ORDER + 1, colptr: vec![0; LDLT_MAXIMUM_ORDER + 2], rowind: Vec::new(), vals: Vec::new() };
    let mut maximum_plus_one = LdltJob::new(operation, refused, 1);
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(maximum_plus_one.step(&mut context), StepOutcome::Fault(_)));

    let mut publishing = LdltJob::new(operation, deadline.state.a.clone(), 1);
    for _ in 0..200_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = publishing.step(&mut context) {
            close_payload(checkpoint.state);
        }
        if publishing.output_writer.is_some() {
            break;
        }
    }
    assert!(publishing.output_writer.is_some(), "retained LDLT result writer becomes interruptible before publication");
    InteractiveJob::begin_close(&mut publishing);
    for _ in 0..200_000 {
        if matches!(InteractiveJob::close_step(&mut publishing, 1, usize::MAX), semio_framework_job::InteractiveJobCloseStep::Complete) {
            break;
        }
    }
    assert!(InteractiveJob::terminal_is_empty(&publishing));

    let checkpoint = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = zero_fuel.step(&mut context) {
            break checkpoint.state;
        }
    };
    let mut roundtrip = restore_ldlt(operation, checkpoint).expect("LDLT retained checkpoint roundtrip");
    assert_eq!(roundtrip.state.identity, zero_fuel.state.identity);
    while !InteractiveJob::terminal_is_empty(&roundtrip) {
        let _ = InteractiveJob::close_step(&mut roundtrip, 1, usize::MAX);
    }
    assert!(matches!(restore_ldlt(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState)), Err(NumericalCheckpointFault::Truncated)));
    let wrong_revision = Operation::new(operation.operation, semio_framework_job::RevisionId(operation.base_revision.0 + 1), operation.generation, operation.seed);
    let wrong_seed = Operation::new(operation.operation, operation.base_revision, operation.generation, operation.seed + 1);
    let fresh_checkpoint = |id: u64| {
        let mut source = LdltJob::new(operation, graph_laplacian_plus_identity(6, &[(0, 1), (1, 2)]).to_csc_sym_upper(), 1);
        let mut local_sequence = id;
        loop {
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut local_sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = source.step(&mut context) {
                break checkpoint.state;
            }
        }
    };
    assert!(matches!(restore_ldlt(wrong_revision, fresh_checkpoint(1)), Err(NumericalCheckpointFault::Stale)));
    assert!(matches!(restore_ldlt(wrong_seed, fresh_checkpoint(2)), Err(NumericalCheckpointFault::Stale)));
    let mut interrupted_restore = LdltRestoreCursor::new(operation, fresh_checkpoint(3));
    let mut restore_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(interrupted_restore.step(&mut restore_context), Ok(None)));
    while !interrupted_restore.terminal_is_empty() {
        match interrupted_restore.close_step(1, usize::MAX) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
            semio_framework_job::InteractiveJobCloseStep::Complete => {}
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("LDLT restore close cannot block"),
        }
    }

    for owner in [&mut deadline, &mut stale, &mut cancelled, &mut maximum_plus_one] {
        while !InteractiveJob::terminal_is_empty(owner) {
            let _ = InteractiveJob::close_step(owner, 1, usize::MAX);
        }
    }

    let mut closing = zero_fuel;
    let mut close_turns = 0;
    loop {
        close_turns += 1;
        match InteractiveJob::close_step(&mut closing, 1, usize::MAX) {
            semio_framework_job::InteractiveJobCloseStep::Complete => break,
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("fixed LDLT close cannot block"),
        }
        assert!(close_turns < 20_000);
    }
    assert!(InteractiveJob::terminal_is_empty(&closing));
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
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match uninterrupted.step(&mut context) {
            StepOutcome::Complete(candidate) => {
                close_payload(candidate.state);
                close_payload(candidate.output);
                break uninterrupted.solution();
            }
            StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
            StepOutcome::PreviewReady(preview) => close_payload(preview),
            _ => {}
        }
    };

    let mut interrupted = SubspaceIterationJob::new(operation, factor, mass, n, 4, 30);
    let checkpoint = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match interrupted.step(&mut context) {
            StepOutcome::CheckpointReady(checkpoint) => break checkpoint.state,
            StepOutcome::Yield | StepOutcome::PreviewReady(_) => {}
            outcome => panic!("unexpected subspace checkpoint outcome: {outcome:?}"),
        }
    };
    let mut resumed = restore_subspace(operation, checkpoint).expect("subspace retained checkpoint restores");
    while !InteractiveJob::terminal_is_empty(&interrupted) {
        let _ = InteractiveJob::close_step(&mut interrupted, 1, usize::MAX);
    }
    loop {
        let mut yielded = StepContext::new(operation.operation, operation.generation, StepBudget::new(0, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        assert!(matches!(resumed.step(&mut yielded), StepOutcome::Yield));
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        match resumed.step(&mut context) {
            StepOutcome::Complete(candidate) => {
                close_payload(candidate.state);
                close_payload(candidate.output);
                break;
            }
            StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
            StepOutcome::PreviewReady(preview) => close_payload(preview),
            _ => {}
        }
    }
    assert_eq!(resumed.solution(), expected);
    assert_eq!(resumed.preview().converged_count, 4);
    while !InteractiveJob::terminal_is_empty(&uninterrupted) {
        let _ = InteractiveJob::close_step(&mut uninterrupted, 1, usize::MAX);
    }
    while !InteractiveJob::terminal_is_empty(&resumed) {
        let _ = InteractiveJob::close_step(&mut resumed, 1, usize::MAX);
    }
}

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
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut replay_sequence);
            match replay.step(&mut context) {
                StepOutcome::Complete(candidate) => {
                    close_payload(candidate.state);
                    close_payload(candidate.output);
                    let solution = replay.solution();
                    while !InteractiveJob::terminal_is_empty(&replay) {
                        let _ = InteractiveJob::close_step(&mut replay, 1, usize::MAX);
                    }
                    return solution;
                }
                StepOutcome::CheckpointReady(checkpoint) => close_payload(checkpoint.state),
                StepOutcome::PreviewReady(preview) => close_payload(preview),
                StepOutcome::Fault(fault) => panic!("subspace replay fault: {:?}", fault.detail),
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
    let mut expired = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, 0), semio_framework_job::root_cancel_token(), || Some(0), &mut validation_sequence);
    assert_eq!(validating.step(&mut expired), StepOutcome::Yield);
    assert!(validating.state == before);
    let mut one = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut validation_sequence);
    assert_eq!(validating.step(&mut one), StepOutcome::Yield);
    assert_eq!(validating.state.factor_validation_cursor, 1, "one construction grant validates one factor owner");
    while !InteractiveJob::terminal_is_empty(&validating) {
        let _ = InteractiveJob::close_step(&mut validating, 1, usize::MAX);
    }

    let mut oversized = Vec::<(u32, f64)>::new();
    oversized.try_reserve_exact(NUMERICAL_OWNER_PAGE_BYTES / size_of::<(u32, f64)>() + 1).expect("hostile factor backing");
    assert!(oversized.capacity() * size_of::<(u32, f64)>() > NUMERICAL_OWNER_PAGE_BYTES);
    let mut columns = vec![Vec::new(); n];
    columns[0] = oversized;
    let mut refused_owner = SubspaceIterationJob::new(operation, LdltFactor { n, l_cols: columns, d: vec![1.0; n] }, mass.clone(), n, 3, 1);
    let mut refused_sequence = 0;
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut refused_sequence);
    assert!(matches!(refused_owner.step(&mut context), StepOutcome::Fault(_)));
    while !InteractiveJob::terminal_is_empty(&refused_owner) {
        let _ = InteractiveJob::close_step(&mut refused_owner, 1, usize::MAX);
    }

    for refused_order in [0, SUBSPACE_MAXIMUM_ORDER + 1] {
        let factor = LdltFactor { n: refused_order, l_cols: vec![Vec::new(); refused_order], d: vec![1.0; refused_order] };
        let mass = Csr::from_owned_parts(refused_order, vec![0; refused_order + 1], Vec::new(), Vec::new());
        let mut refused = SubspaceIterationJob::new(operation, factor, mass, refused_order, usize::from(refused_order != 0), 1);
        let mut refused_sequence = 0;
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut refused_sequence);
        assert!(matches!(refused.step(&mut context), StepOutcome::Fault(_)));
        while !InteractiveJob::terminal_is_empty(&refused) {
            let _ = InteractiveJob::close_step(&mut refused, 1, usize::MAX);
        }
    }

    let mut publishing = SubspaceIterationJob::new(operation, factor.clone(), mass.clone(), n, 3, 1);
    let mut publishing_sequence = 0;
    for _ in 0..200_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut publishing_sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = publishing.step(&mut context) {
            close_payload(checkpoint.state);
        }
        if publishing.preview_writer.is_some() {
            break;
        }
    }
    assert!(publishing.preview_writer.is_some(), "retained preview page writer becomes interruptible before publication");
    InteractiveJob::begin_close(&mut publishing);
    for _ in 0..200_000 {
        if matches!(InteractiveJob::close_step(&mut publishing, 1, usize::MAX), semio_framework_job::InteractiveJobCloseStep::Complete) {
            break;
        }
    }
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
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
            if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
                break checkpoint.state;
            }
        };
        let mut cancelled = restore_subspace(operation, checkpoint).expect("stage retained checkpoint");
        let before = cancelled.state.clone();
        let token = semio_framework_job::root_cancel_token();
        semio_framework_async::block_on(token.cancel());
        let mut cancelled_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || Some(0), &mut sequence);
        assert_eq!(cancelled.step(&mut cancelled_context), StepOutcome::Cancelled);
        assert!(cancelled.state == before);
        while !InteractiveJob::terminal_is_empty(&cancelled) {
            let _ = InteractiveJob::close_step(&mut cancelled, 1, usize::MAX);
        }
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::Fault(fault) = job.step(&mut context) {
            panic!("subspace stage walk fault: {:?}", fault.detail);
        }
    }
    assert_eq!(seen.len(), 16);

    assert!(matches!(restore_subspace(operation, RetainedJobPayload::empty(JobPayloadStream::CheckpointState)), Err(NumericalCheckpointFault::Truncated)));
    let wrong_generation = Operation::new(operation.operation, operation.base_revision, semio_framework_job::Generation(operation.generation.0 + 1), operation.seed);
    job.state.checkpoint_due = true;
    let checkpoint = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
            break checkpoint.state;
        }
    };
    assert!(matches!(restore_subspace(wrong_generation, checkpoint), Err(NumericalCheckpointFault::Stale)));
    job.state.checkpoint_due = true;
    let checkpoint = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
        if let StepOutcome::CheckpointReady(checkpoint) = job.step(&mut context) {
            break checkpoint.state;
        }
    };
    let mut interrupted_restore = SubspaceRestoreCursor::new(operation, checkpoint);
    let mut restore_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
    assert!(matches!(interrupted_restore.step(&mut restore_context), Ok(None)));
    while !interrupted_restore.terminal_is_empty() {
        match interrupted_restore.close_step(1, usize::MAX) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
            semio_framework_job::InteractiveJobCloseStep::Complete => {}
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("subspace restore close cannot block"),
        }
    }
    let mut closing = job;
    let mut close_turns = 0;
    loop {
        close_turns += 1;
        match InteractiveJob::close_step(&mut closing, 1, usize::MAX) {
            semio_framework_job::InteractiveJobCloseStep::Complete => break,
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("fixed subspace close cannot block"),
        }
        assert!(close_turns < 200_000);
    }
    assert!(InteractiveJob::terminal_is_empty(&closing));
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
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence);
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
    let job = construction.take_complete().expect("terminal construction transfers once");
    assert_eq!(job.state.a.n, 3);
    assert!(construction.take_complete().is_none());

    let matrix = Csr::from_owned_parts(3, vec![0, 1, 2, 3], vec![0, 1, 2], vec![2.0, 3.0, 4.0]);
    let mut interrupted = PcgJobConstruction::new(test_operation(108), matrix);
    assert!(!interrupted.step_one().expect("one reservation"));
    let before = interrupted.matrix.as_ref().expect("matrix retained").vals.len();
    let (terminal, _, _) = interrupted.close_step(4_096);
    assert!(!terminal);
    assert_eq!(interrupted.matrix.as_ref().expect("matrix shell retained").vals.len() + 1, before);
}
