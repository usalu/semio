use semio_plugin::{ActionArgDef, AppDefinition, ArgSchema};

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

macro_rules! value_tree_case {
    ($name:ident, $create:path) => {
        #[test]
        fn $name() {
            assert_norm_value_tree_arguments($create());
        }
    };
}
