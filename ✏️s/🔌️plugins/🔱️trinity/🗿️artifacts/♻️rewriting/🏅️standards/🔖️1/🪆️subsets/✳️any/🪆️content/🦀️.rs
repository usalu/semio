//! 🪆️ Rewriting's composed `workingGraph` child (design §20.15 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): readers compose
//! the parent with the exact member-store child, and every working-graph edit is ONE child-lane edit of the shared
//! `s.stdio.semio@v1/graph` vocabulary — no parent leaf reads the child.
use crate::RewritingSnapshot;
use semio_framework_diagnostic::Severity;
use semio_framework_plugin::app::{ChildContentView, ChildEmit,ChildEmitPreparation};
use semio_framework_plugin::{Emit, Fault};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

#[path = "👁️read/🦀️.rs"]
mod read;
pub(crate) use read::{read, WORKING_CHILD_SLOT};

/// 🧩️ The rule as its readers see it: the parent's fields composed on read with the exact published `workingGraph` member — the
/// working graph's single truth, never the parent's genesis owner, which goes stale after a child-lane edit. A root the child no
/// longer holds reads as no root.
pub(crate) fn composed(parent: &RewritingSnapshot, children: &ChildContentView) -> Result<RewritingSnapshot, Fault> {
    let child = read(parent, children)?;
    let mut view = parent.clone();
    if view.working_graph.root_node_id.as_ref().is_some_and(|root| !child.nodes.iter().any(|node| &node.id.value == root)) {
        view.working_graph.root_node_id = None;
    }
    semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut view.working_graph.content, SemioGraphSnapshot::clone(&child));
    Ok(view)
}

/// 🌱️ The genesis pack of the rule's own `workingGraph` child, derived from the parent the document was created or loaded with.
pub(crate) fn genesis_working_child_pack(parent: &RewritingSnapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
    use store::ArtifactPack;
    if slot != WORKING_CHILD_SLOT || child_id != parent.working_graph.content.child_id {
        return Ok(None);
    }
    let owner = semio_s_artifact_trinity_jack::jack_content_for_handle(&parent.working_graph.content)?;
    Ok(Some(SemioGraphSnapshot::encode_pack(owner.snapshot())))
}

/// 🧬️ Publishes `leaves` as ONE edit of the exact composed `workingGraph` child; no leaf is the empty emit.
pub(crate) fn working_child_emit<M, C, D>(parent: &RewritingSnapshot, leaves: Vec<SemioGraphMutation>) -> Emit<M, C, D> {
    if leaves.is_empty() {
        return Emit::default();
    }
    Emit { child_preparations: std::collections::VecDeque::from([ChildEmitPreparation::of::<SemioGraphSnapshot, _>(WORKING_CHILD_SLOT, &parent.working_graph.content.child_id, leaves)]), ..Default::default() }
}

/// 🛂️ Applies `leaf` to the running working child when the graph vocabulary admits it there; a refused leaf leaves it untouched.
pub(crate) fn admit(work: &mut SemioGraphSnapshot, leaf: &SemioGraphMutation) -> bool {
    let refused = |messages: &[protocol::MutationMessage]| messages.iter().any(|message| matches!(message.level, Severity::Error | Severity::Fatal));
    let outcome = <SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::diff(leaf, work);
    if refused(outcome.messages()) {
        return false;
    }
    match protocol::apply_diff(outcome.diff(), work) {
        Ok(next) => {
            *work = next;
            true
        }
        Err(_) => false,
    }
}

/// 📢️ The localized notices of rewriting's own refusal codes (design §20.12).
pub fn rewriting_fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
    use semio_framework_ui_locale::LocalizedLabel;
    static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 8]> = std::sync::LazyLock::new(|| {
        [
            ("trinity.rewriting.child-refused", LocalizedLabel::native("The example graph of this rule is not loaded as a Semio graph.", "Der Beispielgraph dieser Regel ist nicht als Semio-Graph geladen.")),
            ("trinity.rewriting.child-projection", LocalizedLabel::native("The example graph of this rule cannot be restored.", "Der Beispielgraph dieser Regel lässt sich nicht wiederherstellen.")),
            ("trinity.rewriting.window-required", LocalizedLabel::native("This view change needs an open window.", "Diese Ansichtsänderung braucht ein offenes Fenster.")),
            ("trinity.rewriting.graph-window-required", LocalizedLabel::native("This view change needs a graph window.", "Diese Ansichtsänderung braucht ein Graphfenster.")),
            ("trinity.rewriting.lod", LocalizedLabel::native("This level-of-detail name is too long.", "Dieser Detailstufen-Name ist zu lang.")),
            ("trinity.rewriting.viewport", LocalizedLabel::native("This viewport is not valid.", "Dieser Ausschnitt ist ungültig.")),
            ("trinity.rewriting.node-graph.row", LocalizedLabel::native("This graph edit is not possible on this canvas.", "Diese Graphänderung ist auf dieser Fläche nicht möglich.")),
            ("trinity.rewriting.retained-capacity", LocalizedLabel::native("This edit is too large to apply in one step.", "Diese Änderung ist zu groß, um sie in einem Schritt anzuwenden.")),
        ]
    });
    NOTICES.as_slice()
}
