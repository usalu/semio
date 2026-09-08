
use super::*;

#[test]
fn block_fields_roundtrip() {
    let json = r#"{
            "id":"b1",
            "label":"Team size",
            "kind":"slider",
            "required":true,
            "min":1,
            "max":50,
            "step":1,
            "unit":"people",
            "condition":{"kind":"truthy","expr":{"kind":"var","name":"show-team-size"}}
        }"#;
    let block: PlaybookBlock = serde_json::from_str(json).expect("block json");
    assert_eq!(block.min, Some(1.0));
    assert_eq!(block.unit.as_deref(), Some("people"));
    assert!(block.required.unwrap_or(false));
}

#[test]
fn conditional_visibility_filters_blocks() {
    let step = PlaybookStep {
        id: "s".into(),
        title: "Step".into(),
        description: None,
        blocks: vec![
            PlaybookBlock {
                id: "show".into(),
                label: "Show".into(),
                kind: "boolean".into(),
                description: None,
                required: None,
                placeholder: None,
                default: Some(DslValue::Bool(false)),
                min: None,
                max: None,
                step: None,
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: None,
            },
            PlaybookBlock {
                id: "team-size".into(),
                label: "Team size".into(),
                kind: "slider".into(),
                description: None,
                required: None,
                placeholder: None,
                default: Some(DslValue::float(5.0)),
                min: Some(1.0),
                max: Some(50.0),
                step: Some(1.0),
                unit: None,
                text: None,
                options: None,
                fields: None,
                schema: None,
                src: None,
                accept: None,
                fixture_slug: None,
                params: None,
                condition: Some(PlaybookExpr::Truthy { expr: Box::new(PlaybookExpr::Var { name: "show".into() }) }),
            },
        ],
    };
    let mut values = PlaybookValues::new();
    values.insert("show".into(), DslValue::Bool(false));
    assert_eq!(visible_blocks(&step, &values).len(), 1);
    values.insert("show".into(), DslValue::Bool(true));
    assert_eq!(visible_blocks(&step, &values).len(), 2);
}

//#region 🔖️DslAndOpText
fn minimal_block(id: &str, kind: &str) -> PlaybookBlock {
    PlaybookBlock {
        id: id.into(),
        label: format!("Label {id}"),
        kind: kind.into(),
        description: None,
        required: None,
        placeholder: None,
        default: None,
        min: None,
        max: None,
        step: None,
        unit: None,
        text: None,
        options: None,
        fields: None,
        schema: None,
        src: None,
        accept: None,
        fixture_slug: None,
        params: None,
        condition: None,
    }
}

/// 🧱️ A block with EVERY optional property populated (including nested `options`/`fields` and a
/// deeply nested `condition` exercising every `PlaybookExpr` variant) — the DSL round-trip fixture.
fn fully_populated_block() -> PlaybookBlock {
    PlaybookBlock {
        id: "b-full".into(),
        label: "Team Size".into(),
        kind: "slider".into(),
        description: Some("How many people?".into()),
        required: Some(true),
        placeholder: Some("Enter a number".into()),
        default: Some(DslValue::float(5.0)),
        min: Some(1.0),
        max: Some(50.0),
        step: Some(1.0),
        unit: Some("people".into()),
        text: Some("Some note text\nwith a newline".into()),
        options: Some(vec![PlaybookBlockOption { value: "red".into(), label: "Red".into() }, PlaybookBlockOption { value: "blue".into(), label: "Blue".into() }]),
        fields: Some(vec![PlaybookVectorField { key: "x".into(), label: Some("X".into()), value: Some(1.5) }, PlaybookVectorField { key: "y".into(), label: None, value: None }]),
        schema: Some("solid.step".into()),
        src: Some("https://example.com/img.png".into()),
        accept: Some("image/*".into()),
        fixture_slug: Some("hexagonal-mushroom-column".into()),
        params: Some(DslValue::Object(vec![
            ("height".into(), DslValue::float(6.0)),
            ("nested".into(), DslValue::Object(vec![("a".into(), DslValue::Array(vec![DslValue::float(1.0), DslValue::float(2.0), DslValue::String("three\"quoted".into())]))])),
        ])),
        condition: Some(PlaybookExpr::And {
            items: vec![
                PlaybookExpr::Truthy { expr: Box::new(PlaybookExpr::Var { name: "show-team-size".into() }) },
                PlaybookExpr::Eq { left: Box::new(PlaybookExpr::Var { name: "mode".into() }), right: Box::new(PlaybookExpr::Const { value: DslValue::String("advanced".into()) }) },
                PlaybookExpr::Or { items: vec![PlaybookExpr::Var { name: "a".into() }, PlaybookExpr::Var { name: "b".into() }] },
            ],
        }),
    }
}

fn sample_spec() -> PlaybookSpec {
    PlaybookSpec {
        schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        id: "recipe".into(),
        version: "1".into(),
        title: Some("Recipe".into()),
        steps: vec![
            PlaybookStep { id: "s1".into(), title: "Basics".into(), description: Some("First step".into()), blocks: vec![minimal_block("b1", "text"), fully_populated_block()] },
            PlaybookStep { id: "s2".into(), title: "Review".into(), description: None, blocks: Vec::new() },
        ],
    }
}

#[test]
fn empty_playbook_snapshot_dsl_round_trips() {
    store::test_support::assert_dsl_round_trip(&empty_playbook_snapshot());
    store::test_support::assert_dsl_pack_equivalence(&empty_playbook_snapshot());
}

#[test]
fn sample_spec_dsl_round_trips() {
    store::test_support::assert_dsl_round_trip(&sample_spec());
    store::test_support::assert_dsl_pack_equivalence(&sample_spec());
}

//#endregion 🔖️DslAndOpText
