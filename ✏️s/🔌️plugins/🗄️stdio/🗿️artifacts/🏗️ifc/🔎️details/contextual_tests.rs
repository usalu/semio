//! 🧪️ Original enumeration metadata borrows full typed choices under retained authority and retirement.
use super::super::super::DetailsTestProfile;
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_depth: 256, ..Default::default() }
}
fn source() -> IfcSnapshot {
    IfcSnapshot {
        entities: vec![
            IfcEntity {
                id: 1,
                name: "IFCPROPERTYENUMERATEDVALUE".into(),
                args: vec![IfcValue::String("Colour".into()), IfcValue::Unset, IfcValue::Aggregate(vec![IfcValue::TypedValue(IfcTypedValue { name: "IFCLABEL".into(), items: vec![IfcValue::String("Red".into())] })]), IfcValue::Reference(u64::MAX)],
                complex: vec![],
            },
            IfcEntity {
                id: u64::MAX,
                name: "IFCPROPERTYENUMERATION".into(),
                args: vec![
                    IfcValue::String("Palette".into()),
                    IfcValue::Aggregate(vec![
                        IfcValue::TypedValue(IfcTypedValue { name: "IFCLABEL".into(), items: vec![IfcValue::String("Red".into())] }),
                        IfcValue::TypedValue(IfcTypedValue { name: "IFCINTEGER".into(), items: vec![IfcValue::Integer(i64::MAX)] }),
                    ]),
                    IfcValue::Unset,
                ],
                complex: vec![],
            },
        ],
        ..Default::default()
    }
}
fn close(query: &mut IfcEnumerationQuery, caller: &mut OriginalWorkFixture) {
    query.begin_close();
    while !query.terminal_is_empty() {
        caller.turn(grant(), |cx| cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, query.close_demands(), |loan| query.close_step(loan)))).unwrap();
    }
    caller.finish();
}
#[test]
fn contextual_enumeration_original_funding_pins_full_values_and_close() {
    let source = source();
    let path = [P::Key("entities".into()), P::Index(0), P::Key("args".into()), P::Index(2), P::Key("value".into()), P::Index(0)];
    let mut caller = OriginalWorkFixture::new();
    let mut query = IfcEnumerationQuery::default();
    for turn in 0..10000 {
        let demand = query.demands();
        for axis in 0..4 {
            let mut loan = grant();
            let mut fuel = 1;
            match axis {
                0 => loan.maximum_items = 0,
                1 => loan.maximum_copy_bytes = demand.copy_bytes - 1,
                2 => loan.maximum_depth = 0,
                _ => fuel = 0,
            }
            let before = query.state;
            caller
                .work_turn(loan, fuel, |cx| {
                    let receipt = cx.retained_progress();
                    assert!(!query.advance::<DetailsTestProfile>(&source, &path, 71, cx)?);
                    assert_eq!(cx.retained_progress(), receipt);
                    Ok(())
                })
                .unwrap();
            assert_eq!(query.state, before);
        }
        let done = caller
            .work_turn(grant(), 1, |cx| {
                let before = cx.retained_progress();
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| query.advance::<DetailsTestProfile>(&source, &path, 71, cx));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                let _ = before;
                result
            })
            .unwrap();
        if done {
            break;
        }
        assert!(turn < 9999);
    }
    let view = query.result(&source, 71).unwrap();
    assert_eq!(view.enumeration, Some(u64::MAX));
    let choices = view.allowed.unwrap();
    assert_eq!(choices.len(), 2);
    assert_eq!(choices.as_ptr(), source.entities[1].args[1].as_aggregate().unwrap().as_ptr());
    assert_eq!(choices[1], IfcValue::TypedValue(IfcTypedValue { name: "IFCINTEGER".into(), items: vec![IfcValue::Integer(i64::MAX)] }));
    assert!(query.result(&source, 72).is_err());
    assert!(query.result(&source.clone(), 71).is_err());
    caller.cancel().cancel_now();
    close(&mut query, &mut caller);
}
#[test]
fn contextual_enumeration_cancels_every_original_prefix_and_distinguishes_unset_empty() {
    let source = source();
    let path = [P::Key("entities".into()), P::Index(0), P::Key("args".into()), P::Index(2)];
    let mut turns = 0;
    for cancel_at in std::iter::once(None).chain((1..10000).map(Some)) {
        if cancel_at.is_some_and(|at| at > turns) {
            break;
        }
        let mut caller = OriginalWorkFixture::new();
        let mut query = IfcEnumerationQuery::default();
        for turn in 1..10000 {
            if cancel_at == Some(turn) {
                caller.cancel().cancel_now();
            }
            let before = query.state;
            let result = caller.work_turn(grant(), 1, |cx| query.advance::<DetailsTestProfile>(&source, &path, 71, cx));
            if cancel_at == Some(turn) {
                assert!(result.is_err());
                assert_eq!(query.state, before);
                break;
            }
            if result.unwrap() {
                turns = turn;
                break;
            }
            assert!(turn < 9999);
        }
        caller.cancel().cancel_now();
        close(&mut query, &mut caller);
    }
    assert!(turns > 4);
    for unset in [false, true] {
        let mut source = source.clone();
        source.entities[0].args[2] = if unset { IfcValue::Unset } else { IfcValue::Aggregate(vec![]) };
        let mut caller = OriginalWorkFixture::new();
        let mut query = IfcEnumerationQuery::default();
        for turn in 0..10000 {
            if caller.work_turn(grant(), 1, |cx| query.advance::<DetailsTestProfile>(&source, &path, 71, cx)).unwrap() {
                break;
            }
            assert!(turn < 9999);
        }
        assert_eq!(matches!(query.result(&source, 71).unwrap().selection, IfcValue::Unset), unset);
        close(&mut query, &mut caller);
    }
}
