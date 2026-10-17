//! 📚️ Example `🌱️metabolism` for artifact `stdio.gltf` — a real glTF 2.0 fixture (`🧊️base.glb`,
//! 271 meshes, KHR material extensions declared) decoded through the upgraded engine, not a
//! hand-authored stub. Ticket ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION.

use semio_framework_plugin::ExampleSource;
use semio_framework_test_fixtures::repo_root;
use semio_framework_ui_locale::LocalizedLabel;

/// 🏷️ Stable example id for the navbar picker / `setActiveExample`.
pub const ID: &str = "metabolism";

/// 🗣️ Localized picker label.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Metabolism", "Metabolismus")
}

/// 🖼️ Icon id.
pub const ICON: &str = "cube";

/// 🔏️ The digest sidecar of the real `.glb` fixture (genuine glTF 2.0, KHR material extensions declared, 271 meshes/nodes, 1095 accessors, embedded BIN chunk buffer): its repository-relative name, exact byte length and SHA-256, one per line. The fixture itself is no longer compiled in; the descriptor declares the example by this sidecar and the host verifies the delivered file against the same three values.
pub const BASE_GLB_DIGEST: &[u8] = include_bytes!("../../🖼️assets/🌱️metabolism/🏙️base/🔏️.digest");

/// 📦️ What the digest sidecar of the real `.glb` fixture (genuine glTF 2.0, KHR material extensions declared, 271 meshes/nodes, 1095 accessors, embedded BIN chunk buffer) says: the package asset name, its exact size and its SHA-256.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn base_glb_bytes_digest() -> (&'static str, u64, &'static str) {
    let text = std::str::from_utf8(BASE_GLB_DIGEST).expect("the digest sidecar is UTF-8");
    let mut lines = text.lines();
    let name = lines.next().expect("the digest sidecar names the asset");
    let size_bytes = lines.next().and_then(|line| line.parse().ok()).expect("the digest sidecar sizes the asset");
    let sha256 = lines.next().expect("the digest sidecar hashes the asset");
    (name, size_bytes, sha256)
}

/// 🖼️ The real `.glb` fixture (genuine glTF 2.0, KHR material extensions declared, 271 meshes/nodes, 1095 accessors, embedded BIN chunk buffer) as the host delivered it with the instance open. A host-less test of this crate reads the same repository file instead; any other caller without a delivery is a defect.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn base_glb_bytes() -> &'static [u8] {
    let (name, ..) = base_glb_bytes_digest();
    match semio_framework_plugin::reactor::package_asset(name) {
        Some(bytes) => bytes,
        #[cfg(test)]
        None => repository_file(name),
        #[cfg(not(test))]
        None => panic!("package asset {name} was not delivered by the host"),
    }
}

/// 🧪️ Reads the real `.glb` fixture (genuine glTF 2.0, KHR material extensions declared, 271 meshes/nodes, 1095 accessors, embedded BIN chunk buffer) below the repository root once per process, for tests that have no host to deliver the asset.
#[cfg(test)]
fn repository_file(name: &str) -> &'static [u8] {
    static BYTES: std::sync::OnceLock<&'static [u8]> = std::sync::OnceLock::new();
    BYTES.get_or_init(|| {
        let root = repo_root!();
        Box::leak(std::fs::read(root.join(name)).unwrap_or_else(|error| panic!("{name}: {error}")).into_boxed_slice())
    })
}

/// 🧬️ Decodes [`base_glb_bytes`] via the real upgraded `.glb` container codec -- this is the
/// canonical real snapshot every other consumer of this example (and the fixture tests) works
/// against, never a hand-authored stand-in.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decoded_snapshot() -> semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot {
    semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::decode_glb(base_glb_bytes()).unwrap_or_else(|error| panic!("{ID} example base.glb decodes: {error}"))
}

/// 📄️ Full-fidelity JSON serialization of the real decoded snapshot (document + resolved buffer
/// bytes + source form) -- registered verbatim on the manifest, not a trimmed/synthetic stand-in.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn document_json() -> String {
    semio_framework_pack_json::to_json_string(&decoded_snapshot())
}

/// 📚️ Canonical example source for `App::example_source` — DEFERRED: the manifest row and the descriptor are answered from the digest sidecar alone, and the document is decoded from the delivered fixture only when something asks for it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    ExampleSource::deferred(ID, label(), ICON, ".glb.digest", BASE_GLB_DIGEST, document_json)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
