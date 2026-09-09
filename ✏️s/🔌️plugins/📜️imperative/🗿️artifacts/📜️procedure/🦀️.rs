//! 📜️ Imperative artifact — the document entity this plugin's app edits: a `Path` of control-flow
//! `Step`s (`state.set`/`log.print`/`control.if`/`control.while`/`math.add`/…), each addressable by a
//! [`PathRef`] for nested `control.*` bodies (drag-and-drop into blocks).
//!
//! `Path`/`Step` are NOT owned here — they live in the shared kernel crate `imperative_engine`
//! (`✏️s/🔨️modules/📜️imperative`, package `semio-s-kernel-imperative`; **do not confuse this kernel crate
//! with this plugin** — same "imperative" name, different crate, different location, a legitimate
//! dependency this plugin has always had). `Dictionary`/`Registry` come from the framework's
//! `neural_engine` kernel. This component re-exports the app-facing surface so every sibling taxonomy
//! node (`🔺️diff`, `🔧️op`, `🗣️dsl`, `📸️snapshot`, `📡️spr`, `⚙️engine`) names one artifact-owned symbol
//! instead of reaching into either kernel path directly.

// 🧩️ 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME D3 — the five `🧩️extensions/*/🦀️.rs`
// files mounted below each carry a `#[cfg(feature = "extension-entry")]` guard on their own
// `extension_exports!` call (this crate's `Cargo.toml` declares no such feature, so it always
// evaluates false here — see any extension's own `Cargo.toml` for the full rationale).
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_procedure_demo_tests;
extern crate semio_framework_schema as framework_schema;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use std::collections::BTreeMap;

/// 🧩️ Built-in operators mounted by the procedure runtime.
#[path = "."]
pub mod extensions {
    #[path = "../../🧩️extensions/🎮️control/🦀️.rs"]
    pub mod control;
    #[path = "../../🧩️extensions/📣️effect/🦀️.rs"]
    pub mod effect;
    #[path = "../../🧩️extensions/🧠️logic/🦀️.rs"]
    pub mod logic;
    #[path = "../../🧩️extensions/🧮️math/🦀️.rs"]
    pub mod math;
    #[path = "../../🧩️extensions/📝️text/🦀️.rs"]
    pub mod text;
}

//#region 🔖️Types
pub use imperative_engine::{Path, Step};
pub use neural_engine::{Dictionary, Registry, Value};

/// 🎯️ This artifact's `✏️editor`/`👁️viewer` surface coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1) — lives at the ARTIFACT level
/// (not under `editor`/`viewer`) specifically so a viewer file can read it without ever importing
/// through the sibling editor module. `artifact_kind` matches the `#[artifact_schema(id = ..)]`
/// this artifact's own schema declares (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`);
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the
/// canonical surface id is `s.imperative.procedure@1/*#editor` / `s.imperative.procedure@1/*#viewer`.
pub const PROCEDURE_DIALECT: semio_framework_plugin::app::Dialect =
    semio_framework_plugin::app::Dialect { artifact_kind: "s.imperative.procedure", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

/// 🌱️ View of a snapshot seed map as a neural [`Dictionary`] for execution — folds entries straight
/// into the dictionary's own `insert` cursor, no JSON round trip needed.
pub fn seed_dictionary(seed: &BTreeMap<String, Value>) -> Dictionary {
    seed.iter().fold(Dictionary::new(), |dictionary, (key, value)| dictionary.insert(key.clone(), value.clone()))
}

/// 🗂️ The `store::ArtifactStore` schema key — deliberately distinct from the snapshot's `schema`
/// field (`"procedure.document"`, the field inside the document itself): this one keys the store envelope.
pub use crate::schema::mutations::ProcedureMutation;

pub use crate::schema::diff::ProcedureDiff;

pub const PROCEDURE_DOCUMENT_SCHEMA: &str = "procedure.document/v1";

pub use crate::schema::snapshot::ProcedureSnapshot;

/// 📍️ Address of a nested step list inside a control step body.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct PathRef {
    #[value(default)]
    pub owner: Option<String>,
    #[value(default)]
    pub slot: Option<String>,
}
//#endregion 🔖️Types

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle types for the two composed stdio subsets this artifact's persisted
/// `path: Path`/`seed: BTreeMap<String, Value>` inline fields were replaced with (ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, `imperative→C:text,flow`). `path` (the ordered/
/// nested `Step` control-flow tree) maps onto `flow`'s id-keyed node/edge graph; `seed` (the
/// initial variable dictionary) maps onto `text`'s run list as ONE literal-JSON run — an honest,
/// documented, non-prose use of the `text` subset (see `text_content_snapshot_from_seed`'s own doc
/// comment for why), the only persisted-content field left once `path` claims `flow`.
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};

pub type ProcedureFlowChild = store::ArtifactChild<SemioFlowSnapshot>;
pub type ProcedureTextChild = store::ArtifactChild<SemioTextSnapshot>;

/// 🌉 REAL bidirectional converter, `Path` → `SemioFlowSnapshot` half (the "ModelBridge" pattern
/// from `📓️wave3-reports/cad-report.md`, also used by `📓️wave4-reports/flow-report.md`). Each
/// top-level `Step` becomes one `FlowNode` (`id`/`kind` = the step's own, `label` = the step id,
/// `position` a simple sequential layout); `step.params` (a `neural_engine::Dictionary`) becomes
/// one `FlowParam` per entry, JSON-encoding each `Value` into flow's own documented "string-valued
/// is the honest boundary" param shape. `Step::bodies` (nested `control.if`/`control.while` scopes)
/// has no flat id-keyed-graph counterpart in the `flow` subset, so — exactly mirroring how the
/// `flow` plugin's own migration JSON-encoded `Widget::Cluster`'s nested tree
/// (`📓️wave4-reports/flow-report.md`) — it is JSON-encoded wholesale into one reserved `__bodies`
/// param: lossless, honestly opaque to any generic flow-subset consumer. `edges` are a purely
/// derived, honestly redundant "next in sequence" view (`kind = "sequence"`) between adjacent
/// siblings; decode never reads them back — step order is recovered from `nodes`' own `Vec` order,
/// which every encode path here preserves (append-only, never reordered independently of `path`).
pub fn flow_content_snapshot_from_path(path: &Path) -> SemioFlowSnapshot {
    let mut nodes = Vec::with_capacity(path.steps.len());
    let mut edges = Vec::new();
    for (index, step) in path.steps.iter().enumerate() {
        let mut params: Vec<FlowParam> = step.params.keys().map(|key| FlowParam { key: key.clone(), value: dsl::os_pack::json::to_json_string(step.params.get(key).expect("key came from Dictionary::keys()")) }).collect();
        if !step.bodies.is_empty() {
            params.push(FlowParam { key: "__bodies".into(), value: dsl::os_pack::json::to_json_string(&step.bodies) });
        }
        nodes.push(FlowNode { id: step.id.clone(), kind: step.kind.clone(), label: step.id.clone(), params, position: SemioPoint2 { x: index as f64 * 160.0, y: 0.0 } });
        if index > 0 {
            let prev = &path.steps[index - 1];
            edges.push(FlowEdge { id: format!("e-{}-{}", prev.id, step.id), from: PortRef { node: prev.id.clone(), port: "out".into() }, to: PortRef { node: step.id.clone(), port: "in".into() }, kind: "sequence".into() });
        }
    }
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉 Inverse of [`flow_content_snapshot_from_path`] — `nodes`' own `Vec` order IS the step order
/// (see that function's doc comment); the reserved `__bodies` param round-trips back into
/// `Step::bodies`, every other param round-trips back into `step.params` via JSON-decode. `edges`
/// are never read (a purely derived view, see above).
pub fn path_from_flow_content_snapshot(snapshot: &SemioFlowSnapshot) -> Path {
    let steps = snapshot
        .nodes
        .iter()
        .map(|node| {
            let mut params = Dictionary::new();
            let mut bodies: BTreeMap<String, Path> = BTreeMap::new();
            for param in &node.params {
                if param.key == "__bodies" {
                    bodies = dsl::os_pack::json::from_json_str(&param.value).unwrap_or_default();
                } else {
                    let value: Value = dsl::os_pack::json::from_json_str(&param.value).unwrap_or(Value::Atom(neural_engine::Atom::Null));
                    params = params.insert(param.key.clone(), value);
                }
            }
            Step { id: node.id.clone(), kind: node.kind.clone(), params, bodies }
        })
        .collect();
    Path { steps }
}

/// 🌉 REAL bidirectional converter, `seed: BTreeMap<String, Value>` → `SemioTextSnapshot` half.
/// `text`'s `SemioTextRun{language, content, marks}` shape is built for prose (BCP-47 language,
/// inline marks); `seed` is an initial-variable dictionary with no natural-language content at
/// all. The honest, lossless boundary chosen here (matching writer's `document_snapshot_from_text`
/// mapping raw text into ONE `DocBlock::Code` leaf): the WHOLE seed map is JSON-encoded into ONE
/// run's `content` (`language`/`marks` unused, always empty) — never split per-key into runs
/// (there is no natural per-key "prose" to split), and an empty seed maps to zero runs so the
/// default snapshot's `runs` stays empty like every other subset's default.
pub fn text_content_snapshot_from_seed(seed: &BTreeMap<String, Value>) -> SemioTextSnapshot {
    let runs = if seed.is_empty() { Vec::new() } else { vec![SemioTextRun { language: String::new(), content: dsl::os_pack::json::to_json_string(seed), marks: Vec::new() }] };
    SemioTextSnapshot { schema: STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.into(), runs }
}

/// 🌉 Inverse of [`text_content_snapshot_from_seed`] — concatenates every run's `content` (the
/// common, lossless case is exactly one, or zero for an empty seed) and JSON-decodes the result;
/// an empty/unparseable join honestly reads back as an empty seed rather than panicking.
pub fn seed_from_text_content_snapshot(snapshot: &SemioTextSnapshot) -> BTreeMap<String, Value> {
    let joined: String = snapshot.runs.iter().map(|run| run.content.as_str()).collect();
    if joined.is_empty() {
        return BTreeMap::new();
    }
    dsl::os_pack::json::from_json_str(&joined).unwrap_or_default()
}

/// 🕸️ Deterministic content-addressed CHILD handle for `flow` — same `(child_id, target)` for an
/// identical `path`, a different pair once the content actually changes; mirrors `writer`'s
/// `document_child_handle`/`flow`'s own `flow_content_child_handle`.
pub fn procedure_flow_child_handle(path: &Path) -> ProcedureFlowChild {
    use std::hash::{Hash, Hasher};
    let snapshot = flow_content_snapshot_from_path(path);
    let content_json = dsl::os_pack::json::to_json_string(&snapshot);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("imperative-flow-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ `seed`'s content-addressed CHILD handle, the `text`-side twin of [`procedure_flow_child_handle`].
pub fn procedure_text_child_handle(seed: &BTreeMap<String, Value>) -> ProcedureTextChild {
    use std::hash::{Hash, Hasher};
    let snapshot = text_content_snapshot_from_seed(seed);
    let content_json = dsl::os_pack::json::to_json_string(&snapshot);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("imperative-text-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "text".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral combined view of the two exact child owners. It is reconstructed on demand and
/// is never persisted or process-global.
pub struct ProcedureWorkingScene {
    pub path: Path,
    pub seed: BTreeMap<String, Value>,
}

#[derive(Clone)]
pub struct ProcedureFlowWorkingData {
    pub path: Path,
}

#[derive(Clone)]
pub struct ProcedureTextWorkingData {
    pub seed: BTreeMap<String, Value>,
}

/// 📝 Transfers a decoded or test-provided program into one exact flow-child owner.
pub fn materialize_procedure_flow(handle: &mut ProcedureFlowChild, path: &Path) {
    handle.set_local_owner(std::sync::Arc::new(ProcedureFlowWorkingData { path: path.clone() }));
}

/// 🔎 Reads only the addressed flow child's owner. A wire-only handle fails soft until the
/// host materializes its child document.
pub fn procedure_flow_for_handle(handle: &ProcedureFlowChild) -> Path {
    handle.local_owner::<ProcedureFlowWorkingData>().map(|data| data.path.clone()).unwrap_or_default()
}

/// 🔎 `seed`-side twin of [`procedure_flow_for_handle`].
pub fn procedure_seed_for_handle(handle: &ProcedureTextChild) -> BTreeMap<String, Value> {
    handle.local_owner::<ProcedureTextWorkingData>().map(|data| data.seed.clone()).unwrap_or_default()
}

/// 🔎 Reads BOTH composed children's live content off a snapshot's two handles — the single read
/// call site every render/mutation-diff/inference/export path in this plugin uses instead of the
/// old direct `.path`/`.seed` field access.
pub fn procedure_working_scene(snapshot: &ProcedureSnapshot) -> ProcedureWorkingScene {
    ProcedureWorkingScene { path: procedure_flow_for_handle(&snapshot.flow), seed: procedure_seed_for_handle(&snapshot.text) }
}

/// 🏗️ Mints a flow child and transfers its program into that exact owner.
pub fn procedure_flow_child_with_owner(path: &Path) -> ProcedureFlowChild {
    let handle = procedure_flow_child_handle(path);
    handle.with_local_owner(std::sync::Arc::new(ProcedureFlowWorkingData { path: path.clone() }))
}

/// 🏗️ `seed`-side twin of [`procedure_flow_child_with_owner`].
pub fn procedure_text_child_with_owner(seed: &BTreeMap<String, Value>) -> ProcedureTextChild {
    let handle = procedure_text_child_handle(seed);
    handle.with_local_owner(std::sync::Arc::new(ProcedureTextWorkingData { seed: seed.clone() }))
}

/// 🏗️ Builds a full [`ProcedureSnapshot`] from literal `Path`/seed content — the standard fixture/
/// import constructor replacing the old 3-field `ProcedureSnapshot { schema, path, seed }` struct
/// literal now that `flow`/`text` are composed child handles, not plain fields.
pub fn procedure_snapshot_with_content(schema: &str, path: &Path, seed: &BTreeMap<String, Value>) -> ProcedureSnapshot {
    ProcedureSnapshot { schema: schema.into(), flow: procedure_flow_child_with_owner(path), text: procedure_text_child_with_owner(seed) }
}

/// 📸️ A sparse `ProcedureDiff` that whole-handle-replaces `flow` from a fully computed `Path` —
/// composed children are opaque, so a diff never edits a sub-slice, only mints a whole replacement
/// (the "mint+cache whole handle, never apply-then-capture" pattern `writer`'s `diff_set_text`/
/// `flow`'s `diff_replace_content` both establish). `text`/`seed` is left untouched (`None`) since
/// no mutation triad in this plugin edits `seed` — it is write-once at document construction.
pub fn diff_replace_flow(path: &Path) -> ProcedureDiff {
    ProcedureDiff { flow: Some(procedure_flow_child_with_owner(path)), ..Default::default() }
}
//#endregion 🔖️WorkingScene

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from a
/// plugin `.setup()` callback. `bootstrap_imperative_runtime()` runs here too, NOT as a §6 registrar
/// (`register_language`/`register_artifact_schema_descriptor`/… all ARE §6 and now live in the builder
/// chain below) but as this artifact's OWN native-module bootstrap
/// (`register_native_imperative_module` × 4 + `register_default_imperative_contributions`) — it has no
/// `ArtifactDeclaration` field because it isn't one of the census's global SDK registrars, it is
/// imperative's private compute-runtime setup. `Once`-guarded, so calling it eagerly here reproduces
/// the old `register()`'s timing exactly (native modules populated before any `ImperativeHost`/
/// `render()` call can observe an empty registry) without adding a second purpose to `.setup()` — see
/// the plugin root's own doc for why `.setup()` stays narrowed to `register_app_schema` alone. Lives at
/// the artifact root, not `⚙️engine` (reloc-g7 revision of that same ticket) — `declaration()` describes
/// the artifact (kind/schema/io/ownership), it is not engine behaviour.
///
/// 🔄️ UPDATE (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `⚙️engine` is deleted.
/// `bootstrap_imperative_runtime()` and `io_registry` now live in `🚪️io` (multi-caller: this
/// `declaration()`, the app's `🎚️config`, and the app engine's `ImperativeHost::from_snapshot` — an
/// artifact must not depend on its app, so both stayed artifact-side rather than moving to the app),
/// reached below by their full qualified path. `bootstrap_imperative_runtime` stays `pub` (widened from
/// its former `pub(crate)`) since the app engine module now reaches it by the same long qualified path.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.imperative.procedure.standard.v1", "standard", "1", &[], None),
        ("s.imperative.procedure.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.imperative.procedure.schema.artifact", "schema", "s.imperative.procedure", &[("schema", "s.imperative.procedure")], None),
        ("s.imperative.procedure.inference.artifact", "inference", "s.imperative.procedure.inference", &[("schema", "s.imperative.procedure.inference")], None),
        ("s.imperative.procedure.composer.native", "composer", "s.imperative.procedure@1/*", &[("dialect", "s.imperative.procedure@1/*")], None),
        ("s.imperative.procedure.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.imperative.procedure.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.imperative.procedure.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.imperative.procedure.grammar.document", "grammar", "procedure.document", &[("grammar", "procedure.document")], None),
        ("s.imperative.procedure.grammar.op", "grammar", "imperative.procedure.op", &[("grammar", "imperative.procedure.op")], None),
        ("s.imperative.procedure.grammar.diff", "grammar", "imperative.procedure.diff", &[("grammar", "imperative.procedure.diff")], None),
        ("s.imperative.procedure.grammar.pack", "grammar", "procedure.pack", &[("grammar", "procedure.pack")], None),
        ("s.imperative.procedure.grammar.spr", "grammar", "procedure.spr", &[("grammar", "procedure.spr")], None),
        ("s.imperative.procedure.codec.document.v1", "codec", "procedure.document/v1:procedure", &[("codec", "procedure.document/v1"), ("codec-extension", "21:procedure.document/v1:procedure")], None),
        ("s.imperative.procedure.localization.en", "localization", "Procedure", &[], Some(("en", "Procedure"))),
        ("s.imperative.procedure.localization.de", "localization", "Prozedur", &[], Some(("de", "Prozedur"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.imperative.procedure")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    standards::v1::subsets::any::io::bootstrap_imperative_runtime();
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(schema::procedure_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::procedure_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<editor::procedure::ImperativePlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Private:
/// `declaration()` above is its only caller (moved here with it from `⚙️engine`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g7 — kept unexported, not widened).
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "procedure.document",
                    extension: Some("procedure"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("procedure.document"),
                },
                dsl::LanguageSpec {
                    id: "imperative.procedure.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.procedure.op"),
                },
                dsl::LanguageSpec {
                    id: "imperative.procedure.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("imperative.procedure.diff"),
                },
                dsl::LanguageSpec {
                    id: "procedure.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("procedure.pack"),
                },
                dsl::LanguageSpec {
                    id: "procedure.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("procedure.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::procedure::create_imperative_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.procedure".into(),
        name: "Procedure".into(),
        source_format: "procedure.document".into(),
        component_kind: "procedure".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Procedure },
        schema: "procedure.document".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into()],
        import_stdio_kinds: vec!["stdio.csv".into(), "stdio.json".into(), "stdio.md".into()],
    }
}
//#endregion 🔖️ArtifactKind

/// 🧹️ Releases a test document's exact child owners and their nested neural dictionaries.
#[cfg(test)]
pub(crate) fn retire_procedure_fixture(mut snapshot: ProcedureSnapshot) {
    use neural_engine::ColdRetire;
    if let Some(owner) = snapshot.flow.take_local_owner::<ProcedureFlowWorkingData>().expect("flow fixture owner") {
        if let Some(data) = std::sync::Arc::into_inner(owner) {
            let mut paths = vec![data.path];
            while let Some(path) = paths.pop() {
                for Step { params, bodies, .. } in path.steps {
                    params.retire_cold();
                    paths.extend(bodies.into_values());
                }
            }
        }
    }
    if let Some(owner) = snapshot.text.take_local_owner::<ProcedureTextWorkingData>().expect("text fixture owner") {
        if let Some(data) = std::sync::Arc::into_inner(owner) {
            data.seed.retire_cold();
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                        pub mod text;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod topology {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                        pub mod text;
                        pub use text::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                        pub mod binary;
                    }
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                    pub mod operations;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod create_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/🚫️rejects-a-135fff/🦀️.rs"]
                            mod tests_rejects_a_duplicate_step_id_at_the_root_path;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod delete_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/🚫️rejects-a-root-b05b22/🦀️.rs"]
                            mod tests_rejects_a_root_step_id_addressed_inside_a_branch_body;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod reorder_steps {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🧪️tests/🔬️t036/🦀️.rs"]
                            mod tests_warns_that_an_over_clamped_index_leaves_the_tail_step_in_place;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                        #[path = "."]
                        pub mod edit_step_params {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/💾️binary/🦀️.rs"]
                            pub mod binary;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/🔬️t037/🦀️.rs"]
                            mod tests_warns_that_step_1_already_carries_the_requested_params;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/📝️text/🦀️.rs"]
                            pub mod text;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                #[path = "."]
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod csv {
                                    #[path = "."]
                                    pub mod v_rfc4180 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod md {
                                    #[path = "."]
                                    pub mod v_commonmark {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod io {
    pub use super::standards::v1::subsets::any::io::*;
}
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
}
pub mod document_dsl {
    pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
}
pub mod spr {
    pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }
    pub mod text {
        pub use crate::standards::v1::subsets::any::schema::diff::text::*;
    }
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }
    pub mod pack {
        pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
    }
}

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod procedure {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
        pub mod engine;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📍️add-step-at/🦀️.rs"]
            pub mod add_step_at;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️move-step-at/🦀️.rs"]
            pub mod move_step_at;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️remove-step-at/🦀️.rs"]
            pub mod remove_step_at;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏃️run/🦀️.rs"]
            pub mod run;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-step-params/🦀️.rs"]
            pub mod set_step_params;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️set-step-params-at/🦀️.rs"]
            pub mod set_step_params_at;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️main/🦀️.rs"]
                    pub mod main;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📝️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod procedure {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📋️main/🦀️.rs"]
                    pub mod main;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/📝️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }
    }
}
