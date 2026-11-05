//! 🚪️ IO stdio.obj (3.0/📐️geometry) — registration now flows through the `s.stdio.obj`
//! `ArtifactDeclaration` (`crate::declaration`), not per-leaf register().
//!
//! 📐 Documented normal form (codec_retention_law): `encode_obj` always emits, in this fixed
//! order, `mtllib` (if any), the `v`/`vt`/`vn` blocks, then one `f` line per face preceded by
//! whichever of `o`/`g`/`usemtl`/`s` changed since the previous face (transition-only, exactly
//! like the pre-migration encoder), then a trailer of every retained `unknown_statements` entry
//! (comments and any line the codec doesn't otherwise model) in their original relative order.
//! `unknown_statements[].line_index` is informational (the line number AT THE TIME OF THAT
//! DECODE) and is NOT preserved byte-for-byte across a decode→encode cycle — re-encoding always
//! relocates retained comments/unknown lines into the trailer, so a fresh decode of the
//! re-encoded text renumbers them to their new trailer position. Every other field (geometry,
//! `groups`/`objects`/`usemtl`/`smoothing_groups` membership, `mtllib`, and the unknown
//! statements' own text/relative order) is fully retained, and from the SECOND generation
//! onward decode/encode is a true fixed point (see `codec_retention_law` below). A second,
//! narrower limitation: `groups`/`objects`/`usemtl`/`smoothing_groups` reconstruction assumes
//! the "sticky range" shape real parsing always produces (once a name/material/group becomes
//! active it stays active until explicitly changed) — every real `.obj` file has this shape by
//! construction; a hand-built snapshot with genuinely disjoint/non-contiguous membership for the
//! SAME name is a synthetic case this text codec doesn't attempt to round-trip (diff/mutation
//! semantics are unaffected either way, since those operate on the snapshot directly, never
//! through the text codec).
//!
//! ⚠️ An object run that ENDS is closed with a bare `o` line — the exact mirror of the bare `g`
//! this encoder already emitted when a group run ends, and of `decode_obj`'s own
//! `Some("o") => cur_active_object = parts.next()`, which reads an argument-less `o` as "no
//! object from here on". Without it `encode_obj` could not say "these faces belong to no
//! object": every `o` run silently ran to end-of-file, so narrowing an object's membership
//! (`ObjMutation::SetObject { name, faces }` over a strict subset of the faces it held)
//! re-rendered to the ORIGINAL membership — the encoded document was semantically unchanged and
//! the mutation unobservable (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`, subject scenario
//! `mutate-set-object` of `📐️mutate-obj-3-0`; pinned by
//! `an_object_run_that_ends_is_closed_with_a_bare_o`).
pub use semio_s_artifact_stdio_obj_inferences::natural_file::{decode_obj, encode_obj};
use crate::schema::snapshot::{ObjFace, ObjFaceVertex, ObjGroup, ObjNormal, ObjObject, ObjSmoothingRange, ObjTexCoord, ObjUnknownStatement, ObjUsemtlRange, ObjVertex};
use crate::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};


//#region 🔖️DemoSnapshot
/// 📄️ Raw demo Wavefront OBJ text — a two-triangle quad split across two named groups/materials,
/// matching `📚️examples/🎬️demo/🖼️assets/🧊️example.obj` verbatim (this module's own single source
/// of truth for the demo fixture — exercises every statement kind: `mtllib`, `v`/`vt`/`vn`
/// (incl. `w`-omitted forms), `f` (`v/vt/vn` triangles), `o`, `g`, `usemtl`, `s` (both numeric and
/// `off`), a `#` comment, and one genuinely-unrecognized keyword line retained via
/// `unknown_statements`).
const DEMO_OBJ_TEXT: &str = "# stdio.obj demo -- a two-triangle quad split across two named groups/materials\n\
mtllib demo.mtl\n\
v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\n\
vt 0 0\nvt 1 0\nvt 1 1\nvt 0 1\n\
vn 0 0 1\n\
o Quad\ng Front\nusemtl Red\ns 1\n\
f 1/1/1 2/2/1 3/3/1\n\
f 1/1/1 3/3/1 4/4/1\n\
g Back\nusemtl Blue\ns off\n\
f 3/3/1 2/2/1 1/1/1\n\
# trailing note retained verbatim\n\
weird_directive foo bar\n";

/// 📄️ The `demo` example snapshot, parsed once from [`DEMO_OBJ_TEXT`] and stabilized to the
/// SECOND-generation decode/encode fixed point this module's own doc comment documents
/// (`unknown_statements[].line_index` renumbers into the trailer on the first re-encode) — so
/// `print_dsl(demo_obj_snapshot())` is genuinely stable, matching `🗣️.dsl.semio`'s own
/// `fixture_honesty_law` requirement exactly. Same pattern `stdio.txt`'s own
/// `demo_txt_snapshot()`/`stdio.csv`'s own `demo_csv_snapshot()` establish.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_obj_snapshot() -> ObjSnapshot {
    let gen1 = decode_obj(DEMO_OBJ_TEXT).unwrap_or_else(|_| crate::standards::v3_0::subsets::any::schema::empty_obj_snapshot());
    let gen2_text = encode_obj(&gen1);
    decode_obj(&gen2_text).unwrap_or(gen1)
}
//#endregion 🔖️DemoSnapshot

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v3_0::subsets::any::io::ObjAnalyzer;
    use crate::ObjSnapshot;
    use {semio_plugin::io::AnalyzeSource,semio_plugin::ArtifactComposition,semio_plugin::io::ComposeError,semio_plugin::io::ComposeSource,semio_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct ObjComposerComposition;

    impl ArtifactComposition for ObjComposerComposition {
        type Snapshot = ObjSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "ObjComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = ObjAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "ObjComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::standards::v3_0::subsets::any::io::ObjComposer as ObjRawAnyComposer;
    use semio_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<ObjRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;



pub mod derived_construction {
    use crate::{ObjDiff, ObjMutation, ObjSnapshot};
    use semio_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.obj` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct ObjBuilderConstruction {
        snapshot: ObjSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for ObjBuilderConstruction {
        type Snapshot = ObjSnapshot;
        type Mutation = ObjMutation;
        type Diff = ObjDiff;
        fn empty() -> Self {
            Self { snapshot: ObjSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<ObjSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ObjSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff_cold(&diff, &self.snapshot)?;
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
    //#endregion 🔖️Builder
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::ObjSnapshot;
    use {semio_plugin::io::Analysis,semio_plugin::io::AnalyzeSource,semio_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.obj` parts.
    #[derive(Clone, Debug, Default)]
    pub struct ObjParts {
        pub snapshot: Option<ObjSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Sniff
    /// 🔍 OBJ has no magic byte signature (it's plain text) — sniff by scanning the first
    /// ~200 non-blank lines for real Wavefront keyword shapes (`v `/`f ` are the strong
    /// signal; `vt`/`vn`/`o`/`g`/`usemtl`/`s`/`mtllib` are weaker corroborating signals).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn looks_like_obj(text: &str) -> semio_plugin::io::Confidence {
        let mut vertex_lines = 0u32;
        let mut face_lines = 0u32;
        let mut other_tokens = 0u32;
        for line in text.lines().take(200) {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            match line.split_whitespace().next() {
                Some("v") => vertex_lines += 1,
                Some("f") => face_lines += 1,
                Some("vt") | Some("vn") | Some("o") | Some("g") | Some("usemtl") | Some("s") | Some("mtllib") => other_tokens += 1,
                _ => {}
            }
        }
        if vertex_lines > 0 && face_lines > 0 {
            semio_plugin::io::Confidence::High
        } else if vertex_lines > 0 || face_lines > 0 || other_tokens > 0 {
            semio_plugin::io::Confidence::Medium
        } else {
            semio_plugin::io::Confidence::Low
        }
    }
    //#endregion 🔖️Sniff

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.obj` (3.0/📐️geometry) sources.
    pub struct ObjAnalyzerAnalysis;

    impl ArtifactAnalysis for ObjAnalyzerAnalysis {
        type Parts = ObjParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_plugin::io::Confidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    looks_like_obj(body)
                }
                AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                    Ok((_, inner)) => match String::from_utf8(inner) {
                        Ok(text) => looks_like_obj(&text),
                        Err(_) => semio_plugin::io::Confidence::Low,
                    },
                    Err(_) => match std::str::from_utf8(bytes) {
                        Ok(text) => looks_like_obj(text),
                        Err(_) => semio_plugin::io::Confidence::Low,
                    },
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ObjParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ObjSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <ObjSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;

semio_plugin::derive_artifact_facets!(
    pub spec ObjBuilderFacets {
        construction: ObjBuilderConstruction,
        analysis: ObjAnalyzerAnalysis,
        composition: ObjComposerComposition,
    }
    builder: ObjBuilder,
    analyzer: ObjAnalyzer,
    composer: ObjComposer,
);

pub use crate::schema::inferences::scene_geometry;
