//! 📜️ Section and statute schemas of the repo GraphQL schema: the section, definition, statute, breach and policy types. See [`super`].

use super::*;

//#region 🏗️SchemaSection
/// 💗️ The section type and the abstract item a section contains.
pub(super) fn schema_section() -> Vec<NamedType> {
    vec![
        NamedType::Interface(InterfaceType {
            name: "SectionItem".to_string(),
            fields: vec![Field::new("id", nn(ty("ID"))), Field::new("name", nn(ty("String"))), Field::new("range", ty("Range"))],
        }),
        NamedType::Object(ObjectType {
            name: "Section".to_string(),
            interfaces: vec!["SectionItem".to_string()],
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::resolved("file", ty("File"), "item.file"),
                Field::new("parent", ty("Section")),
                Field::resolved("children", lst(ty("SectionItem")), "section.children"),
                Field::new("definitions", lst(ty("Definition"))),
                Field::resolved("breachs", lst(ty("Breach")), "empty-list"),
                Field::new("range", nn(ty("Range"))),
            ],
        }),
    ]
}
//#endregion 🏗️SchemaSection

//#region 🏗️SchemaDefinition
/// 💕️ The definition type.
pub(super) fn schema_definition() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Definition".to_string(),
        interfaces: vec!["SectionItem".to_string()],
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("kind", nn(ty("DefinitionKind"))),
            Field::resolved("file", nn(ty("File")), "item.file"),
            Field::resolved("section", ty("Section"), "definition.section"),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
            Field::new("range", nn(ty("Range"))),
        ],
    })]
}
//#endregion 🏗️SchemaDefinition

//#region 🏗️SchemaStatute
/// 📜️ The statute type.
pub(super) fn schema_statute() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Statute".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::resolved("policy", nn(ty("Policy")), "statute.policy"),
            Field::new("priority", nn(ty("BreachPriority"))),
            Field::new("autofixable", nn(ty("Boolean"))),
            Field::new("reason", nn(ty("String"))),
            Field::new("solution", nn(ty("String"))),
            Field::new("breachs", nn_list(ty("Breach"))).arg("scope", ty("String")),
        ],
    })]
}
//#endregion 🏗️SchemaStatute

//#region 🏗️SchemaBreach
/// 🔶️ The breach type.
pub(super) fn schema_breach() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Breach".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("kindId", nn(ty("ID"))),
            Field::new("kind", nn(ty("Statute"))),
            Field::new("scope", nn(ty("String"))),
            Field::new("file", ty("File")),
            Field::new("folder", ty("Folder")),
            Field::new("line", ty("Int")),
            Field::new("column", ty("Int")),
            Field::new("excerpt", ty("String")),
            Field::new("summary", nn(ty("String"))),
            Field::new("priority", nn(ty("BreachPriority"))),
            Field::new("autofixable", nn(ty("Boolean"))),
        ],
    })]
}
//#endregion 🏗️SchemaBreach

//#region 🏗️SchemaPolicy
/// 👮️ The policy and territory types.
pub(super) fn schema_policy() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Territory".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("name", nn(ty("String"))),
                Field::new("description", nn(ty("String"))),
                Field::new("scopes", nn_list(ty("String"))),
                Field::new("groups", nn_list(ty("Territory"))),
                Field::new("kinds", nn_list(ty("Statute"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Policy".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("description", ty("String")),
                Field::new("scopes", nn_list(ty("String"))),
                Field::new("groups", nn_list(ty("Territory"))),
                Field::new("statutes", nn_list(ty("Statute"))),
            ],
        }),
    ]
}
//#endregion 🏗️SchemaPolicy
