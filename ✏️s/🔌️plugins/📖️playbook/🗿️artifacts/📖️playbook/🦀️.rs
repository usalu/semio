//! 📖️ Playbook artifact — the document entity this plugin's app edits.
//!
//! Step/block/expr records live in the shared kernel `playbook` crate; this plugin owns
//! `PlaybookSnapshot`, `PlaybookArtifact`, facet schemas, and app-facing wrappers.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`playbook→C:document,flow`): the inline
//! `steps: Vec<PlaybookStep>` field is replaced by TWO composed CHILD slots — `document` (stdio's
//! `s.stdio.semio.document`, a narrative projection: title + per-step Heading/Paragraph) and `flow`
//! (stdio's `s.stdio.semio.flow`, the LOSSLESS procedural source of truth: one `FlowNode` per step,
//! its `blocks`/`description` JSON-encoded into params, sequential `FlowEdge`s witnessing step
//! order) — see `🔖️ContentBridge` below.

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocRun, SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{
    FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,
};
use std::sync::Arc;

//#region 🔖️Types
pub use crate::artifacts::playbook::schema::diff::{PlaybookDiff, PlaybookStringList};
pub use crate::artifacts::playbook::schema::mutations::PlaybookMutation;
pub use crate::artifacts::playbook::schema::snapshot::PlaybookSnapshot;
pub use crate::artifacts::playbook::schema::PlaybookArtifact;
pub use crate::playbook::{PlaybookBlock, PlaybookBlockOption, PlaybookExpr, PlaybookStep, PlaybookVectorField, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DOCUMENT_SCHEMA};

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
    crate::playbook::flatten_playbook_blocks(&snapshot.as_kernel()).into_iter().cloned().collect()
}
//#endregion 🔖️Types

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle types for the composed `s.stdio.semio.document`/`s.stdio.semio.flow`
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
            let mut params = vec![SemioFlowParam { key: "blocksJson".into(), value: serde_json::to_string(&step.blocks).unwrap_or_default() }];
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
            let blocks_json = node.params.iter().find(|param| param.key == "blocksJson").map(|param| param.value.as_str()).unwrap_or("[]");
            let blocks: Vec<PlaybookBlock> = serde_json::from_str(blocks_json).unwrap_or_default();
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
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("playbook-flow-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "playbook-flow".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the narrative document projection — same
/// `(child_id, target)` for identical `(title, steps)`.
pub fn document_child_handle(title: Option<&str>, steps: &[PlaybookStep]) -> PlaybookDocumentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = document_snapshot_from_steps(title, steps);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("playbook-document-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "document".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "playbook-document".into(), dialect };
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
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.playbook.standard.v1", "standard", "1", &[], None),
        ("s.playbook.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.playbook.schema.artifact", "schema", "s.playbook.playbook", &[("schema", "s.playbook.playbook")], None),
        ("s.playbook.inference.artifact", "inference", "s.playbook.playbook.inference", &[("schema", "s.playbook.playbook.inference")], None),
        // 🐛️ D2-capability-claim-repairs: `io_registry::entries()` registers SIX composer rows, not
        // five — the five below plus `composer_entry_of::<PlaybookAnyComposer>()` (`🚪️io/🦀️.rs`),
        // whose `writes` is this artifact's own native dialect (`PLAYBOOK_DIALECT`, `s.playbook@1/*`),
        // the same gap class `🗒️note` hit first (see that file's own `definition()` doc comment).
        ("s.playbook.composer.playbook", "composer", "s.playbook@1/*", &[("dialect", "s.playbook@1/*")], None),
        ("s.playbook.composer.txt", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.playbook.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.playbook.composer.docx", "composer", "s.stdio.docx@ecma-376/*", &[("dialect", "s.stdio.docx@ecma-376/*")], None),
        ("s.playbook.composer.md", "composer", "s.stdio.md@commonmark/*", &[("dialect", "s.stdio.md@commonmark/*")], None),
        ("s.playbook.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.playbook.grammar.document", "grammar", "playbook.playbook", &[("grammar", "playbook.playbook")], None),
        ("s.playbook.grammar.op", "grammar", "playbook.playbook.op", &[("grammar", "playbook.playbook.op")], None),
        ("s.playbook.grammar.diff", "grammar", "playbook.playbook.diff", &[("grammar", "playbook.playbook.diff")], None),
        ("s.playbook.grammar.pack", "grammar", "playbook.pack", &[("grammar", "playbook.pack")], None),
        ("s.playbook.grammar.spr", "grammar", "playbook.spr", &[("grammar", "playbook.spr")], None),
        ("s.playbook.codec.document.v1", "codec", "playbook.playbook:playbook", &[("codec", "playbook.playbook"), ("extension", "playbook")], None),
        ("s.playbook.localization.en", "localization", "Playbook", &[], Some(("en", "Playbook"))),
        ("s.playbook.localization.de", "localization", "Playbook", &[], Some(("de", "Playbook"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.playbook")?);
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
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::PlaybookApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.playbook.playbook").expect("canonical playbook kind"), localization: &[], standards: vec![crate::artifacts::playbook::standards::v1::standard()] }
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
                    grammar: Some(crate::artifacts::playbook::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::playbook::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.playbook"),
                },
                dsl::LanguageSpec {
                    id: "playbook.playbook.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::playbook::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::playbook::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.playbook.op"),
                },
                dsl::LanguageSpec {
                    id: "playbook.playbook.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::playbook::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::playbook::schema::diff::text::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::playbook::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("playbook.pack"),
                },
                dsl::LanguageSpec {
                    id: "playbook.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::playbook::spr::COMPONENT_PROTOCOL_PATH),
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_uses_the_playbook_media_kind_as_both_id_and_schema() {
        assert_eq!(artifact_kind().id, "text.playbook");
        assert_eq!(artifact_kind().schema, PLAYBOOK_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn block_fields_roundtrip() {
        let json = r#"{
            "id":"b1",
            "label":"Panel Count",
            "kind":"number",
            "required":true,
            "min":4,
            "max":64,
            "step":1,
            "unit":"panels"
        }"#;
        let block: PlaybookBlock = serde_json::from_str(json).expect("block json");
        assert_eq!(block.min, Some(4.0));
        assert_eq!(block.unit.as_deref(), Some("panels"));
        assert!(block.required.unwrap_or(false));
    }

    //#region 🌉️ContentBridgeLaws
    fn sample_steps() -> Vec<PlaybookStep> {
        vec![
            PlaybookStep {
                id: "intro".into(),
                title: "Introduction".into(),
                description: Some("What this playbook does.".into()),
                blocks: vec![PlaybookBlock {
                    id: "name".into(),
                    label: "Name".into(),
                    kind: "text".into(),
                    description: None,
                    required: Some(true),
                    placeholder: None,
                    default: None,
                    min: None,
                    max: None,
                    step: None,
                    unit: None,
                    text: None,
                    options: None,
                    fields: None,
                    schema: None,
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: Some(PlaybookExpr::Truthy { expr: Box::new(PlaybookExpr::Var { name: "enabled".into() }) }),
                }],
            },
            PlaybookStep { id: "review".into(), title: "Review".into(), description: None, blocks: Vec::new() },
        ]
    }

    /// ⚖️ LAW: `flow` is the LOSSLESS source of truth — every step field (including nested
    /// `condition` trees) round-trips through `flow_content_snapshot_from_steps`/
    /// `steps_from_flow_content` exactly.
    #[semio_framework_async_macros::async_test]
    async fn flow_content_round_trips_every_step_field_losslessly() {
        let steps = sample_steps();
        let content = crate::artifacts::playbook::flow_content_snapshot_from_steps(&steps);
        assert_eq!(content.nodes.len(), steps.len());
        assert_eq!(content.edges.len(), steps.len() - 1, "sequential steps chain via one edge per adjacent pair");
        let restored = crate::artifacts::playbook::steps_from_flow_content(&content);
        assert_eq!(restored, steps);
    }

    /// ⚖️ LAW: `document` is an HONEST narrative projection — `steps -> document` preserves every
    /// title/description, and `document -> steps` recovers exactly that title/description skeleton
    /// (never `blocks`/`condition`, which prose carries none of — documented lossy by design).
    #[semio_framework_async_macros::async_test]
    async fn document_projection_round_trips_titles_and_descriptions_only() {
        let steps = sample_steps();
        let content = crate::artifacts::playbook::document_snapshot_from_steps(Some("My Playbook"), &steps);
        let (title, restored) = crate::artifacts::playbook::steps_from_document(&content);
        assert_eq!(title.as_deref(), Some("My Playbook"));
        assert_eq!(restored.len(), steps.len());
        for (original, projected) in steps.iter().zip(restored.iter()) {
            assert_eq!(projected.title, original.title);
            assert_eq!(projected.description, original.description);
            assert!(projected.blocks.is_empty(), "document alone cannot recover block data — flow is that data's source of truth");
        }
    }

    fn one_step(title: &str) -> Vec<PlaybookStep> {
        vec![PlaybookStep { id: "step".into(), title: title.into(), description: None, blocks: Vec::new() }]
    }

    #[semio_framework_async_macros::async_test]
    async fn scene_owner_fixture_proves_identity_isolation_aba_wire_omission_and_bounded_close() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/playbook-scene-owner-law.json")).expect("language-neutral playbook scene fixture");
        let cases = fixture["cases"].as_array().expect("fixture cases");
        assert_eq!(fixture["schemaVersion"], 1);
        assert_eq!(fixture["ownedSlots"], 1);
        assert_eq!(cases.len(), fixture["maximumCases"].as_u64().expect("bounded maximum") as usize);
        assert_eq!(cases.len(), 5);

        for case in cases {
            let law = case["law"].as_str().expect("law");
            let first = case["first"].as_str().expect("first");
            let second = case["second"].as_str().expect("second");
            match law {
                "cloneIdentity" => {
                    let snapshot = playbook_snapshot_with_steps(PLAYBOOK_DOCUMENT_SCHEMA, "identity", "1", None, one_step(first));
                    let retained = playbook_working_scene_owner(&snapshot.flow);
                    let cloned = snapshot.clone();
                    let cloned_owner = playbook_working_scene_owner(&cloned.flow);
                    assert!(Arc::ptr_eq(&retained, &cloned_owner));
                    assert_eq!(cloned_owner.steps[0].title, first);
                    assert_eq!(Arc::strong_count(&retained), 4);
                }
                "instanceIsolation" => {
                    let mut left = flow_content_child_handle(&one_step(first));
                    let mut right = left.clone();
                    attach_playbook_steps(&mut left, one_step(first));
                    attach_playbook_steps(&mut right, one_step(second));
                    assert_eq!(playbook_working_scene_owner(&left).steps[0].title, first);
                    assert_eq!(playbook_working_scene_owner(&right).steps[0].title, second);
                }
                "abaIsolation" => {
                    let mut stale = flow_content_child_handle(&one_step("same-identity"));
                    attach_playbook_steps(&mut stale, one_step(first));
                    let mut reused_identity = flow_content_child_handle(&one_step("same-identity"));
                    assert_eq!(stale.child_id, reused_identity.child_id);
                    attach_playbook_steps(&mut reused_identity, one_step(second));
                    assert_eq!(playbook_working_scene_owner(&stale).steps[0].title, first);
                    assert_eq!(playbook_working_scene_owner(&reused_identity).steps[0].title, second);
                }
                "wireOmission" => {
                    let snapshot = playbook_snapshot_with_steps(PLAYBOOK_DOCUMENT_SCHEMA, "wire", "1", None, one_step(first));
                    let wire = serde_json::to_value(&snapshot).expect("third-party serde oracle serializes snapshot");
                    assert!(wire.pointer("/flow/localOwner").is_none());
                    let decoded: PlaybookSnapshot = serde_json::from_value(wire).expect("third-party serde oracle decodes snapshot");
                    assert!(decoded.flow.local_owner::<PlaybookWorkingScene>().is_none());
                    assert!(playbook_working_scene_owner(&decoded.flow).steps.is_empty());
                    assert_eq!(playbook_working_scene_owner(&snapshot.flow).steps[0].title, first);
                }
                "boundedClose" => {
                    let snapshot = playbook_snapshot_with_steps(PLAYBOOK_DOCUMENT_SCHEMA, "close", "1", None, one_step(first));
                    let retained = playbook_working_scene_owner(&snapshot.flow);
                    let weak = Arc::downgrade(&retained);
                    assert_eq!(Arc::strong_count(&retained), fixture["ownedSlots"].as_u64().expect("owned slots") as usize + 1);
                    drop(snapshot);
                    assert_eq!(Arc::strong_count(&retained), 1);
                    drop(retained);
                    assert!(weak.upgrade().is_none());
                }
                other => panic!("unexpected playbook scene law {other}"),
            }
        }
    }
    //#endregion 🌉️ContentBridgeLaws
}
//#endregion 🧪️Tests
