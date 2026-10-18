use super::*;
use semio_framework_test_fixtures::fixture_text;

/// 🔌️ Actual job selectors preserve the authored six-port core types checked by an independent engine.
#[test]
fn original_owned_job_export_selectors_match_authored_contract_and_wasmtime() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/🔌️exports/🔣️.json")).unwrap();
    let bytes = [0, 97, 115, 109, 1, 0, 0, 0, 1, 7, 1, 96, 2, 127, 127, 1, 126, 3, 2, 1, 0, 7, 7, 1, 3, 106, 111, 98, 0, 0, 10, 6, 1, 4, 0, 66, 0, 11];
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::new(&engine, bytes).unwrap();
    let Some(wasmtime::ExternType::Func(oracle)) = module.get_export("job") else { panic!("authored job type missing from independent engine") };
    assert!(oracle.params().all(|kind| matches!(kind, wasmtime::ValType::I32)));
    assert_eq!(oracle.params().len(), fixture["parameters"].as_array().unwrap().len());
    assert!(oracle.results().all(|kind| matches!(kind, wasmtime::ValType::I64)));
    assert_eq!(oracle.results().len(), fixture["results"].as_array().unwrap().len());
    let jobs = fixture["jobs"].as_array().unwrap();
    assert_eq!(jobs.len(), 6);
    for row in jobs {
        let name = row["core"].as_str().unwrap();
        let actual: Vec<_> = OwnedSemioExport::ALL.iter().filter(|export| export.core_name() == name).collect();
        assert_eq!(actual.len(), 1);
        let kind = actual[0].core_type();
        assert_eq!(kind.parameters, [ValueType::I32, ValueType::I32]);
        assert_eq!(kind.results, [ValueType::I64]);
    }
}

