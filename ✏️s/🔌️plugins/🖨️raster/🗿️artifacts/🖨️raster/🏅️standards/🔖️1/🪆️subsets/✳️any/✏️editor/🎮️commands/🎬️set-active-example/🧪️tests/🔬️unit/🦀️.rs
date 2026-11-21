use super::*;

#[test]
fn demo_example_declares_the_emblem_media() {
    let media = example_media(semio_s_artifact_raster_raster_examples::art_raster_demo::ID);
    assert_eq!(media.len(), 1);
    assert_eq!(media[0].0, "semio-emblem");
    assert!(!media[0].1.frames.is_empty());
}

#[test]
fn demo_example_document_carries_its_media_in_the_pool() {
    let document = example_document(semio_s_artifact_raster_raster_examples::art_raster_demo::ID).expect("the demo example document");
    assert!(semio_s_artifact_raster_raster_io::raster_asset(&document.assets, "semio-emblem").is_some(), "the load document resolves the emblem pixels");
    dismantle(document);
}

/// ⚖️ LAW: switching examples is a load — the example becomes one load-document effect, never mutation rows.
#[test]
fn switching_examples_builds_a_load_document_effect() {
    let document = example_document(semio_s_artifact_raster_raster_examples::art_raster_demo::ID).expect("the demo example document");
    let effect = crate::raster_reset_document_effect(&document);
    assert!(matches!(effect, semio_plugin::Effect::LoadDocument { ref pack, .. } if !pack.is_empty()));
    dismantle(document);
}

#[test]
fn switch_compares_document_and_materialized_asset_identity() {
    use semio_plugin::{ArtifactView, ConfigView, Effect, HistoryView};
    use crate::config::RasterConfig;
    use semio_s_artifact_raster_raster_model::RasterLayerNode;

    let cases: serde_json::Value = serde_json::from_str(include_str!("fixture.json")).unwrap();
    let example_id = semio_s_artifact_raster_raster_examples::art_raster_demo::ID;
    let config = RasterConfig::default();
    for row in cases.as_array().unwrap() {
        let mut current = example_document(example_id).unwrap();
        match row[1].as_str().unwrap() {
            "none" => {}
            "id" => current.id = "another-document".into(),
            "title" => current.title = Some("Edited title".into()),
            "asset" => current.assets.get_mut("semio-emblem").unwrap().child_id = "different-content".into(),
            "owner" => current.assets.get_mut("semio-emblem").unwrap().local_owner = None,
            "lock" => match &mut current.layers[0] {
                RasterLayerNode::Pixel { locked, .. } => *locked = true,
                _ => panic!("demo starts with a pixel layer"),
            },
            _ => panic!("unknown fixture edit"),
        }
        let outcome = handle(
            &SetActiveExample { example_id: example_id.into() },
            &ArtifactView::new(&current, &HistoryView::empty()),
            &ConfigView { snapshot: &config, window: None },
        );
        dismantle(current);
        let actual = match outcome {
            Ok(emit) => match emit.effects.as_slice() {
                [] => "unchanged",
                [Effect::LoadDocument { pack, .. }] if !pack.is_empty() => "load",
                _ => panic!("unexpected effects"),
            },
            Err(_) => "refused",
        };
        assert_eq!(Some(actual), row[2].as_str(), "{}", row[0]);
    }
}
