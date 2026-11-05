//! 🧿️ Every shipped semio subset editor mounts the `.semio` natural-file codec and registers its pack envelope with the semio dispatcher exactly once.

use semio_plugin::ArtifactEditor;
use semio_os_kernel::os_store::semio_format::{dispatch, registered_envelope_keys};

macro_rules! shipped_subsets {
    ($($subset:literal => $editor:ty),+ $(,)?) => {
        const SHIPPED: &[&str] = &[$($subset),+];

        fn codec_of(subset: &str) -> semio_plugin::NaturalFileCodec {
            match subset { $($subset => <$editor as ArtifactEditor>::natural_file_codec().expect("a semio subset editor mounts the natural-file codec"),)+ other => panic!("unshipped subset {other}") }
        }

        fn round_trip(from: &str, to: &str) -> Result<(), String> {
            fn bytes_of<E: ArtifactEditor>() -> Vec<u8> {
                let snapshot = E::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
                E::encode_natural_file(&snapshot).expect("natural export")
            }
            let bytes = match from { $($subset => bytes_of::<$editor>(),)+ other => panic!("unshipped subset {other}") };
            match to { $($subset => <$editor as ArtifactEditor>::decode_natural_file(&bytes).map(|_| ()).map_err(|error| format!("{error:?}")),)+ other => panic!("unshipped subset {other}") }
        }
    };
}

shipped_subsets! {
    "semio" => semio_s_artifact_stdio_semio::editor::SemioAnyEditor,
    "animation" => semio_s_artifact_stdio_semio_animation::editor::SemioAnimationEditor,
    "audio" => semio_s_artifact_stdio_semio_audio::editor::SemioAudioEditor,
    "brep" => semio_s_artifact_stdio_semio_brep::editor::SemioBrepEditor,
    "cad" => semio_s_artifact_stdio_semio_cad::editor::SemioCadEditor,
    "document" => semio_s_artifact_stdio_semio_document::editor::SemioDocumentEditor,
    "drawing" => semio_s_artifact_stdio_semio_drawing::editor::SemioDrawingEditor,
    "flow" => semio_s_artifact_stdio_semio_flow::editor::SemioFlowEditor,
    "graph" => semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor,
    "image" => semio_s_artifact_stdio_semio_image::editor::SemioImageEditor,
    "kit" => semio_s_artifact_stdio_semio_kit::editor::SemioKitEditor,
    "mesh" => semio_s_artifact_stdio_semio_mesh::editor::SemioMeshEditor,
    "model" => semio_s_artifact_stdio_semio_model::editor::SemioModelEditor,
    "object" => semio_s_artifact_stdio_semio_object::editor::SemioObjectEditor,
    "presentation" => semio_s_artifact_stdio_semio_presentation::editor::SemioPresentationEditor,
    "table" => semio_s_artifact_stdio_semio_table::editor::SemioTableEditor,
    "text" => semio_s_artifact_stdio_semio_text::editor::SemioTextEditor,
    "value" => semio_s_artifact_stdio_semio_value::editor::SemioValueEditor,
    "video" => semio_s_artifact_stdio_semio_video::editor::SemioVideoEditor,
}

#[test]
fn every_shipped_semio_subset_is_registered_exactly_once_and_routes_its_own_bytes() {
    assert_eq!(SHIPPED.len(), 19);
    for subset in SHIPPED {
        let codec = codec_of(subset);
        assert_eq!(codec_of(subset), codec, "reading the codec again registers nothing new");
        assert_eq!((codec.extension, codec.media_type, codec.binary), (".semio", "application/vnd.semio", true));
        assert!(codec.format_kind.starts_with("stdio.semio") && codec.format_kind.ends_with("@v1"), "{subset}: {}", codec.format_kind);
    }
    let keys = registered_envelope_keys().expect("the registry is available");
    let distinct: std::collections::BTreeSet<_> = keys.iter().collect();
    assert_eq!(keys.len(), SHIPPED.len(), "{keys:?}");
    assert_eq!(distinct.len(), keys.len(), "no envelope is registered twice");
    for subset in SHIPPED {
        round_trip(subset, subset).unwrap_or_else(|error| panic!("{subset} reopens its own export: {error}"));
    }
    for (from, to) in [("graph", "table"), ("table", "graph"), ("mesh", "semio"), ("semio", "mesh")] {
        assert!(round_trip(from, to).is_err(), "{to} refuses a {from} file");
    }
    let graph_bytes = {
        fn export<E: ArtifactEditor>() -> Vec<u8> {
            E::encode_natural_file(&E::reference_birth().expect("producer")().expect("birth")).expect("export")
        }
        export::<semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor>()
    };
    let (envelope, _) = dispatch(&graph_bytes).expect("the dispatcher finds the graph subset");
    assert_eq!(envelope.envelope_id(), "stdio.semio.graph", "the graph subset's actual ArtifactDsl envelope id is its document schema id");
}
