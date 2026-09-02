//! 📚️ Trinity Rewriting app — Catalogue panel (manifest kinds + add-to-LHS/RHS clause shortcuts).

use crate::editor::rewriting::terminology::TrinityRewritingLabels;
use semio_framework_plugin::{tree_item, tree_item_with_action, PanelTreeBuilder};

fn catalogue_add_item(id: &str, label: impl AsRef<str>, clause_kind: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::editor::rewriting::ui_value_map([("kind", crate::editor::rewriting::ui_value_text(clause_kind)?)])?;
    tree_item_with_action(id, crate::editor::rewriting::ui_label(label)?, None, crate::editor::rewriting::rewriting_action("addRuleClause", Some(args))?)
}

pub(crate) fn render(labels: &TrinityRewritingLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    PanelTreeBuilder::new("trinity-catalogue")?
        .section(
            "trinity-catalogue.kinds",
            Some(labels.catalogue.as_str().into()),
            true,
            crate::editor::rewriting::ui_node_list([
                tree_item("trinity-catalogue.piece", crate::editor::rewriting::ui_label(labels.piece.as_str())?),
                tree_item("trinity-catalogue.connection", crate::editor::rewriting::ui_label(labels.connection.as_str())?),
                tree_item("trinity-catalogue.connector", crate::editor::rewriting::ui_label(labels.connector.as_str())?),
            ])?,
        )?
        .section("trinity-catalogue.lhs", Some(labels.add_to_lhs.as_str().into()), true, crate::editor::rewriting::ui_node_list([catalogue_add_item("trinity-catalogue.add-where", "Where clause", "where")])?)?
        .section(
            "trinity-catalogue.rhs",
            Some(labels.add_to_rhs.as_str().into()),
            true,
            crate::editor::rewriting::ui_node_list([
                catalogue_add_item("trinity-catalogue.add-create", "Create pattern", "create"),
                catalogue_add_item("trinity-catalogue.add-merge", "Merge pattern", "merge"),
                catalogue_add_item("trinity-catalogue.add-set", "Set assignment", "set"),
                catalogue_add_item("trinity-catalogue.add-delete", "Delete pattern", "delete"),
                catalogue_add_item("trinity-catalogue.add-parameter", "Parameter", "parameter"),
            ])?,
        )?
        .selected([])?
        .build()
}
