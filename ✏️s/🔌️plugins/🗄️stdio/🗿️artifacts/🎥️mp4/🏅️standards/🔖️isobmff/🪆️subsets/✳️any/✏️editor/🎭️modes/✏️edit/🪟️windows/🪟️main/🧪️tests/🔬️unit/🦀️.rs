use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use semio_ui_locale::Locale;
use crate::editor::mp4::modes::edit::windows::main::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = Mp4Snapshot::default();
    let _node = render(&document, Locale::En, None, None);
}

#[semio_framework_async_macros::async_test]
async fn the_transport_is_ready_only_with_a_live_revision_bound_resource_and_explains_otherwise_in_both_locales() {
    use semio_plugin::app::{MediaCapabilityStatus, MediaResource};
    let resource = MediaResource { controller_id: "s.stdio.mp4@isobmff/*#editor".into(), app_instance_id: 1, parent_document_id: "document".into(), output_port: "playback:out".into(), revision: "7".into(), generation: "1".into() };
    for locale in [Locale::En, Locale::De] {
        let ready = view(&Mp4Snapshot::default(), locale, Some(resource.clone()), None);
        assert_eq!(ready.capability, MediaCapabilityStatus::Ready);
        assert_eq!(ready.capability_reason, None);
        assert_eq!(ready.revision, "7");
        assert_eq!(ready.resource.as_ref(), Some(&resource));
        let waiting = view(&Mp4Snapshot::default(), locale, None, None);
        assert_eq!(waiting.capability, MediaCapabilityStatus::Unsupported);
        assert_eq!(waiting.revision, "0");
        assert_eq!(waiting.capability_reason.as_deref(), Some(match locale { Locale::En => "Playback waits for a revision-bound document resource.", Locale::De => "Die Wiedergabe wartet auf eine revisionsgebundene Dokumentressource." }));
        assert!(render(&Mp4Snapshot::default(), locale, Some(resource.clone()), None).is_ok());
        assert!(render(&Mp4Snapshot::default(), locale, None, None).is_ok());
    }
}
