use super::*;

#[test]
fn mounted_snapshot_preparation_records_original_receipts_before_readiness() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let number = |field: &str, index: usize| row[field][index].as_u64().unwrap() as usize;
        let grant = RetainedCloneGrant { maximum_items: number("grant", 0), maximum_copy_bytes: number("grant", 1), maximum_capacity_bytes: number("grant", 2), maximum_release_bytes: number("grant", 3), maximum_depth: number("grant", 4) };
        let progress = RetainedCloneProgress { copied_items: number("progress", 0), copied_bytes: number("progress", 1), retained_capacity_bytes: number("progress", 2), released_bytes: number("progress", 3) };
        let mut sequence = 0;
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(8, u64::MAX, grant), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, crate::artifact_app_laws::fixture_progress());
        let readiness = row["ready"].as_bool().unwrap();
        let result = MountedSnapshotReadPreparation { ready: readiness, progress }.record(grant, &mut cx);
        assert_eq!(result.is_err(), row["fault"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(cx.retained_progress(), progress);
        if let Ok(actual) = result { assert_eq!(actual, readiness); }
        println!("[DEBUG] mounted-snapshot-preparation {} readiness={} fault={}", row["name"], readiness, result.is_err());
    }
}
