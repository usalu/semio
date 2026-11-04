use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use semio_framework_ui_locale::Locale;
use crate::editor::mp3::modes::edit::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = Mp3Snapshot::default();
    let _node = render(&document, Locale::En, None, None);
}

#[semio_framework_async_macros::async_test]
async fn the_transport_is_ready_only_with_a_live_revision_bound_resource_and_explains_otherwise_in_both_locales() {
    use semio_framework_plugin::app::{MediaCapabilityStatus, MediaResource};
    let resource = MediaResource { controller_id: "s.stdio.mp3@mpeg1-layer3/*#editor".into(), app_instance_id: 1, parent_document_id: "document".into(), output_port: "playback:out".into(), revision: "7".into(), generation: "1".into() };
    for locale in [Locale::En, Locale::De] {
        let ready = view(&Mp3Snapshot::default(), locale, Some(resource.clone()), None);
        assert_eq!(ready.capability, MediaCapabilityStatus::Ready);
        assert_eq!(ready.capability_reason, None);
        assert_eq!(ready.revision, "7");
        assert_eq!(ready.resource.as_ref(), Some(&resource));
        let waiting = view(&Mp3Snapshot::default(), locale, None, None);
        assert_eq!(waiting.capability, MediaCapabilityStatus::Unsupported);
        assert_eq!(waiting.revision, "0");
        assert_eq!(waiting.capability_reason.as_deref(), Some(match locale { Locale::En => "Playback waits for a revision-bound document resource.", Locale::De => "Die Wiedergabe wartet auf eine revisionsgebundene Dokumentressource." }));
        assert!(render(&Mp3Snapshot::default(), locale, Some(resource.clone()), None).is_ok());
        assert!(render(&Mp3Snapshot::default(), locale, None, None).is_ok());
    }
}

#[semio_framework_async_macros::async_test]
async fn the_transport_declares_the_frame_cut_from_the_stream_timing_and_the_canonical_revision() {
    let snapshot = crate::standards::mpeg1_layer3::subsets::any::io::decode_mp3(include_bytes!("../../../../../../../🧪️tests/🧫️fixtures/🔊️.mp3")).expect("real MP3 fixture decodes");
    let declared = trim(&snapshot, Some("ab".repeat(32))).expect("a decoded stream declares its cut");
    assert_eq!((declared.action_id.as_str(), declared.units_per_second_numerator, declared.units_per_second_denominator, declared.total_units), ("cut-frames", 44_100, 1152, 462));
    assert!(trim(&snapshot, None).is_none());
    assert!(trim(&Mp3Snapshot::default(), Some("r".into())).is_none());
}
