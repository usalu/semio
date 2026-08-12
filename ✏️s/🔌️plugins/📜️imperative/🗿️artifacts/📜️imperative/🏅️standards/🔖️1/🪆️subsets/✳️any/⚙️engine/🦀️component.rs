//! ⚙️ Imperative artifact — headless compute over the `ImperativeSnapshot` projection (constitutional:
//! engine).

use crate::artifacts::imperative::{Dictionary, ImperativeSnapshot, Path, PathRef, Registry, Step};
use imperative_engine::{compile_to_text, imperative_catalogue_json, imperative_module_registry, contributions_json_from_entries, register_default_imperative_contributions, register_native_imperative_module, sync_imperative_module_contributions, Executor, RunResult};
use std::sync::{Mutex, Once, OnceLock};

//#region 🔖️Bootstrap
/// 🧩️ Default in-process `imperative.module` contribution entries for dev hosts and config defaults.
pub fn default_imperative_contributions_json() -> String {
    static ENTRIES: OnceLock<String> = OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            let entries = vec![
                crate::extensions::effect::imperative_module_contribution(),
                crate::extensions::math::imperative_module_contribution(),
                crate::extensions::text::imperative_module_contribution(),
                crate::extensions::logic::imperative_module_contribution(),
                crate::extensions::control::imperative_module_contribution(),
            ];
            contributions_json_from_entries(&entries)
        })
        .clone()
}

fn bootstrap_imperative_runtime() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        register_native_imperative_module("imperative-extension-core", crate::extensions::effect::register);
        register_native_imperative_module("imperative-extension-math", crate::extensions::math::register);
        register_native_imperative_module("imperative-extension-text", crate::extensions::text::register);
        register_native_imperative_module("imperative-extension-logic", crate::extensions::logic::register);
        register_default_imperative_contributions(default_imperative_contributions_json);
        sync_imperative_module_contributions(&default_imperative_contributions_json());
    });
}
//#endregion 🔖️Bootstrap

//#region 🔖️Register
/// 🗂️ Registers `ImperativeSnapshot`'s pack↔dsl codec under `IMPERATIVE_DOCUMENT_SCHEMA` so
/// `framework/sync`'s folder endpoints and any other schema-string-keyed caller can print/parse
/// imperative documents. Called from the plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::imperative::io_registry::register();

    bootstrap_imperative_runtime();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::imperative::ImperativePlayApp>(crate::artifacts::imperative::IMPERATIVE_DOCUMENT_SCHEMA);
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "imperative.document",
        extension: Some("imperative"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::imperative::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::imperative::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("imperative.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "imperative.imperative.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::imperative::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::imperative::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("imperative.imperative.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "imperative.imperative.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::imperative::diff::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::imperative::diff::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("imperative.imperative.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "imperative.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("imperative.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "imperative.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("imperative.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifacts::imperative::artifact_kind()` already declares (`computation.imperative`, reused
/// verbatim as this port's `kind_id`), plus one extra output port: `result:out`, the imperative path's
/// last `run` scope as a generic data value (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub fn imperative_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::artifacts::imperative::IMPERATIVE_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Imperative },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: Some("computation.imperative".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "computation.imperative".into(), name: "Imperative".into(), dimension: "graph".into(), component_kind: "imperative".into() },
    }
}
//#endregion 🔖️Io

//#region ⚠️ Errors
/// 🚨️ Imperative core's fallible operations.
#[derive(Debug, thiserror::Error)]
pub enum ImperativeCoreError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("unsupported schema: {0}")]
    UnsupportedSchema(String),
    #[error("missing owner")]
    MissingOwner,
    #[error("missing slot")]
    MissingSlot,
    #[error("unknown owner step: {0}")]
    UnknownOwnerStep(String),
    #[error("unknown step: {0}")]
    UnknownStep(String),
}
//#endregion ⚠️ Errors

/// 📄️ The default `imperative` document, handcrafted in the `.imperative` DSL (see `🗣️dsl`) instead of a
/// hand-built Rust literal or a JSON fixture — {@link default_snapshot} is the only way it should be
/// consumed.
pub fn default_snapshot() -> ImperativeSnapshot {
    crate::artifacts::imperative::dsl::parse_dsl(crate::artifacts::imperative::dsl::IMPERATIVE_EXAMPLE_TEXT).expect("📜️default.imperative is a static, hand-authored fixture that must always parse")
}

// #region 🔖️Host
/// 🎛️ Native imperative path host.
pub struct ImperativeHost {
    pub document: ImperativeSnapshot,
    registry: Registry,
    next_serial: u64,
}

impl Default for ImperativeHost {
    fn default() -> Self {
        Self::from_snapshot(default_snapshot())
    }
}

impl ImperativeHost {
    pub fn from_snapshot(document: ImperativeSnapshot) -> Self {
        bootstrap_imperative_runtime();
        Self { document, registry: imperative_module_registry(), next_serial: 100 }
    }

    pub fn load_json(json: &str) -> Result<Self, ImperativeCoreError> {
        let document: ImperativeSnapshot = serde_json::from_str(json)?;
        if document.schema != "imperative.document" {
            return Err(ImperativeCoreError::UnsupportedSchema(document.schema));
        }
        Ok(Self::from_snapshot(document))
    }

    pub fn to_json(&self) -> Result<String, ImperativeCoreError> {
        Ok(serde_json::to_string(&self.document)?)
    }

    pub fn catalogue_json(&self) -> String {
        imperative_catalogue_json(&self.registry)
    }

    fn resolve_path_mut<'a>(&'a mut self, path_ref: &PathRef) -> Result<&'a mut Path, ImperativeCoreError> {
        if path_ref.owner.is_none() && path_ref.slot.is_none() {
            return Ok(&mut self.document.path);
        }
        let owner = path_ref.owner.as_ref().ok_or(ImperativeCoreError::MissingOwner)?;
        let slot = path_ref.slot.as_ref().ok_or(ImperativeCoreError::MissingSlot)?;
        let owner_step = self.document.path.steps.iter_mut().find(|step| step.id == *owner).ok_or_else(|| ImperativeCoreError::UnknownOwnerStep(owner.clone()))?;
        Ok(owner_step.bodies.entry(slot.clone()).or_insert_with(Path::new))
    }

    pub fn add_step(&mut self, kind: &str, index: Option<usize>) -> String {
        self.add_step_at(&PathRef::default(), kind, index).expect("root PathRef always resolves — resolve_path_mut only fails for a non-default owner/slot")
    }

    pub fn add_step_at(&mut self, path_ref: &PathRef, kind: &str, index: Option<usize>) -> Result<String, ImperativeCoreError> {
        self.next_serial += 1;
        let id = format!("step-{}", self.next_serial);
        let step = Step { id: id.clone(), kind: kind.into(), params: Dictionary::new(), bodies: std::collections::BTreeMap::new() };
        let path = self.resolve_path_mut(path_ref)?;
        let insert_at = index.unwrap_or(path.steps.len()).min(path.steps.len());
        path.steps.insert(insert_at, step);
        Ok(id)
    }

    pub fn remove_step(&mut self, id: &str) -> bool {
        self.remove_step_at(&PathRef::default(), id)
    }

    pub fn remove_step_at(&mut self, path_ref: &PathRef, id: &str) -> bool {
        let path = match self.resolve_path_mut(path_ref) {
            Ok(path) => path,
            Err(_) => return false,
        };
        let before = path.steps.len();
        path.steps.retain(|step| step.id != id);
        path.steps.len() != before
    }

    pub fn move_step(&mut self, id: &str, new_index: usize) -> bool {
        self.move_step_at(&PathRef::default(), id, new_index)
    }

    pub fn move_step_at(&mut self, path_ref: &PathRef, id: &str, new_index: usize) -> bool {
        let path = match self.resolve_path_mut(path_ref) {
            Ok(path) => path,
            Err(_) => return false,
        };
        let Some(current) = path.steps.iter().position(|step| step.id == id) else {
            return false;
        };
        let step = path.steps.remove(current);
        let insert_at = new_index.min(path.steps.len());
        path.steps.insert(insert_at, step);
        true
    }

    pub fn set_step_params_json(&mut self, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        self.set_step_params_at(&PathRef::default(), id, json)
    }

    pub fn set_step_params_at(&mut self, path_ref: &PathRef, id: &str, json: &str) -> Result<(), ImperativeCoreError> {
        let params: Dictionary = serde_json::from_str(json)?;
        let path = self.resolve_path_mut(path_ref)?;
        let Some(step) = path.steps.iter_mut().find(|step| step.id == id) else {
            return Err(ImperativeCoreError::UnknownStep(id.into()));
        };
        step.params = params;
        Ok(())
    }

    pub fn run(&self) -> RunResult {
        Executor::new(&self.registry).run(&self.document.path, &crate::artifacts::imperative::seed_dictionary(&self.document.seed))
    }

    pub fn compile_text(&self) -> String {
        compile_to_text(&self.document.path)
    }
}
// #endregion 🔖️Host

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imperative_io_declares_result_out_reusing_the_computation_imperative_kind() {
        let io = imperative_io();
        assert_eq!(io.document_schema, "imperative.document/v1");
        assert_eq!(io.artifact.id, "computation.imperative");
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "result:out");
        assert_eq!(port.kind_id.as_deref(), Some("computation.imperative"));
        assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert!(!port.required);
    }

    #[test]
    fn host_runs_default_snapshot() {
        let host = ImperativeHost::default();
        let result = host.run();
        assert_eq!(result.effects.len(), 2);
        assert!(result.effects.iter().all(|entry| entry.error.is_none()));
    }

    #[test]
    fn host_adds_nested_step_in_control_body() {
        let mut host = ImperativeHost::default();
        let owner = host.add_step("control.if", None);
        let path_ref = PathRef { owner: Some(owner.clone()), slot: Some("then".into()) };
        let nested = host.add_step_at(&path_ref, "log.print", None).expect("add nested");
        assert_eq!(nested, "step-102");
        let owner_step = host.document.path.steps.iter().find(|step| step.id == owner).expect("owner");
        assert_eq!(owner_step.bodies.get("then").map(|path| path.steps.len()), Some(1));
    }

    #[test]
    fn imperative_core_error_messages() {
        assert_eq!(ImperativeCoreError::MissingOwner.to_string(), "missing owner");
        assert_eq!(ImperativeCoreError::MissingSlot.to_string(), "missing slot");
        assert_eq!(ImperativeCoreError::UnsupportedSchema("bad.schema".into()).to_string(), "unsupported schema: bad.schema");
        assert_eq!(ImperativeCoreError::UnknownOwnerStep("step-9".into()).to_string(), "unknown owner step: step-9");
        assert_eq!(ImperativeCoreError::UnknownStep("step-9".into()).to_string(), "unknown step: step-9");
    }

    #[test]
    fn host_load_json_rejects_unsupported_schema() {
        let json = r#"{"schema":"not.imperative","path":{"steps":[]},"seed":{}}"#;
        assert!(matches!(ImperativeHost::load_json(json), Err(ImperativeCoreError::UnsupportedSchema(schema)) if schema == "not.imperative"));
    }

    #[test]
    fn host_load_json_rejects_invalid_json() {
        assert!(matches!(ImperativeHost::load_json("not json"), Err(ImperativeCoreError::Json(_))));
    }

    #[test]
    fn host_load_json_and_to_json_round_trip() {
        let json = ImperativeHost::default().to_json().expect("serializes");
        let host = ImperativeHost::load_json(&json).expect("parses back");
        assert_eq!(host.to_json().expect("serializes again"), json);
    }

    #[test]
    fn host_catalogue_json_is_nonempty() {
        assert!(!ImperativeHost::default().catalogue_json().is_empty());
    }

    #[test]
    fn host_add_step_at_reports_missing_owner_and_slot() {
        let mut host = ImperativeHost::default();
        let missing_owner = PathRef { owner: None, slot: Some("then".into()) };
        assert!(matches!(host.add_step_at(&missing_owner, "log.print", None), Err(ImperativeCoreError::MissingOwner)));
        let missing_slot = PathRef { owner: Some("step-1".into()), slot: None };
        assert!(matches!(host.add_step_at(&missing_slot, "log.print", None), Err(ImperativeCoreError::MissingSlot)));
    }

    #[test]
    fn host_add_step_at_reports_unknown_owner_step() {
        let mut host = ImperativeHost::default();
        let path_ref = PathRef { owner: Some("does-not-exist".into()), slot: Some("then".into()) };
        assert!(matches!(host.add_step_at(&path_ref, "log.print", None), Err(ImperativeCoreError::UnknownOwnerStep(owner)) if owner == "does-not-exist"));
    }

    #[test]
    fn host_add_step_clamps_out_of_range_index() {
        let mut host = ImperativeHost::default();
        let before = host.document.path.steps.len();
        let id = host.add_step("log.print", Some(9999));
        assert_eq!(host.document.path.steps.last().map(|step| &step.id), Some(&id));
        assert_eq!(host.document.path.steps.len(), before + 1);
    }

    #[test]
    fn host_remove_step_false_for_unresolvable_path_ref_and_unknown_id() {
        let mut host = ImperativeHost::default();
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(!host.remove_step_at(&bad_path_ref, "step-1"));
        assert!(!host.remove_step("does-not-exist"));
    }

    #[test]
    fn host_remove_step_true_when_removed() {
        let mut host = ImperativeHost::default();
        assert!(host.remove_step("step-1"));
        assert!(host.document.path.steps.iter().all(|step| step.id != "step-1"));
    }

    #[test]
    fn host_move_step_false_for_unresolvable_path_ref_and_unknown_id() {
        let mut host = ImperativeHost::default();
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(!host.move_step_at(&bad_path_ref, "step-1", 0));
        assert!(!host.move_step("does-not-exist", 0));
    }

    #[test]
    fn host_move_step_true_and_reorders() {
        let mut host = ImperativeHost::default();
        assert!(host.move_step("step-2", 0));
        assert_eq!(host.document.path.steps[0].id, "step-2");
    }

    #[test]
    fn host_set_step_params_at_rejects_invalid_json_and_unknown_step() {
        let mut host = ImperativeHost::default();
        assert!(matches!(host.set_step_params_json("step-1", "not json"), Err(ImperativeCoreError::Json(_))));
        assert!(matches!(host.set_step_params_json("does-not-exist", "{}"), Err(ImperativeCoreError::UnknownStep(id)) if id == "does-not-exist"));
        let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(matches!(host.set_step_params_at(&bad_path_ref, "step-1", "{}"), Err(ImperativeCoreError::UnknownOwnerStep(_))));
    }

    #[test]
    fn host_set_step_params_updates_existing_step() {
        use neural_engine::{Atom, Value};
        let mut host = ImperativeHost::default();
        host.set_step_params_json("step-2", r#"{"message":"updated"}"#).expect("sets params");
        let step = host.document.path.steps.iter().find(|step| step.id == "step-2").expect("step-2 exists");
        assert_eq!(step.params.get("message"), Some(&Value::Atom(Atom::String("updated".into()))));
    }

    #[test]
    fn host_compile_text_contains_step_kinds() {
        let host = ImperativeHost::default();
        let compiled = host.compile_text();
        assert!(compiled.contains("state.set"));
        assert!(compiled.contains("log.print"));
    }
}
//#endregion 🧪️Tests

//#region 🔖️ArtifactEngine
/// 🧬️ UI-independent document engine — owns the artifact; every transition is an `ImperativeMutation`.
pub struct ImperativeEngine {
    artifact: crate::artifacts::imperative::schema::ImperativeArtifact,
    snapshot: crate::artifacts::imperative::ImperativeSnapshot,
}

impl ImperativeEngine {
    pub fn new(snapshot: crate::artifacts::imperative::ImperativeSnapshot) -> Self {
        let artifact = crate::artifacts::imperative::schema::ImperativeArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::imperative::ImperativeSnapshot {
        self.snapshot
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🔖️SchemaRegistry
/// 📌️ Registers the twenty handcrafted schema leaves for `s.imperative.imperative`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::imperative::schema::imperative_artifact_schema_descriptor());
}

/// 💡️ Registers `s.imperative.imperative.inference`'s five handcrafted facet leaves into the
/// OS-wide inference catalog — sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::imperative::standards::v1::subsets::any::schema::inferences::imperative_artifact_inference_descriptor());
}
//#endregion 🔖️SchemaRegistry
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ArtifactBuilder, ComposerEntry, ComposedArtifact, ComposeError, Dialect, StandardId, SubsetId, ErasedComposeSource, IoPayload, IoConfidence, composer_entry_of};
    use crate::artifacts::imperative::standards::v1::subsets::any::schema::ImperativeComposer as ImperativeAnyComposer;
    use crate::artifacts::imperative::standards::v1::subsets::any::schema::ImperativeBuilder as ImperativeAnyBuilder;

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
    const IMPERATIVE_DIALECT: Dialect = Dialect { artifact_kind: "s.imperative", standard: StandardId("1"), subset: SubsetId("*") };
    const IMPERATIVE_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::imperative::ImperativeSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == IMPERATIVE_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => ImperativeAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => ImperativeAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "ImperativeComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == IMPERATIVE_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::imperative::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "ImperativeComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId("*") };
    fn compose_export_csv(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::imperative::io::export::serializers::artifacts::csv::v_rfc4180::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_CSV_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };
    fn compose_export_md(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::imperative::io::export::serializers::artifacts::md::v_commonmark::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_MD_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::artifacts::imperative::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
        Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
    }
    //#endregion 🔖️ExportEntries


    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![
            composer_entry_of::<ImperativeAnyComposer>(),
            ComposerEntry { writes: EXPORT_CSV_DIALECT, reads: &[IMPERATIVE_DIALECT], compose: compose_export_csv },
            ComposerEntry { writes: EXPORT_MD_DIALECT, reads: &[IMPERATIVE_DIALECT], compose: compose_export_md },
            ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[IMPERATIVE_DIALECT], compose: compose_export_json },
        ]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
