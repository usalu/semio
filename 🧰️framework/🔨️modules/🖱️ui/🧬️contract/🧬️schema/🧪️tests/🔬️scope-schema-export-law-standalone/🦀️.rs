use semio_framework_schema_registry::{resolve_schema_export, scope_schema_exports_registered, SchemaFormat};

#[test]
fn registers_and_resolves_exactly_the_declared_formats() {
    super::register_scope_exports();
    assert!(scope_schema_exports_registered("framework.ui.contract"));
    for export in super::EXPORTS.map(|declaration| declaration.id) {
        for format in SchemaFormat::ALL {
            let resolved = resolve_schema_export("framework.ui.contract", export, format);
            assert_eq!(resolved.is_ok(), super::DECLARED_FORMATS.contains(&format), "{export} resolves {format} but x-semio-formats declares {:?}", super::DECLARED_FORMATS);
        }
        assert!(resolve_schema_export("framework.ui.contract", export, SchemaFormat::JsonSchema).is_ok_and(|leaf| leaf.contains("framework/ui/contract/schema.json")));
        assert!(resolve_schema_export("framework.ui.contract", export, SchemaFormat::Rust).is_ok_and(|leaf| leaf.contains("register_scope_exports")));
    }
    assert!(resolve_schema_export("framework.ui.contract", "NotAnExport", SchemaFormat::JsonSchema).is_err());
}

#[test]
fn restricted_formats_match_the_annotation() {
    let module: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("module json");
    let expected: Vec<&str> = super::DECLARED_FORMATS.iter().map(|format| format.taxonomy_key()).collect();
    let defs = module["$defs"].as_object().expect("$defs");
    assert_eq!(defs.len(), super::EXPORTS.len());
    for export in super::EXPORTS.map(|declaration| declaration.id) {
        let declared: Vec<&str> = defs[export]["x-semio-formats"].as_array().expect("x-semio-formats").iter().map(|entry| entry.as_str().expect("format id")).collect();
        assert_eq!(declared, expected, "{export} must declare exactly the formats this module registers");
    }
}

#[test]
fn presence_update_export_matches_the_wire_shape() {
    let module: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("module json");
    let export = &module["$defs"]["PresenceUpdate"];
    let full = crate::PresenceUpdate {
        surface: crate::SurfaceId::try_from("law").expect("bounded surface id"),
        node_key: "node".into(),
        own: crate::OwnPresence { hovered: true, selected: true, previewed: true, color: Some(7) },
        peers: vec![crate::PeerMark { actor: "actor".into(), color: Some(1), hovered: true, selected: true, label: "A".into() }],
        ttl_ms: 1,
    };
    let wire = serde_json::to_value(&full).expect("serialize");
    assert_eq!(json_keys(&wire), json_keys(&export["properties"]), "PresenceUpdate wire fields and $defs.PresenceUpdate.properties must be the same set");
    assert_eq!(json_keys(&wire["own"]), json_keys(&module["definitions"]["presenceOwnMarks"]["properties"]));
    assert_eq!(json_keys(&wire["peers"][0]), json_keys(&module["definitions"]["presencePeerMark"]["properties"]));
    assert_eq!(export["properties"]["own"]["$ref"], "#/definitions/presenceOwnMarks");
    assert_eq!(export["properties"]["peers"]["items"]["$ref"], "#/definitions/presencePeerMark");
    let required: Vec<&str> = export["required"].as_array().expect("required").iter().map(|entry| entry.as_str().expect("field")).collect();
    assert_eq!(required, ["surface", "nodeKey", "own", "ttlMs"], "only `peers` is omitted from the wire when empty");
    let overlay = &module["$defs"]["ContractFixture"]["properties"]["cases"]["items"]["properties"]["update"];
    assert_eq!(overlay["allOf"][0]["$ref"], format!("{}#/$defs/PresenceUpdate", module["$id"].as_str().expect("$id")), "the fixture case references the export instead of restating it");
    assert!(json_keys(&overlay["allOf"][1]["properties"]).iter().all(|field| json_keys(&export["properties"]).contains(field)), "the overlay may only narrow fields the export already declares");
}

/// 🔑️ The property names of one JSON object, sorted, so a comparison is order-independent whatever
/// `serde_json`'s map backing is.
fn json_keys(value: &serde_json::Value) -> std::collections::BTreeSet<String> {
    value.as_object().expect("json object").keys().cloned().collect()
}
