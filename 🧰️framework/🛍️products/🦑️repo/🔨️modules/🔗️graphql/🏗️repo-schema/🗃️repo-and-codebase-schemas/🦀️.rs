//! 🗃️ Repo and codebase schemas of the repo GraphQL schema: the repo, technology, bundle, folder and file types. See [`super`].

use super::*;

//#region 🏗️SchemaRepo
/// 💠️ The repository root type: every aggregate reachable from one query.
pub(super) fn schema_repo() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Repo".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("path", nn(ty("String"))),
            Field::resolved("technologies", nn_list(ty("Technology")), "technologies"),
            Field::resolved("checkpoints", nn_list(ty("Checkpoint")), "checkpoints").arg("limit", ty("Int")),
            Field::resolved("bundles", nn_list(ty("Bundle")), "bundles"),
            Field::resolved("folders", nn_list(ty("Folder")), "folders"),
            Field::resolved("files", nn_list(ty("File")), "files"),
            Field::resolved("sections", nn_list(ty("Section")), "sections"),
            Field::resolved("definitions", nn_list(ty("Definition")), "definitions"),
            Field::resolved("contributors", nn_list(ty("Contributor")), "contributors"),
            Field::resolved("goals", nn_list(ty("Goal")), "goals"),
            Field::resolved("tickets", nn_list(ty("Ticket")), "tickets")
                .arg("year", ty("Int"))
                .arg("month", ty("Int"))
                .arg("day", ty("Int"))
                .arg("status", ty("TicketStatus")),
            Field::resolved("policies", nn_list(ty("Policy")), "policies"),
            Field::resolved("statutes", nn_list(ty("Statute")), "statutes"),
            Field::resolved("breachs", nn_list(ty("Breach")), "breachs").arg("scope", ty("String")),
        ],
    })]
}
//#endregion 🏗️SchemaRepo

//#region 🏗️SchemaTechnology
/// 📜️ The technology type and the package it publishes.
pub(super) fn schema_technology() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Technology".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("name", nn(ty("String"))),
                Field::new("root", nn(ty("String"))),
                Field::new("kind", nn(ty("String"))),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("uri", nn(ty("String"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Package".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("name", nn(ty("String"))),
                Field::new("version", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::new("kind", nn(ty("String"))),
            ],
        }),
    ]
}
//#endregion 🏗️SchemaTechnology

//#region 🏗️SchemaBundle
/// 🔵️ The bundle type.
pub(super) fn schema_bundle() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Bundle".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("name", nn(ty("String"))),
            Field::new("root", nn(ty("String"))),
            Field::new("sourceRoot", ty("String")),
            Field::new("projectType", ty("String")),
            Field::new("tags", nn_list(ty("String"))),
            Field::new("packages", nn_list(ty("Package"))),
            Field::new("kind", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::resolved("folders", nn_list(ty("Folder")), "empty-list"),
            Field::resolved("files", nn_list(ty("File")), "empty-list"),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
        ],
    })]
}
//#endregion 🏗️SchemaBundle

//#region 🏗️SchemaFolder
/// 🩷️ The folder type.
pub(super) fn schema_folder() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Folder".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("path", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::new("name", nn(ty("String"))),
            Field::new("kind", nn(ty("String"))),
            Field::new("parent", ty("Folder")),
            Field::resolved("children", nn_list(ty("Folder")), "folder.children"),
            Field::resolved("files", nn_list(ty("File")), "folder.files"),
            Field::new("ignored", nn(ty("Boolean"))),
            Field::new("generated", nn(ty("Boolean"))),
            Field::new("bundle", ty("Bundle")),
            Field::resolved("breachs", nn_list(ty("Breach")), "empty-list"),
        ],
    })]
}
//#endregion 🏗️SchemaFolder

//#region 🏗️SchemaFile
/// 📄️ The file type.
pub(super) fn schema_file() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "File".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("path", nn(ty("String"))),
            Field::new("uri", nn(ty("String"))),
            Field::new("name", nn(ty("String"))),
            Field::new("extension", nn(ty("String"))),
            Field::new("folder", ty("Folder")),
            Field::new("kind", nn(ty("String"))),
            Field::new("ignored", nn(ty("Boolean"))),
            Field::new("generated", nn(ty("Boolean"))),
            Field::new("bundle", ty("Bundle")),
            Field::resolved("sections", lst(ty("Section")), "file.sections"),
            Field::resolved("definitions", lst(ty("Definition")), "file.definitions"),
            Field::resolved("breachs", lst(ty("Breach")), "empty-list"),
            Field::new("content", ty("String")),
            Field::resolved("contributors", nn_list(ty("Contributor")), "empty-list"),
        ],
    })]
}
//#endregion 🏗️SchemaFile
