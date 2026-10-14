//! 🧬️ DwgArtifact schema — full artifact state.

use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationHistory;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgApplicationInfo;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgAuxiliaryHeader;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgClass;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgDependency;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgIndexedPreview;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgRevisionHistory;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgSummaryInfo;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::DwgTemplate;
use semio_s_artifact_stdio_dwg_snapshot_document::standards::v_ac1024::subsets::any::schema::snapshot::document::header::DwgHeaderVariables;
use semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::DwgLogicalDrawing;
use semio_s_artifact_stdio_dwg_snapshot::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dwg")]
pub struct DwgArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[value(default)]
    pub maintenance_version: u8,
    #[state(artifact)]
    #[value(default)]
    pub codepage: u16,
    #[state(artifact)]
    #[value(default)]
    pub drawing: DwgLogicalDrawing,
    #[state(artifact)]
    #[value(default)]
    pub header: DwgHeaderVariables,
    #[state(artifact)]
    #[value(default)]
    pub classes: Vec<DwgClass>,
    #[state(artifact)]
    #[value(default)]
    pub dependencies: Vec<DwgDependency>,
    #[state(artifact)]
    #[value(default)]
    pub summary: DwgSummaryInfo,
    #[state(artifact)]
    #[value(default)]
    pub application: DwgApplicationInfo,
    #[state(artifact)]
    #[value(default)]
    pub template: DwgTemplate,
    #[state(artifact)]
    #[value(default)]
    pub auxiliary_header: DwgAuxiliaryHeader,
    #[state(artifact)]
    #[value(default)]
    pub revision_history: DwgRevisionHistory,
    #[state(artifact)]
    #[value(default)]
    pub preview: DwgIndexedPreview,
    #[state(artifact)]
    #[value(default)]
    pub application_history: DwgApplicationHistory,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DwgArtifact {
    fn default() -> Self {
        Self::from_snapshot(DwgSnapshot::default())
    }
}

impl DwgArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> DwgSnapshot {
        DwgSnapshot {
            schema: self.schema.clone(),
            version: self.version.clone(),
            maintenance_version: self.maintenance_version,
            codepage: self.codepage,
            drawing: self.drawing.clone(),
            header: self.header.clone(),
            classes: self.classes.clone(),
            dependencies: self.dependencies.clone(),
            summary: self.summary.clone(),
            application: self.application.clone(),
            template: self.template.clone(),
            auxiliary_header: self.auxiliary_header.clone(),
            revision_history: self.revision_history.clone(),
            preview: self.preview.clone(),
            application_history: self.application_history.clone(),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: DwgSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            version: snapshot.version,
            maintenance_version: snapshot.maintenance_version,
            codepage: snapshot.codepage,
            drawing: snapshot.drawing,
            header: snapshot.header,
            classes: snapshot.classes,
            dependencies: snapshot.dependencies,
            summary: snapshot.summary,
            application: snapshot.application,
            template: snapshot.template,
            auxiliary_header: snapshot.auxiliary_header,
            revision_history: snapshot.revision_history,
            preview: snapshot.preview,
            application_history: snapshot.application_history,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: DwgSnapshot) {
        self.schema = snapshot.schema;
        self.version = snapshot.version;
        self.maintenance_version = snapshot.maintenance_version;
        self.codepage = snapshot.codepage;
        self.drawing = snapshot.drawing;
        self.header = snapshot.header;
        self.classes = snapshot.classes;
        self.dependencies = snapshot.dependencies;
        self.summary = snapshot.summary;
        self.application = snapshot.application;
        self.template = snapshot.template;
        self.auxiliary_header = snapshot.auxiliary_header;
        self.revision_history = snapshot.revision_history;
        self.preview = snapshot.preview;
        self.application_history = snapshot.application_history;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.dwg",
        artifact: semio_framework_schema_registry::FacetLeaves {
            rust: semio_s_artifact_stdio_dwg_exports_rust::ARTIFACT,
            typescript: semio_s_artifact_stdio_dwg_exports_typescript::ARTIFACT,
            graphql: semio_s_artifact_stdio_dwg_exports_graphql::ARTIFACT,
            json_schema: semio_s_artifact_stdio_dwg_exports_json::ARTIFACT,
            proto: semio_s_artifact_stdio_dwg_exports_proto::ARTIFACT,
        },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: semio_s_artifact_stdio_dwg_exports_rust::SNAPSHOT,
            typescript: semio_s_artifact_stdio_dwg_exports_typescript::SNAPSHOT,
            graphql: semio_s_artifact_stdio_dwg_exports_graphql::SNAPSHOT,
            json_schema: semio_s_artifact_stdio_dwg_exports_json::SNAPSHOT,
            proto: semio_s_artifact_stdio_dwg_exports_proto::SNAPSHOT,
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: semio_s_artifact_stdio_dwg_exports_rust::DIFF,
            typescript: semio_s_artifact_stdio_dwg_exports_typescript::DIFF,
            graphql: semio_s_artifact_stdio_dwg_exports_graphql::DIFF,
            json_schema: semio_s_artifact_stdio_dwg_exports_json::DIFF,
            proto: semio_s_artifact_stdio_dwg_exports_proto::DIFF,
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: semio_s_artifact_stdio_dwg_exports_rust::MUTATIONS,
            typescript: semio_s_artifact_stdio_dwg_exports_typescript::MUTATIONS,
            graphql: semio_s_artifact_stdio_dwg_exports_graphql::MUTATIONS,
            json_schema: semio_s_artifact_stdio_dwg_exports_json::MUTATIONS,
            proto: semio_s_artifact_stdio_dwg_exports_proto::MUTATIONS,
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers

//#endregion 🔖️DocumentHelpers

//#region 🔖️RegisterSchemaSpecs
/// 📇️ `DwgSnapshot`/`DwgDiff` (ac1024) both derive real `semio_framework_dsl_record_derive::DslRecord`/`dsl::DslDiff` —
/// genuinely callable, same 2-call shape as `stdio.binary`/`stdio.txt`'s own
/// `register_schema_specs`. Per-mutation-variant specs are NOT registered here — no single
/// canonical id exists for a `Mutation` enum's N independently-shaped variants (same documented
/// scope boundary every other pilot's own `register_schema_specs` observes). Dissolved out of
/// `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — one of the ten
/// deliberate imperative `engine::register()`-family calls left in place at the stdio plugin
/// root's own `.setup(crate::standards::v_ac1024::subsets::any::schema::register_schema_specs)`, reached through the
/// root `engine` shim (ac1024-only) and this standard's own `engine` barrel shim.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.dwg", DwgSnapshot::__dsl_spec));
    ::semio_framework_async::poll::resolve_ready(dsl::registry::register_schema_spec("stdio.dwg#diff", semio_s_artifact_stdio_dwg_diff::standards::v_ac1024::subsets::any::schema::diff::DwgDiff::__dsl_spec));
}

#[cfg(target_arch = "wasm32")]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_schema_specs() {}
//#endregion 🔖️RegisterSchemaSpecs
