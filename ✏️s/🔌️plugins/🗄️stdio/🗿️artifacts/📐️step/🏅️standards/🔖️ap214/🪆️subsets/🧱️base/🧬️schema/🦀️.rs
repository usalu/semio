//! 🧬️ StepArtifact schema — full artifact state, mirrors `StepSnapshot`'s own typed HEADER +
//! id-keyed entity graph (never a raw `Part21Document` — same specific-code mandate as the
//! snapshot itself).

use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepEntity;
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepHeader;
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::STDIO_STEP_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.step")]
pub struct StepArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header: StepHeader,
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<StepEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for StepArtifact {
    fn default() -> Self {
        Self::from_snapshot(StepSnapshot::default())
    }
}

impl StepArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> StepSnapshot {
        StepSnapshot { schema: self.schema.clone(), header: self.header.clone(), entities: self.entities.clone() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: StepSnapshot) -> Self {
        Self { schema: snapshot.schema, header: snapshot.header, entities: snapshot.entities }
    }

    /// 🧐️ Derived BrepMesh analyzer view — computed on demand from the typed entity graph via
    /// `StepSnapshot::to_part21_document`, never stored.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn brep_mesh(&self) -> semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::brep::BrepMeshView {
        semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::brep::analyze_brep_mesh(&self.to_snapshot().to_part21_document())
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn step_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.step", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.step",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: "",
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: "",
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: "",
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
/// 🌱 Empty persisted snapshot. Dissolved out of `⚙️engine`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — reached as
/// `crate::standards::v_ap214::subsets::base::schema::empty_step_snapshot` through the `engine` barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_step_snapshot() -> StepSnapshot {
    StepSnapshot::default()
}

/// 📄️ P2-FG1: the demo `stdio.step` document — a real, minimal AP214 exchange structure (typed
/// HEADER triple + two `CARTESIAN_POINT` entities). The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`) and
/// for `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_step_snapshot() -> StepSnapshot {
    use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepEntity as _StepEntity;
    use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepFileDescription;
    use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepFileName;
    use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepFileSchema;
    use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepHeader as _StepHeader;
    use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepValue;
    StepSnapshot {
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        header: _StepHeader {
            file_description: StepFileDescription { description: vec!["".into()], implementation_level: "2;1".into() },
            file_name: StepFileName {
                name: "semio.step".into(),
                timestamp: "2026-08-11T00:00:00".into(),
                author: vec!["Ueli".into()],
                organization: vec!["semio".into()],
                preprocessor_version: "semio".into(),
                originating_system: "".into(),
                authorization: "".into(),
            },
            file_schema: StepFileSchema { schemas: vec!["AUTOMOTIVE_DESIGN".into()] },
        },
        entities: vec![
            _StepEntity { id: 1, name: "CARTESIAN_POINT".into(), args: vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(0.0), StepValue::Real(0.0), StepValue::Real(0.0)])], complex: Vec::new() },
            _StepEntity { id: 2, name: "CARTESIAN_POINT".into(), args: vec![StepValue::String("".into()), StepValue::Aggregate(vec![StepValue::Real(10.0), StepValue::Real(0.0), StepValue::Real(0.0)])], complex: Vec::new() },
        ],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
