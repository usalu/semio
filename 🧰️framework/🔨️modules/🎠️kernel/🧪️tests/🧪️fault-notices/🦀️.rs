use super::*;
use serde_json::{json, Value};

#[test]
fn every_corpus_fault_resolves_to_its_notice_framework_first() {
    let corpus: Value = serde_json::from_str(include_str!("../../../🛂️manifest/🧫️fixtures/🧫️fault-notices/🔣️.json")).unwrap();
    let table = corpus["tables"].as_array().unwrap().iter().find(|table| table["id"] == corpus["appNotices"]).unwrap();
    let notices: Vec<crate::manifest::FaultNoticeDefinition> = serde_json::from_value(table["notices"].clone()).unwrap();
    for row in corpus["resolutions"].as_array().unwrap() {
        let declared = &row["fault"];
        let mut fault = Fault::new(FaultOrigin::App, declared["code"].as_str().unwrap().to_string(), declared["message"].as_str().unwrap_or(""));
        for (name, value) in declared["params"].as_object().into_iter().flatten() {
            fault = fault.with_param(name.clone(), value.as_str().unwrap());
        }
        fault.causes = declared["causes"].as_array().into_iter().flatten().map(|cause| FaultCause { message: String::new(), code: Some(FaultCode::new(cause["code"].as_str().unwrap().to_string())) }).collect();
        let terminology = semio_framework_ui_locale::Terminology::parse(row["terminology"].as_str().unwrap()).unwrap();
        let locale = semio_framework_ui_locale::Locale::parse(row["locale"].as_str().unwrap()).unwrap();
        let actual = fault_notice(&fault, &notices, terminology, locale).map_or(Value::Null, |notice| json!({"code": notice.code, "text": notice.text}));
        assert_eq!(actual, row["expected"], "{}", row["id"]);
    }
}
