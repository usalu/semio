//! 📖️ Playbook artifact — the document entity this plugin's app edits.
//!
//! Step/block/expr records live in the shared kernel `playbook` crate; this plugin owns
//! `PlaybookSnapshot`, `PlaybookArtifact`, facet schemas, and app-facing wrappers.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`playbook→C:document,flow`): the inline
//! `steps: Vec<PlaybookStep>` field is replaced by TWO composed CHILD slots — `document` (stdio's
//! `s.stdio.semio`/`document`, a narrative projection: title + per-step Heading/Paragraph) and `flow`
//! (stdio's `s.stdio.semio`/`flow`, the LOSSLESS procedural source of truth: one `FlowNode` per step,
//! its `blocks`/`description` JSON-encoded into params, sequential `FlowEdge`s witnessing step
//! order) — see `🔖️ContentBridge` below.

extern crate semio_framework as semio_framework;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_playbook_demo_tests;
extern crate semio_framework_schema as framework_schema;
use semio_framework_artifact_playbook_playbook as playbook;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use std::sync::Arc;

//#region 🔖️Types
pub use crate::playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookExpr, PlaybookStep, PlaybookVectorField, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DOCUMENT_SCHEMA};
pub use crate::schema::diff::{PlaybookDiff, PlaybookStringList};
pub use crate::schema::mutations::PlaybookMutation;
pub use crate::schema::snapshot::PlaybookSnapshot;
pub use crate::schema::PlaybookArtifact;

pub const PLAYBOOK_ARTIFACT_SCHEMA_ID: &str = "s.playbook.playbook";

/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1/§7.4 — the canonical
/// `(artifact_kind, standard, subset)` coordinate for this artifact's `✳️any` subset, at the ARTIFACT
/// level (not under `✏️editor`/`👁️viewer`) so the viewer can read it without ever importing through
/// the sibling `editor` module. `artifact_kind` matches this file's own `#[artifact_schema(id = …)]`
/// row (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`); `standard`/`subset` match this
/// location on disk — the canonical surface id is `s.playbook.playbook@1/*#editor` /
/// `s.playbook.playbook@1/*#viewer`.
pub const PLAYBOOK_DIALECT: Dialect = Dialect { artifact_kind: "s.playbook.playbook", standard: StandardId("1"), subset: SubsetId::ANY };

/// 📸️ Default persisted playbook document for new stores and demos.
pub fn empty_playbook_snapshot() -> PlaybookSnapshot {
    PlaybookSnapshot::default()
}

/// 🧱️ Flattens all blocks across steps — delegates to the kernel helper.
pub fn flatten_playbook_blocks(snapshot: &PlaybookSnapshot) -> Vec<PlaybookBlock> {
    playbook::flatten_playbook_blocks(&snapshot.as_kernel()).into_iter().cloned().collect()
}
//#endregion 🔖️Types

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle types for the composed `s.stdio.semio` `document`/`flow`
/// documents — playbook's steps now live in these composed children rather than inline on
/// `PlaybookSnapshot`.
pub type PlaybookDocumentChild = store::ArtifactChild<SemioDocumentSnapshot>;
pub type PlaybookFlowChild = store::ArtifactChild<SemioFlowSnapshot>;

/// 🌉 REAL, LOSSLESS converter: steps -> the `flow` child's node/edge graph — the procedural source
/// of truth. Each step becomes one `FlowNode` (`kind = "step"`, `label` = step title); the step's
/// `blocks` (its full ~18-field form-field vocabulary, including nested `condition` trees) are
/// JSON-encoded wholesale into one `blocksJson` param — the same "honest string boundary" flow's own
/// `Widget -> FlowNode` converter (`📓️wave4-reports/flow-report.md`) established for a generic flow
/// DAG's per-node config; `description` becomes its own param, present only when `Some`. Steps are
/// chained via sequential `FlowEdge`s (`kind = "sequence"`) as a redundant procedural witness of
/// document order — `nodes`' own `Vec` order is the actual source read back by
/// [`steps_from_flow_content`], never the edges (a `Vec` already carries order; the edges exist so a
/// flow-graph consumer sees genuine `next`/`prev` connectivity, not just an implicit array position).
pub fn flow_content_snapshot_from_steps(steps: &[PlaybookStep]) -> SemioFlowSnapshot {
    let nodes: Vec<SemioFlowNode> = steps
        .iter()
        .enumerate()
        .map(|(index, step)| {
            let mut params = vec![SemioFlowParam { key: "blocksJson".into(), value: protocol::json::to_json_string(&step.blocks) }];
            if let Some(description) = &step.description {
                params.push(SemioFlowParam { key: "description".into(), value: description.clone() });
            }
            SemioFlowNode { id: step.id.clone(), kind: "step".into(), label: step.title.clone(), params, position: SemioPoint2 { x: index as f64 * 220.0, y: 0.0 } }
        })
        .collect();
    let edges: Vec<SemioFlowEdge> = steps
        .windows(2)
        .map(|pair| SemioFlowEdge { id: format!("seq-{}-{}", pair[0].id, pair[1].id), from: SemioPortRef { node: pair[0].id.clone(), port: "next".into() }, to: SemioPortRef { node: pair[1].id.clone(), port: "prev".into() }, kind: "sequence".into() })
        .collect();
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉 Inverse of [`flow_content_snapshot_from_steps`] — real and lossless: every `PlaybookStep`
/// field (including the full `blocks` vocabulary) round-trips through `blocksJson`/`description`.
pub fn steps_from_flow_content(content: &SemioFlowSnapshot) -> Vec<PlaybookStep> {
    content
        .nodes
        .iter()
        .map(|node| {
            let blocks_json = node.params.iter().find(|param| param.key == "blocksJson").map_or("[]", |param| param.value.as_str());
            let blocks: Vec<PlaybookBlock> = protocol::json::from_json_str(blocks_json).unwrap_or_default();
            let description = node.params.iter().find(|param| param.key == "description").map(|param| param.value.clone());
            PlaybookStep { id: node.id.clone(), title: node.label.clone(), description, blocks }
        })
        .collect()
}

/// 🌉 REAL converter: (title, steps) -> a narrative projection into the `document` child's block
/// tree — one `Heading(1)` for the playbook title (if present), then one `Heading(2)` + optional
/// `Paragraph` per step (title/description). LOSSY BY DESIGN in the reverse direction only: a bare
/// document cannot recover a step's `blocks`/`condition` data (see [`steps_from_document`]'s own doc
/// comment) — `flow` is this data's lossless source of truth, `document` is a read/export companion.
pub fn document_snapshot_from_steps(title: Option<&str>, steps: &[PlaybookStep]) -> SemioDocumentSnapshot {
    let mut blocks = Vec::new();
    if let Some(title) = title {
        blocks.push(DocBlock::Heading { level: 1, style_id: None, runs: vec![DocRun::plain(title)] });
    }
    for step in steps {
        blocks.push(DocBlock::Heading { level: 2, style_id: None, runs: vec![DocRun::plain(step.title.clone())] });
        if let Some(description) = &step.description {
            blocks.push(DocBlock::paragraph(description.clone()));
        }
    }
    SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Vec::new(), images: Vec::new(), blocks }
}

/// 🌉 Inverse of [`document_snapshot_from_steps`] — HONESTLY LOSSY: a `Heading(2)`/`Paragraph` pair
/// recovers only a step's `title`/`description` skeleton, never `blocks`/`condition` (prose carries
/// none of that). Only used when a caller genuinely has nothing but narrative content to start from
/// (e.g. a bare txt/md/pdf import with no procedural side) — every in-app mutation instead reads/
/// writes through the lossless `flow` child via [`steps_from_flow_content`].
pub fn steps_from_document(content: &SemioDocumentSnapshot) -> (Option<String>, Vec<PlaybookStep>) {
    let mut title = None;
    let mut steps: Vec<PlaybookStep> = Vec::new();
    let mut index = 0usize;
    for block in &content.blocks {
        match block {
            DocBlock::Heading { level: 1, runs, .. } if title.is_none() && steps.is_empty() => {
                title = Some(runs.iter().map(|run| run.text.as_str()).collect::<String>());
            }
            DocBlock::Heading { level: 2, runs, .. } => {
                index += 1;
                steps.push(PlaybookStep { id: format!("s{index}"), title: runs.iter().map(|run| run.text.as_str()).collect::<String>(), description: None, blocks: Vec::new() });
            }
            DocBlock::Paragraph { runs, .. } => {
                if let Some(last) = steps.last_mut() {
                    last.description = Some(runs.iter().map(|run| run.text.as_str()).collect::<String>());
                }
            }
            _ => {}
        }
    }
    (title, steps)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the flow content — same `(child_id, target)`
/// for identical `steps`, a different pair once the content actually changes; mirrors writer's
/// `document_child_handle`/flow's `flow_content_child_handle`.
pub fn flow_content_child_handle(steps: &[PlaybookStep]) -> PlaybookFlowChild {
    use std::hash::{Hash, Hasher};
    let snapshot = flow_content_snapshot_from_steps(steps);
    let content_json = protocol::json::to_json_string(&snapshot);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("playbook-flow-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the narrative document projection — same
/// `(child_id, target)` for identical `(title, steps)`.
pub fn document_child_handle(title: Option<&str>, steps: &[PlaybookStep]) -> PlaybookDocumentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = document_snapshot_from_steps(title, steps);
    let content_json = protocol::json::to_json_string(&snapshot);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("playbook-document-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "document".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side working representation of the composed `flow` child's live steps —
/// NEVER persisted, NEVER a durable field on `PlaybookSnapshot` itself (matches the `EngineRep`
/// contract: wholly derived, droppable at any instant, rebuilt from base). Exists because
/// `protocol::MutationKind::diff(&self, base: &PlaybookSnapshot)` — the sole signature every
/// mutation triad's `🔺️diff` leaf builds against — receives only the opaque-handle-bearing `base`,
/// never a live children view, so a persisted content-addressed HANDLE cannot round-trip to real
/// steps within that call. The scene is retained by the exact `PlaybookFlowChild` instance —
/// mirrors `WriterWorkingScene`/`FlowWorkingScene` without a process-global id map.
///
/// ⚠️ **Checked against the real resolver seam before building this** (per this ticket's migration
/// recipe §3): `🔌️plugin/🦀️.rs`'s `ArtifactView::with_children`/`ChildContentView` IS real
/// and IS generically threaded through `VcsArtifactApp`'s `handle`/`render`/`import_media` call
/// sites (`ArtifactView::with_children(snapshot, history, ChildContentView::new(children))`, not
/// `ArtifactView::new`) — traced directly in the framework source, not assumed. Mutation traits do
/// not receive that resolver, so the opaque child handle carries an ephemeral local owner used only
/// while the handle is live in this process. Cloning the handle retains the same immutable owner;
/// minting a new handle attaches a new owner.
///
/// A deserialized handle has no ephemeral owner and therefore resolves to an empty scene until the
/// child store attaches one. This is fail-soft and instance-local: equal ids cannot leak content
/// across documents, sessions, threads, or ABA handle reuse.
#[derive(Clone, Debug, Default)]
pub struct PlaybookWorkingScene {
    pub steps: Vec<PlaybookStep>,
}

/// 📝 Attaches one immutable working scene to this exact flow child owner.
pub fn attach_playbook_steps(handle: &mut PlaybookFlowChild, steps: Vec<PlaybookStep>) {
    handle.set_local_owner(Arc::new(PlaybookWorkingScene { steps }));
}

/// 🧵️ Retains this exact flow child's immutable scene without cloning its rows.
pub fn playbook_working_scene_owner(handle: &PlaybookFlowChild) -> Arc<PlaybookWorkingScene> {
    handle.local_owner::<PlaybookWorkingScene>().unwrap_or_else(|| Arc::new(PlaybookWorkingScene::default()))
}

/// 🔎 Reads an owned scene clone for mutation paths that edit a private next value.
pub fn playbook_working_scene_for_handle(handle: &PlaybookFlowChild) -> PlaybookWorkingScene {
    playbook_working_scene_owner(handle).as_ref().clone()
}

/// 🔎 Reads the current document's live steps off its `flow` child handle — the single read call
/// site every mutation diff/inverse/render path in this plugin uses instead of the old
/// `snapshot.steps` field access.
pub fn playbook_working_scene(snapshot: &PlaybookSnapshot) -> PlaybookWorkingScene {
    playbook_working_scene_for_handle(&snapshot.flow)
}

/// 🔎 Convenience: just the steps (see [`playbook_working_scene`]).
pub fn playbook_steps(snapshot: &PlaybookSnapshot) -> Vec<PlaybookStep> {
    playbook_working_scene(snapshot).steps
}

/// 🏗️ Mints new content-addressed `document`+`flow` handles and attaches the exact flow handle's
/// immutable local working scene in one call.
pub fn playbook_content_handles(title: Option<&str>, steps: Vec<PlaybookStep>) -> (PlaybookDocumentChild, PlaybookFlowChild) {
    let mut flow_handle = flow_content_child_handle(&steps);
    let document_handle = document_child_handle(title, &steps);
    attach_playbook_steps(&mut flow_handle, steps);
    (document_handle, flow_handle)
}

/// 🏗️ Builds a full `PlaybookSnapshot` from literal steps — the standard fixture/import constructor
/// replacing the old 5-field `PlaybookSnapshot { ..., steps }` struct literal now that
/// `document`/`flow` are composed child handles, not a plain field.
pub fn playbook_snapshot_with_steps(schema: &str, id: &str, version: &str, title: Option<String>, steps: Vec<PlaybookStep>) -> PlaybookSnapshot {
    let (document, flow) = playbook_content_handles(title.as_deref(), steps);
    PlaybookSnapshot { schema: schema.into(), id: id.into(), version: version.into(), title, document, flow }
}
//#endregion 🔖️WorkingScene

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. `crate::editor::playbook::config::schema::register_app_schema()` is the
/// one exception, still called from this file's own `.setup()`: it registers the `PlaybookPlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set. Lives at the artifact root, not `⚙️engine` (reloc-g7 revision of that same ticket) —
/// `declaration()` describes the artifact (kind/schema/io/ownership), it is not engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    type CapabilityRow<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], Option<(&'a str, &'a str)>);
    let rows: &[CapabilityRow<'_>] = &[
        ("s.playbook.playbook.standard.v1", "standard", "1", &[], None),
        ("s.playbook.playbook.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.playbook.playbook.schema.artifact", "schema", "s.playbook.playbook", &[("schema", "s.playbook.playbook")], None),
        ("s.playbook.playbook.inference.artifact", "inference", "s.playbook.playbook.inference", &[("schema", "s.playbook.playbook.inference")], None),
        // 🐛️ D2-capability-claim-repairs: `io_registry::entries()` registers SIX composer rows, not
        // five — the five below plus `composer_entry_of::<PlaybookAnyComposer>()` (`🚪️io/🦀️.rs`),
        // whose `writes` is this artifact's own native dialect (`PLAYBOOK_DIALECT`, `s.playbook@1/*`),
        // the same gap class `🗒️note` hit first (see that file's own `definition()` doc comment).
        ("s.playbook.playbook.composer.playbook", "composer", "s.playbook.playbook@1/*", &[("dialect", "s.playbook.playbook@1/*")], None),
        ("s.playbook.playbook.composer.txt", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.playbook.playbook.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.playbook.playbook.composer.docx", "composer", "s.stdio.docx@ecma-376/*", &[("dialect", "s.stdio.docx@ecma-376/*")], None),
        ("s.playbook.playbook.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.playbook.playbook.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.playbook.playbook.grammar.document", "grammar", "playbook.playbook", &[("grammar", "playbook.playbook")], None),
        ("s.playbook.playbook.grammar.op", "grammar", "playbook.playbook.op", &[("grammar", "playbook.playbook.op")], None),
        ("s.playbook.playbook.grammar.diff", "grammar", "playbook.playbook.diff", &[("grammar", "playbook.playbook.diff")], None),
        ("s.playbook.playbook.grammar.pack", "grammar", "playbook.pack", &[("grammar", "playbook.pack")], None),
        ("s.playbook.playbook.grammar.spr", "grammar", "playbook.spr", &[("grammar", "playbook.spr")], None),
        ("s.playbook.playbook.codec.document.v1", "codec", "playbook.playbook:playbook", &[("codec", "playbook.playbook"), ("codec-extension", "17:playbook.playbook:playbook")], None),
        ("s.playbook.playbook.localization.en", "localization", "Playbook", &[], Some(("en", "Playbook"))),
        ("s.playbook.playbook.localization.de", "localization", "Playbook", &[], Some(("de", "Playbook"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.playbook.playbook")?);
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

/// 🌳️ This artifact's declaration tree root (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-
/// RUNTIME`, `terra-descriptors` packet, following the `terra-fleet-trinity-recipe` recipe) —
/// replaces the old `declaration()` (`ArtifactDeclaration::builder(...).schema(...).inferences(...)
/// .composers(...).languages(...).document_codec(...)` chain, deleted outright, no dual channel) as
/// the ONLY registration channel for schema/io/viewer/editor rows. `definition()` (old
/// `ArtifactDefinition`/capability rows, above) is kept per debt D1.
pub fn artifact<A: PlaybookApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.playbook.playbook").expect("canonical playbook kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 📖️ Application variants required to assemble the Playbook artifact.
pub trait PlaybookApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::playbook::PlaybookPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::playbook::PlaybookViewer>>>
{
}

impl<A> PlaybookApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::playbook::PlaybookPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::playbook::PlaybookViewer>>>
{
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Private:
/// `declaration()` above is its only caller (moved here with it from `⚙️engine`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g7 — kept unexported, not widened).
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "playbook.playbook",
                    extension: Some("playbook"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.playbook"),
                },
                dsl::LanguageSpec {
                    id: "playbook.playbook.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.playbook.op"),
                },
                dsl::LanguageSpec {
                    id: "playbook.playbook.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(schema::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("playbook.playbook.diff"),
                },
                dsl::LanguageSpec {
                    id: "playbook.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.pack"),
                },
                dsl::LanguageSpec {
                    id: "playbook.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::playbook::create_playbook_play_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.playbook".into(),
        name: "Playbook".into(),
        source_format: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        component_kind: "playbook".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                pub use component::*;

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
                        pub mod move_block {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀move-block/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀move-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀move-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀move-block/🧪️tests/🧪️rejects-moving-a-block-into-a-missing-step/🦀️.rs"]
                            mod tests_rejects_moving_a_block_into_a_missing_step;
                        }
                        #[path = "."]
                        pub mod move_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-step/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-step/🧪️tests/🧪️no-ops-when-the-step-is-already-at-that-index/🦀️.rs"]
                            mod tests_no_ops_when_the_step_is_already_at_that_index;
                        }
                        #[path = "."]
                        pub mod add_block {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱add-block/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱add-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱add-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱add-block/🧪️tests/🧪️rejects-adding-a-block-to-a-missing-step/🦀️.rs"]
                            mod tests_rejects_adding_a_block_to_a_missing_step;
                        }
                        #[path = "."]
                        pub mod add_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-step/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-step/🧪️tests/🧪️no-ops-on-a-duplicate-step-id/🦀️.rs"]
                            mod tests_no_ops_on_a_duplicate_step_id;
                        }
                        #[path = "."]
                        pub mod remove_block {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-block/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-block/🧪️tests/🧪️rejects-removing-a-block-missing-from-its-step/🦀️.rs"]
                            mod tests_rejects_removing_a_block_missing_from_its_step;
                        }
                        #[path = "."]
                        pub mod remove_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-step/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-step/🧪️tests/🧪️rejects-removing-a-missing-step/🦀️.rs"]
                            mod tests_rejects_removing_a_missing_step;
                        }
                        #[path = "."]
                        pub mod change_title {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️change-title/🧪️tests/🧪️changes-the-playbook-title/🦀️.rs"]
                            mod tests_changes_the_playbook_title;
                        }
                        #[path = "."]
                        pub mod replace_block {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-block/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-block/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-block/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄replace-block/🧪️tests/🧪️no-ops-when-the-block-is-already-identical/🦀️.rs"]
                            mod tests_no_ops_when_the_block_is_already_identical;
                        }
                        #[path = "."]
                        pub mod update_step {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-step/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-step/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-step/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-step/🧪️tests/🧪️no-ops-when-the-header-is-already-current/🦀️.rs"]
                            mod tests_no_ops_when_the_header_is_already_current;
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
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod docx {
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
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
                                pub mod pdf {
                                    #[path = "."]
                                    pub mod v1_4 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                            mod component;
                                            pub use component::*;
                                        }
                                    }
                                }
                                #[path = "."]
                                pub mod docx {
                                    #[path = "."]
                                    pub mod v_ecma_376 {
                                        #[path = "."]
                                        pub mod any {
                                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📜️docx/🔖️ecma-376/✳️any/🦀️.rs"]
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
    pub use crate::standards::v1::subsets::any::schema::mutations::{apply_playbook_mutation, PlaybookMutation};
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
    pub mod playbook {
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

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️add-block/🦀️.rs"]
            pub mod add_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-block/🦀️.rs"]
            pub mod move_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚮️remove-block/🦀️.rs"]
            pub mod remove_block;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/♻️update-playbook/🦀️.rs"]
            pub mod update_playbook;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod builder {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod builder {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod playbook {
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
                    #[path = "."]
                    pub mod steps {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️steps/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
