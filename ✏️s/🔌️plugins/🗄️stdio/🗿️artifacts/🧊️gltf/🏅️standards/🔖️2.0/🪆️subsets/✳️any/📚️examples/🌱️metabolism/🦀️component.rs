//! 📚️ Example `🌱️metabolism` for artifact `stdio.gltf` — a real glTF 2.0 fixture (`🧊️base.glb`,
//! 271 meshes, KHR material extensions declared) decoded through the upgraded engine, not a
//! hand-authored stub. Ticket ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "metabolism";

/// 🗣️ Localized picker label.
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Metabolism", "Metabolismus")
}

/// 🖼️ Icon id.
pub const ICON: &str = "cube";

/// 🎒️ Real `.glb` fixture bytes — genuine glTF 2.0, KHR material extensions declared
/// (`extensionsUsed`), 271 meshes/nodes, 1095 accessors, embedded BIN chunk buffer.
pub const BASE_GLB_BYTES: &[u8] = include_bytes!("🖼️assets/🧊️base.glb");

/// 🧬️ Decodes [`BASE_GLB_BYTES`] via the real upgraded `.glb` container codec -- this is the
/// canonical real snapshot every other consumer of this example (and the fixture tests) works
/// against, never a hand-authored stand-in.
pub async fn decoded_snapshot() -> crate::artifacts::gltf::GltfSnapshot {
    crate::artifacts::gltf::engine::decode_glb(BASE_GLB_BYTES).unwrap_or_else(|error| panic!("{ID} example base.glb decodes: {error}"))
}

/// 📄️ Full-fidelity JSON serialization of the real decoded snapshot (document + resolved buffer
/// bytes + source form) -- registered verbatim on the manifest, not a trimmed/synthetic stand-in.
async fn document_json() -> String {
    serde_json::to_string(&decoded_snapshot()).expect("serialize example")
}

/// 📚️ Canonical example source for `App::example_source`.
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), document_json(), ICON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn base_glb_decodes_to_a_nonempty_real_document() {
        let snapshot = decoded_snapshot();
        assert_eq!(snapshot.document.asset.version, "2.0");
        assert!(!snapshot.buffers.is_empty());
    }

    #[test]
    async fn demo_source_nonempty() {
        let source = source();
        assert_eq!(source.id(), ID);
        assert!(!source.document_json().is_empty());
    }
}
