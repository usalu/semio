//! 🧪️ Actual details render preparation retains contextual source choices and small original request payloads.
use super::super::super::DetailsTestProfile;
use super::*;
use semio_framework_plugin::app_testing::OriginalWorkFixture;
fn source() -> IfcSnapshot {
    let typed = |text: &str| IfcValue::TypedValue(IfcTypedValue { name: "IFCLABEL".into(), items: vec![IfcValue::String(text.into())] });
    IfcSnapshot {
        entities: vec![
            IfcEntity { id: 1, name: "IFCPROPERTYENUMERATEDVALUE".into(), args: vec![IfcValue::String("Colour".into()), IfcValue::Unset, IfcValue::Aggregate(vec![typed("Red")]), IfcValue::Reference(2)], complex: vec![] },
            IfcEntity { id: 2, name: "IFCPROPERTYENUMERATION".into(), args: vec![IfcValue::String("Palette".into()), IfcValue::Aggregate(vec![typed("Red"), typed("Blue")]), IfcValue::Unset], complex: vec![] },
        ],
        ..Default::default()
    }
}
fn close(work: IfcDetailsRenderWork, caller: &mut OriginalWorkFixture, loan: RetainedCloneGrant) {
    let mut close = semio_framework_value::retained_clone::RetainedCloneClose::default();
    let mut owner = Some(work);
    while owner.is_some() || !close.is_empty() {
        let demand = close.next_owner_demands::<IfcDetailsRenderWork>(owner.is_some(), &None, 4096).unwrap();
        caller
            .turn(loan, |cx| {
                cx.with_original_retirement(|cx| semio_framework_job::close_original_retained_step(cx, demand, |grant| if close.is_empty() { close.begin_granted(&mut owner, grant).map(|step| step.unwrap()) } else { close.step_granted(grant) }))
            })
            .unwrap();
    }
    caller.finish();
}
#[test]
fn original_details_render_work_prepares_full_context_before_bounded_provider_choices() {
    let snapshot = source();
    let mut work = IfcDetailsRenderWork::default();
    let mut caller = OriginalWorkFixture::new();
    let loan = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 256 };
    for turn in 0..10000 {
        let demand = work.demands().unwrap();
        for axis in 0..6 {
            let mut denied = loan;
            let mut fuel = 1;
            match axis {
                0 => denied.maximum_items = 0,
                1 => denied.maximum_copy_bytes = demand.copy_bytes - 1,
                2 if demand.capacity_bytes > 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1,
                3 if demand.release_bytes > 0 => denied.maximum_release_bytes = demand.release_bytes - 1,
                4 => denied.maximum_depth = 0,
                5 => fuel = 0,
                _ => continue,
            }
            let state = work.state;
            let query = format!("{:?}", work.query);
            let shape = (work.entries.len(), work.entries.capacity());
            caller
                .work_turn(denied, fuel, |cx| {
                    let before = cx.retained_progress();
                    assert!(!work.advance::<DetailsTestProfile>(&snapshot, 71, cx)?);
                    assert_eq!(cx.retained_progress(), before);
                    Ok(())
                })
                .unwrap();
            assert_eq!(work.state, state);
            assert_eq!(format!("{:?}", work.query), query);
            assert_eq!((work.entries.len(), work.entries.capacity()), shape);
        }
        if caller.work_turn(loan, 1, |cx| work.advance::<DetailsTestProfile>(&snapshot, 71, cx)).unwrap() {
            break;
        }
        assert!(turn < 9999);
    }
    let provider = IfcDetailsProvider::<DetailsTestProfile>::new(&snapshot).with_contextual_enumerations(&work, 71).unwrap();
    let path = [P::Key("entities".into()), P::Index(0), P::Key("args".into()), P::Index(2), P::Key("value".into()), P::Index(0)];
    assert_eq!(provider.variant_count(&path, Locale::En), 2);
    let choice = provider.variant(&path, Locale::En, 1).unwrap();
    assert_eq!(choice.value, choice_request(2, 1, false));
    assert!(provider.variant(&path, Locale::De, 1).unwrap().label.len() < 128);
    assert!(IfcDetailsProvider::<DetailsTestProfile>::new(&snapshot).with_contextual_enumerations(&work, 72).is_err());
    assert!(IfcDetailsProvider::<DetailsTestProfile>::new(&snapshot.clone()).with_contextual_enumerations(&work, 71).is_err());
    fn source_choice(node: &semio_framework_ui_contract::BuiltNode) -> Option<&str> {
        for binding in node.bindings.iter() {
            if let Some(semio_framework_ui_contract::UiValue::Map(arguments)) = &binding.args {
                for (key, value) in arguments.iter() {
                    if key.as_str() == "value" {
                        if let semio_framework_ui_contract::UiValue::Text(value) = value {
                            if value.as_str().contains("$ifcEnumerationChoice") {
                                return Some(value.as_str());
                            }
                        }
                    }
                }
            }
        }
        node.children.iter().find_map(source_choice)
    }
    for locale in [Locale::En, Locale::De] {
        let tree = editing::render_snapshot_details_provider_revisioned(&provider, semio_framework_ui_contract::UiPublicationRevision(71), locale, "test.enum#editor", &semio_framework_plugin::TreeWindows::unhosted()).unwrap();
        let request = source_choice(&tree).expect("actual details tree must bind original enumeration choice");
        let parsed = semio_framework_pack_json::parse(request, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(semio_framework_pack_json::to_dsl_value(&parsed).get("$ifcEnumerationChoice").unwrap().get("definition").unwrap().as_u64(), Some(2));
    }
    drop(provider);
    close(work, &mut caller, loan);
}
#[test]
fn original_details_render_every_preparation_prefix_cancels_and_closes_actual_index_and_context() {
    let snapshot = source();
    let original = snapshot.clone();
    let loan = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 256 };
    let mut turns = 0;
    for cancel_at in std::iter::once(None).chain((1..10000).map(Some)) {
        if cancel_at.is_some_and(|turn| turn > turns) {
            break;
        }
        let mut caller = OriginalWorkFixture::new();
        let mut work = IfcDetailsRenderWork::default();
        for turn in 1..10000 {
            let before = work.state;
            if cancel_at == Some(turn) {
                caller.cancel().cancel_now();
            }
            let result = caller.work_turn(loan, 1, |cx| work.advance::<DetailsTestProfile>(&snapshot, 71, cx));
            assert_eq!(snapshot, original);
            if cancel_at == Some(turn) {
                assert!(result.is_err());
                assert_eq!(work.state, before);
                break;
            }
            if result.unwrap() {
                turns = turn;
                break;
            }
            assert!(turn < 9999);
        }
        caller.cancel().cancel_now();
        close(work, &mut caller, loan);
    }
    assert!(turns > 20);
}
