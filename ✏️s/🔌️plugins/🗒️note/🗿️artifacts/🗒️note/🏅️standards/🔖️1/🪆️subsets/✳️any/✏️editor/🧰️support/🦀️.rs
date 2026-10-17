//! 🧰️ Note editor support — the action factory, UI admission, interaction domain constants and topology every taxonomy node shares.

use crate::{NoteBlockNode, NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ActionDefinition, ActionDescriptor, ActionKind, DomainTopology, DslValue, TopologyNode, UtilityCategory, UtilityDefinition};
use semio_framework_ui_locale::LocalizedLabel;

/// 👁️✏️ C2 §2.1: the hand-written app id is retired — the canonical surface id
/// (`s.note.note@1/*#editor`) is now derived from `NotePlayApp::DIALECT`/`ROLE` via `surface_app_id`.
pub const NOTE_PLAY_CONTROLLER_ID: &str = "s.note.note@1/*#editor";

/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `setActiveExample`/`loadDocumentJson` build a
/// `Effect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry. The spr is a
/// fresh, edit-free op-log (`store::os_store::text_format::empty_document_spr`) — never a live `ArtifactEnvelope` minted just to
/// print it: dropping such an envelope trapped the guest (`artifact envelope terminal shell reached Drop
/// before its app-owned bounded retirement authority detached every nested owner`) on the react shell's
/// boot `setActiveExample` (ticket 26/09/17/NOTE-PLUGIN-END-TO-END).
pub fn reset_document_effect(document: &NoteSnapshot) -> semio_framework::kernel::Effect {
    let pack = <NoteSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("note", NOTE_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`☑️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn note_action(action: &str, args: Option<DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: NOTE_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🛠️ An internal (non-palette) action declaration — the pointer/gesture/inspector/keybound vocabulary
/// dispatched by the canvas/panels, never surfaced as a standalone command palette entry.
pub fn note_internal_action(id: &str, label: LocalizedLabel, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🧰️ One canvas utility declaration (id/label/icon reused verbatim from the retired `utilities()`/
/// utility bar).
pub fn note_utility(id: &str, label: LocalizedLabel, icon: &str, group: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧰️ The utility armed for the window being rendered or dispatched: the React host arms utilities per
/// window instance (`active_utility_by_window_id`) and mirrors only the shell's ACTIVE window into the flat
/// `active_utility_id`, so a pane that is not the active window read `None` and ran every pencil drag as a
/// `selectDirect` pick (ticket 26/09/17/NOTE-PLUGIN-END-TO-END; draw's `drawing_active_utility` precedent).
pub fn note_active_utility(view: &semio_framework_plugin::ViewModel) -> &str {
    view.window_id
        .as_deref()
        .and_then(|window| view.active_utility_by_window_id.get(window))
        .or_else(|| view.focused_window_id.as_deref().and_then(|window| view.active_utility_by_window_id.get(window)))
        .map(String::as_str)
        .filter(|utility| !utility.is_empty())
        .or(view.active_utility_id.as_deref())
        .unwrap_or("selectDirect")
}

/// 🕹️ "blocks" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Topology` over the document's own Group nesting, transitive
/// (selecting/hovering a group covers its descendants).
pub const NOTE_INTERACTION_BLOCKS: &str = "blocks";

/// 🕹️ The only granularity the "blocks" domain declares — stamped on every pick row of the document
/// tree so the host can synthesize `interactionSelect` without a per-row argument map.
pub const NOTE_INTERACTION_GRANULARITY: &str = "block";

/// 🌳️ `blocks` domain topology from the document's own Group nesting — row-id-prefixed ids (matching
/// the document panel tree's own item ids), so `validate_state` prunes deleted blocks and
/// range/transitive selection walk the real tree structure.
pub fn note_blocks_topology(document: &NoteSnapshot) -> DomainTopology {
    fn visit(blocks: &[NoteBlockNode], parent: Option<&str>, out: &mut Vec<TopologyNode>) {
        for block in blocks {
            let id = crate::schema::block_tree_row_id(block);
            out.push(TopologyNode { id: id.clone(), granularity: "block".into(), parent: parent.map(str::to_string) });
            if let NoteBlockNode::Group { children, .. } = block {
                visit(children, Some(id.as_str()), out);
            }
        }
    }
    let mut ordered = Vec::new();
    visit(&document.blocks, None, &mut ordered);
    DomainTopology { ordered }
}

/// 🏷️ Admits Note display text into a bounded semantic label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "note label admission failed"))
}
