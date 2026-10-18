/// 🔢️ Input shapes, lists, unions and sources: input pointers and host values, bounded list inputs, union variants, sourced options and snap sources.

/// 🧭️ The pointer and coercion helpers behind `historyEditInput`: object members and array slots are addressed by
/// RFC 6901 pointers, host values take the input's declared shape, and a pointer into a scalar is refused.
#[test]
fn input_pointers_and_host_values_take_the_declared_shape() {
    let mut value = DslValue::object([("pivot".to_string(), DslValue::object([("x".to_string(), DslValue::float(1.0))])), ("targets".to_string(), DslValue::Array(vec![DslValue::String("a".into())]))]);
    assert!(time_travel::time_travel_pointer_set(&mut value, "/pivot/x", DslValue::float(2.0)));
    assert!(time_travel::time_travel_pointer_set(&mut value, "/targets/1", DslValue::String("b".into())));
    assert!(!time_travel::time_travel_pointer_set(&mut value, "/pivot/x/deeper", DslValue::Null));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/pivot/x").and_then(DslValue::as_f64), Some(2.0));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/targets/1").and_then(DslValue::as_str), Some("b"));
    let count = semio_framework::ActionArgDef::index("/value", LocalizedLabel::native("Value", "Wert"));
    assert_eq!(time_travel::time_travel_coerce(&count, false, &DslValue::String("4".into())), Some(DslValue::uint(4)));
    assert_eq!(time_travel::time_travel_coerce(&count, false, &DslValue::String("four".into())), None);
    let flag = semio_framework::ActionArgDef::toggle("/on", LocalizedLabel::native("On", "An"));
    assert_eq!(time_travel::time_travel_coerce(&flag, false, &DslValue::String("true".into())), Some(DslValue::Bool(true)));
    let angle = semio_framework::ActionArgDef::number("/angle", LocalizedLabel::native("Angle", "Winkel"));
    let handles = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(semio_framework::ArgSchema::Object { fields: vec![angle] }), min_items: None, max_items: None },
        ..semio_framework::ActionArgDef::text("/handles", LocalizedLabel::native("Handles", "Griffe"))
    };
    let inputs = vec![
        semio_framework::ActionArgDef::object("/pivot", LocalizedLabel::native("Pivot", "Drehpunkt"), vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("X", "X"))]),
        semio_framework::ActionArgDef::vector("/offset", LocalizedLabel::native("Offset", "Versatz"), 2),
        handles,
    ];
    let payload = dsl(&serde_json::json!({ "pivot": { "x": 1.0 }, "offset": [0.0, 0.0], "handles": [{ "angle": 0.0 }, { "angle": 90.0 }] }));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/pivot/x").is_some_and(|(input, element)| input.id == "/x" && !element));
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/offset/1").is_some_and(|(input, element)| input.id == "/offset" && element));
    assert!(
        time_travel::time_travel_input_at(&inputs, &payload, "/handles/1/angle").is_some_and(|(input, element)| input.id == "/angle" && !element && matches!(input.schema, semio_framework::ArgSchema::Number { .. })),
        "a field inside an array of objects is addressable"
    );
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/1").is_some_and(|(input, element)| !element && matches!(input.schema, semio_framework::ArgSchema::Object { .. })), "an item takes the array's item schema");
    assert!(time_travel::time_travel_input_at(&inputs, &payload, "/handles/x/angle").is_none() && time_travel::time_travel_input_at(&inputs, &payload, "/missing").is_none());
    let rows = time_travel::time_travel_input_rows(&inputs, &payload);
    assert_eq!(
        rows.iter().map(|row| row.pointer.as_str()).collect::<Vec<_>>(),
        ["/pivot/x", "/offset", "/handles", "/handles/0", "/handles/0/angle", "/handles/1", "/handles/1/angle"],
        "objects flatten into field rows; a list is its own row, then every item's row and fields"
    );
    assert_eq!(
        (rows[2].list, rows[2].item, rows[3].list, rows[3].item, rows[4].item),
        (Some(time_travel::TimeTravelList { len: 2, addable: true, max: None }), None, None, Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: None }), None)
    );
    assert_eq!((rows[6].label.resolve(Terminology::Native, Locale::En), rows[6].label.resolve(Terminology::Native, Locale::De), rows[6].value.as_f64()), ("Handles 2 \u{b7} Angle", "Griffe 2 \u{b7} Winkel", Some(90.0)));
}

/// ✂️ LAW (design §6, gap N2): a list input of any item kind is reachable item by item — its own row adds an item until
/// `maxItems`, every item's row removes it while more than `minItems` remain — and the list edits of `historyEditInput`
/// insert before an index or after the last item (`-`) and remove by index; a new item starts from its schema default.
#[test]
fn list_inputs_add_and_remove_items_within_their_bounds() {
    let point = semio_framework::ArgSchema::Object { fields: vec![semio_framework::ActionArgDef::number("/x", LocalizedLabel::native("X", "X")).required(), semio_framework::ActionArgDef::text("/tag", LocalizedLabel::native("Tag", "Markierung"))] };
    let points = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(point.clone()), min_items: Some(1), max_items: Some(2) },
        ..semio_framework::ActionArgDef::text("/points", LocalizedLabel::native("Points", "Punkte"))
    };
    let weights = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::Array { items: Box::new(semio_framework::ArgSchema::number(Some(0.0), Some(1.0), None, false)), min_items: None, max_items: None },
        ..semio_framework::ActionArgDef::text("/weights", LocalizedLabel::native("Weights", "Gewichte"))
    };
    let inputs = vec![points, weights];
    let mut value = dsl(&serde_json::json!({ "points": [{ "x": 1.0 }, { "x": 2.0 }], "weights": [0.5] }));
    let rows = time_travel::time_travel_input_rows(&inputs, &value);
    assert_eq!(rows.iter().map(|row| row.pointer.as_str()).collect::<Vec<_>>(), ["/points", "/points/0", "/points/0/x", "/points/0/tag", "/points/1", "/points/1/x", "/points/1/tag", "/weights", "/weights/0"]);
    assert_eq!(rows[0].list, Some(time_travel::TimeTravelList { len: 2, addable: false, max: Some(2) }), "two points fill maxItems");
    assert_eq!((rows[1].item, rows[4].item), (Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: Some(1) }), Some(time_travel::TimeTravelListItem { index: 1, removable: true, min: Some(1) })));
    assert!(matches!(rows[8].input.schema, semio_framework::ArgSchema::Number { .. }) && rows[8].item == Some(time_travel::TimeTravelListItem { index: 0, removable: true, min: None }), "a number item is its own row: {:?}", rows[8]);
    assert!(time_travel::time_travel_pointer_insert(&mut value, "/weights/-", DslValue::float(0.25)));
    assert!(time_travel::time_travel_pointer_insert(&mut value, "/weights/0", DslValue::float(0.75)));
    assert!(time_travel::time_travel_pointer_remove(&mut value, "/points/0"));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/weights").and_then(DslValue::as_array).map(|items| items.iter().filter_map(DslValue::as_f64).collect::<Vec<_>>()), Some(vec![0.75, 0.5, 0.25]));
    assert_eq!(time_travel::time_travel_pointer_get(&value, "/points/0/x").and_then(DslValue::as_f64), Some(2.0));
    assert!(
        !time_travel::time_travel_pointer_insert(&mut value, "/weights/9", DslValue::Null)
            && !time_travel::time_travel_pointer_remove(&mut value, "/weights/3")
            && !time_travel::time_travel_pointer_remove(&mut value, "/missing/0")
            && !time_travel::time_travel_pointer_insert(&mut value, "/points/0/x/-", DslValue::Null)
    );
    let rows = time_travel::time_travel_input_rows(&inputs, &value);
    assert_eq!(
        (rows[0].list, rows[1].item),
        (Some(time_travel::TimeTravelList { len: 1, addable: true, max: Some(2) }), Some(time_travel::TimeTravelListItem { index: 0, removable: false, min: Some(1) })),
        "one point left: addable, not removable below minItems"
    );
    let item = time_travel::time_travel_default_value(&point);
    assert_eq!((time_travel::time_travel_pointer_get(&item, "/x").and_then(DslValue::as_f64), time_travel::time_travel_pointer_get(&item, "/tag")), (Some(0.0), None), "a new point holds its required fields only");
    let mut floor = semio_framework::ArgSchema::number(Some(0.0), Some(4.0), Some(0.5), false);
    if let semio_framework::ArgSchema::Number { min_exclusive, .. } = &mut floor {
        *min_exclusive = true;
    }
    assert_eq!(time_travel::time_travel_default_value(&floor).as_f64(), Some(0.5), "an excluded floor is stepped over");
    let limits = [(8, true, Locale::En), (8, true, Locale::De), (1, false, Locale::En), (1, false, Locale::De), (1, true, Locale::En), (3, false, Locale::De)].map(|(limit, max, locale)| time_travel::time_travel_item_limit_text(limit, max, locale));
    assert_eq!(limits, ["Maximum 8 items", "Höchstens 8 Einträge", "Minimum 1 item", "Mindestens 1 Eintrag", "Maximum 1 item", "Mindestens 3 Einträge"].map(str::to_string), "a disabled list edit names its bound");
}

/// 🔀️ A union payload's inputs resolve against the variant the draft is in: the reader groups every variant's fields by
/// the variant's value (a glTF `create-node` declares `/value` once per variant), the editor shows the variant selector
/// and the active variant's fields only (a hidden restore diff never), switching the selector drops the members only
/// other variants declare, and layout `group`s of a payload that is no union hide nothing.
#[test]
fn union_inputs_resolve_against_the_active_variant() {
    let option = |value: &str, en: &str, de: &str| semio_framework::ActionArgOption { value: value.into(), label: LocalizedLabel::native(en, de) };
    let phase = semio_framework::ActionArgDef {
        schema: semio_framework::ArgSchema::String { options: vec![option("apply", "Apply", "Anwenden"), option("restore", "Restore", "Wiederherstellen")], option_source: None, min_len: None, max_len: None, pattern: None, format: None },
        required: true,
        ..semio_framework::ActionArgDef::text("/phase", LocalizedLabel::native("Mutation phase", "Mutationsphase"))
    };
    let parameters = semio_framework::ActionArgDef {
        group: Some("apply".into()),
        ..semio_framework::ActionArgDef::object("/value", LocalizedLabel::native("Parameters", "Parameter"), vec![semio_framework::ActionArgDef::index("/position", LocalizedLabel::native("Insert position", "Einfügeposition"))])
    };
    let weight = semio_framework::ActionArgDef { group: Some("apply".into()), ..semio_framework::ActionArgDef::number("/weight", LocalizedLabel::native("Weight", "Gewicht")) };
    let diff = semio_framework::ActionArgDef {
        group: Some("restore".into()),
        presentation: Some(semio_framework::ArgPresentation::Hidden),
        schema: semio_framework::ArgSchema::Any,
        ..semio_framework::ActionArgDef::text("/value", LocalizedLabel::native("Restore diff", "Wiederherstellungs-Diff"))
    };
    let union = vec![phase, parameters, weight, diff];
    assert_eq!(time_travel::time_travel_variant_selector(&union).map(|selector| selector.id.as_str()), Some("/phase"));
    let applying = dsl(&serde_json::json!({ "phase": "apply", "value": { "position": 2 }, "weight": 1.0 }));
    let restoring = dsl(&serde_json::json!({ "phase": "restore", "value": { "nodes": [] } }));
    assert!(time_travel::time_travel_input_at(&union, &applying, "/value/position").is_some_and(|(input, _)| input.id == "/position"), "the apply variant's parameters are addressable");
    assert!(time_travel::time_travel_input_at(&union, &restoring, "/value").is_some_and(|(input, _)| input.group.as_deref() == Some("restore")), "the restore variant's /value wins while restoring");
    assert!(time_travel::time_travel_input_at(&union, &restoring, "/value/position").is_none() && time_travel::time_travel_input_at(&union, &restoring, "/weight").is_none(), "another variant's fields are not addressable");
    let pointers = |value: &DslValue| time_travel::time_travel_input_rows(&union, value).into_iter().map(|row| row.pointer).collect::<Vec<_>>();
    assert_eq!(pointers(&applying), ["/phase", "/value/position", "/weight"]);
    assert_eq!(pointers(&restoring), ["/phase"], "a hidden restore diff is never a row");
    let switched = time_travel::time_travel_switch_variant(&union, &union[0], &applying, "restore");
    assert_eq!(switched, dsl(&serde_json::json!({ "phase": "restore", "value": { "position": 2 } })), "members only other variants declare are dropped");
    let laid_out = vec![
        semio_framework::ActionArgDef { group: Some("position".into()), ..semio_framework::ActionArgDef::number("/newX", LocalizedLabel::native("X", "X")) },
        semio_framework::ActionArgDef { group: Some("target".into()), ..semio_framework::ActionArgDef::text("/id", LocalizedLabel::native("Target region", "Zielregion")) },
    ];
    assert!(time_travel::time_travel_variant_selector(&laid_out).is_none());
    assert_eq!(time_travel::time_travel_input_rows(&laid_out, &dsl(&serde_json::json!({ "newX": 1.0, "id": "r" }))).len(), 2, "layout groups hide nothing");
}

/// 🗝️ LAW (design §20.11): an `optionSource` choice offers the keys of the object its template leads to in the previewed
/// document, each `{field}` segment taking the edited payload's member — an object's key, or the record of an array whose own
/// `field` equals it; a key names itself by its record's `label`/`name`, else the glossary, else the key. The row's own value stays an option when the document lacks it — the editor adds no enum, so the
/// fold refuses an unknown key — and a template the payload cannot fill offers nothing (a plain text field).
#[test]
fn sourced_options_are_the_keys_of_the_previewed_document() {
    let schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "additionalProperties": false,
        "required": ["mutation", "id", "channel"],
        "properties": {
            "mutation": { "const": "changeWidgetInput" },
            "id": { "type": "string", "minLength": 1, "x-semio-ui": { "label": { "en": "Widget", "de": "Widget" } } },
            "channel": { "type": "string", "minLength": 1, "x-semio-ui": { "widget": "select", "label": { "en": "Channel", "de": "Kanal" }, "optionSource": { "snapshot": "/hostSnapshot/widgets/{id}/params" } } }
        }
    }"#;
    let inputs = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document).expect("the sourced choice reads");
    let document = dsl(&serde_json::json!({ "hostSnapshot": { "widgets": [{ "kind": "source", "id": "w0" }, { "kind": "operator", "id": "w/1", "params": { "gain": { "label": { "en": "Gain", "de": "Verstärkung" } }, "zorp": 1.0 } }] } }));
    let resolved = |payload: serde_json::Value| {
        let payload = dsl(&payload);
        let mut rows = time_travel::time_travel_input_rows(&inputs, &payload);
        time_travel::time_travel_resolve_options(&mut rows, &payload, &document);
        rows.into_iter().find(|row| row.pointer == "/channel").expect("the channel row")
    };
    let keyed = dsl(&serde_json::json!({ "groups": { "a/b": { "x": 1 } }, "list": [{ "n": 7, "v": { "y": 2 } }] }));
    assert_eq!(time_travel::time_travel_option_target("/groups/{g}", &dsl(&serde_json::json!({ "g": "a/b" })), &keyed), Some(&dsl(&serde_json::json!({ "x": 1 }))), "a filled segment is an object key, never re-parsed as a pointer");
    assert_eq!(time_travel::time_travel_option_target("/list/{n}/v", &dsl(&serde_json::json!({ "n": 7 })), &keyed), Some(&dsl(&serde_json::json!({ "y": 2 }))), "on an array a filled segment selects the record whose own member matches");
    assert_eq!(time_travel::time_travel_option_target("/list/0/v", &DslValue::Null, &keyed), Some(&dsl(&serde_json::json!({ "y": 2 }))), "a literal segment indexes");
    assert_eq!(time_travel::time_travel_option_target("/list/{n}/v", &dsl(&serde_json::json!({ "n": 8 })), &keyed), None, "no record matches");
    let channel = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "id": "w/1", "channel": "gain" }));
    let semio_framework::ActionArgControl::Select { options } = channel.input.control() else { panic!("a sourced choice is a select: {:?}", channel.input.control()) };
    let named = options.iter().map(|option| (option.value.as_str(), option.label.resolve(Terminology::Native, Locale::En).to_string(), option.label.resolve(Terminology::Native, Locale::De).to_string())).collect::<Vec<_>>();
    assert_eq!(named, [("gain", "Gain".to_string(), "Verstärkung".to_string()), ("zorp", "zorp".to_string(), "zorp".to_string())], "keys in document order, named by their record");
    let stale = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "id": "w/1", "channel": "gone" }));
    assert!(
        matches!(&stale.input.schema, semio_framework::ArgSchema::String { options, option_source: Some(_), .. } if options.iter().map(|option| option.value.as_str()).collect::<Vec<_>>() == ["gain", "zorp", "gone"]),
        "the recorded key stays selectable for the fold to judge: {:?}",
        stale.input.schema
    );
    assert!(!format!("{:?}", stale.input.json_schema(Terminology::Native, Locale::En)).contains("enum"), "the editor never narrows the payload schema");
    let unfilled = resolved(serde_json::json!({ "mutation": "changeWidgetInput", "channel": "gain" }));
    assert!(
        matches!(unfilled.input.control(), semio_framework::ActionArgControl::Select { ref options } if options.iter().map(|option| option.value.as_str()).collect::<Vec<_>>() == ["gain"]),
        "an unfillable template offers only the recorded key: {:?}",
        unfilled.input.control()
    );
}

/// 🧲️ LAW: every `snapSource` is numbers before a host sees it. Puzzle 2d's position inputs (`newX`, `snapSource:
/// {config: "gridFactor"}`, here read by the real input reader) step by the configured grid factor, a bounded input
/// snaps to the factor's multiples, and a `Snapshot` pointer reads the document the open session previews.
#[semio_framework_async_macros::async_test]
async fn snap_sources_resolve_from_the_grid_factor_and_the_previewed_document() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=original_time_travel_fixture_caller();
    let mut observer=|_|true;
    let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(maximum_identity_bytes,&mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).unwrap();
    let schema = r#"{
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "https://json.schemas.assets.semio-tech.com/framework/os/plugin/test/snap-source/schema.json",
        "type": "object",
        "additionalProperties": false,
        "required": ["mutation", "newX", "radius"],
        "properties": {
            "mutation": { "const": "moveTargetRegion" },
            "newX": { "type": "number", "x-semio-ui": { "widget": "stepper", "label": { "en": "X", "de": "X" }, "step": 1, "precision": 2, "snapSource": { "config": "gridFactor" } } },
            "radius": { "type": "number", "minimum": 0, "maximum": 10, "x-semio-ui": { "widget": "slider", "label": { "en": "Radius", "de": "Radius" }, "snapSource": { "config": "gridFactor" } } }
        }
    }"#;
    let inputs = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document).expect("the puzzle-shaped schema reads");
    let mut rows = time_travel::time_travel_input_rows(&inputs, &dsl(&serde_json::json!({ "newX": 3.0, "radius": 1.0 })));
    let config = dsl(&serde_json::json!({ "gridFactor": 0.5 }));
    time_travel::time_travel_resolve_snaps(&mut rows, |source| match source {
        semio_framework::SnapSource::Config { key } => time_travel::time_travel_pointer_get(&config, &format!("/{key}")).and_then(DslValue::as_f64),
        _ => None,
    });
    assert!(matches!(rows[0].input.control(), semio_framework::ActionArgControl::Stepper { step: Some(step), .. } if step == 0.5), "newX steps by the grid factor: {:?}", rows[0].input.control());
    assert!(matches!(&rows[1].input.schema, semio_framework::ArgSchema::Number { snaps, snap_source: None, .. } if snaps.len() == 21 && snaps[1] == 0.5 && snaps[20] == 10.0), "a bounded radius snaps to the grid: {:?}", rows[1].input.schema);
    let fixture = fixture();
    let mut app = seeded_app(&fixture, mounted_policy, &mut identity).await;
    run_step(&mut app, &fixture, &serde_json::json!({ "begin": 3 }), mounted_policy, &mut identity,cx).await;
    let mut panel = app.time_travel.panel().expect("an editing session");
    let editor = panel.editor.as_mut().expect("the draft editor");
    let mut count = semio_framework::ArgSchema::number(Some(0.0), Some(20.0), None, false);
    if let semio_framework::ArgSchema::Number { snap_source, .. } = &mut count {
        *snap_source = Some(semio_framework::SnapSource::Snapshot { pointer: "/count".into() });
    }
    editor.rows[0].input.schema = count;
    app.time_travel_ref().resolve_time_travel_snaps(&mut panel, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let resolved = &panel.editor.as_ref().expect("the draft editor").rows[0].input.schema;
    assert!(matches!(resolved, semio_framework::ArgSchema::Number { snaps, snap_source: None, .. } if *snaps == vec![0.0, 5.0, 10.0, 15.0, 20.0]), "the previewed document's count is the grid: {resolved:?}");
    verb(&mut app, &fixture, "historyEditExit", Vec::new(), mounted_policy, &mut identity).await;
    pump_until(&mut app, "retirement settles", |app| !app.time_travel.has_pending_work(), mounted_policy, &mut identity,cx).await;
    close(&mut app,cx);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
