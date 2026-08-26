//! 📚️ Trinity Rewrite app — Catalogue panel (manifest kinds + add-to-LHS/RHS clause shortcuts).

use crate::editor::rewrite::terminology::TrinityRewriteLabels;
use semio_framework_plugin::{tree_item, tree_item_with_action, Label, PanelTreeBuilder, UiNode, UiTreeItemNode};

fn catalogue_add_item(id: &str, label: impl TryInto<Label>, clause_kind: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::editor::rewrite::ui_value_map([("kind", crate::editor::rewrite::ui_value_text(clause_kind)?)])?;
    tree_item_with_action(id, label, None, crate::editor::rewrite::rewrite_action("addRuleClause", Some(args))?)
}

pub(crate) fn render(labels: &TrinityRewriteLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    PanelTreeBuilder::new("trinity-catalogue")?
        .section(
            "trinity-catalogue.kinds",
            Some(labels.catalogue.into()),
            true,
            crate::editor::rewrite::ui_node_list([tree_item("trinity-catalogue.piece", labels.piece), tree_item("trinity-catalogue.connection", labels.connection), tree_item("trinity-catalogue.connector", labels.connector)])?,
        )?
        .section("trinity-catalogue.lhs", Some(labels.add_to_lhs.into()), true, crate::editor::rewrite::ui_node_list([catalogue_add_item("trinity-catalogue.add-where", Label::data("Where clause"), "where")])?)?
        .section(
            "trinity-catalogue.rhs",
            Some(labels.add_to_rhs.into()),
            true,
            crate::editor::rewrite::ui_node_list([
                catalogue_add_item("trinity-catalogue.add-create", Label::data("Create pattern"), "create"),
                catalogue_add_item("trinity-catalogue.add-merge", Label::data("Merge pattern"), "merge"),
                catalogue_add_item("trinity-catalogue.add-set", Label::data("Set assignment"), "set"),
                catalogue_add_item("trinity-catalogue.add-delete", Label::data("Delete pattern"), "delete"),
                catalogue_add_item("trinity-catalogue.add-parameter", Label::data("Parameter"), "parameter"),
            ])?,
        )?
        .selected([])?
        .build()
}
