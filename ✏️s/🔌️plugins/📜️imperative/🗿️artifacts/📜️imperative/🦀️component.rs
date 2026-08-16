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

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Types
pub use imperative_engine::{Path, Step};
pub use neural_engine::{Dictionary, Registry, Value};

/// 🎯️ This artifact's `✏️editor`/`👁️viewer` surface coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1) — lives at the ARTIFACT level
/// (not under `editor`/`viewer`) specifically so a viewer file can read it without ever importing
/// through the sibling editor module. `artifact_kind` matches the `#[artifact_schema(id = ..)]`
/// this artifact's own schema declares (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`);
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the
/// canonical surface id is `s.imperative.imperative@1/*#editor` / `s.imperative.imperative@1/*#viewer`.
pub const IMPERATIVE_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.imperative.imperative", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

/// 🌱️ View of a snapshot seed map as a neural [`Dictionary`] for execution.
pub fn seed_dictionary(seed: &BTreeMap<String, Value>) -> Dictionary {
    serde_json::from_value(serde_json::to_value(seed).expect("seed serializes")).expect("seed is a dictionary")
}

/// 🗂️ The `store::ArtifactStore` schema key — deliberately distinct from the snapshot's `schema`
/// field (`"imperative.document"`, the field inside the document itself): this one keys the store envelope.
pub use crate::artifacts::imperative::schema::mutations::ImperativeMutation;

pub use crate::artifacts::imperative::schema::diff::ImperativeDiff;

pub const IMPERATIVE_DOCUMENT_SCHEMA: &str = "imperative.document/v1";

pub use crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot;

/// 📍️ Address of a nested step list inside a control step body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, FlowParam, PortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};

pub type ImperativeFlowChild = store::ArtifactChild<SemioFlowSnapshot>;
pub type ImperativeTextChild = store::ArtifactChild<SemioTextSnapshot>;

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
        let mut params: Vec<FlowParam> = step.params.keys().map(|key| FlowParam { key: key.clone(), value: serde_json::to_string(step.params.get(key).expect("key came from Dictionary::keys()")).unwrap_or_default() }).collect();
        if !step.bodies.is_empty() {
            params.push(FlowParam { key: "__bodies".into(), value: serde_json::to_string(&step.bodies).unwrap_or_default() });
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
                    bodies = serde_json::from_str(&param.value).unwrap_or_default();
                } else {
                    let value: Value = serde_json::from_str(&param.value).unwrap_or(Value::Atom(neural_engine::Atom::Null));
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
    let runs = if seed.is_empty() { Vec::new() } else { vec![SemioTextRun { language: String::new(), content: serde_json::to_string(seed).unwrap_or_default(), marks: Vec::new() }] };
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
    serde_json::from_str(&joined).unwrap_or_default()
}

/// 🕸️ Deterministic content-addressed CHILD handle for `flow` — same `(child_id, target)` for an
/// identical `path`, a different pair once the content actually changes; mirrors `writer`'s
/// `document_child_handle`/`flow`'s own `flow_content_child_handle`.
pub fn imperative_flow_child_handle(path: &Path) -> ImperativeFlowChild {
    use std::hash::{Hash, Hasher};
    let snapshot = flow_content_snapshot_from_path(path);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("imperative-flow-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ `seed`'s content-addressed CHILD handle, the `text`-side twin of [`imperative_flow_child_handle`].
pub fn imperative_text_child_handle(seed: &BTreeMap<String, Value>) -> ImperativeTextChild {
    use std::hash::{Hash, Hasher};
    let snapshot = text_content_snapshot_from_seed(seed);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
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
/// 🌱 Ephemeral, session-side working representation of BOTH composed children's live content —
/// NEVER persisted, NEVER a durable field on [`ImperativeSnapshot`] itself (matches the `EngineRep`
/// contract: wholly derived, droppable at any instant, rebuilt from base). Exists because no
/// `LinkResolver`/child-dispatch seam is wired into `ArtifactApp::handle` yet (checked directly
/// against `🔌️plugin/🦀️component.rs`, W1-owned — the same standing gap `writer`'s/`flow`'s own
/// reports document) — until one exists, the only way a persisted content-addressed HANDLE can
/// round-trip to real `Path`/seed content within one process is this cache, keyed by each child's
/// own `child_id`, mirroring `WriterWorkingScene`/`FlowWorkingScene`.
///
/// ⚠️ Same documented gap as writer's/flow's own working scenes: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, and a bare `parse_dsl`/`decode_pack` of persisted bytes in a
/// FRESH process recovers only the opaque handles, never the content (no resolver exists yet). Every
/// accessor here fails soft (an empty `Path`/`BTreeMap`), never panics.
pub struct ImperativeWorkingScene {
    pub path: Path,
    pub seed: BTreeMap<String, Value>,
}

thread_local! {
    static IMPERATIVE_FLOW_SCRATCH: RefCell<HashMap<String, Path>> = RefCell::new(HashMap::new());
    static IMPERATIVE_SEED_SCRATCH: RefCell<HashMap<String, BTreeMap<String, Value>>> = RefCell::new(HashMap::new());
}

/// 📝 Seeds the `flow` scratch cache for a handle — call whenever new `Path` content is about to
/// become a document's `flow` field (every mutation-diff/fixture builder in this plugin does, via
/// [`imperative_flow_child_handle_and_cache`]).
pub fn cache_imperative_flow(child_id: &str, path: &Path) {
    IMPERATIVE_FLOW_SCRATCH.with(|cache| cache.borrow_mut().insert(child_id.to_string(), path.clone()));
}

/// 📝 `seed`-side twin of [`cache_imperative_flow`].
pub fn cache_imperative_seed(child_id: &str, seed: &BTreeMap<String, Value>) {
    IMPERATIVE_SEED_SCRATCH.with(|cache| cache.borrow_mut().insert(child_id.to_string(), seed.clone()));
}

/// 🔎 Reads the cached live `Path` for a `flow` child handle — an empty `Path` (never a panic) when
/// nothing has cached it yet (see [`ImperativeWorkingScene`]'s doc comment for why that can happen).
pub fn imperative_flow_for_handle(handle: &ImperativeFlowChild) -> Path {
    IMPERATIVE_FLOW_SCRATCH.with(|cache| cache.borrow().get(&handle.child_id).cloned().unwrap_or_default())
}

/// 🔎 `seed`-side twin of [`imperative_flow_for_handle`].
pub fn imperative_seed_for_handle(handle: &ImperativeTextChild) -> BTreeMap<String, Value> {
    IMPERATIVE_SEED_SCRATCH.with(|cache| cache.borrow().get(&handle.child_id).cloned().unwrap_or_default())
}

/// 🔎 Reads BOTH composed children's live content off a snapshot's two handles — the single read
/// call site every render/mutation-diff/inference/export path in this plugin uses instead of the
/// old direct `.path`/`.seed` field access.
pub fn imperative_working_scene(snapshot: &ImperativeSnapshot) -> ImperativeWorkingScene {
    ImperativeWorkingScene { path: imperative_flow_for_handle(&snapshot.flow), seed: imperative_seed_for_handle(&snapshot.text) }
}

/// 🏗️ Mints a new content-addressed `flow` handle AND seeds the scratch cache with its `Path` in
/// one call — the standard way every mutation-diff/fixture builder in this plugin creates a `flow`
/// field value; never construct a handle without also caching, or [`imperative_flow_for_handle`]
/// will read back empty.
pub fn imperative_flow_child_handle_and_cache(path: &Path) -> ImperativeFlowChild {
    let handle = imperative_flow_child_handle(path);
    cache_imperative_flow(&handle.child_id, path);
    handle
}

/// 🏗️ `seed`-side twin of [`imperative_flow_child_handle_and_cache`].
pub fn imperative_text_child_handle_and_cache(seed: &BTreeMap<String, Value>) -> ImperativeTextChild {
    let handle = imperative_text_child_handle(seed);
    cache_imperative_seed(&handle.child_id, seed);
    handle
}

/// 🏗️ Builds a full [`ImperativeSnapshot`] from literal `Path`/seed content — the standard fixture/
/// import constructor replacing the old 3-field `ImperativeSnapshot { schema, path, seed }` struct
/// literal now that `flow`/`text` are composed child handles, not plain fields.
pub fn imperative_snapshot_with_content(schema: &str, path: &Path, seed: &BTreeMap<String, Value>) -> ImperativeSnapshot {
    ImperativeSnapshot { schema: schema.into(), flow: imperative_flow_child_handle_and_cache(path), text: imperative_text_child_handle_and_cache(seed) }
}

/// 📸️ A sparse `ImperativeDiff` that whole-handle-replaces `flow` from a fully computed `Path` —
/// composed children are opaque, so a diff never edits a sub-slice, only mints a whole replacement
/// (the "mint+cache whole handle, never apply-then-capture" pattern `writer`'s `diff_set_text`/
/// `flow`'s `diff_replace_content` both establish). `text`/`seed` is left untouched (`None`) since
/// no mutation triad in this plugin edits `seed` — it is write-once at document construction.
pub fn diff_replace_flow(path: &Path) -> ImperativeDiff {
    ImperativeDiff { flow: Some(imperative_flow_child_handle_and_cache(path)), ..Default::default() }
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
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.imperative.standard.v1", "standard", "1", &[], None),
        ("s.imperative.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.imperative.schema.artifact", "schema", "s.imperative.imperative", &[("schema", "s.imperative.imperative")], None),
        ("s.imperative.inference.artifact", "inference", "s.imperative.imperative.inference", &[("schema", "s.imperative.imperative.inference")], None),
        ("s.imperative.composer.csv", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.imperative.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.imperative.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.imperative.grammar.document", "grammar", "imperative.document", &[("grammar", "imperative.document")], None),
        ("s.imperative.grammar.op", "grammar", "imperative.imperative.op", &[("grammar", "imperative.imperative.op")], None),
        ("s.imperative.grammar.diff", "grammar", "imperative.imperative.diff", &[("grammar", "imperative.imperative.diff")], None),
        ("s.imperative.grammar.pack", "grammar", "imperative.pack", &[("grammar", "imperative.pack")], None),
        ("s.imperative.grammar.spr", "grammar", "imperative.spr", &[("grammar", "imperative.spr")], None),
        ("s.imperative.codec.document.v1", "codec", "imperative.document/v1:imperative", &[("codec", "imperative.document/v1"), ("extension", "imperative")], None),
        ("s.imperative.localization.en", "localization", "Imperative", &[], Some(("en", "Imperative"))),
        ("s.imperative.localization.de", "localization", "Imperativ", &[], Some(("de", "Imperativ"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.imperative")?);
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
    crate::artifacts::imperative::standards::v1::subsets::any::io::bootstrap_imperative_runtime();
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::imperative::schema::imperative_artifact_schema_descriptor())
        .inferences([crate::artifacts::imperative::standards::v1::subsets::any::schema::inferences::imperative_artifact_inference_descriptor()])
        .composers(crate::artifacts::imperative::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::imperative::ImperativePlayApp>>()
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
                    id: "imperative.document",
                    extension: Some("imperative"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::imperative::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::imperative::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.document"),
                },
                dsl::LanguageSpec {
                    id: "imperative.imperative.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::imperative::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::imperative::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.imperative.op"),
                },
                dsl::LanguageSpec {
                    id: "imperative.imperative.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::imperative::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::imperative::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("imperative.imperative.diff"),
                },
                dsl::LanguageSpec {
                    id: "imperative.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::imperative::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.pack"),
                },
                dsl::LanguageSpec {
                    id: "imperative.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::imperative::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("imperative.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::imperative::create_imperative_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.imperative".into(),
        name: "Imperative".into(),
        source_format: "imperative.document".into(),
        component_kind: "imperative".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Imperative },
        schema: "imperative.document".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.md"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("imperative.document") is deliberately NOT
    /// `IMPERATIVE_DOCUMENT_SCHEMA` ("imperative.document/v1") — the former names the artifact kind in
    /// the OS media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
    /// merge them.
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "imperative.document");
        assert_eq!(IMPERATIVE_DOCUMENT_SCHEMA, "imperative.document/v1");
    }

    #[test]
    fn default_snapshot_is_empty_with_the_bare_schema() {
        let snapshot = ImperativeSnapshot::default();
        assert_eq!(snapshot.schema, "imperative.document");
        let scene = imperative_working_scene(&snapshot);
        assert!(scene.path.steps.is_empty());
        assert!(scene.seed.keys().next().is_none());
    }
}
//#endregion 🧪️Tests
