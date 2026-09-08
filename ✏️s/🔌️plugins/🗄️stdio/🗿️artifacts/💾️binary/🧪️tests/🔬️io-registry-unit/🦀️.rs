mod tests {
    use super::*;
    use semio_framework_plugin::{IoDirection, IoKey, IoPayload, StandardId, SubsetId, io_resolve};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    #[semio_framework_async_macros::async_test]
    async fn compose_direct_round_trips_a_native_binary_payload() {
        let snapshot = crate::standards::v_raw::subsets::any::schema::empty_binary_snapshot();
        let bytes = store::ArtifactPack::encode_pack(&snapshot);
        let sources = [ErasedComposeSource { dialect: DIALECT, payload: IoPayload::Binary(bytes) }];
        let composed = compose(DIALECT, &sources).expect("compose");
        assert_eq!(composed.dialect, DIALECT);
        assert!(matches!(composed.payload, IoPayload::Binary(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn register_then_resolve_through_the_typed_registry_finds_this_composer() {
        register();
        let key = IoKey { artifact_kind: "s.stdio.binary".into(), standard: "raw".into(), subset: "*".into(), direction: IoDirection::Import, format_kind: "s.stdio.binary".into(), format_standard: "raw".into(), format_subset: "*".into() };
        let entry = io_resolve(&key).await.expect("resolve");
        assert_eq!(entry.writes, DIALECT);
    }
}
