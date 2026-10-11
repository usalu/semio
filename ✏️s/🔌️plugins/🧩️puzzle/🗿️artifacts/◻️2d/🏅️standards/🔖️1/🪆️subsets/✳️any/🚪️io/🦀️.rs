//! 🚪️ IO s.puzzle2d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.json", "stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.dxf", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.txt"]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::Puzzle2dAnalyzer;
    use crate::Puzzle2dSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle.puzzle2d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Puzzle2dComposerComposition;

    impl ArtifactComposition for Puzzle2dComposerComposition {
        type Snapshot = Puzzle2dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_JSON, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = Puzzle2dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "Puzzle2dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1::subsets::any::io::Puzzle2dBuilder as Puzzle2dAnyBuilder;
    use crate::standards::v1::subsets::any::io::Puzzle2dComposer as Puzzle2dAnyComposer;
    use {semio_framework_plugin::composer_entry_of,semio_framework_plugin::ArtifactBuilder,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const PUZZLE2D_DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle.puzzle2d", standard: StandardId("1"), subset: SubsetId("*") };
    const PUZZLE2D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::Puzzle2dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == PUZZLE2D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Puzzle2dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Puzzle2dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Puzzle2dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == PUZZLE2D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Puzzle2dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    const EXPORT_DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };
    fn compose_export_dxf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::dxf::v_r12::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DXF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<Puzzle2dAnyComposer>(),
                    ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_svg },
                    ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_pdf },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_dwg },
                    ComposerEntry { writes: EXPORT_DXF_DIALECT, reads: &[PUZZLE2D_DIALECT], compose: compose_export_dxf },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🕸️BoardDrawing
/// 🎲️ The board as a node-link diagram: circle or rectangle nodes (the viewer's own shape rule and
/// defaults) labelled with their text, edges between node centres, target regions as labelled frames;
/// hidden items are left out. Every page export (svg, pdf, png, dxf, dwg) draws this one picture.
pub fn puzzle2d_board_drawing(snapshot: &crate::Puzzle2dSnapshot) -> semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::{diagram_drawing, SemioDiagram, SemioDiagramFrame, SemioDiagramLink, SemioDiagramNode, SemioDiagramShape};
    let nodes = snapshot
        .nodes
        .iter()
        .filter(|node| node.visible != Some(false))
        .map(|node| {
            let scale = node.scale.unwrap_or(1.0);
            let rectangle = node.shape.as_ref().is_some_and(|shape| shape.eq_str("rectangle")) || (node.shape.is_none() && node.width.is_some());
            let shape = if rectangle { SemioDiagramShape::Rectangle { width: node.width.unwrap_or(48.0) * scale, height: node.height.unwrap_or(48.0) * scale } } else { SemioDiagramShape::Circle { radius: node.radius.unwrap_or(24.0) * scale } };
            SemioDiagramNode { id: node.id.to_string_owner(), x: node.x, y: node.y, shape, label: node.text.as_ref().filter(|text| !text.is_empty()).map(|text|text.to_string_owner()) }
        })
        .collect();
    let links = snapshot.edges.iter().filter(|edge| edge.visible != Some(false)).map(|edge| SemioDiagramLink { from: edge.source.to_string_owner(), to: edge.target.to_string_owner(), label: None }).collect();
    let frames = snapshot.target_regions.iter().filter(|region| !region.hidden).map(|region| SemioDiagramFrame { x: region.x, y: region.y, width: region.width, height: region.height, label: region.label.as_ref().map(|text|text.to_string_owner()) }).collect();
    diagram_drawing(&SemioDiagram { nodes, links, frames })
}
//#endregion 🕸️BoardDrawing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{Puzzle2dDiff, Puzzle2dMutation, Puzzle2dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle2dBuilderConstruction {
        snapshot: Puzzle2dSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Puzzle2dBuilderConstruction {
        type Snapshot = Puzzle2dSnapshot;
        type Mutation = Puzzle2dMutation;
        type Diff = Puzzle2dDiff;
        fn empty() -> Self {
            Self { snapshot: Puzzle2dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self { snapshot: <Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(text)?, diagnostics: Vec::new() })
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <Puzzle2dSnapshot as store::ArtifactPack>::decode_pack(bytes)?, diagnostics: Vec::new() })
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::Puzzle2dSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Puzzle2dParts {
        pub snapshot: Option<Puzzle2dSnapshot>,
    }

    pub struct Puzzle2dAnalyzerAnalysis;

    impl ArtifactAnalysis for Puzzle2dAnalyzerAnalysis {
        type Parts = Puzzle2dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.puzzle.puzzle2d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Puzzle2dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Puzzle2dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Puzzle2dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec Puzzle2dBuilderFacets {
        construction: Puzzle2dBuilderConstruction,
        analysis: Puzzle2dAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::any::io::derived_composition::Puzzle2dComposerComposition,
    }
    builder: Puzzle2dBuilder,
    analyzer: Puzzle2dAnalyzer,
    composer: Puzzle2dComposer,
);
