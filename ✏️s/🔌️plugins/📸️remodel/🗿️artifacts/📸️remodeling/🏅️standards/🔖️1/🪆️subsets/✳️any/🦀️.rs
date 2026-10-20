//! 🪆️ Subset root for `s.remodel.remodeling@1/*` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
//! MECHANISM design.md §2). Exports `subset() -> SubsetDeclaration`, assembling the `🧬️schema`/
//! `🚪️io`/`👁️viewer`/`✏️editor`/`📚️examples` children — `crate::editor::remodeling`/
//! `crate::viewer::remodeling` stay mounted at the plugin's top-level `editor`/`viewer` modules
//! (`🗒️note`/`🖍️draw` recipe §5 gotcha 1), not here.
//!
//! 📚️ `examples` is the channel that finally reaches `manifest.apps[].examples` and therefore
//! ShellHost's example picker: `project_artifact_declarations` copies this slice onto the EDITOR
//! surface's `App`, which the old `PluginBuilder::.crate::editor::remodeling::<E>(AppDefinition)` call had no field for.

use semio_s_artifact_remodel_remodeling_io::standards::v1::subsets::any::io;
use io::text;
use semio_s_artifact_remodel_remodeling_schema::standards::v1::subsets::any::schema;
use semio_s_artifact_remodel_remodeling_viewer::viewer::remodeling as viewer;
use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};
use semio_framework_plugin::ExampleSource;

/// 📚️ The registry in `✏️editor/📚️examples/🦀️.rs` is the single append-only source; this is only
/// its `&'static` projection.
fn examples() -> &'static [ExampleSource] {
    semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::examples::example_source_slice()
}

fn inference_descriptors() -> &'static [::semio_framework_schema_registry::ArtifactInferenceDescriptor] {
    static DESCRIPTORS: std::sync::OnceLock<Vec<::semio_framework_schema_registry::ArtifactInferenceDescriptor>> = std::sync::OnceLock::new();
    DESCRIPTORS.get_or_init(|| vec![schema::inferences::remodeling_artifact_inference_descriptor()]).as_slice()
}

/// 🧬️ Descriptor for `s.remodel.remodeling`, assembled from the four facets' own format leaves.
pub fn remodeling_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor { id: "s.remodel.remodeling", artifact: schema::ARTIFACT_LEAVES, snapshot: semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::SNAPSHOT_LEAVES, diff: semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::DIFF_LEAVES, mutations: semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::MUTATION_LEAVES }
}

/// 🌳️ `standard "1" / subset "any"`'s complete declaration — the only subset this artifact has.
pub fn subset<PA>() -> SubsetDeclaration<PA>
where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<semio_s_artifact_remodel_remodeling_editor::editor::remodeling::RemodelingPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<semio_s_artifact_remodel_remodeling_viewer::viewer::remodeling::RemodelingViewer>>>,
{
    SubsetDeclaration {
        dialect: semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::REMODELING_DIALECT,
        schema: SchemaDeclaration { descriptor: remodeling_artifact_schema_descriptor(), inferences: inference_descriptors(), inference_services: Vec::new() },
        io: io(),
        viewer: viewer_surface::<viewer::RemodelingViewer, PA>(viewer::create_remodeling_viewer()),
        editor: editor_surface::<semio_s_artifact_remodel_remodeling_editor::editor::remodeling::RemodelingPlayApp, PA>(semio_s_artifact_remodel_remodeling_editor::editor::remodeling::create_remodeling_app()),
        examples: examples(),
    }
}

//#region 🎹️DerivedComposition
/// 🎹️ Native-dialect-only composition facet, bound by `derive_artifact_facets!` in this subset's
/// `🧬️schema/🦀️.rs`. Its nine foreign-format branches are GONE (W6): each called a
/// `📥️import` leaf's `deserialize_bytes`, all of which were cross-type `ArtifactPack` casts. Foreign
/// dialects now reach this subset only through the typed `IoEntry` rows `io()` publishes.
pub mod derived_composition {
    use crate::standards::v1::subsets::any::RemodelingAnalyzer;
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
    use {store::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,store::io::ComposeError,store::io::ComposeSource,store::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel.remodeling", standard: StandardId("1"), subset: SubsetId("*") };

    pub struct RemodelingComposerComposition;

    impl ArtifactComposition for RemodelingComposerComposition {
        type Snapshot = RemodelingSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = RemodelingAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
            }
            Err(ComposeError { message: "RemodelingComposerComposition: no source in this subset's own native dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️NativeComposer
/// 🎹️ The ONE surviving row of the old `ComposerEntry` channel: this subset's own derive-generated
/// native composer, which the OS document-open path still resolves by `IoKey`. The eight
/// `compose_export_*` rows that sat beside it are deleted (see this file's module doc); every foreign
/// hop now lives on the typed `io()` channel below.
pub fn native_composer_entries() -> &'static [semio_framework_plugin::app::ComposerEntry] {
    use crate::standards::v1::subsets::any::RemodelingComposer;
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::app::ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![semio_framework_plugin::composer_entry_of::<RemodelingComposer>()]).as_slice()
}
//#endregion 🚪️NativeComposer

//#region 🔖️IoDeclaration
/// 🚪️ This subset's complete io surface on the framework's `io_mechanism` channel — the five native
/// `dsl::LanguageSpec`s plus one `IoEntry` per registered foreign hop, preflighted and registered by
/// `commit_artifact_declarations` (one `io_register` per subset).
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use semio_s_artifact_remodel_remodeling_write::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use semio_s_artifact_remodel_remodeling_read::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::REMODELING_DIALECT;
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::REMODELING_DOCUMENT_SCHEMA;
    use semio_framework_os_kernel::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    /// 🗣️ The five hand-authored `dsl::LanguageSpec`s this subset carries — `OnceLock` because
    /// `dsl::passthrough_hooks` is not `const fn`. Indices: 0=document 1=op 2=diff 3=pack 4=spr.
    fn languages() -> &'static [semio_framework_dsl::LanguageSpec; 5] {
        static LANGUAGES: OnceLock<[semio_framework_dsl::LanguageSpec; 5]> = OnceLock::new();
        LANGUAGES.get_or_init(|| {
            [
                semio_framework_dsl::LanguageSpec {
                    id: "remodeling.document",
                    extension: Some("remodeling"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("remodeling.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "remodeling.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("remodeling.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "remodeling.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("remodeling.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "remodeling.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("remodeling.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "remodeling.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("remodeling.spr"),
                },
            ]
        })
    }

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<RemodelingSnapshot, export::json::v_rfc8259::any::RemodelingIntoJson>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::json::v_rfc8259::any::JsonIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::txt::v_utf_8::any::RemodelingIntoTxt>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::txt::v_utf_8::any::TxtIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::ply::v1_0::any::RemodelingIntoPly>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::ply::v1_0::any::PlyIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::las::v1_0::any::RemodelingIntoLas>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::las::v1_0::any::LasIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::obj::v3_0::any::RemodelingIntoObj>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::obj::v3_0::any::ObjIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::stl::v_ascii::any::RemodelingIntoStl>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::stl::v_ascii::any::StlIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::gltf::v2_0::any::RemodelingIntoGltf>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::gltf::v2_0::any::GltfIntoRemodeling>(REMODELING_DIALECT),
                    serializer_entry::<RemodelingSnapshot, export::png::v1_2::any::RemodelingIntoPng>(REMODELING_DIALECT),
                    deserializer_entry::<RemodelingSnapshot, import::png::v1_2::any::PngIntoRemodeling>(REMODELING_DIALECT),
                ]
            })
            .as_slice()
    }

    let langs = languages();
    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: Some(&langs[0]), binary: Some(&langs[3]) },
            diff: LanguagePair { text: Some(&langs[2]), binary: None },
            mutations: LanguagePair { text: Some(&langs[1]), binary: Some(&langs[4]) },
            inferences: None,
            codec: store::bare_artifact_codec::<RemodelingSnapshot, RemodelingMutation>(REMODELING_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration

pub mod derived_construction {
    use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
    use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct RemodelingBuilderConstruction {
        snapshot: RemodelingSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for RemodelingBuilderConstruction {
        type Snapshot = RemodelingSnapshot;
        type Mutation = RemodelingMutation;
        type Diff = RemodelingDiff;
        fn empty() -> Self {
            Self { snapshot: RemodelingSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<RemodelingSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <RemodelingMutation as protocol::Mutation<RemodelingSnapshot>>::diff(&mutation, &self.snapshot);
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
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
    use {store::io::Analysis,store::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct RemodelingParts {
        pub snapshot: Option<RemodelingSnapshot>,
    }

    pub struct RemodelingAnalyzerAnalysis;

    impl ArtifactAnalysis for RemodelingAnalyzerAnalysis {
        type Parts = RemodelingParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.remodel.remodeling", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = RemodelingParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <RemodelingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <RemodelingSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes) {
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
    pub spec RemodelingBuilderFacets {
        construction: RemodelingBuilderConstruction,
        analysis: RemodelingAnalyzerAnalysis,
        composition: RemodelingComposerComposition,
    }
    builder: RemodelingBuilder,
    analyzer: RemodelingAnalyzer,
    composer: RemodelingComposer,
);

//#region 🧪️Tests
#[cfg(test)]
#[path = "🚪️io/🧪️tests/🔬️io/🦀️.rs"]
mod io_tests;
//#endregion 🧪️Tests
