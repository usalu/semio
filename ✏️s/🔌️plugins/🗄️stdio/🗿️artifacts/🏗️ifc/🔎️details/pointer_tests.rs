//! 🧪️ Original bounded pointer admission preserves native owners across every retained frontier.
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::FromValue;
fn grant() -> semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_depth: 256, ..Default::default() }
}
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🧪️tests/pointer.json")).unwrap()
}
fn dsl(value: &serde_json::Value) -> DslValue {
    let parsed = semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    semio_framework_pack_json::to_dsl_value(&parsed)
}
fn close(caller: &mut OriginalWorkFixture, admission: &mut PathAdmission) {
    admission.begin_close();
    while !admission.terminal_is_empty() {
        caller
            .turn(grant(), |cx| {
                cx.with_original_retirement(|cx| {
                    semio_framework_job::close_original_retained_step(cx, admission.close_demands(), |grant| admission.close_step(grant))?;
                    Ok(())
                })
            })
            .unwrap();
    }
}
#[test]
fn original_pointer_neutral_utf8_escape_array_and_exact_identity_cases() {
    let fixture = fixture();
    let document = dsl(&fixture["document"]);
    for row in fixture["paths"].as_array().unwrap() {
        let path = row["path"].as_str().unwrap();
        let mode = match row["mode"].as_str().unwrap() {
            "insert" => Mode::Insert,
            "member" => Mode::Member,
            _ => Mode::Existing,
        };
        let mut pointer = Pointer::default();
        let mut caller = OriginalWorkFixture::new();
        let mut accepted = None;
        for turn in 0..100_000 {
            accepted = caller.work_turn(grant(), 1, |cx| pointer.advance(&document, path, mode, cx)).unwrap();
            if accepted.is_some() {
                break;
            }
            assert!(turn < 99_999);
        }
        assert_eq!(accepted, Some(row["accepted"].as_bool().unwrap()), "{path}");
        pointer.clear();
        assert!(pointer.terminal_is_empty());
        caller.finish();
    }
    assert_eq!(fixture["document"]["id"], "18446744073709551615");
}
#[test]
fn original_pointer_every_prefix_cancellation_and_actual_grant_axes_have_no_heap_or_custody_change() {
    let mut document = dsl(&fixture()["document"]);
    let key = "界".repeat(512);
    let path = format!("/{key}");
    let DslValue::Object(fields) = &mut document else { unreachable!() };
    fields.push((key, DslValue::Null));
    let identity = (&document as *const DslValue as usize, path.as_ptr() as usize, path.len());
    let mut turns = 0;
    for cancel_at in std::iter::once(None).chain((1..=10_000).map(Some)) {
        if cancel_at.is_some_and(|turn| turn > turns) {
            break;
        }
        let mut caller = OriginalWorkFixture::new();
        let mut pointer = Pointer::default();
        for turn in 1..10_000 {
            if cancel_at == Some(turn) {
                caller.cancel().cancel_now();
            }
            let result = caller.work_turn(grant(), 1, |cx| pointer.advance(&document, &path, Mode::Existing, cx));
            assert_eq!((&document as *const DslValue as usize, path.as_ptr() as usize, path.len()), identity);
            if cancel_at == Some(turn) {
                assert!(result.is_err());
                break;
            }
            if result.unwrap().is_some() {
                turns = turn;
                break;
            }
        }
        pointer.clear();
        assert!(pointer.terminal_is_empty());
        caller.finish();
    }
    for axis in ["items", "copyBytes", "capacityBytes", "releaseBytes", "depth", "fuel"] {
        let mut caller = OriginalWorkFixture::new();
        let mut pointer = Pointer::default();
        for turn in 0..10_000 {
            let demand = pointer.demands();
            let mut loan = grant();
            let mut fuel = 1;
            let short = match axis {
                "items" => {
                    loan.maximum_items = 0;
                    true
                }
                "copyBytes" => {
                    loan.maximum_copy_bytes = demand.copy_bytes.saturating_sub(1);
                    demand.copy_bytes > 0
                }
                "capacityBytes" => {
                    loan.maximum_capacity_bytes = 0;
                    demand.capacity_bytes > 0
                }
                "releaseBytes" => {
                    loan.maximum_release_bytes = 0;
                    demand.release_bytes > 0
                }
                "depth" => {
                    loan.maximum_depth = demand.depth.saturating_sub(1);
                    true
                }
                "fuel" => {
                    fuel = 0;
                    true
                }
                _ => unreachable!(),
            };
            let state = format!("{pointer:?}");
            let (result, before, after, heap) = caller
                .work_turn(loan, fuel, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pointer.advance(&document, &path, Mode::Existing, cx));
                    Ok((result?, before, cx.retained_progress(), heap))
                })
                .unwrap();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(!heap.overflowed);
            if short {
                assert!(result.is_none());
                assert_eq!(before, after);
                assert_eq!(format!("{pointer:?}"), state);
                assert_eq!(pointer.demands(), demand);
            }
            if result.is_some() || short && caller.work_turn(grant(), 1, |cx| pointer.advance(&document, &path, Mode::Existing, cx)).unwrap().is_some() {
                break;
            }
            assert!(turn < 9_999);
        }
        pointer.clear();
        caller.finish();
    }
}
#[test]
fn original_pointer_separator_scan_quotes_one_actual_range_and_denies_independent_short_byte_loan() {
    let fixture = fixture();
    let law = &fixture["scanGrantLaw"];
    let independent = (law["rangeWords"].as_u64().unwrap() + law["scalarWords"].as_u64().unwrap()) as usize * std::mem::size_of::<usize>() + law["booleanBytes"].as_u64().unwrap() as usize + law["phaseBytes"].as_u64().unwrap() as usize;
    assert_eq!(independent, 5 * std::mem::size_of::<usize>() + 2);
    for depth in [128, 129] {
        let path = "/".repeat(depth);
        let document = DslValue::Null;
        let mut pointer = Pointer::default();
        let mut caller = OriginalWorkFixture::new();
        assert!(caller.work_turn(grant(), 1, |cx| pointer.advance(&document, &path, Mode::Syntax, cx)).unwrap().is_none());
        for turn in 0..10_000 {
            assert_eq!(pointer.phase, Phase::Scan);
            assert_eq!(pointer.demands().copy_bytes, independent);
            let state = format!("{pointer:?}");
            let before_count = pointer.count;
            let denied = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_copy_bytes: independent - 1, ..grant() };
            let (before, after, heap) = caller
                .work_turn(denied, 1, |cx| {
                    let before = cx.retained_progress();
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pointer.advance(&document, &path, Mode::Syntax, cx));
                    assert!(result?.is_none());
                    Ok((before, cx.retained_progress(), heap))
                })
                .unwrap();
            assert_eq!(before, after);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(format!("{pointer:?}"), state);
            let loan = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_copy_bytes: independent, ..grant() };
            let (done, progress, heap) = caller
                .work_turn(loan, 1, |cx| {
                    let before = cx.retained_progress().copied_bytes;
                    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| pointer.advance(&document, &path, Mode::Syntax, cx));
                    Ok((result?, cx.retained_progress().copied_bytes - before, heap))
                })
                .unwrap();
            assert_eq!(progress, independent);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(pointer.count - before_count <= law["maximumPublishedRangesPerTurn"].as_u64().unwrap() as usize);
            if let Some(accepted) = done {
                assert_eq!(accepted, depth == 128);
                break;
            }
            assert!(turn < 9_999);
        }
        pointer.clear();
        caller.finish();
    }
}
#[test]
fn original_native_paths_keep_virtual_reference_and_omitted_constituent_addresses() {
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: u64::MAX, name: "IFCWALL".into(), args: vec![IfcValue::String("original".into()), IfcValue::Unset, IfcValue::Aggregate(Vec::new())], complex: Vec::new() }], ..Default::default() };
    for row in fixture()["ifc"].as_array().unwrap() {
        let event = editing::SnapshotEditEvent::from_value(dsl(&row["event"])).unwrap();
        let mut admission = PathAdmission::default();
        let mut caller = OriginalWorkFixture::new();
        let mut accepted = None;
        for turn in 0..10_000 {
            accepted = caller.work_turn(grant(), 1, |cx| admission.advance(&event, &snapshot, |_, _| None, cx)).unwrap();
            if accepted.is_some() {
                break;
            }
            assert!(turn < 9_999);
        }
        assert_eq!(accepted, Some(row["accepted"].as_bool().unwrap()));
        assert_eq!(snapshot.entities[0].id, u64::MAX);
        caller.cancel().cancel_now();
        close(&mut caller, &mut admission);
        caller.finish();
    }
}
#[test]
fn original_pointer_move_sources_semantic_hook_and_rebinding_preserve_controlled_pins() {
    let snapshot = IfcSnapshot { entities: vec![IfcEntity { id: u64::MAX, name: "IFCWALL".into(), args: vec![IfcValue::String("original".into())], complex: Vec::new() }], ..Default::default() };
    for row in fixture()["nativeMoves"].as_array().unwrap() {
        let event = editing::SnapshotEditEvent::MoveValue { from: row["from"].as_str().unwrap().into(), path: row["path"].as_str().unwrap().into() };
        let mut caller = OriginalWorkFixture::new();
        let mut admission = PathAdmission::default();
        let mut accepted = None;
        for turn in 0..10_000 {
            accepted = caller.work_turn(grant(), 1, |cx| admission.advance(&event, &snapshot, |_, _| None, cx)).unwrap();
            if accepted.is_some() {
                break;
            }
            assert!(turn < 9_999);
        }
        assert_eq!(accepted, Some(row["accepted"].as_bool().unwrap()));
        caller.cancel().cancel_now();
        close(&mut caller, &mut admission);
        caller.finish();
    }
    let event = editing::SnapshotEditEvent::SetValue { path: "/ifc-18446744073709551615/name".into(), value: DslValue::String("semantic".into()) };
    let mut caller = OriginalWorkFixture::new();
    let mut admission = PathAdmission::default();
    let mut accepted = None;
    let mut hooks = 0;
    for turn in 0..10_000 {
        accepted = caller
            .work_turn(grant(), 1, |cx| {
                admission.advance(
                    &event,
                    &snapshot,
                    |_, _| {
                        hooks += 1;
                        Some(true)
                    },
                    cx,
                )
            })
            .unwrap();
        if accepted.is_some() {
            break;
        }
        assert!(turn < 9_999);
    }
    assert_eq!(accepted, Some(true));
    assert_eq!(hooks, 1);
    let rebound = snapshot.clone();
    assert!(caller.work_turn(grant(), 1, |cx| admission.advance(&event, &rebound, |_, _| None, cx)).is_err());
    let copied = event.clone();
    assert!(caller.work_turn(grant(), 1, |cx| admission.advance(&copied, &snapshot, |_, _| None, cx)).is_err());
    caller.cancel().cancel_now();
    admission.begin_close();
    let original = (admission.event, admission.snapshot, admission.path.binding, admission.from.binding);
    let demand = admission.close_demands();
    let denied = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant() };
    caller
        .turn(denied, |cx| {
            cx.with_original_retirement(|cx| {
                let before = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| semio_framework_job::close_original_retained_step(cx, demand, |grant| admission.close_step(grant)));
                result?;
                assert_eq!(before, cx.retained_progress());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                Ok(())
            })
        })
        .unwrap();
    assert_eq!((admission.event, admission.snapshot, admission.path.binding, admission.from.binding), original);
    close(&mut caller, &mut admission);
    assert!(admission.terminal_is_empty());
    caller.finish();
}
