//! 🤝️ Interaction, goal and todo schemas of the repo GraphQL schema: the interaction, goal, draft and todo types. See [`super`].

use super::*;

//#region 🏗️SchemaInteraction
/// 💬️ The interaction types.
pub(super) fn schema_interaction() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Interaction".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("kind", nn(ty("String"))),
                Field::new("prompt", nn(ty("String"))),
                Field::new("checkpoint", ty("String")),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("date", nn(ty("String"))),
                Field::new("system", nn(ty("String"))),
                Field::new("client", nn(ty("String"))),
                Field::new("author", nn(ty("String"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "InteractionResource".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("kind", nn(ty("String"))),
                Field::new("prompt", nn(ty("String"))),
                Field::new("checkpoint", ty("String")),
                Field::new("author", nn(ty("String"))),
                Field::new("sourceKind", nn(ty("String"))),
                Field::new("sourceId", nn(ty("String"))),
                Field::new("goalId", ty("String")),
                Field::new("ticketId", ty("String")),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("date", nn(ty("String"))),
                Field::new("system", nn(ty("String"))),
                Field::new("client", nn(ty("String"))),
            ],
        }),
    ]
}
//#endregion 🏗️SchemaInteraction

//#region 🏗️SchemaGoal
/// 🎯️ The goal type.
pub(super) fn schema_goal() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Goal".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("title", nn(ty("String"))),
            Field::new("description", ty("String")),
            Field::new("prompt", ty("String")),
            Field::new("dueDate", ty("String")),
            Field::new("createdAt", ty("String")),
            Field::new("client", ty("String")),
            Field::new("llm", ty("String")),
            Field::new("effort", ty("String")),
            Field::new("status", nn(ty("String"))),
            Field::new("milestone", ty("Int")),
            Field::new("issue", ty("String")),
            Field::new("parent", ty("String")),
            Field::new("interactions", nn_list(ty("Interaction"))),
        ],
    })]
}
//#endregion 🏗️SchemaGoal

//#region 🏗️SchemaDraft
/// 📝️ The draft type.
pub(super) fn schema_draft() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType { name: "Draft".to_string(), interfaces: Vec::new(), fields: vec![Field::new("id", nn(ty("ID")))] })]
}
//#endregion 🏗️SchemaDraft

//#region 🏗️SchemaTodo
/// ✅️ The todo type and the position it points at.
pub(super) fn schema_todo() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Location".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("filePath", ty("String")), Field::new("line", ty("Int")), Field::new("column", ty("Int"))],
        }),
        NamedType::Object(ObjectType {
            name: "Todo".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("description", ty("String")),
                Field::new("parentId", nn(ty("ID"))),
                Field::new("location", ty("Location")),
            ],
        }),
    ]
}
//#endregion 🏗️SchemaTodo
