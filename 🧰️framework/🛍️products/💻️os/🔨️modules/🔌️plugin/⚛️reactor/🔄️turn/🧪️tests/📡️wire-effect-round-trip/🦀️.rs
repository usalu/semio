use super::*;
use protocol::ToValue;
use semio_framework::kernel::{Effect, RequestId};

#[test]
fn request_file_open_survives_wire_effect_round_trip() {
    let effect = Effect::RequestFileOpen {
        req: RequestId(121),
        accept: "application/json,.json".into(),
        read_as: Some("text".into()),
        import_action: "importFixture".into(),
        multiple: false,
    };
    let bytes = store::pack_rt::encode_wire_value(&effect.to_value());
    let decoded = decode_wire_effect(&bytes).expect("RequestFileOpen must survive the browser wire table");
    match decoded {
        Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. } => {
            assert!(accept.contains("json"), "{accept}");
            assert_eq!(read_as.as_deref(), Some("text"));
            assert_eq!(import_action, "importFixture");
            assert!(!multiple);
        }
        other => panic!("wire dropped RequestFileOpen: {other:?}"),
    }
}

fn effect_wire_kind(effect: &Effect) -> &'static str {
    match effect {
        Effect::OpenWindow { .. } => "openWindow",
        Effect::CloseWindow { .. } => "closeWindow",
        Effect::Notify { .. } => "notify",
        Effect::ClipboardWrite { .. } => "clipboardWrite",
        Effect::RequestSync => "requestSync",
        Effect::Navigate { .. } => "navigate",
        Effect::LoadDocument { .. } => "loadDocument",
        Effect::OpenExternalUrl { .. } => "openExternalUrl",
        Effect::SetPanel { .. } => "setPanel",
        Effect::DownloadMediaExport { .. } => "downloadMediaExport",
        Effect::IconRenderExport { .. } => "iconRenderExport",
        Effect::RequestFileOpen { .. } => "requestFileOpen",
        Effect::RequestMediaFrames { .. } => "requestMediaFrames",
        Effect::SpawnPluginInstance { .. } => "spawnPluginInstance",
        Effect::OpenPluginInstance { .. } => "openPluginInstance",
        Effect::SetActiveUtility { .. } => "setActiveUtility",
        Effect::SetActiveTool { .. } => "setActiveTool",
        Effect::OpenDialog { .. } => "openDialog",
        Effect::DispatchAction { .. } => "dispatchAction",
        Effect::ReplayShellCommand { .. } => "replayShellCommand",
        Effect::InvokeExtension { .. } => "invokeExtension",
        Effect::SendMessage { .. } => "sendMessage",
        Effect::PublishEvent { .. } => "publishEvent",
        Effect::BlobWrite { .. } => "blobWrite",
        Effect::BlobLoad { .. } => "blobLoad",
        Effect::HttpRequest { .. } => "httpRequest",
        Effect::DocumentRead { .. } => "documentRead",
        Effect::DocumentWrite { .. } => "documentWrite",
        Effect::LinkResolve { .. } => "linkResolve",
        Effect::RegistryQuery { .. } => "registryQuery",
        Effect::IoCompose { .. } => "ioCompose",
        Effect::CacheDerive { .. } => "cacheDerive",
        Effect::CacheRead { .. } => "cacheRead",
        Effect::SetTimer { .. } => "setTimer",
        Effect::SpawnJob { .. } => "spawnJob",
        Effect::CancelJob { .. } => "cancelJob",
        Effect::Respond { .. } => "respond",
        Effect::StorageRead { .. } => "storageRead",
        Effect::StorageWrite { .. } => "storageWrite",
        Effect::StorageDelete { .. } => "storageDelete",
        Effect::RequestCapability { .. } => "requestCapability",
        Effect::ReleaseCapability { .. } => "releaseCapability",
        Effect::Subscribe { .. } => "subscribe",
        Effect::Unsubscribe { .. } => "unsubscribe",
        Effect::RequestInferenceProposal { .. } => "requestInferenceProposal",
    }
}

fn all_effect_wire_fixtures() -> Vec<Effect> {
    use semio_framework::kernel::{ArtifactHandle, CapabilityId, CapabilityRequest, ClipboardFragment, IconRenderExportItem, InferenceProposalKind, JobPlacement, MessageEndpoint, RequestOutcome, WindowHandle, WindowKindId};
    use semio_framework::{MediaClass, MediaForm, MediaType};
    let req = RequestId(7);
    let media = MediaType { class: MediaClass::Data, form: MediaForm::Value };
    vec![
        Effect::OpenWindow { req, kind: WindowKindId("main".into()), params: dsl::DslValue::Null },
        Effect::CloseWindow { window: WindowHandle(1) },
        Effect::Notify { message: "n".into() },
        Effect::ClipboardWrite { fragment: ClipboardFragment { schema: "s".into(), media_type: media.clone(), dsl_text: "{}".into(), pack_bytes: None, source_app: "a".into(), label: "l".into() } },
        Effect::RequestSync,
        Effect::Navigate { uri: "semio://x".into() },
        Effect::LoadDocument { pack: vec![1], spr: vec![2] },
        Effect::OpenExternalUrl { url: "https://example.test".into() },
        Effect::SetPanel { panel_json: "{}".into() },
        Effect::DownloadMediaExport { filename: "a.bin".into(), mime_type: "application/octet-stream".into(), data: "AA==".into(), encoding: None },
        Effect::IconRenderExport { items: vec![IconRenderExportItem { filename: "i.png".into(), request: dsl::DslValue::Null }] },
        Effect::RequestFileOpen { req, accept: "*".into(), read_as: None, import_action: "import".into(), multiple: false },
        Effect::RequestMediaFrames { req, accept: "video/*".into(), frame_action: "frame".into(), done_action: "done".into(), fallback_action: "fallback".into(), sample_stride: 0, max_frames: 0, max_long_edge_px: 0, fps_hint: 0.0, payload: None, args: None },
        Effect::SpawnPluginInstance { req, plugin_id: "p".into(), app_id: "a".into(), os_instance_id: None, label: None, document_json: None },
        Effect::OpenPluginInstance { plugin_id: "p".into(), app_id: "a".into(), os_instance_id: None },
        Effect::SetActiveUtility { window_id: "w".into(), utility_id: "u".into() },
        Effect::SetActiveTool { tool_id: "t".into() },
        Effect::OpenDialog { req, dialog_id: "d".into(), args: None },
        Effect::DispatchAction { req, action: "act".into(), args: None, delay_ms: 0 },
        Effect::ReplayShellCommand { action_id: "panelTab".into(), args: None },
        Effect::invoke_extension(req, "ext".into(), "cap".into(), "{}".into()),
        Effect::SendMessage { target: MessageEndpoint::Topic { name: "t".into() }, payload: vec![1] },
        Effect::PublishEvent { topic: "t".into(), payload: vec![1] },
        Effect::BlobWrite { req, media_type: media, bytes: vec![1] },
        Effect::BlobLoad { req, hash: "h".into() },
        Effect::HttpRequest { req, method: "GET".into(), url: "https://example.test".into(), headers: Vec::new(), body: None, stream: false },
        Effect::DocumentRead { req, doc: ArtifactHandle(1), lane: "main".into() },
        Effect::DocumentWrite { req, doc: ArtifactHandle(1), lane: "main".into(), ops: vec![1] },
        Effect::LinkResolve { req, link: "l".into() },
        Effect::RegistryQuery { req, kind: "k".into(), filter: None },
        Effect::IoCompose { req, key: "k".into(), sources: vec!["s".into()] },
        Effect::CacheDerive { req, engine_id: "e".into(), input: vec![1] },
        Effect::CacheRead { req, engine_id: "e".into(), key: "k".into() },
        Effect::SetTimer { id: 1, after_ms: 1, repeat: false },
        Effect::SpawnJob { job: 1, kind: "framework.reserved.tool".into(), input: vec![1], placement: JobPlacement::Isolated },
        Effect::CancelJob { job: 1 },
        Effect::Respond { req, result: RequestOutcome::Ok(vec![1]) },
        Effect::StorageRead { req, key: "k".into() },
        Effect::StorageWrite { req, key: "k".into(), bytes: vec![1] },
        Effect::StorageDelete { req, key: "k".into() },
        Effect::RequestCapability { req, capability: CapabilityRequest { id: CapabilityId("c".into()), scope: "s".into(), reason: "r".into(), optional: false } },
        Effect::ReleaseCapability { id: CapabilityId("c".into()) },
        Effect::Subscribe { topic: "t".into() },
        Effect::Unsubscribe { topic: "t".into() },
        Effect::RequestInferenceProposal { kind: InferenceProposalKind::GisMapBoundsRegion },
    ]
}

/// 🧪 W-G3 §8.21 — every `Effect` kind survives leftover `pack_rt` encode / `decode_wire_effect`.
/// A new variant that is not in `effect_wire_kind` fails compile; a fixture gap fails this count.
#[test]
fn every_effect_kind_survives_wire_effect_round_trip() {
    let fixtures = all_effect_wire_fixtures();
    let mut seen = std::collections::BTreeSet::new();
    for effect in &fixtures {
        let kind = effect_wire_kind(effect);
        assert!(seen.insert(kind), "duplicate wire-table fixture {kind}");
        let bytes = store::pack_rt::encode_wire_value(&effect.to_value());
        let decoded = decode_wire_effect(&bytes).unwrap_or_else(|_| panic!("wire table dropped {kind}"));
        assert_eq!(effect_wire_kind(&decoded), kind, "{kind} decoded as a different arm");
    }
    assert_eq!(seen.len(), fixtures.len(), "wire-table completeness fixtures must be unique");
    assert_eq!(fixtures.len(), 45, "every Effect kind must have a leftover wire-table fixture");
}
