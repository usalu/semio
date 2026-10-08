use super::*;

#[test]
fn demo_example_declares_the_emblem_media() {
    let media = example_media(crate::examples::art_raster_demo::ID);
    assert_eq!(media.len(), 1);
    assert_eq!(media[0].0, "semio-emblem");
    assert!(!media[0].1.frames.is_empty());
}

#[test]
fn demo_example_document_carries_its_media_in_the_pool() {
    let document = example_document(crate::examples::art_raster_demo::ID).expect("the demo example document");
    assert!(crate::raster_asset(&document.assets, "semio-emblem").is_some(), "the load document resolves the emblem pixels");
    dismantle(document);
}

/// ⚖️ LAW: switching examples is a load — the example becomes one load-document effect, never mutation rows.
#[test]
fn switching_examples_builds_a_load_document_effect() {
    let document = example_document(crate::examples::art_raster_demo::ID).expect("the demo example document");
    let effect = crate::editor::raster::raster_reset_document_effect(&document);
    assert!(matches!(effect, semio_framework_plugin::Effect::LoadDocument { ref pack, .. } if !pack.is_empty()));
    dismantle(document);
}
