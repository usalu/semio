//! 📚️ Trinity Rewrite app — Catalogue panel (manifest kinds + add-to-LHS/RHS clause shortcuts).

use crate::apps::rewrite::terminology::TrinityRewriteLabels;
use semio_framework_plugin::{tree_item, tree_item_with_action, Label, PanelTreeBuilder, UiNode, UiTreeItemNode};
use serde_json::json;

fn catalogue_add_item(id: &str, label: impl Into<Label>, clause_kind: &str) -> UiTreeItemNode {
    let jack_action = crate::apps::rewrite::rewrite_action;
    UiTreeItemNode { ..tree_item_with_action(id, label, None, jack_action("addRuleClause", Some(json!({ "kind": clause_kind })))) }
}

pub(crate) fn render(labels: &TrinityRewriteLabels) -> UiNode {
    PanelTreeBuilder::new("trinity-catalogue")
        .section(
            "trinity-catalogue.kinds",
            Some(labels.catalogue.into()),
            true,
            vec![tree_item("trinity-catalogue.piece", labels.piece), tree_item("trinity-catalogue.connection", labels.connection), tree_item("trinity-catalogue.connector", labels.connector)],
        )
        .section("trinity-catalogue.lhs", Some(labels.add_to_lhs.into()), true, vec![catalogue_add_item("trinity-catalogue.add-where", Label::data("Where clause"), "where")])
        .section(
            "trinity-catalogue.rhs",
            Some(labels.add_to_rhs.into()),
            true,
            vec![
                catalogue_add_item("trinity-catalogue.add-create", Label::data("Create pattern"), "create"),
                catalogue_add_item("trinity-catalogue.add-merge", Label::data("Merge pattern"), "merge"),
                catalogue_add_item("trinity-catalogue.add-set", Label::data("Set assignment"), "set"),
                catalogue_add_item("trinity-catalogue.add-delete", Label::data("Delete pattern"), "delete"),
                catalogue_add_item("trinity-catalogue.add-parameter", Label::data("Parameter"), "parameter"),
            ],
        )
        .selected(vec![])
        .build()
}
