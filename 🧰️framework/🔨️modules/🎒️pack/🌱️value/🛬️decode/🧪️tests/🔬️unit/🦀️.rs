//! 🧷️ Retained value and record-body cursor laws that read cursor internals.
use super::*;
fn retained_value_cursor(limits: PackLimits) -> RetainedValueCursor {
    let mut cursor = RetainedValueCursor::try_new(limits, 1 << 20).expect("cursor");
    let requested = cursor.next_allocation_bytes().expect("allocation demand").expect("frame allocation");
    let step = cursor.reserve_allocation(requested).expect("frame allocation");
    assert!(step.progressed);
    assert_eq!(cursor.grant().expect("first root"), None);
    assert_eq!(cursor.grant().expect("second root"), None);
    cursor
}

fn close_retained_value(cursor: &mut RetainedValueCursor) {
    loop {
        let maximum_bytes = cursor.next_release_allocation_bytes().unwrap_or(0);
        if cursor.close_step(1, maximum_bytes).expect("retained close") == semio_framework_pack_format_retained::RetainedPackCloseStep::Complete {
            break;
        }
    }
}

fn retained_varint(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        bytes.push(if value == 0 { byte } else { byte | 0x80 });
        if value == 0 {
            return bytes;
        }
    }
}

fn drive_retained_record_pending(cursor: &mut RetainedRecordBodyCursor, allocated: &mut usize, tokens: &mut Vec<RetainedRecordBodyToken>) {
    for _ in 0..100_000 {
        while let Some(exact) = cursor.next_allocation_bytes().expect("record-body allocation demand") {
            let step = cursor.reserve_allocation(exact).expect("record-body allocation");
            assert!(step.progressed);
            *allocated += step.allocated_bytes;
        }
        if let Some(token) = cursor.grant().expect("record-body grant") {
            tokens.push(token);
        }
        if cursor.pending.is_none() && cursor.ingress_ready() {
            return;
        }
    }
    panic!("record-body pending input did not hand back");
}

fn admit_retained_record_bytes(cursor: &mut RetainedRecordBodyCursor, bytes: &[u8], allocated: &mut usize, tokens: &mut Vec<RetainedRecordBodyToken>) {
    for &byte in bytes {
        let offset = cursor.offset;
        cursor.admit_byte(offset, byte).expect("record-body byte admission");
        drive_retained_record_pending(cursor, allocated, tokens);
    }
}

fn finish_retained_record(cursor: &mut RetainedRecordBodyCursor, allocated: &mut usize, tokens: &mut Vec<RetainedRecordBodyToken>) {
    cursor.seal(cursor.offset).expect("record-body seal");
    for _ in 0..100_000 {
        while let Some(exact) = cursor.next_allocation_bytes().expect("record-body final allocation demand") {
            let step = cursor.reserve_allocation(exact).expect("record-body final allocation");
            assert!(step.progressed);
            *allocated += step.allocated_bytes;
        }
        if let Some(token) = cursor.grant().expect("record-body final grant") {
            let complete = matches!(token, RetainedRecordBodyToken::Value(RetainedValueToken::Complete { .. }));
            tokens.push(token);
            if complete {
                return;
            }
        }
    }
    panic!("record-body value did not complete");
}

fn close_retained_record(cursor: &mut RetainedRecordBodyCursor) -> usize {
    let mut released = 0;
    for _ in 0..100_000 {
        let maximum_bytes = cursor.next_release_allocation_bytes().expect("record-body release demand").unwrap_or(0);
        match cursor.close_step(1, maximum_bytes).expect("record-body close") {
            semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_bytes, .. } => released += released_bytes,
            semio_framework_pack_format_retained::RetainedPackCloseStep::Complete => return released,
        }
    }
    panic!("record-body owner did not close");
}

#[test]
fn retained_value_stack_refuses_subexact_allocation_and_closes_partial_roots_exactly() {
    let limits = PackLimits { max_file_len: 64, max_segment_len: 16, max_symbols: 1, max_depth: 8, max_items: 8, max_total_alloc: 64 };
    let requested = usize::from(limits.max_depth) * 8 * size_of::<Expect>();
    assert!(RetainedValueCursor::try_new(limits.clone(), requested - 1).is_err());
    for initialized_roots in 0..=2 {
        let mut cursor = RetainedValueCursor::try_new(limits.clone(), requested).expect("value stack credits");
        assert_eq!(cursor.progress(), RetainedValueProgress::default());
        assert_eq!(cursor.admit_byte(0, TAG_NULL), Err((0, TAG_NULL)));
        let exact = cursor.next_allocation_bytes().expect("stack demand").expect("stack allocation");
        assert_eq!(exact, requested);
        let before = cursor.progress();
        assert_eq!(cursor.reserve_allocation(0).expect("zero allocation refusal"), RetainedValueAllocationStep::default());
        assert_eq!(cursor.reserve_allocation(exact - 1).expect("subexact allocation refusal"), RetainedValueAllocationStep::default());
        assert_eq!(cursor.progress(), before);
        let step = cursor.reserve_allocation(exact).expect("exact stack allocation");
        assert!(step.progressed);
        assert_eq!(step.allocated_bytes, cursor.allocated_bytes());
        let pointer = cursor.stack.as_ptr();
        for _ in 0..initialized_roots {
            assert_eq!(cursor.grant().expect("root initialization"), None);
        }
        assert_eq!(cursor.progress().initialized_roots, initialized_roots);
        if initialized_roots == 2 {
            cursor.admit_byte(0, 0).expect("pending record count");
        } else {
            assert_eq!(cursor.admit_byte(0, 0), Err((0, 0)));
        }
        let before = cursor.progress();
        assert_eq!(cursor.close_step(0, 0).expect("zero close"), semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(cursor.progress(), before);
        while cursor.next_release_allocation_bytes().is_none() {
            assert!(matches!(cursor.close_step(1, 0).expect("logical stack close"), semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_bytes: 0, .. }));
        }
        let release = cursor.next_release_allocation_bytes().expect("stack release demand");
        assert_eq!(cursor.close_step(1, release - 1).expect("subexact stack release"), semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(cursor.stack.as_ptr(), pointer);
        assert_eq!(cursor.allocated_bytes(), step.allocated_bytes);
        assert_eq!(cursor.close_step(1, release).expect("exact stack release"), semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: release });
        assert_eq!(cursor.close_step(1, 0).expect("terminal stack close"), semio_framework_pack_format_retained::RetainedPackCloseStep::Complete);
        assert!(cursor.terminal_is_empty());
        assert_eq!(release, step.allocated_bytes);
    }
    eprintln!("retained-value partial-roots=0,1,2 subexact-preserved=true logical-frame-bound={} exact-release=true", usize::from(limits.max_depth) * 8);
}

#[test]
fn retained_record_body_zero_grants_preserve_unopened_cancellation() {
    let limits = PackLimits { max_file_len: 64, max_segment_len: 8, max_symbols: 1, max_depth: 1, max_items: 1, max_total_alloc: 64 };
    let mut cursor = RetainedRecordBodyCursor::try_new(limits, 1, 1, 1, 64 * 1024).expect("unopened record-body credits");
    cursor.admit_byte(0, 1).expect("pending symbol count");
    let state = |cursor: &RetainedRecordBodyCursor| {
        (
            (
                cursor.phase,
                cursor.pending,
                cursor.catalog_complete_pending,
                cursor.expected_symbols,
                cursor.symbol_utf8_bytes,
                cursor.offset,
                cursor.value_offset,
            ),
            (
                cursor.closing,
                cursor.close_phase,
                cursor.value.is_some(),
                cursor.symbols.len(),
                cursor.symbols.scalar_len(),
                cursor.allocated_bytes(),
            ),
        )
    };
    let before = state(&cursor);
    assert_eq!(
        cursor.close_step(0, 0).expect("unopened zero close"),
        semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 }
    );
    assert_eq!(state(&cursor), before);
    assert_eq!(
        cursor.close_step(1, 0).expect("pending byte close"),
        semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 }
    );
    for _ in 0..64 {
        if cursor.close_phase == RetainedRecordBodyClosePhase::ValuePhysical {
            break;
        }
        let before = state(&cursor);
        assert_eq!(
            cursor.close_step(0, 0).expect("intermediate zero close"),
            semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 }
        );
        assert_eq!(state(&cursor), before);
        assert!(matches!(
            cursor.close_step(1, 0).expect("unopened logical close"),
            semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_bytes: 0, .. }
        ));
    }
    assert_eq!(cursor.close_phase, RetainedRecordBodyClosePhase::ValuePhysical);
    let before = state(&cursor);
    assert_eq!(
        cursor.close_step(0, 0).expect("physical-phase zero close"),
        semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 }
    );
    assert_eq!(state(&cursor), before);
    assert_eq!(cursor.close_step(0, 1).expect("bytes-only metadata close"), semio_framework_pack_format_retained::RetainedPackCloseStep::Complete);
    assert!(cursor.terminal_is_empty());
}

#[test]
fn retained_record_body_indexes_multibyte_symbols_and_retires_all_logical_owners_before_backing() {
    let limits = PackLimits { max_file_len: 256, max_segment_len: 64, max_symbols: 3, max_depth: 8, max_items: 16, max_total_alloc: 256 };
    let mut cursor = RetainedRecordBodyCursor::try_new(limits, 3, 12, 7, 256 * 1024).expect("record-body credits");
    assert_eq!(cursor.allocated_bytes(), 0);
    cursor.admit_byte(0, 3).expect("pending symbol count");
    let exact = cursor.next_allocation_bytes().expect("span demand").expect("span allocation");
    let before = (cursor.offset, cursor.pending, cursor.allocated_bytes());
    assert_eq!(cursor.reserve_allocation(exact - 1).expect("subexact span refusal"), RetainedRecordBodyAllocationStep::default());
    assert_eq!((cursor.offset, cursor.pending, cursor.allocated_bytes()), before);
    let mut allocated = 0;
    let mut tokens = Vec::new();
    drive_retained_record_pending(&mut cursor, &mut allocated, &mut tokens);
    admit_retained_record_bytes(&mut cursor, &[0, 4, b'a', b'x', b'i', b's', 8, b'A', 0xe2, 0x82, 0xac, 0xf0, 0x90, 0x8d, 0x88, 0], &mut allocated, &mut tokens);
    finish_retained_record(&mut cursor, &mut allocated, &mut tokens);
    for (symbol, expected) in ["", "axis", "A€𐍈"].into_iter().enumerate() {
        let actual: String = (0..cursor.symbol_chars(symbol as u64).expect("symbol scalar count"))
            .map(|index| cursor.symbol_char(symbol as u64, index).expect("symbol lookup").expect("symbol scalar"))
            .collect();
        assert_eq!(actual, expected);
    }
    assert!(cursor.symbol_chars(u64::MAX).is_err());
    let value_allocation = cursor.value.as_ref().expect("retained value owner").allocated_bytes();
    assert!(value_allocation > 0 && cursor.symbols.allocated_bytes() > 0);
    let total_allocation = cursor.allocated_bytes();
    assert_eq!(total_allocation, allocated);
    let mut released_before_physical = 0;
    while cursor.close_phase != RetainedRecordBodyClosePhase::ValuePhysical {
        match cursor.close_step(1, usize::MAX).expect("logical record-body close") {
            semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_bytes, .. } => released_before_physical += released_bytes,
            semio_framework_pack_format_retained::RetainedPackCloseStep::Complete => panic!("record body closed before physical phase"),
        }
    }
    assert_eq!(released_before_physical, 0);
    assert_eq!((cursor.symbols.len(), cursor.symbols.scalar_len()), (0, 0));
    let exact = cursor.next_release_allocation_bytes().expect("value release query").expect("value release demand");
    assert_eq!(exact, value_allocation);
    assert_eq!(cursor.close_step(0, exact - 1).expect("subexact value release"), semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(cursor.next_release_allocation_bytes().expect("preserved value release demand"), Some(exact));
    let mut released = match cursor.close_step(0, exact).expect("exact value release") {
        semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes } => released_bytes,
        step => panic!("unexpected value release {step:?}"),
    };
    released += close_retained_record(&mut cursor);
    assert_eq!(released, total_allocation);
    assert!(cursor.terminal_is_empty());
    assert!(tokens.iter().any(|token| matches!(token, RetainedRecordBodyToken::CatalogComplete)));
}

#[test]
fn retained_record_body_crosses_a_symbol_leaf_and_preserves_maximum_plus_one_pending_fault() {
    let count = 512usize;
    let limits = PackLimits { max_file_len: 4_096, max_segment_len: 16, max_symbols: count as u32, max_depth: 8, max_items: 16, max_total_alloc: 4_096 };
    let mut cursor = RetainedRecordBodyCursor::try_new(limits, count, 0, 0, 512 * 1024).expect("multi-leaf record-body credits");
    let count_bytes = retained_varint(count as u64);
    let mut allocated = 0;
    let mut tokens = Vec::new();
    admit_retained_record_bytes(&mut cursor, &count_bytes[..count_bytes.len() - 1], &mut allocated, &mut tokens);
    cursor.admit_byte(cursor.offset, *count_bytes.last().expect("count byte")).expect("final count byte");
    let mut first_leaf_capacity = None;
    while let Some(exact) = cursor.next_allocation_bytes().expect("multi-leaf allocation demand") {
        let step = cursor.reserve_allocation(exact).expect("multi-leaf allocation");
        assert!(step.progressed);
        allocated += step.allocated_bytes;
        if cursor.symbols.symbol_capacity() != 0 {
            first_leaf_capacity.get_or_insert(cursor.symbols.symbol_capacity());
        }
    }
    assert!(first_leaf_capacity.is_some_and(|capacity| count > capacity));
    drive_retained_record_pending(&mut cursor, &mut allocated, &mut tokens);
    for _ in 0..count {
        admit_retained_record_bytes(&mut cursor, &[0], &mut allocated, &mut tokens);
    }
    admit_retained_record_bytes(&mut cursor, &[0], &mut allocated, &mut tokens);
    finish_retained_record(&mut cursor, &mut allocated, &mut tokens);
    assert_eq!(cursor.symbols.len(), count);
    assert_eq!(close_retained_record(&mut cursor), allocated);
    assert!(cursor.terminal_is_empty());

    let limits = PackLimits { max_file_len: 16, max_segment_len: 8, max_symbols: 3, max_depth: 8, max_items: 8, max_total_alloc: 16 };
    let mut excessive = RetainedRecordBodyCursor::try_new(limits, 3, 0, 0, 64 * 1024).expect("bounded symbol credits");
    excessive.admit_byte(0, 4).expect("maximum plus one pending count");
    let pending = excessive.pending;
    let first = excessive.next_allocation_bytes().expect_err("maximum plus one symbol count");
    assert_eq!(excessive.pending, pending);
    assert_eq!(excessive.grant().expect_err("sticky symbol count fault"), first);
    assert_eq!(excessive.close_step(0, usize::MAX).expect("bytes-only cancellation refusal"), semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert_eq!(excessive.pending, pending);
    assert_eq!(close_retained_record(&mut excessive), 0);
    assert!(excessive.terminal_is_empty());
}

#[test]
fn retained_value_vm_covers_every_wire_tag_and_terminal_empty_close() {
    let mut bytes = vec![23];
    let values: Vec<Vec<u8>> = vec![
        vec![TAG_ABSENT],
        vec![TAG_FALSE],
        vec![TAG_TRUE],
        vec![TAG_INT, 2],
        vec![TAG_UINT, 1],
        [vec![TAG_F64], 1f64.to_le_bytes().to_vec()].concat(),
        vec![TAG_STR, 0],
        vec![TAG_STR_INLINE, 1, b'a'],
        vec![TAG_BYTES, 1, 7],
        vec![TAG_BYTES_CHUNKED, 1, 0],
        vec![TAG_ENUM, 1],
        vec![TAG_TUPLE, 0],
        vec![TAG_LIST, 0],
        vec![TAG_RECORD, 0],
        vec![TAG_BLOCK, TAG_TRUE],
        vec![TAG_STATEMENTS, 0],
        vec![TAG_MAP, 0],
        vec![TAG_VALUE, TAG_NULL],
        vec![TAG_WIRE, 0, 0, TAG_STR_INLINE, 1, b'n', TAG_NULL],
        vec![TAG_TABLE_SOA, 0, 0],
        vec![TAG_PACKED_F64, 0],
        vec![TAG_PACKED_VARINT, 0],
        vec![TAG_EXPR, TAG_STR_INLINE, 1, b'x'],
    ];
    for (field, value) in values.iter().enumerate() {
        bytes.push(field as u8);
        bytes.extend_from_slice(value);
    }
    let limits = PackLimits { max_file_len: 4096, max_segment_len: 4096, max_symbols: 8, max_depth: 32, max_items: 64, max_total_alloc: 4096 };
    let mut cursor = retained_value_cursor(limits);
    let mut tags = Vec::new();
    for (offset, byte) in bytes.iter().copied().enumerate() {
        cursor.admit_byte(offset as u64, byte).expect("admission");
        while cursor.pending.is_some() {
            match cursor.grant().expect("grant") {
                Some(RetainedValueToken::Tag { value, .. }) => tags.push(value),
                Some(RetainedValueToken::Begin { kind: RetainedValueContainer::Wire, .. }) => tags.push(TAG_WIRE),
                _ => {}
            }
        }
    }
    cursor.seal(bytes.len() as u64).expect("seal");
    loop {
        if matches!(cursor.grant().expect("finish"), Some(RetainedValueToken::Complete { .. })) {
            break;
        }
    }
    for tag in 0u8..=0x17 {
        assert!(tags.contains(&tag), "missing retained tag {tag:#04x}");
    }
    close_retained_value(&mut cursor);
    assert!(cursor.terminal_is_empty());
}

#[test]
fn retained_value_vm_rejects_truncation_utf8_depth_and_counts() {
    let limits = PackLimits { max_file_len: 64, max_segment_len: 8, max_symbols: 1, max_depth: 1, max_items: 1, max_total_alloc: 64 };
    let mut truncated = retained_value_cursor(limits.clone());
    truncated.admit_byte(0, 1).expect("count");
    truncated.grant().expect("count grant");
    truncated.seal(1).expect("seal");
    let mut truncation = None;
    for _ in 0..=usize::from(limits.max_depth) * 8 {
        match truncated.grant() {
            Err(fault) => {
                truncation = Some(fault);
                break;
            }
            Ok(_) => {}
        }
    }
    assert_eq!(truncation, Some(PackRefusal::Truncated(1)));
    close_retained_value(&mut truncated);
    assert!(truncated.terminal_is_empty());

    let mut count = retained_value_cursor(limits.clone());
    count.admit_byte(0, 2).expect("count byte");
    assert!(matches!(count.grant(), Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, .. })));
    close_retained_value(&mut count);

    let mut utf8 = retained_value_cursor(limits);
    let utf8_allocation = utf8.allocated_bytes();
    let mut invalid_utf8 = None;
    for (offset, byte) in [1, 0, TAG_STR_INLINE, 1, 0xff].into_iter().enumerate() {
        utf8.admit_byte(offset as u64, byte).expect("utf8 admission");
        while utf8.pending.is_some() {
            if let Err(fault) = utf8.grant() {
                invalid_utf8 = Some(fault);
                break;
            }
        }
    }
    let expected_utf8 = PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset: 4, detail: "invalid leading byte" };
    assert_eq!(invalid_utf8, Some(expected_utf8.clone()));
    assert_eq!(utf8.grant(), Err(expected_utf8));
    assert_eq!(utf8.allocated_bytes(), utf8_allocation);
    close_retained_value(&mut utf8);

    let depth_limits = PackLimits { max_file_len: 64, max_segment_len: 8, max_symbols: 1, max_depth: 1, max_items: 4, max_total_alloc: 64 };
    let mut depth = retained_value_cursor(depth_limits);
    let mut depth_failed = false;
    for (offset, byte) in [1, 0, TAG_BLOCK, TAG_BLOCK, TAG_TRUE].into_iter().enumerate() {
        depth.admit_byte(offset as u64, byte).expect("depth admission");
        while depth.pending.is_some() {
            if matches!(depth.grant(), Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::DepthLimit, .. })) {
                depth_failed = true;
                break;
            }
        }
        if depth_failed {
            break;
        }
    }
    assert!(depth_failed);
    close_retained_value(&mut depth);
}

#[test]
fn retained_record_body_fault_is_inline_sticky_and_closes_exactly() {
    let limits = PackLimits { max_file_len: 64, max_segment_len: 8, max_symbols: 1, max_depth: 1, max_items: 1, max_total_alloc: 64 };
    let mut cursor = RetainedRecordBodyCursor::try_new(limits, 1, 1, 1, 64 * 1024).expect("diagnostic record-body credits");
    let mut allocated = 0;
    let mut tokens = Vec::new();
    admit_retained_record_bytes(&mut cursor, &[1, 1], &mut allocated, &mut tokens);
    cursor.admit_byte(cursor.offset, 0xff).expect("invalid UTF-8 pending byte");
    let before = (cursor.offset, cursor.pending, cursor.allocated_bytes());
    let expected = PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset: 2, detail: "invalid leading byte" };
    for _ in 0..2 {
        let (quote, requested, released) = crate::test_allocation::observe_backing(|| cursor.source_step_demands());
        assert_eq!(quote, Err(expected.clone()));
        assert_eq!((requested, released), (0, 0));
        assert!(cursor.fault.is_none());
        assert_eq!((cursor.offset, cursor.pending, cursor.allocated_bytes()), before);
    }
    assert_eq!(cursor.next_allocation_bytes(), Err(expected.clone()));
    assert_eq!(cursor.fault.as_ref(), Some(&expected));
    assert_eq!((cursor.offset, cursor.pending, cursor.allocated_bytes()), before);
    assert_eq!(cursor.grant(), Err(expected));
    assert_eq!((cursor.offset, cursor.pending, cursor.allocated_bytes()), before);
    assert_eq!(close_retained_record(&mut cursor), allocated);
    assert!(cursor.terminal_is_empty());
}