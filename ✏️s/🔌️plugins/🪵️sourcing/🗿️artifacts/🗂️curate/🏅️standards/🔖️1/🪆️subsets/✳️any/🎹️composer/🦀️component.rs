//! 🎹️ CurateComposer (1/✳️any) — analyzer + builder glued. Reads native `s.curate` sources
//! plus any of: stdio.glb, stdio.json, stdio.obj, stdio.png, stdio.stl, stdio.txt, stdio.zip. Writes one `s.curate` (1/✳️any) snapshot.

use semio_framework_plugin::{ArtifactComposer, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::curate::CurateSnapshot;
use crate::artifacts::curate::standards::v1::subsets::any::analyzer::CurateAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.curate", standard: StandardId("1"), subset: SubsetId("*") };
const DEP_GLB: Dialect = Dialect { artifact_kind: "s.stdio.glb", standard: StandardId("2.0"), subset: SubsetId("*") };
const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };


pub struct CurateComposer;

impl ArtifactComposer for CurateComposer {
    type Snapshot = CurateSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {
        &[DIALECT, DEP_GLB, DEP_JSON, DEP_OBJ, DEP_PNG, DEP_STL, DEP_TXT, DEP_ZIP]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        for source in sources {
            if source.dialect == DIALECT {
                let native = match &source.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                };
                let analysis = CurateAnalyzer::analyze(&[native]);
                if let Some(snapshot) = analysis.parts.snapshot {
                    return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                }
            }
            if source.dialect == DEP_GLB {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::curate::io::import::deserializers::artifacts::glb::v2_0::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_JSON {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::curate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_OBJ {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::curate::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_PNG {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::curate::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_STL {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::curate::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_TXT {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::curate::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }
            if source.dialect == DEP_ZIP {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::curate::io::import::deserializers::artifacts::zip::v2_0::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }

        }
        Err(ComposeError { message: "CurateComposer: no source in a known read dialect".into(), diagnostics: Vec::new() })
    }
}
