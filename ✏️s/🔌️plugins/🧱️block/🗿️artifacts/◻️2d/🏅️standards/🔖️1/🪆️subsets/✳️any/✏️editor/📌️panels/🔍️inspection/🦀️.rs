//! 🔍️ Block 2D play app panel — the inspector: the node kind's identity fields plus a handle count.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::editor::block2d::terminology::Block2dLabels;
use crate::editor::block2d::{block2d_action, ui_label, ui_text, ui_value_map, ui_value_text};
use semio_framework_plugin::plugin_app_close_prelude::{field, input, Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiFixedList, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const BLOCK2D_BODY_INSPECTOR: &str = "block2d.play.inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BLOCK2D_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn admission(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", stage)
}

/// ✍️ One editable identity field: a `blur`-committed text control bound to `patchNodeKind` with the
/// document field name as its only argument (the React interpreter merges `{ value }` at commit).
fn text_field(id: &str, label: &str, value: &str, document_field: &str) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = block2d_action("patchNodeKind", Some(ui_value_map([("field", ui_value_text(document_field)?)])?))?;
    let control = input(InputKind::Text).value(ui_text(value)?).commit(ui_text("blur")?).try_id(format!("{id}.input")).map_err(|_| admission("block2d inspector control id admission failed"))?;
    let control = match args {
        Some(args) => control.try_on_with(Trigger::Change, action, args),
        None => control.try_on(Trigger::Change, action),
    }
    .map_err(|_| admission("block2d inspector control binding admission failed"))?
    .try_build()
    .map_err(|_| admission("block2d inspector control admission failed"))?;
    field(ui_label(label)?)
        .try_id(id)
        .map_err(|_| admission("block2d inspector field id admission failed"))?
        .try_child(control)
        .map_err(|_| admission("block2d inspector field child admission failed"))?
        .try_build()
        .map_err(|_| admission("block2d inspector field admission failed"))
}

pub fn render(definition: &Block2dSnapshot, labels: &Block2dLabels) -> UiAssemblyResult<BuiltNode> {
    let mut fields = UiFixedList::<BuiltNode>::default();
    for node in [
        text_field("block2d-play-inspector.name", labels.name.as_str(), &definition.node_kind.name, "name")?,
        text_field("block2d-play-inspector.label", labels.label.as_str(), &definition.node_kind.label, "label")?,
        text_field("block2d-play-inspector.variant", labels.variant.as_str(), definition.node_kind.variant.as_deref().unwrap_or(""), "variant")?,
        text_field("block2d-play-inspector.description", labels.description.as_str(), &definition.node_kind.description, "description")?,
        tree_item("block2d-play-inspector.handle-count", format!("{}: {}", labels.handles.as_str(), definition.handles.len()))?,
    ] {
        fields.try_push(node).map_err(|_| admission("block2d inspector fields admission failed"))?;
    }
    PanelTreeBuilder::new("block2d-play-inspector")?.section("block2d-play-inspector.summary", Some(ui_label(labels.summary.as_str())?), true, fields)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block2d::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_inspector_fields() {
        let mut app = new_app();
        let json = render_body(&mut app, BLOCK2D_BODY_INSPECTOR);
        assert!(json.contains("\"type\":\"tree\""), "inspection body must be a tree like document");
        assert!(json.contains("Name"));
        assert!(!json.contains("\"type\":\"stack\""), "inspection body must not be a free-form stack");
    }
}
//#endregion 🧪️Tests
