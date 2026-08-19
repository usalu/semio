//! 🌳️ Playbook viewer — Steps window: a read-only tree of steps → blocks, built on the framework's
//! `TreeWindowKit` (contract §2.6) rather than a bespoke render — the playbook document is naturally
//! tree-shaped (an ordered list of steps, each an ordered list of blocks) and needs none of the
//! editor-only affordances (drag handles, a block palette, per-kind form fields) to be legible
//! read-only.

use crate::artifacts::playbook::PlaybookSnapshot;
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
pub async fn definition() -> WindowKindDefinition {
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
pub async fn render(spec: &PlaybookSnapshot) -> semio_framework_plugin::UiNode {
    let roots = spec
        .steps()
        .into_iter()
        .map(|step| {
            let label = if step.title.is_empty() { step.id.clone() } else { step.title.clone() };
            let children = step.blocks.iter().map(|block| TreeNodeView { id: format!("{}/{}", step.id, block.id), label: format!("{} ({})", block.label, block.kind), children: Vec::new() }).collect();
            TreeNodeView { id: step.id.clone(), label, children }
        })
        .collect();
    TreeWindowKit::render(&TreeView { roots })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::playbook_snapshot_with_steps;
    use crate::playbook::{PlaybookBlock, PlaybookStep};

    async fn sample_block(id: &str, label: &str, kind: &str) -> PlaybookBlock {
        PlaybookBlock {
            id: id.into(),
            label: label.into(),
            kind: kind.into(),
            description: None,
            required: None,
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
            condition: None,
        }
    }

    #[test]
    async fn definition_restamps_the_tree_window_kit_with_this_windows_own_id_and_body_key() {
        let definition = definition();
        assert_eq!(definition.id, PLAYBOOK_VIEW_WINDOW_STEPS);
        assert_eq!(definition.body_key, PLAYBOOK_VIEW_BODY_STEPS);
        assert_eq!(definition.surface_kind, TreeWindowKit::window_kind().surface_kind, "restamping the id/body-key must not change the underlying surface shape");
    }

    #[test]
    async fn render_nests_every_blocks_label_and_kind_under_its_own_step() {
        let step = PlaybookStep { id: "s1".into(), title: "Intro".into(), description: None, blocks: vec![sample_block("b1", "Name", "text")] };
        let spec = playbook_snapshot_with_steps("playbook.program", "playbook", "1", Some("Recipe".into()), vec![step]);
        let node = render(&spec);
        let json = serde_json::to_string(&node).expect("tree json");
        assert!(json.contains("Intro"), "step title must appear as a root node label: {json}");
        assert!(json.contains("Name (text)"), "block label+kind must appear as a leaf node label: {json}");
    }

    #[test]
    async fn render_falls_back_to_the_step_id_when_the_title_is_empty() {
        let step = PlaybookStep { id: "s1".into(), title: String::new(), description: None, blocks: Vec::new() };
        let spec = playbook_snapshot_with_steps("playbook.program", "playbook", "1", None, vec![step]);
        let node = render(&spec);
        let json = serde_json::to_string(&node).expect("tree json");
        assert!(json.contains("\"s1\""), "an empty step title must fall back to the step id: {json}");
    }
}
//#endregion 🧪️Tests
