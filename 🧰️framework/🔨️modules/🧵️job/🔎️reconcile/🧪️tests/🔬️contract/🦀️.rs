use super::*;

#[test]
fn neutral_reconcile_envelopes_match_language_agnostic_vectors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let value = &row["value"];
        let accepted = (|| {
            let object = value.as_object()?;
            let keys: &[&str] = if row["kind"] == "request" { &["schema", "version", "requestId"] } else { &["schema", "version", "requestId", "found", "job"] };
            if object.len() != keys.len() || keys.iter().any(|key| !object.contains_key(*key)) { return None; }
            let schema = value["schema"].as_str()?;
            let version = u32::try_from(value["version"].as_u64()?).ok()?;
            let request_id = value["requestId"].as_str()?;
            if row["kind"] == "request" { parse_job_reconcile_request_v1(schema, version, request_id).ok()?; }
            else {
                let job = if value["job"].is_null() { None } else { Some(value["job"].as_object()?) };
                parse_job_reconcile_result_v1(schema, version, request_id, value["found"].as_bool()?, job).ok()?;
            }
            Some(())
        })().is_some();
        assert_eq!(accepted, row["accepted"].as_bool().unwrap(), "{}", row["name"]);
    }
    println!("job-reconcile-contract: shared Rust vectors={}", fixture["cases"].as_array().unwrap().len());
}
