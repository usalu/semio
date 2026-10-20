#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]
//! 🎪 `stdio.step` artifact — stdio reference format.
#[cfg(test)] use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;


use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::schema::mutations::StepMutation;
pub use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot;
use semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::schema::snapshot::STDIO_STEP_DOCUMENT_SCHEMA;




/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("📜️artifact-definition.json");

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}

fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::bare_artifact_codec::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA);
    codec.extension = "step";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.step.v1", artifact: "step", kind: artifact_kind, codec: native_codec }]
}

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "step", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}

//#region 🔖️Declaration
/// 🧩️ Binds this executable root to its sole schema-owned definition.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("step", definition()?, declaration)
}

/// 🔖️ This artifact's declaration: a headless library artifact whose `.composers(...)` are the `ap214` composer union of
/// [`io_registry::composers`] and whose `.subset_validators(...)` register each conformance class's snapshot validator, which the
/// base subset's SQLite codec dispatches through `store::io::validate_registered_snapshot_subset`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    let formats = formats()?;
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_stdio_step_base::standards::v_ap214::subsets::base::schema::step_artifact_schema_descriptor())
        .formats(formats)
        .inferences([semio_s_artifact_stdio_step_inferences::standards::v_ap214::subsets::base::schema::inferences::step_artifact_inference_descriptor()])
        .composers(io_registry::composers())
        .subset_validators(step_subset_validators())
        .languages(pilot_languages())
        .document_codec_bare::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.step", standard: semio_framework_artifact_reference::StandardId("ap214"), subset: semio_framework_artifact_reference::SubsetId("*") })
        .document_codec_bare::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.step", standard: semio_framework_artifact_reference::StandardId("ap214"), subset: semio_framework_artifact_reference::SubsetId("cc1") })
        .document_codec_bare::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.step", standard: semio_framework_artifact_reference::StandardId("ap214"), subset: semio_framework_artifact_reference::SubsetId("cc2") })
        .document_codec_bare::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.step", standard: semio_framework_artifact_reference::StandardId("ap214"), subset: semio_framework_artifact_reference::SubsetId("cc3") })
        .document_codec_bare::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.step", standard: semio_framework_artifact_reference::StandardId("ap214"), subset: semio_framework_artifact_reference::SubsetId("cc4") })
        .document_codec_bare::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.step", standard: semio_framework_artifact_reference::StandardId("ap214"), subset: semio_framework_artifact_reference::SubsetId("cc5") })
        .document_codec_bare::<StepSnapshot, StepMutation>(STDIO_STEP_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.step", standard: semio_framework_artifact_reference::StandardId("ap214"), subset: semio_framework_artifact_reference::SubsetId("cc6") })
        .try_build()
}

/// 🛡️ The six `ap214` conformance-class `SubsetValidator`s, combined — see `declaration()`'s own
/// doc for why this exists as one owned slice instead of six separate `register_subset_validator`
/// calls.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn step_subset_validators() -> &'static [semio_framework_plugin::io::SubsetValidatorEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<semio_framework_plugin::io::SubsetValidatorEntry>> = std::sync::OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            vec![
                store::io::snapshot_subset_validator_entry_of::<semio_s_artifact_stdio_step_cc1::standards::v_ap214::subsets::cc1::io::StepCc1Validator>(),
                store::io::snapshot_subset_validator_entry_of::<semio_s_artifact_stdio_step_cc2::standards::v_ap214::subsets::cc2::io::StepCc2Validator>(),
                store::io::snapshot_subset_validator_entry_of::<semio_s_artifact_stdio_step_cc3::standards::v_ap214::subsets::cc3::io::StepCc3Validator>(),
                store::io::snapshot_subset_validator_entry_of::<semio_s_artifact_stdio_step_cc4::standards::v_ap214::subsets::cc4::io::StepCc4Validator>(),
                store::io::snapshot_subset_validator_entry_of::<semio_s_artifact_stdio_step_cc5::standards::v_ap214::subsets::cc5::io::StepCc5Validator>(),
                store::io::snapshot_subset_validator_entry_of::<semio_s_artifact_stdio_step_cc6::standards::v_ap214::subsets::cc6::io::StepCc6Validator>(),
            ]
        })
        .as_slice()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built
/// once and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, copied
/// verbatim (five `LanguageSpec` rows, one per role) from `crate::standards::
/// v_ap214::engine::register_pilot_languages`'s own `dsl::register_language(...)` call bodies.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.step",
                    extension: Some("step"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.step"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.step.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.step.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.step.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.step.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.step.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_step_snapshot::standards::v_ap214::subsets::base::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.step.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "stdio.step.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_stdio_step_mutations::standards::v_ap214::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("stdio.step.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.step".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Step", "Step"),
        source_format: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_STEP_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use semio_s_artifact_stdio_step_base::standards::v_ap214::subsets::base::io::StepComposer as StepRawAnyComposer;
    use semio_s_artifact_stdio_step_cc1::standards::v_ap214::subsets::cc1::io::StepCc1Composer;
    use semio_s_artifact_stdio_step_cc2::standards::v_ap214::subsets::cc2::io::StepCc2Composer;
    use semio_s_artifact_stdio_step_cc3::standards::v_ap214::subsets::cc3::io::StepCc3Composer;
    use semio_s_artifact_stdio_step_cc4::standards::v_ap214::subsets::cc4::io::StepCc4Composer;
    use semio_s_artifact_stdio_step_cc5::standards::v_ap214::subsets::cc5::io::StepCc5Composer;
    use semio_s_artifact_stdio_step_cc6::standards::v_ap214::subsets::cc6::io::StepCc6Composer;
    use {semio_framework_plugin::composer_entry_of,semio_framework_plugin::io::register_composer_entries,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposedArtifact,semio_framework_plugin::io::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::ErasedComposeSource};
    use std::sync::OnceLock;

    static COMPOSERS: OnceLock<Vec<ComposerEntry>> = OnceLock::new();
    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    /// 🎹️ The `ap214` composer union: the base subset and every conformance class.
    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn composers() -> &'static [ComposerEntry] {
        COMPOSERS
            .get_or_init(|| {
                vec![
                    composer_entry_of::<StepRawAnyComposer>(),
                    composer_entry_of::<StepCc1Composer>(),
                    composer_entry_of::<StepCc2Composer>(),
                    composer_entry_of::<StepCc3Composer>(),
                    composer_entry_of::<StepCc4Composer>(),
                    composer_entry_of::<StepCc5Composer>(),
                    composer_entry_of::<StepCc6Composer>(),
                ]
            })
            .as_slice()
    }

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| composers().iter().collect()).as_slice()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("StepComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        ::semio_framework_async::poll::resolve_ready((entry.compose)(sources))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_composer_entries(composers()).expect("static Stdio registration must be available and conflict-free");
    }
}
//#endregion 🚪️DerivedIoRegistry







#[path = "🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry/🦀️.rs"]
pub mod geometry;

#[cfg(test)]
#[path = "🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod base_sqlite_snapshot_tests;
