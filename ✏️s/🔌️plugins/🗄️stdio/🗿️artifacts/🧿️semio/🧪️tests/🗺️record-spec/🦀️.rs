//! 🗺️ Law: every record-pack subset exports the record spec its binary pack is derived from as one language-agnostic JSON document beside its snapshot schema, so any other implementation can drive the same record document codec from it. The committed document must equal what the derive produces now, apart from its authored `annotations` (which statements fields hold exactly one variant, which floats are single precision — what the record graph cannot say and the committed Rust packs prove); `SEMIO_RECORD_SPEC_WRITE=1` prints the current documents as `RECORD-SPEC|<subset>|<json>` lines instead of asserting.
//! @see ../../🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json

use pack::record::{schema_hash, PackSchemaGraph};
use pack::snapshot::{ArtifactDsl, ArtifactPack};
use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot;
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn document<S: ArtifactDsl + ArtifactPack>() -> serde_json::Value {
    let spec = S::record_spec().expect("record-pack subset exports its record spec");
    let graph = PackSchemaGraph::of(&spec);
    serde_json::json!({
        "schema": "semio.pack.record-spec/v1",
        "envelope": S::envelope_id(),
        "schemaHash": hex(&schema_hash(&spec)),
        "graph": serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_string(&graph.to_json())).expect("graph json"),
    })
}

fn subsets() -> Vec<(&'static str, &'static str, serde_json::Value)> {
    vec![
        ("🎞️animation", "🎞️animation", document::<SemioAnimationSnapshot>()),
        ("🔊️audio", "🔊️audio", document::<SemioAudioSnapshot>()),
        ("🧊️brep", "🧊️brep", document::<SemioBrepSnapshot>()),
        ("📐️cad", "📐️cad", document::<SemioCadSnapshot>()),
        ("📑️document", "📑️document", document::<SemioDocumentSnapshot>()),
        ("🖊️drawing", "🖊️drawing", document::<SemioDrawingSnapshot>()),
        ("🕸️graph", "🕸️graph", document::<SemioGraphSnapshot>()),
        ("🖼️image", "🖼️image", document::<SemioImageSnapshot>()),
        ("🧰️kit", "🧰️kit", document::<SemioKitSnapshot>()),
        ("🔺️mesh", "🔺️mesh", document::<SemioMeshSnapshot>()),
        ("🏛️model", "🏛️model", document::<SemioModelSnapshot>()),
        ("📦️object", "📦️object", document::<SemioObjectSnapshot>()),
        ("📽️presentation", "📽️presentation", document::<SemioPresentationSnapshot>()),
        ("📊️table", "📊️table", document::<SemioTableSnapshot>()),
        ("🔤️text", "🔤️text", document::<SemioTextSnapshot>()),
        ("🔢️value", "🔢️value", document::<SemioValueSnapshot>()),
        ("🎬️video", "🎬️video", document::<SemioVideoSnapshot>()),
    ]
}

fn committed_path(folder: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️v1/🪆️subsets").join(folder).join("🧬️schema/📸️snapshot/🗺️record-spec/🔣️.json")
}

#[test]
fn committed_record_specs_equal_the_derived_record_specs() {
    let write = std::env::var("SEMIO_RECORD_SPEC_WRITE").is_ok_and(|value| value == "1");
    for (name, folder, derived) in subsets() {
        if write {
            println!("RECORD-SPEC|{name}|{}", serde_json::to_string(&derived).expect("document json"));
            continue;
        }
        let path = committed_path(folder);
        let mut committed: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))).expect("committed record spec json");
        committed.as_object_mut().expect("committed record spec object").remove("annotations");
        assert_eq!(committed, derived, "{name}: the committed record spec drifted from the derived one");
    }
}
