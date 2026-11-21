use super::*;
use semio_framework_test_fixtures::fixture_text;

/// 🔌️ Actual job selectors preserve the authored six-port core types checked by an independent engine.
#[test]
fn original_owned_job_export_selectors_match_authored_contract_and_wasmtime() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/🔌️exports/🔣️.json")).unwrap();
    let engine = wasmtime::Engine::default();
    let jobs = fixture["jobs"].as_array().unwrap();
    assert_eq!(jobs.len(), 6);
    assert_eq!(fixture["results"], serde_json::json!(["i64"]));
    for row in jobs {
        let name = row["core"].as_str().unwrap();
        let parameters: Vec<_> = row["parameters"].as_array().unwrap().iter()
            .map(|parameter| parameter.as_str().unwrap()).collect();
        let signature = parameters.join(" ");
        let wat = format!(r#"(module (func (export "{name}") (param {signature}) (result i64) i64.const 0))"#);
        let module = wasmtime::Module::new(&engine, wat).unwrap();
        let Some(wasmtime::ExternType::Func(oracle)) = module.get_export(name) else {
            panic!("{name}: authored job type missing from independent engine");
        };
        assert_eq!(oracle.params().len(), parameters.len(), "{name}");
        assert!(oracle.params().all(|kind| matches!(kind, wasmtime::ValType::I64)), "{name}");
        assert_eq!(oracle.results().len(), 1, "{name}");
        assert!(oracle.results().all(|kind| matches!(kind, wasmtime::ValType::I64)), "{name}");
        let actual: Vec<_> = OwnedSemioExport::ALL.iter().filter(|export| export.core_name() == name).collect();
        assert_eq!(actual.len(), 1, "{name}");
        let kind = actual[0].core_type();
        assert_eq!(kind.parameters, vec![ValueType::I64; oracle.params().len()], "{name}");
        assert_eq!(kind.results, [ValueType::I64], "{name}");
    }
}
