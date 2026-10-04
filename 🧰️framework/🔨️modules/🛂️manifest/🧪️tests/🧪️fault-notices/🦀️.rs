use super::*;
use serde_json::{json, Value};

fn corpus() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧫️fault-notices/🔣️.json")).unwrap()
}

fn notices(table: &Value) -> Vec<FaultNoticeDefinition> {
    serde_json::from_value(table["notices"].clone()).unwrap()
}

fn projected(error: &FaultNoticeError) -> Value {
    match error {
        FaultNoticeError::EmptyText { code, terminology, locale } | FaultNoticeError::Placeholder { code, terminology, locale } => json!({"rule": error.rule(), "code": code, "terminology": terminology.as_str(), "locale": locale.as_str()}),
        FaultNoticeError::Syntax { code } | FaultNoticeError::Duplicate { code } => json!({"rule": error.rule(), "code": code}),
    }
}

#[test]
fn every_corpus_table_answers_exactly_its_refusals() {
    let corpus = corpus();
    for table in corpus["tables"].as_array().unwrap() {
        let errors: Vec<Value> = validate_fault_notices(&notices(table)).iter().map(projected).collect();
        assert_eq!(Value::Array(errors), table["errors"], "{}", table["id"]);
    }
    println!("[DEBUG] fault notice corpus: {} tables answered their exact refusals", corpus["tables"].as_array().unwrap().len());
}

#[test]
fn published_notices_round_trip_through_serde_and_value_codecs() {
    let corpus = corpus();
    let valid = corpus["tables"].as_array().unwrap().iter().find(|table| table["id"] == corpus["appNotices"]).unwrap();
    for notice in notices(valid) {
        assert_eq!(serde_json::from_value::<FaultNoticeDefinition>(serde_json::to_value(&notice).unwrap()).unwrap(), notice);
        assert_eq!(FaultNoticeDefinition::from_value(notice.to_value()).unwrap(), notice);
    }
    assert!(serde_json::from_value::<FaultNoticeDefinition>(json!({"code": "generation3d.gumball.mesh-missing", "label": {"native": {"en": "x"}, "reuse": {"en": "x", "de": "y"}}})).is_err(), "a notice without every locale is refused on decode");
}

#[test]
fn a_notice_fills_only_named_params_and_is_never_half_filled() {
    let params = semio_framework_diagnostic::FaultParams(vec![("kind".into(), "brep.mesh.translate".into())]);
    assert_eq!(fill_fault_notice("Die Widget-Art {kind} fehlt.", Some(&params)).as_deref(), Some("Die Widget-Art brep.mesh.translate fehlt."));
    assert_eq!(fill_fault_notice("Plain text.", None).as_deref(), Some("Plain text."));
    assert_eq!(fill_fault_notice("Needs {kind}.", None), None);
    assert_eq!(fill_fault_notice("Needs {count}.", Some(&params)), None);
    assert_eq!(fault_notice_placeholders("{a} and {b} and {a}").map(|names| names.into_iter().collect::<Vec<_>>()), Some(vec!["a", "b"]));
    assert_eq!(fault_notice_placeholders("{a"), None);
    assert!(is_fault_notice_code("generation3d.gumball.mesh-missing") && !is_fault_notice_code("history.full") && !is_fault_notice_code("app.command.Rejected"));
}
