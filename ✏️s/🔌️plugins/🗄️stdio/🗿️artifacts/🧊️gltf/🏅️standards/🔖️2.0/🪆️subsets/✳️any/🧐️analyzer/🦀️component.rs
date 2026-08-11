//! 🧐️ GltfAnalyzer (2.0/✳️any) — read-only analysis, successor to the pre-migration
//! GltfDecomposer. Real logic; artifact/standard levels delegate here.
//!
//! 🧊️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 gltf/glb merge step 1 item
//! 6: `sniff`/`analyze` use their argument for real -- a `.glb` container is recognized by its
//! `glTF` magic + version-2 header, a `.gltf` document by parsing as JSON with a top-level
//! `asset.version`. Both dialects land in the SAME `GltfSnapshot` shape (see `⚙️engine`).

use semio_framework_plugin::{ArtifactAnalyzer, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
use crate::artifacts::gltf::GltfSnapshot;

//#region 🔖️Parts
/// 🧩 Analyzed `stdio.gltf` parts.
#[derive(Clone, Debug, Default)]
pub struct GltfParts {
    pub snapshot: Option<GltfSnapshot>,
}
//#endregion 🔖️Parts

//#region 🔖️Sniff
/// 👃️ `.glb` binary container magic: `glTF` + little-endian version `2`.
fn looks_like_glb(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"glTF" && u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) == 2
}

/// 👃️ `.gltf` JSON text: a JSON object whose top-level `asset` object carries a `version` string
/// -- the one field glTF 2.0 §3.9 makes universally mandatory, so this is a real (if cheap) probe
/// rather than a content-blind guess.
fn looks_like_gltf_json(text: &str) -> bool {
    let trimmed = text.trim_start();
    if !trimmed.starts_with('{') {
        return false;
    }
    match serde_json::from_str::<serde_json::Value>(trimmed) {
        Ok(value) => value.get("asset").and_then(|a| a.get("version")).and_then(|v| v.as_str()).is_some(),
        Err(_) => false,
    }
}
//#endregion 🔖️Sniff

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.gltf` (2.0/✳️any) sources.
pub struct GltfAnalyzer;

impl ArtifactAnalyzer for GltfAnalyzer {
    type Parts = GltfParts;
    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        match source {
            AnalyzeSource::Binary(bytes) => {
                if looks_like_glb(bytes) {
                    IoConfidence::High
                } else {
                    IoConfidence::Low
                }
            }
            AnalyzeSource::Text(text) => {
                if looks_like_gltf_json(text) {
                    IoConfidence::High
                } else {
                    IoConfidence::Medium
                }
            }
        }
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let mut parts = GltfParts::default();
        let mut diagnostics = Vec::new();
        let mut confidence = IoConfidence::High;
        for source in sources {
            match source {
                AnalyzeSource::Text(text) => {
                    // A genuine `.gltf` JSON document parses directly through the real codec; only
                    // fall back to the SemioEnvelope-wrapped `ArtifactDsl` preamble form (used by
                    // this crate's own internal store round-trips) when the text isn't bare JSON.
                    let result = if looks_like_gltf_json(text) {
                        crate::artifacts::gltf::engine::parse_gltf_document(text.trim().as_bytes())
                    } else {
                        <GltfSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|e| e.to_string())
                    };
                    match result {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err));
                        }
                    }
                }
                AnalyzeSource::Binary(bytes) => {
                    // A genuine raw `.glb` container decodes directly through the real codec; only
                    // fall back to the SemioEnvelope-wrapped `ArtifactPack` form (this crate's own
                    // internal store round-trip encoding) when the bytes aren't a `.glb` container.
                    let result = if looks_like_glb(bytes) {
                        crate::artifacts::gltf::engine::decode_glb(bytes)
                    } else {
                        <GltfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
                    };
                    match result {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err));
                        }
                    }
                }
            }
        }
        Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
    }
}
//#endregion 🔖️Analyzer

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sniff_recognizes_glb_magic() {
        let mut bytes = vec![b'g', b'l', b'T', b'F'];
        bytes.extend_from_slice(&2u32.to_le_bytes());
        bytes.extend_from_slice(&[0u8; 4]);
        assert_eq!(GltfAnalyzer::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
        assert_eq!(GltfAnalyzer::sniff(&AnalyzeSource::Binary(b"not a glb")), IoConfidence::Low);
    }

    #[test]
    fn sniff_recognizes_gltf_json() {
        assert_eq!(GltfAnalyzer::sniff(&AnalyzeSource::Text(r#"{"asset":{"version":"2.0"}}"#)), IoConfidence::High);
        assert_eq!(GltfAnalyzer::sniff(&AnalyzeSource::Text("not json")), IoConfidence::Medium);
    }

    #[test]
    fn analyze_decodes_real_gltf_json_text_directly() {
        let text = r#"{"asset":{"version":"2.0"},"scenes":[]}"#;
        let analysis = GltfAnalyzer::analyze(&[AnalyzeSource::Text(text)]);
        assert_eq!(analysis.confidence, IoConfidence::High);
        let snap = analysis.parts.snapshot.expect("snapshot");
        assert_eq!(snap.document.asset.version, "2.0");
    }
}
//#endregion 🧪️Tests
