//! 🌳️ Playbook viewer — Steps window: a read-only tree of steps → blocks, built on the framework's
//! `TreeWindowKit` (contract §2.6) rather than a bespoke render — the playbook document is naturally
//! tree-shaped (an ordered list of steps, each an ordered list of blocks) and needs none of the
//! editor-only affordances (drag handles, a block palette, per-kind form fields) to be legible
//! read-only.

use crate::PlaybookSnapshot;
use semio_framework_plugin::WindowKindDefinition;
// 🚧️ SDK GAP: the seven framework window kits (contract §2.6 — `TreeWindowKit`/`TreeView`/
// `TreeNodeView`/the `WindowKit` trait) are not yet in `semio_framework_plugin`'s curated crate-root
// re-export list (only reachable through `app`, unlike `ArtifactEditor`/`ArtifactViewer`/`Editor`/
// `Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit`, which w0-f already promoted). Not fixable here
// (`🧰️framework/**` is outside this packet's lease); flagged in this packet's notes file.
use semio_framework_plugin::app::{TreeNodeView, TreeView, TreeWindowKit, WindowKit};

//#region 🔖️Constants
pub const PLAYBOOK_VIEW_WINDOW_STEPS: &str = "playbook-view-steps";
pub const PLAYBOOK_VIEW_BODY_STEPS: &str = "playbook.view.steps";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Read-only variant of the shared `TreeWindowKit` definition, restamped with this window's own
/// id/body-key (the kit's own id/body-key are a generic `"framework.window.tree"`, shared across every
/// app that composes it — each composing app restamps both, exactly like `steps::render` below reuses
/// the kit's `render` unmodified).
pub fn definition() -> WindowKindDefinition {
    let mut definition = TreeWindowKit::window_kind();
    definition.id = PLAYBOOK_VIEW_WINDOW_STEPS.into();
    definition.body_key = PLAYBOOK_VIEW_BODY_STEPS.into();
    definition
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🌳️ One root node per step (labeled with the step title, falling back to its id when the title is
/// empty), one leaf child per block (labeled `"<label> (<kind>)"`) — a faithful, read-only reflection
/// of the same step/block nesting the editor's block-list builder edits.
pub fn render(spec: &PlaybookSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let roots = spec
        .steps()
        .into_iter()
        .map(|step| {
            let label = if step.title.is_empty() { step.id.clone() } else { step.title.clone() };
            let children = step.blocks.iter().map(|block| TreeNodeView { id: format!("{}/{}", step.id, block.id), label: format!("{} ({})", block.label, block.kind), children: Vec::new() }).collect();
            TreeNodeView { id: step.id, label, children }
        })
        .collect();
    TreeWindowKit::render(&TreeView { roots })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
