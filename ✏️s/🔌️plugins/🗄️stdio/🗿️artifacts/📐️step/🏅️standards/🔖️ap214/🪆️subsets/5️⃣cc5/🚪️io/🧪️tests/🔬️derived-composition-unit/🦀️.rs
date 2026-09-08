mod tests {
    use super::*;
    use crate::standards::v_ap214::engine::part21::{Part21Document, Part21Header, Part21Instance};
    use semio_framework_plugin::AnalyzeSource;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn clean_bytes() -> Vec<u8> {
        let doc = Part21Document {
            header: Part21Header { file_schema: vec![], ..Part21Header::default() },
            instances: vec![
                Part21Instance { id: 1, entities: vec![("PRODUCT".into(), vec![])] },
                Part21Instance { id: 2, entities: vec![("PRODUCT_DEFINITION_FORMATION".into(), vec![])] },
                Part21Instance { id: 3, entities: vec![("PRODUCT_DEFINITION".into(), vec![])] },
            ],
        };
        <StepSnapshot as store::ArtifactPack>::encode_pack(&StepSnapshot::from_part21_document(&doc))
    }

    #[semio_framework_async_macros::async_test]
    async fn composer_injects_file_schema_and_stamps_clean_document() {
        let bytes = clean_bytes();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = StepCc5ComposerComposition::compose(&sources).expect("a document with no illegal representation must compose to cc5");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        assert!(crate::standards::v_ap214::engine::ladder::file_schema_contains(&composed.snapshot.to_part21_document(), "AUTOMOTIVE_DESIGN"), "composer must inject FILE_SCHEMA=AUTOMOTIVE_DESIGN");
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_missing_file_schema_on_the_raw_wire_payload() {
        // Unlike `compose`, `validate` never runs `ensure_file_schema` -- a wire payload that
        // skipped this subset's own composer genuinely lacks the injection.
        let bytes = clean_bytes();
        let diagnostics = StepCc5Validator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().any(|d| d.code.0 == crate::standards::v_ap214::subsets::cc5::schema::CODE_FILE_SCHEMA), "got {diagnostics:?}");
    }
}
