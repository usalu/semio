use super::*;
use semio_s_fem_test_support::{close_payload_page_bytes, close_writer_released, test_operation, TEST_GRANT};

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
    use semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️numerical-pages/🔣️.json")).unwrap();
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
                let mut sequence_receipt = RetainedCloneProgress::default();
                let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut sequence_receipt);
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
                Ok(payload) => {
                    for index in 0..payload.page_count() {
                        let bytes = payload.page(index).unwrap();
                        let owner = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
                        let item = u64::from_le_bytes(bytes[24..32].try_into().unwrap());
                        let start = if owner == 0 { 40 } else { 32 };
                        let scalars = (bytes.len() - start) / 8;
                        pages.push(serde_json::json!({ "owner": owner, "item": item, "bytes": bytes.len(), "scalars": scalars }));
                        for bytes in bytes[start..].chunks_exact(8) { decoded.push(f64::from_bits(u64::from_le_bytes(bytes.try_into().unwrap()))); }
                    }
                    released += close_payload_page_bytes(payload);
                }
                Err(returned) => { writer = Some(returned); complete = false; }
            }
        }
        if let Some(writer) = writer.as_mut() {
            released += close_writer_released(writer);
            assert!(writer.terminal_is_empty());
        }
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
        let mut sequence = 0;
        let mut writer = RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut sequence_receipt);
        writer.begin_staged_page(&mut context).unwrap();
        writer.write_staged(&[0; JOB_PAYLOAD_PAGE_BYTES][..JOB_PAYLOAD_PAGE_BYTES - remaining]).unwrap();
        let mut cursor = NumericalPageCursor { field: 0, owner: usize::from(kind != "length" && kind != "matrix-shape"), item: 0 };
        let before = cursor;
        let mut fault = advance(kind, &mut writer, &mut cursor, &matrix, &integers, &scalars).err();
        let committed = writer.staged_page_len().is_none();
        let held = !committed || cursor == before;
        drop(context);
        if committed && fault.is_none() {
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence, &mut sequence_receipt);
            writer.begin_staged_page(&mut context).unwrap();
            advance_numerical_page_header(&mut writer, b"FEMPGT1\0", 42, cursor).unwrap();
            fault = advance(kind, &mut writer, &mut cursor, &matrix, &integers, &scalars).err();
        }
        writer.commit_staged_page().unwrap();
        let payload = writer.finish().unwrap();
        let page_count = payload.page_count();
        let last = payload.page(page_count - 1).unwrap();
        let entry = last[last.len() - width..].to_vec();
        let actual = serde_json::json!({ "committed": committed, "owner": cursor.owner, "item": cursor.item, "entry": entry });
        let released = close_payload_page_bytes(payload);
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
