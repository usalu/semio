//! 🧬️ Ifc2x3Artifact schema — full artifact state for the `2x3` standard (buildingSMART
//! Coordination View 2.0 era, ISO/PAS 16739:2005 schema). Sibling of `4️⃣4`'s `IfcArtifact`, own
//! distinct schema id `s.stdio.ifc.2x3` so the two standards' descriptors never collide in the
//! flat `::semio_framework_schema_registry::register_artifact_schema_descriptor` registry.

use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ifc.2x3")]
pub struct Ifc2x3Artifact {
    #[state(artifact)]
    pub schema: String,
    /// 📦️ The full, lossless generic Part-21 graph, wrapped in this standard's own
    /// [`Ifc2x3Snapshot`] type — the actual persisted state.
    #[state(artifact)]
    #[value(default)]
    pub document: semio_s_artifact_stdio_contract::part21::Part21Document,
    #[state(artifact)]
    #[value(default)]
    pub edm_preamble: Option<Ifc2x3EdmPreamble>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Ifc2x3Artifact {
    fn default() -> Self {
        Self::from_snapshot(Ifc2x3Snapshot::default())
    }
}

impl Ifc2x3Artifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> Ifc2x3Snapshot {
        Ifc2x3Snapshot { schema: self.schema.clone(), document: self.document.clone(), edm_preamble: self.edm_preamble.clone() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: Ifc2x3Snapshot) -> Self {
        Self { schema: snapshot.schema, document: snapshot.document, edm_preamble: snapshot.edm_preamble }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: Ifc2x3Snapshot) {
        self.schema = snapshot.schema;
        self.document = snapshot.document;
        self.edm_preamble = snapshot.edm_preamble;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ifc2x3_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.ifc.2x3",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
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
/// `crate::standards::v2x3::engine::empty_ifc2x3_snapshot` through the `engine`
/// barrel shim.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    Ifc2x3Snapshot::default()
}

/// 📄️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the demo
/// `stdio.ifc.2x3` document — a real, minimal IFC2X3 exchange structure (raw HEADER value tuples +
/// two real entities incl. an `IFCOWNERHISTORY` reference chain), matching `4`'s own
/// `demo_ifc_snapshot()` shape but declaring `FILE_SCHEMA(('IFC2X3'))` so `decode_ifc2x3`'s own
/// schema gate accepts it. Fodder for `mutations::demo_mutation_cases()`/`diff::demo_diff_cases()`
/// and this standard's own `conformance_laws` tests (a non-empty snapshot, unlike the prior
/// `empty_ifc2x3_snapshot()` stub, so every recognizer/walk law actually exercises real content).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_ifc2x3_snapshot() -> Ifc2x3Snapshot {
    use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Header, Part21Instance, Part21Value};
    let document = Part21Document {
        header: Part21Header {
            file_description: vec![Part21Value::List(vec![Part21Value::Str(String::new())]), Part21Value::Str("2;1".into())],
            file_name: vec![
                Part21Value::Str("semio.ifc".into()),
                Part21Value::Str("2026-08-11T00:00:00".into()),
                Part21Value::List(vec![Part21Value::Str("Ueli".into())]),
                Part21Value::List(vec![Part21Value::Str("semio".into())]),
                Part21Value::Str("semio".into()),
                Part21Value::Str("".into()),
                Part21Value::Str("".into()),
            ],
            file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])],
        },
        instances: vec![
            Part21Instance { id: 1, entities: vec![("IFCPROJECT".into(), vec![Part21Value::Str("gid-project".into()), Part21Value::Ref(2), Part21Value::Str("Demo Project".into())])] },
            Part21Instance { id: 2, entities: vec![("IFCOWNERHISTORY".into(), vec![Part21Value::Unset, Part21Value::Int(0)])] },
        ],
    };

    Ifc2x3Snapshot { schema: crate::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: None }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ **Deliberately left imperative and callable** (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-
/// APP-STATE-MACHINES, per the ticket's own explicit instruction: "leave ifc's registration
/// alone" — `ArtifactDeclaration` has exactly one `.schema()`/`.document_codec()` slot and
/// cannot hold both `4`'s and `2x3`'s independent descriptors/codecs at once, see the artifact
/// root `🦀️.rs`'s own doc comment). Only physically dissolved out of `⚙️engine`; reached
/// as `crate::standards::v2x3::engine::register()` through the `engine` barrel
/// shim, which is exactly the path `🦀️.rs`'s root `ifc::engine::register()` override calls
/// explicitly (alongside `v4::engine::register()`).
///
/// Registers this standard's schema descriptor, document codec, 5-role `LanguageSpec`s, and (via
/// each real subset's own composer) its `SubsetValidator`s. Does NOT call the artifact-level
/// `ifc::composer::register()` (that union is already invoked once from `4`'s own
/// `engine::register()`, extended by this ticket to also union `v2x3::composer::entries()` —
/// calling it a second time here would be a redundant registration, same reasoning gif's
/// `89a::engine::register` doc comment gives).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {
    ::semio_framework_schema_registry::register_artifact_schema_descriptor(ifc2x3_artifact_schema_descriptor()).expect("schema descriptor publication");
    register_artifact_inferences();
    register_pilot_languages();
    semio_framework_plugin::io::register_native_document_codec(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_plugin::StandardId("2x3"), subset: semio_framework_plugin::SubsetId("*") }, store::ArtifactCodec::bare::<Ifc2x3Snapshot, crate::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(crate::standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA))
        .expect("static Stdio registration must be available and conflict-free");
    // 🛡️ D5's generic validate-on-build hook: registers each real subset's `SubsetValidator` so
    // `io_dispatch`/`wire_artifact_compose` re-check them for free. Each subset's `ComposerEntry`
    // is registered separately via this standard's own `composer::entries()` aggregation.
    crate::standards::v2x3::subsets::cv20::io::register();
    crate::standards::v2x3::subsets::sav::io::register();
    crate::standards::v2x3::subsets::cobie::io::register();
}

/// 💡️ Registers `s.stdio.ifc.2x3.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to the schema descriptor registration above (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_artifact_inferences() {
    ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v2x3::subsets::base::schema::inferences::ifc2x3_artifact_inference_descriptor()).expect("schema descriptor publication");
}

/// 📌️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: 5-role
/// `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per the recipe's json exemplar —
/// `stdio.ifc.2x3`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol`
/// slot stays `None` matching the exemplar's own shape exactly (the 5-role scheme has no dedicated
/// "diff binary" role even though `🔺️diff/💾️binary/📡️.protocol.semio` is a real,
/// conformance-tested file — its binary form is exercised directly by `protocol_walk_law` below,
/// just not wired through a 6th `LanguageRole`), same precedent `4`'s own
/// `register_pilot_languages` established.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register_pilot_languages() {
    use crate::standards::v2x3::subsets::base::schema::{diff, mutations, snapshot};
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3",
        extension: Some("ifc"),
        role: semio_framework_dsl::LanguageRole::Document,
        grammar: Some(crate::standards::v2x3::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v2x3::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v2x3::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v2x3::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.op",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Ops,
        grammar: Some(crate::standards::v2x3::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v2x3::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::standards::v2x3::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v2x3::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.op"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.diff",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Diff,
        grammar: Some(crate::standards::v2x3::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::standards::v2x3::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.diff"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.pack",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v2x3::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v2x3::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.pack"),
    });
    semio_framework_dsl::register_language(semio_framework_dsl::LanguageSpec {
        id: "stdio.ifc.2x3.spr",
        extension: None,
        role: semio_framework_dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::standards::v2x3::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::standards::v2x3::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
        hooks: semio_framework_dsl::passthrough_hooks("stdio.ifc.2x3.spr"),
    });
}

// 📌️ `dsl::registry::register_schema_spec` is intentionally NOT called here — `Part21Value` (a
// genuine data-carrying enum) has no `DslField` impl, so no `fn() -> RecordSpec` exists for
// `Ifc2x3Snapshot`/`Ifc2x3Diff` at all (same `register-schema-spec-needs-recordspec` mechanism gap
// `4`'s own `IfcSnapshot`/`IfcDiff` doc comment documents for the isomorphic shape) — filed as a
// `mechanism_gaps` entry rather than fabricating an unrelated spec.
//#endregion 🔖️Register

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3EdmPreamble;
//#endregion 🔁️Re-exports
