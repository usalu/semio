//! 🎹️ GltfComposer (raw/✳️any at 2.0) — analyzer + builder glued. Reads native
//! `stdio.gltf` sources plus its DAG dependencies: json (`.gltf` text carried as `stdio.json`) and
//! binary (raw `.glb` container bytes) -- writes one `stdio.gltf` (2.0/✳️any) snapshot.
//!
//! 🧊️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 gltf/glb merge step 1
//! item 5: `DEP_BINARY` is what "registers `.glb` on the gltf artifact itself" in this codebase's
//! Dialect/ComposerEntry vocabulary -- `register_composer_entries` (see
//! `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`) turns every entry in `reads()` into BOTH an
//! Import row (gltf ← that dialect) and an Export row (that dialect ← gltf), so adding this one
//! dependency makes gltf's registered rows cover both the `.gltf` JSON dialect (via `DEP_JSON`)
//! and the `.glb` binary dialect (via `DEP_BINARY`) of the SAME `s.stdio.gltf@2.0/*` coordinate --
//! there is no separate MIME registry at this layer: every current stdio artifact leaves
//! `ArtifactKindSpec.{export, import}_formats` empty, gltf included, for consistency.

use semio_framework_plugin::{ArtifactComposer, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::standards::v2_0::subsets::any::analyzer::GltfAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };


pub struct GltfComposer;

impl ArtifactComposer for GltfComposer {
    type Snapshot = GltfSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {
        &[DIALECT, DEP_JSON, DEP_BINARY]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
        // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
        // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
        // like binary) that payload IS the same byte/text shape `analyze` already accepts. Binary
        // sources are analyzed with real `.glb`-vs-pack sniffing (see `GltfAnalyzer::analyze`), so
        // a `DEP_BINARY` source carrying raw `.glb` bytes decodes through the exact same path a
        // hand-fed `AnalyzeSource::Binary` would.
        let native: Vec<AnalyzeSource<'_>> = sources
            .iter()
            .filter(|s| s.dialect == DIALECT || s.dialect == DEP_JSON || s.dialect == DEP_BINARY)
            .map(|s| match &s.payload {
                AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
            })
            .collect();
        if native.is_empty() {
            return Err(ComposeError { message: "GltfComposer: no source in a known read dialect".into(), diagnostics: Vec::new() });
        }
        let analysis = GltfAnalyzer::analyze(&native);
        let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
            message: "GltfComposer: analysis produced no snapshot".into(),
            diagnostics: analysis.diagnostics.clone(),
        })?;
        Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
    }
}
