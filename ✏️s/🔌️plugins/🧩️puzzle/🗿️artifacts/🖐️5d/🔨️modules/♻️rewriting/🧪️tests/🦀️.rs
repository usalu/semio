use super::*;
use semio_framework_pack_json::{JsonMemberPolicy, from_json_str};
use semio_s_artifact_puzzle_5d_mut_any_5::apply_puzzle5d_mutation;

fn fixture<T: FromValue>(key: &str) -> T {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔣️fixture.json")).unwrap();
    from_json_str(&fixture[key].to_string(), JsonMemberPolicy::Reject).unwrap()
}

fn rewrite_puzzle5d(
    snapshot: &Puzzle5dSnapshot,
    catalog: &Puzzle5dKindCatalogs,
    rule: &Rule,
    bindings: &PropertyBag,
) -> Result<Vec<Puzzle5dMutation>, Puzzle5dRewriteError> {
    let mut execution = start_puzzle5d_rewrite(snapshot, catalog, rule, bindings)?;
    loop {
        if let Some((_, effects)) = execution.step().map_err(|error| invalid(error.to_string()))? {
            return prepare_puzzle5d_effects(snapshot, catalog, &effects);
        }
    }
}

#[test]
fn graph_edits_preserve_other_puzzle_fields() {
    let snapshot: Puzzle5dSnapshot = fixture("before");
    let catalog = Puzzle5dKindCatalogs::default();
    let graph = graph_from_puzzle5d(&snapshot, &catalog).unwrap();
    assert_eq!(graph.nodes["a"].name, "A");
    assert_eq!(graph.edges["f"].source, "a@rim");
    let effects = [
        GraphEffect::RenameNode { id: "a".into(), name: "Renamed".into() },
        GraphEffect::MoveNode { id: "a".into(), x: 3.0, y: 4.0 },
        GraphEffect::SetProperty { entity: EntityRef::Edge("f".into()), key: "gap".into(), value: PropertyValue::Number(7.0) },
    ];
    let mut actual = snapshot.clone();
    for mutation in prepare_puzzle5d_effects(&snapshot, &catalog, &effects).unwrap() {
        apply_puzzle5d_mutation(&mut actual, &mutation).unwrap();
    }
    assert_eq!(actual, fixture("after"));
    assert_eq!(snapshot, fixture("before"));

    let reference: serde_json::Value = serde_json::from_str(include_str!("../🔣️fixture.json")).unwrap();
    let mut expected = reference["before"].clone();
    expected["parts"][0]["2d"]["text"] = serde_json::json!("Renamed");
    expected["parts"][0]["2d"]["x"] = serde_json::json!(3);
    expected["parts"][0]["2d"]["y"] = serde_json::json!(4);
    expected["fasteners"][0]["gap"] = serde_json::json!(7);
    assert_eq!(expected, reference["after"]);
}

#[test]
fn rejects_an_entire_effect_batch_if_an_endpoint_cannot_be_resolved() {
    let snapshot: Puzzle5dSnapshot = fixture("before");
    let effects = [
        GraphEffect::DeleteEdge("f".into()),
        GraphEffect::CreateEdge(Edge {
            id: "new".into(),
            kind: FASTENER.into(),
            source: "a@rim".into(),
            target: "b@missing".into(),
            properties: PropertyBag::new(),
        }),
    ];
    assert!(prepare_puzzle5d_effects(&snapshot, &Puzzle5dKindCatalogs::default(), &effects).is_err());
    assert_eq!(snapshot, fixture("before"));
}

#[test]
fn supported_no_ops_emit_nothing_and_unknown_fields_are_refused() {
    let snapshot: Puzzle5dSnapshot = fixture("before");
    let catalog = Puzzle5dKindCatalogs::default();
    let unchanged = GraphEffect::MoveNode { id: "a".into(), x: 1.0, y: 2.0 };
    assert!(prepare_puzzle5d_effects(&snapshot, &catalog, &[unchanged]).unwrap().is_empty());
    let unsupported = GraphEffect::SetProperty {
        entity: EntityRef::Node("a".into()),
        key: "unmapped".into(),
        value: PropertyValue::Bool(true),
    };
    assert!(prepare_puzzle5d_effects(&snapshot, &catalog, &[unsupported]).is_err());
}

#[test]
fn typed_rule_preserves_puzzle_metadata_and_composes_mutations() {
    let mut actual: Puzzle5dSnapshot = fixture("before");
    let mutations = rewrite_puzzle5d(&actual, &Puzzle5dKindCatalogs::default(), &fixture("rule"), &PropertyBag::new()).unwrap();
    assert_eq!(mutations.len(), 4);
    for mutation in mutations {
        apply_puzzle5d_mutation(&mut actual, &mutation).unwrap();
    }
    assert_eq!(actual, fixture("after"));
}

#[test]
fn typed_rule_creates_and_connects_a_part_with_real_grip_endpoints() {
    let mut actual: Puzzle5dSnapshot = fixture("before");
    let catalog = Puzzle5dKindCatalogs::default();
    let mutations = rewrite_puzzle5d(&actual, &catalog, &fixture("createRule"), &PropertyBag::new()).unwrap();
    assert!(matches!(mutations.as_slice(), [
        Puzzle5dMutation::DisconnectGrips(_), Puzzle5dMutation::CreatePart(_), Puzzle5dMutation::ConnectGrips(_),
        Puzzle5dMutation::EditPart3dLabel(_), Puzzle5dMutation::ReplaceFastenerGeometry(_),
    ]));
    for mutation in mutations {
        apply_puzzle5d_mutation(&mut actual, &mutation).unwrap();
    }
    assert_eq!(actual.parts.len(), 3);
    assert_eq!(actual.fasteners.len(), 1);
    assert_eq!(actual.fasteners[0].source, "a:rim");
    assert_eq!(actual.fasteners[0].target, format!("{}:in", actual.parts[2].id));
    assert_eq!(actual.parts[2].part_3d.label.as_deref(), Some("New world part"));
    assert_eq!(actual.fasteners[0].gap, 2.0);
    graph_from_puzzle5d(&actual, &catalog).unwrap();
}

#[test]
fn creation_preserves_the_inferred_kind_template_from_a_catalogue_less_puzzle() {
    let mut snapshot: Puzzle5dSnapshot = fixture("before");
    snapshot.parts[0].part_kind = Some("Beam Section.v1".into());
    snapshot.parts[0].part_3d.mesh_url = Some("beam.glb".into());
    let catalog = Puzzle5dKindCatalogs::default();
    let rule: Rule = from_json_str(
        r#"{"name":"Create placed kind","lhs":{"pattern":{"leftVar":"a","leftKind":"Beam Section.v1"}},"rhs":{"create":[{"leftVar":"b","leftKind":"Beam Section.v1"}],"delete":[],"set":[],"merge":[],"parameters":[]}}"#,
        JsonMemberPolicy::Reject,
    ).unwrap();
    let mutations = rewrite_puzzle5d(&snapshot, &catalog, &rule, &PropertyBag::new()).unwrap();
    let [Puzzle5dMutation::CreatePart(create)] = mutations.as_slice() else { panic!("expected one new part"); };
    assert_eq!(create.part.part_kind, snapshot.parts[0].part_kind);
    assert_eq!(create.part.part_3d.mesh_url, snapshot.parts[0].part_3d.mesh_url);
    assert_eq!(create.part.grips, snapshot.parts[0].grips);
}
