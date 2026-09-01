use super::*;

//#region 🧵️NativeCodecSend
fn require_send<F: std::future::Future + Send>(_: &F) {}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🔣️.json")).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn document_codec_native_send_compile_dsl_preserves_exact_snapshots() {
    let fixture = fixture();
    let codec = ArtifactCodec::of::<DemoSnapshot, DemoMutation>(fixture["schema"].as_str().unwrap());
    for value in fixture["snapshots"].as_array().unwrap() {
        let snapshot: DemoSnapshot = serde_json::from_value(value.clone()).unwrap();
        let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>(fixture["schema"].as_str().unwrap(), "native-send", snapshot.clone(), None);
        let text = print_document_text(&envelope).await.expect("exact schema-owned input");
        let future = (codec.compile_dsl)(&text.dsl, &text.ops);
        require_send(&future);
        let (files, mirror) = future.await.expect("actual registered compile future");
        let decoded = DemoSnapshot::decode_pack(&files.pack).expect("actual typed PACK decoder");
        assert_eq!(serde_json::to_value(decoded).unwrap(), *value);
        assert_eq!(mirror, snapshot.print_dsl());
    }
}

#[semio_framework_async_macros::async_test]
async fn document_codec_native_send_print_mirror_preserves_exact_snapshots() {
    let fixture = fixture();
    let codec = ArtifactCodec::of::<DemoSnapshot, DemoMutation>(fixture["schema"].as_str().unwrap());
    for value in fixture["snapshots"].as_array().unwrap() {
        let snapshot: DemoSnapshot = serde_json::from_value(value.clone()).unwrap();
        let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>(fixture["schema"].as_str().unwrap(), "native-send", snapshot.clone(), None);
        let files = print_document_pack(&envelope).await.expect("exact schema-owned PACK input");
        let future = (codec.print_mirror)(&files.pack, &files.spr);
        require_send(&future);
        let mirror = future.await.expect("actual registered mirror future");
        let decoded = DemoSnapshot::parse_dsl(&mirror.dsl).expect("actual typed DSL decoder");
        assert_eq!(serde_json::to_value(decoded).unwrap(), *value);
        assert_eq!(mirror.dsl, snapshot.print_dsl());
    }
}
//#endregion 🧵️NativeCodecSend
