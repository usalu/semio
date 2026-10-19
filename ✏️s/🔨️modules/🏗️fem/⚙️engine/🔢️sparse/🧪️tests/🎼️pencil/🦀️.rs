use super::*;

#[test]
fn original_symmetric_pencil_matches_independent_sparse_storage_and_refuses_owner_transfer() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎼️pencil/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let n = case["order"].as_u64().unwrap() as usize;
        let mut first = Coo::new(n);
        for row in 0..n {
            first.add(row, row, case["kDiagonal"].as_f64().unwrap());
            if row + 1 < n { let value = case["kOffDiagonal"].as_f64().unwrap(); first.add(row, row + 1, value); first.add(row + 1, row, value); }
        }
        let u32s = |key: &str| case[key].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect::<Vec<_>>();
        let f64s = |key: &str| case[key].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>();
        let second = Csr::from_owned_parts(n, u32s("bIndptr"), u32s("bIndices"), f64s("bValues"));
        let original_pointer = second.vals.backing_ptr(0).unwrap();
        let original_bytes = second.allocated_bytes();
        let mut owner = SymmetricPencilConstruction::new_pair(first.to_csr(), second);
        let operation = test_operation(n as u64 + 83_100);
        let mut sequence = 0;
        let step = |owner: &mut SymmetricPencilConstruction, grant: RetainedCloneGrant, sequence: &mut u64| {
            let mut receipt = RetainedCloneProgress::default();
            let result = { let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), sequence, &mut receipt); owner.step_one(&mut context) };
            assert!(receipt.copied_items <= 1); assert!(receipt.retained_capacity_bytes <= grant.maximum_capacity_bytes); (result, receipt)
        };
        let one = RetainedCloneGrant { maximum_items: 1, ..TEST_GRANT };
        assert_eq!(step(&mut owner, one, &mut sequence).0, Ok(false));
        assert_eq!(owner.stage, PencilConstructionStage::ReserveColptr);
        for axis in 0..4 {
            let mut grant = one;
            match axis { 0 => grant.maximum_items = 0, 1 => grant.maximum_copy_bytes = 0, 2 => grant.maximum_capacity_bytes = 0, 3 => grant.maximum_depth = 0, _ => unreachable!() }
            let before = (owner.stage, owner.row, owner.entry, owner.colptr.allocated_bytes(), owner.second.as_ref().unwrap().vals.backing_ptr(0));
            let (_, receipt) = step(&mut owner, grant, &mut sequence);
            assert_eq!(receipt, RetainedCloneProgress::default());
            assert_eq!(before, (owner.stage, owner.row, owner.entry, owner.colptr.allocated_bytes(), owner.second.as_ref().unwrap().vals.backing_ptr(0)));
        }
        let mut complete = false;
        for _ in 0..20_000 { if step(&mut owner, one, &mut sequence).0.unwrap() { complete = true; break; } }
        assert!(complete, "{} paged pencil completes", case["id"]);
        assert!(owner.take_complete(RetainedCloneGrant { maximum_items: 0, ..one }).is_none());
        let pair = owner.complete.as_ref().unwrap();
        assert_eq!(pair.1.vals.backing_ptr(0), Some(original_pointer)); assert_eq!(pair.1.allocated_bytes(), original_bytes);
        assert!(owner.take_complete(RetainedCloneGrant { maximum_copy_bytes: (size_of::<CscSym>() + size_of::<Csr>()) * 2 - 1, ..one }).is_none());
        let ((mut first, mut second), receipt) = owner.take_complete(one).unwrap(); assert_eq!(receipt.copied_items, 1);
        assert_eq!(first.colptr.iter().copied().collect::<Vec<_>>(), u32s("kColptr"));
        assert_eq!(first.rowind.iter().copied().collect::<Vec<_>>(), u32s("kRows"));
        assert_eq!(first.vals.iter().copied().collect::<Vec<_>>(), f64s("kValues"));
        assert_eq!(second.vals.backing_ptr(0), Some(original_pointer));
        let allocated = first.allocated_bytes() + second.allocated_bytes() + owner.matrix.as_ref().unwrap().allocated_bytes();
        let mut released = 0;
        for _ in 0..20_000 { let turn = first.close_step(4096); released += turn.2; if turn.0 { break; } }
        for _ in 0..20_000 { let turn = second.close_step(4096); released += turn.2; if turn.0 { break; } }
        let mut closed = false;
        for _ in 0..20_000 { let turn = owner.close_step(4096); released += turn.2; if turn.0 { closed = true; break; } }
        assert!(closed && first.terminal_is_empty() && second.terminal_is_empty()); assert_eq!(allocated, released);
        println!("[DEBUG] {} original signed pencil order{n} preserves B backing and exact page conservation", case["id"]);
    }
}
