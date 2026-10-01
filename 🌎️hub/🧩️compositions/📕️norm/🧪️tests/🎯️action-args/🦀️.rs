//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them
//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema
//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`/`setSnapshot` (measured over MCP: optional
//! text `path` → `INTERNAL "path must not be empty"` on all fifteen kinds; text `index` read as 0; `insertItem.value`
//! undeclared). Ticket 26/09/23 slice S19.
use semio_framework_plugin::{ActionArgDef, AppDefinition, ArgSchema};

fn argument<'a>(definition: &'a AppDefinition, action: &str, argument: &str) -> &'a ActionArgDef {
    let declared = definition.actions.iter().find(|candidate| candidate.id == action).unwrap_or_else(|| panic!("{} declares no {action}", definition.id));
    declared.args.iter().find(|candidate| candidate.id == argument).unwrap_or_else(|| panic!("{} {action} declares no {argument}", definition.id))
}

fn assert_text(definition: &AppDefinition, action: &str, name: &str, required: bool) {
    let declared = argument(definition, action, name);
    assert!(matches!(declared.schema, ArgSchema::String { .. }), "{} {action}.{name} must be text", definition.id);
    assert_eq!(declared.required, required, "{} {action}.{name} required", definition.id);
}

fn assert_ordinal(definition: &AppDefinition, action: &str, name: &str) {
    let declared = argument(definition, action, name);
    assert!(matches!(declared.schema, ArgSchema::Number { integer: true, min: Some(min), .. } if min == 0.0), "{} {action}.{name} must be a non-negative integer", definition.id);
    assert!(declared.required, "{} {action}.{name} must be required", definition.id);
}

fn assert_value(definition: &AppDefinition, action: &str, required: bool) {
    let declared = argument(definition, action, "value");
    assert!(matches!(declared.schema, ArgSchema::Any), "{} {action}.value must be a typed value", definition.id);
    assert_eq!(declared.required, required, "{} {action}.value required", definition.id);
}

fn assert_norm_value_tree_arguments(definition: AppDefinition) {
    assert_text(&definition, "setField", "path", true);
    assert_value(&definition, "setField", true);
    assert_text(&definition, "insertItem", "path", true);
    assert_ordinal(&definition, "insertItem", "index");
    assert_value(&definition, "insertItem", false);
    assert_text(&definition, "removeItem", "path", true);
    assert_ordinal(&definition, "removeItem", "index");
    assert_text(&definition, "applyRemedy", "checkId", true);
    assert_ordinal(&definition, "applyRemedy", "remedyIndex");
    assert_text(&definition, "setSnapshot", "snapshot", true);
}

#[test]
fn din4108_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_din4108::editor::din4108::create_din4108_app());
}

#[test]
fn din16798_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_din16798::editor::din16798::create_din16798_app());
}

#[test]
fn din18599_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_din18599::editor::din18599::create_din18599_app());
}

#[test]
fn en1990_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1990::editor::en1990::create_en1990_app());
}

#[test]
fn en1991_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1991::editor::en1991::create_en1991_app());
}

#[test]
fn en1992_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1992::editor::en1992::create_en1992_app());
}

#[test]
fn en1993_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1993::editor::en1993::create_en1993_app());
}

#[test]
fn en1994_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1994::editor::en1994::create_en1994_app());
}

#[test]
fn en1995_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1995::editor::en1995::create_en1995_app());
}

#[test]
fn en1996_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1996::editor::en1996::create_en1996_app());
}

#[test]
fn en1997_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1997::editor::en1997::create_en1997_app());
}

#[test]
fn en1998_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1998::editor::en1998::create_en1998_app());
}

#[test]
fn en1999_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_en1999::editor::en1999::create_en1999_app());
}

#[test]
fn iso16757_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_iso16757::editor::iso16757::create_iso16757_app());
}

#[test]
fn vdi3805_declares_its_value_tree_arguments() {
    assert_norm_value_tree_arguments(semio_s_artifact_norm_vdi3805::editor::vdi3805::create_vdi3805_app());
}
