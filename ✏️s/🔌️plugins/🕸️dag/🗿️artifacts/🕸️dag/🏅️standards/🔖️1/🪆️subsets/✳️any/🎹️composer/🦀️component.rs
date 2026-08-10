//! 🎹️ DagComposer (1/✳️any) — analyzer + builder glued. Reads native `s.dag` sources
//! plus any of: stdio.csv, stdio.json, stdio.md, stdio.png, stdio.svg, stdio.txt. Writes one `s.dag` (1/✳️any) snapshot.

use semio_framework_plugin::{ArtifactComposer, ArtifactBuilder, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
use crate::artifacts::dag::DagSnapshot;
use crate::artifacts::dag::standards::v1::subsets::any::analyzer::DagAnalyzer;
use semio_framework_plugin::ArtifactAnalyzer as _;

const DIALECT: Dialect = Dialect { artifact_kind: "s.dag", standard: StandardId("1"), subset: SubsetId("*") };
const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
const DEP_MD: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };


pub struct DagComposer;

impl ArtifactComposer for DagComposer {
    type Snapshot = DagSnapshot;
    const WRITES: Dialect = DIALECT;

    fn reads() -> &'static [Dialect] {
        &[DIALECT, DEP_JSON, DEP_MD, DEP_TXT]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        for source in sources {
            if source.dialect == DIALECT {
                let native = match &source.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(*t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(*b),
                };
                let analysis = DagAnalyzer::analyze(&[native]);
                if let Some(snapshot) = analysis.parts.snapshot {
                    return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                }
            }
            if source.dialect == DEP_JSON {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::dag::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_MD {
                let text: Option<String> = match &source.payload {
                    AnalyzeSource::Text(t) => Some(t.to_string()),
                    AnalyzeSource::Binary(b) => std::str::from_utf8(b).ok().map(|s| s.to_string()),
                };
                if let Some(text) = text {
                    if let Ok(snapshot) = crate::artifacts::dag::io::import::deserializers::artifacts::md::v_commonmark::any::deserialize_text(&text) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            if source.dialect == DEP_TXT {
                let bytes: Vec<u8> = match &source.payload {
                    AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                    AnalyzeSource::Binary(b) => b.to_vec(),
                };
                if let Ok(snapshot) = crate::artifacts::dag::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                    return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                }
            }

        }
        Err(ComposeError { message: "DagComposer: no source in a known read dialect".into(), diagnostics: Vec::new() })
    }
}
