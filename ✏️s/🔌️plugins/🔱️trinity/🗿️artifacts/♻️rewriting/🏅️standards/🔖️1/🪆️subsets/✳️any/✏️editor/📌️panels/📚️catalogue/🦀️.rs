//! 📚️ Trinity Rewriting app — Catalogue panel (manifest kinds + add-to-LHS/RHS clause shortcuts).

use crate::editor::rewriting::terminology::TrinityRewritingLabels;
use semio_framework_plugin::{tree_item, tree_item_with_action, ui_node_list, PanelTreeBuilder, TreeWindows};

//#region 🔖️Render
/// 📚️ One "add a clause" shortcut: the row id and the rule-clause kind its click appends.
struct ClausePreset {
    id: &'static str,
    label: &'static str,
    kind: &'static str,
}

const LHS_CLAUSES: [ClausePreset; 1] = [ClausePreset { id: "trinity-catalogue.add-where", label: "Where clause", kind: "where" }];

const RHS_CLAUSES: [ClausePreset; 5] = [
    ClausePreset { id: "trinity-catalogue.add-create", label: "Create pattern", kind: "create" },
    ClausePreset { id: "trinity-catalogue.add-merge", label: "Merge pattern", kind: "merge" },
    ClausePreset { id: "trinity-catalogue.add-set", label: "Set assignment", kind: "set" },
    ClausePreset { id: "trinity-catalogue.add-delete", label: "Delete pattern", kind: "delete" },
    ClausePreset { id: "trinity-catalogue.add-parameter", label: "Parameter", kind: "parameter" },
];

fn clause_row(preset: &ClausePreset) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::editor::rewriting::ui_value_map([("kind", crate::editor::rewriting::ui_value_text(preset.kind)?)])?;
    tree_item_with_action(preset.id, crate::editor::rewriting::ui_label(preset.label)?, None, crate::editor::rewriting::rewriting_action("addRuleClause", Some(args))?)
}

pub(crate) fn render(labels: &TrinityRewritingLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let kind_items = ui_node_list([
        tree_item("trinity-catalogue.piece", crate::editor::rewriting::ui_label(labels.piece.as_str())?),
        tree_item("trinity-catalogue.connection", crate::editor::rewriting::ui_label(labels.connection.as_str())?),
        tree_item("trinity-catalogue.connector", crate::editor::rewriting::ui_label(labels.connector.as_str())?),
    ])?;
    PanelTreeBuilder::new("trinity-catalogue")?
        .section("trinity-catalogue.kinds", Some(crate::editor::rewriting::ui_label(labels.catalogue.as_str())?), true, kind_items)?
        .window_section(windows, "trinity-catalogue.lhs", Some(crate::editor::rewriting::ui_label(labels.add_to_lhs.as_str())?), true, &LHS_CLAUSES, clause_row)?
        .window_section(windows, "trinity-catalogue.rhs", Some(crate::editor::rewriting::ui_label(labels.add_to_rhs.as_str())?), true, &RHS_CLAUSES, clause_row)?
        .selected([])?
        .build()
}
//#endregion 🔖️Render
