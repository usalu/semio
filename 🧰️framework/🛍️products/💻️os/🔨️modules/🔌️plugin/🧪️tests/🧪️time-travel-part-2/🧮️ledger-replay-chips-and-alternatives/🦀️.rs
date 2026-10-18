/// 🧮️ Ledger replay, chips and alternatives: the blocking ledger replay, numeric snap sources, reference chips, selection values and the overwrite and alternative branch supersedes.

/// ⚖️ LAW: a ledger whose check-in replay blocks under `MergePolicy::Normal` crosses the guest codec boundary as the
/// typed `history.ledger-not-replayable` naming only its blocking messages; any other refusal keeps its own fault.
#[test]
fn a_blocking_ledger_replay_crosses_the_guest_boundary_as_ledger_not_replayable() {
    let fault = replay_envelopes_fault(store::VcsError::Rejected {
        policy: protocol::MergePolicy::Normal,
        messages: vec![protocol::MutationMessage::error("mutation.target-missing", "gone"), protocol::MutationMessage::warning("mutation.clamped", "clamped")],
    });
    assert_eq!(fault.code, FaultCode::new("history.ledger-not-replayable"));
    assert!(fault.message.contains("mutation.target-missing") && !fault.message.contains("mutation.clamped"), "{}", fault.message);
    assert_ne!(replay_envelopes_fault(store::VcsError::Deserialize("malformed".into())).code, FaultCode::new("history.ledger-not-replayable"));
}

/// 🧲️ A `Config`/`Snapshot` snap source becomes numbers on the wire: the grid multiples inside the travel range when at
/// most one fixed list of them fits, else the grid spacing as the step; a colour input takes `#rrggbb` text.
#[test]
fn snap_sources_resolve_to_numbers_and_colours_take_hex_text() {
    let grid = |min: f64, max: f64| semio_framework::ArgSchema::Number {
        min: Some(min),
        min_exclusive: false,
        max: Some(max),
        max_exclusive: false,
        step: None,
        integer: false,
        unit: None,
        snaps: Vec::new(),
        snap_source: Some(semio_framework::SnapSource::Config { key: "gridFactor".into() }),
        soft_min: None,
        soft_max: None,
        precision: None,
        display_unit: None,
        display_factor: None,
        scale: None,
    };
    let mut fine = grid(-1.0, 1.0);
    time_travel::time_travel_apply_snap_spacing(&mut fine, Some(0.5));
    assert!(matches!(&fine, semio_framework::ArgSchema::Number { snaps, snap_source: None, step: None, .. } if *snaps == vec![-1.0, -0.5, 0.0, 0.5, 1.0]), "{fine:?}");
    let mut wide = grid(0.0, 1_000.0);
    time_travel::time_travel_apply_snap_spacing(&mut wide, Some(0.5));
    assert!(matches!(&wide, semio_framework::ArgSchema::Number { snaps, snap_source: None, step: Some(step), .. } if snaps.is_empty() && *step == 0.5), "{wide:?}");
    let mut unresolved = grid(0.0, 1.0);
    time_travel::time_travel_apply_snap_spacing(&mut unresolved, None);
    assert!(matches!(&unresolved, semio_framework::ArgSchema::Number { snap_source: None, step: None, .. }), "an unresolvable source never reaches a host");
    assert_eq!(time_travel::time_travel_color_components("#ff8000").map(|components| components.len()), Some(3));
    assert_eq!(time_travel::time_travel_color_components("#ff800040").map(|components| components.len()), Some(4), "a text with alpha sets the opacity too");
    assert_eq!(time_travel::time_travel_color_components("#f80").map(|components| components.len()), Some(3), "the contract's short form parses");
    let colour = semio_framework::ActionArgDef { presentation: Some(semio_framework::ArgPresentation::Color), ..semio_framework::ActionArgDef::vector("/tint", LocalizedLabel::native("Tint", "Tönung"), 4) };
    assert!(time_travel::time_travel_coerce(&colour, false, &DslValue::String("#00ff00".into())).is_some_and(|value| value.as_array().is_some_and(|components| components.len() == 3)), "an rgb pick keeps the alpha apart");
    assert_eq!(time_travel::time_travel_coerce(&colour, false, &DslValue::String("green".into())), None);
}

/// 🪧️ LAW (gap N3): a reference chip no app hook names reads the framework's generic default — the entity's own name where
/// the document gives one (a label, name, title or text; data or an `{en, de}` pair; a blank one names nothing), else its
/// kind word from the framework input-label glossary (or the kind spelled out) and a short id, in every locale; in a German
/// shell the toy's anonymous child reads "Kindelement anon-1".
#[semio_framework_async_macros::async_test]
async fn reference_chips_fall_back_to_the_document_name_then_the_kind_and_a_short_id() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let both = |label: &LocalizedLabel| (label.resolve(Terminology::Native, Locale::En).to_string(), label.resolve(Terminology::Native, Locale::De).to_string());
    let pair = |en: &str, de: &str| (en.to_string(), de.to_string());
    let document = dsl(&serde_json::json!({ "nodes": [{ "id": "a", "label": "Alpha" }, { "id": "b", "name": { "en": "Bee", "de": "Biene" } }, { "id": "c", "text": "  " }], "edges": [{ "id": "c", "title": "Gamma" }] }));
    let names = time_travel::time_travel_entity_names(&document, &["a", "b", "c", "z"].into_iter().collect());
    assert_eq!(names.iter().map(|(id, label)| (id.as_str(), both(label))).collect::<Vec<_>>(), vec![("a", pair("Alpha", "Alpha")), ("b", pair("Bee", "Biene")), ("c", pair("Gamma", "Gamma"))], "z is not in the document");
    let fallback = |kinds: &[&str], id: &str| both(&time_travel::time_travel_reference_fallback_label(&kinds.iter().map(|kind| kind.to_string()).collect::<Vec<_>>(), id));
    assert_eq!(fallback(&["s.test.child"], "anon-1!s.test.child@native/*"), pair("Child anon-1", "Kindelement anon-1"));
    assert_eq!(fallback(&["node", "edge"], "n1"), pair("Node n1", "Knoten n1"), "the first declared kind names it");
    assert_eq!(fallback(&["vortex.targetRegion"], "0123456789abcdefXYZ"), pair("Target region 0123456789ab\u{2026}", "Target region 0123456789ab\u{2026}"), "an unknown kind is spelled out, a long id cut");
    assert_eq!(fallback(&[], "x"), pair("x", "x"));
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    for step in [serde_json::json!({ "begin": 2 }), serde_json::json!({ "input": { "path": "/children", "value": ["anon-1!s.test.child@native/*", "child-2!s.test.child@native/*"] } })] {
        run_step(&mut app, &fixture, &step, mounted_policy, &mut identity,cx).await;
    }
    let history = render_history(&mut app, Locale::De).await;
    let chip = |index: usize| find_node(&history, &format!("framework.history.editor.input.children.chip.{index}.row")).and_then(|chip| chip["component"]["label"].as_str()).unwrap_or_else(|| panic!("chip {index}: {history}")).to_string();
    assert_eq!((chip(0), chip(1)), ("Kindelement anon-1".to_string(), "Kind child-2".to_string()), "the generic default where the app names nothing, the app's own label elsewhere");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🎯️ "Use selection" reads the reference input's declared domain at its granularity: every selected id up to
/// `maxItems` for a many-reference, the first for a single one, typed by the input's id type (an id the type cannot
/// spell is skipped); an input without a domain is no selection target, and a missing, empty or other-granularity
/// selection selects nothing.
#[test]
fn selection_values_follow_the_reference_domain_granularity_count_and_id_type() {
    use semio_framework::{ArgSchema, ReferenceIdType};
    use time_travel::{time_travel_selection_value, TimeTravelActionRefusal};
    let reference = |domain: Option<&str>, many: bool, max_items: Option<u32>, id_type: ReferenceIdType| semio_framework::ActionArgDef {
        schema: ArgSchema::Reference { kinds: vec!["node".into()], domain: domain.map(str::to_string), granularity: Some("node".into()), many, min_items: None, max_items, id_type },
        ..semio_framework::ActionArgDef::number("/targets", LocalizedLabel::native("Targets", "Ziele"))
    };
    let selection = |granularity: &str, ids: &[&str]| protocol::DomainSelection { granularity: granularity.into(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: None };
    let text = |id: &str| DslValue::String(id.into());
    let many = reference(Some("board"), true, Some(2), ReferenceIdType::String);
    assert_eq!(time_travel_selection_value(&many, Some(&selection("node", &["a", "b", "c"]))), Ok(DslValue::Array(vec![text("a"), text("b")])), "a many-reference takes the selection up to maxItems");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), true, None, ReferenceIdType::String), Some(&selection("node", &["a", "b", "c"]))), Ok(DslValue::Array(vec![text("a"), text("b"), text("c")])), "without maxItems every selected id");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), false, None, ReferenceIdType::String), Some(&selection("node", &["a", "b"]))), Ok(text("a")), "a single reference takes the first");
    assert_eq!(time_travel_selection_value(&reference(None, true, None, ReferenceIdType::String), Some(&selection("node", &["a"]))), Err(TimeTravelActionRefusal::UnknownInput), "an input without a domain is no selection target");
    assert_eq!(time_travel_selection_value(&semio_framework::ActionArgDef::number("/dx", LocalizedLabel::native("dx", "dx")), Some(&selection("node", &["a"]))), Err(TimeTravelActionRefusal::UnknownInput), "a number is no reference");
    assert_eq!(time_travel_selection_value(&many, None), Err(TimeTravelActionRefusal::NoSelection), "the domain selects nothing");
    assert_eq!(time_travel_selection_value(&many, Some(&selection("region", &["a"]))), Err(TimeTravelActionRefusal::NoSelection), "a selection at another granularity");
    assert_eq!(time_travel_selection_value(&many, Some(&selection("node", &[]))), Err(TimeTravelActionRefusal::NoSelection), "an empty selection");
    let integers = reference(Some("board"), true, None, ReferenceIdType::Integer);
    assert_eq!(time_travel_selection_value(&integers, Some(&selection("node", &["7", "x", "-3"]))), Ok(DslValue::Array(vec![DslValue::uint(7), DslValue::int(-3)])), "integer ids stage the integers they spell");
    assert_eq!(time_travel_selection_value(&integers, Some(&selection("node", &["x"]))), Err(TimeTravelActionRefusal::NoSelection), "no id the type can spell");
    assert_eq!(time_travel_selection_value(&reference(Some("board"), false, None, ReferenceIdType::Integer), Some(&selection("node", &["x", "12"]))), Ok(DslValue::uint(12)), "a single integer reference skips text it cannot spell");
}

/// 🌿️ The history transitions `app` gained since it held `before` of them, decoded.
fn added_transitions(app: &ToyApp, before: usize) -> Vec<store::os_spr::HistoryTransition> {
    app.stores.store.envelope().transitions[before..].iter().map(|envelope| store::os_spr::history_transition_from_envelope(envelope).expect("a transition decodes").expect("a history transition")).collect()
}

/// ⚖️ LAW (R15): from a clean review the user begins another mutation and accepts it, so the session holds both drafts;
/// one finalize commits exactly ONE unscoped `Supersede` naming both, and the head equals a fresh fold of the edited log.
#[semio_framework_async_macros::async_test]
async fn several_drafts_from_a_review_finalize_as_one_overwrite_supersede() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let before = app.stores.store.envelope().transitions.len();
    commit_scenario(&mut app, &fixture, "several-drafts-accumulated-from-a-review-finalize-once", mounted_policy, &mut identity).await;
    let added = added_transitions(&app, before);
    let [store::os_spr::HistoryTransition::Supersede(supersede)] = added.as_slice() else { panic!("an overwrite is one Supersede: {added:?}") };
    let targets: BTreeSet<String> = supersede.inputs.iter().map(|input| input.target.0.clone()).collect();
    assert_eq!((supersede.scope.as_deref(), targets), (None, [0, 1].map(|index| seeded_mutation(&app, index)).into_iter().collect()), "one unscoped supersede names both drafts");
    let mut fresh = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest(), app.stores.store.local_actor_id().clone(), mounted_policy, &mut identity).await;
    for (index, op) in fixture["seed"].as_array().expect("seed").iter().enumerate() {
        let op = match index {
            0 => SetCount { value: 7 }.into(),
            1 => SetLabel { value: "b".into() }.into(),
            _ => seed_op(op),
        };
        fresh.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![op], transaction: None }, &mut identity).await.expect("fresh edit");
    }
    assert_eq!(app.stores.store.snapshot().expect("edited head"), fresh.stores.store.snapshot().expect("fresh head"));
    close(&mut app,cx);
    close(&mut fresh,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW (design §2, §4): a new alternative is one `Branch` followed by one `Supersede` scoped to that alternative
/// (after the checkpoint of any pending edit), and the new alternative is current.
#[semio_framework_async_macros::async_test]
async fn a_new_alternative_is_one_branch_then_one_scoped_supersede() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    let before = app.stores.store.envelope().transitions.len();
    commit_scenario(&mut app, &fixture, "a-new-alternative-keeps-the-original", mounted_policy, &mut identity).await;
    let added = added_transitions(&app, before);
    let alternative = app.stores.store.envelope().active_alternative_id.clone().expect("the new alternative is current");
    let (checkpoints, tail) = added.split_at(added.len().saturating_sub(2));
    assert!(checkpoints.iter().all(|transition| matches!(transition, store::os_spr::HistoryTransition::Commit(..))), "only a checkpoint of pending edits precedes the branch: {added:?}");
    assert!(
        matches!(tail, [store::os_spr::HistoryTransition::Branch { .. }, store::os_spr::HistoryTransition::Supersede(supersede)] if supersede.scope.as_deref() == Some(alternative.as_str())),
        "Branch, then a Supersede scoped to the new alternative: {added:?}"
    );
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
