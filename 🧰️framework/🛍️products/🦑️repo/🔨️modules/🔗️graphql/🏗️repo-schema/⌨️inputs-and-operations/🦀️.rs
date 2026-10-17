//! ⌨️ Inputs and operations of the repo GraphQL schema: the input object types and the query and mutation root types. See [`super`].

use super::*;

//#region 🏗️SchemaInputs
/// 📥️ Every input object the query and mutation roots accept.
pub(super) fn schema_inputs() -> Vec<NamedType> {
    let input = |name: &str, fields: Vec<(&str, TypeRef)>| {
        NamedType::InputObject(InputObjectType { name: name.to_string(), fields: fields.into_iter().map(|(field, kind)| (field.to_string(), kind)).collect() })
    };
    vec![
        input(
            "FilterInput",
            vec![
                ("filter", ty("String")),
                ("regex", ty("Boolean")),
                ("matchCase", ty("Boolean")),
                ("matchWholeWord", ty("Boolean")),
                ("showIgnored", ty("Boolean")),
                ("showGenerated", ty("Boolean")),
                ("excludeKinds", lst(ty("FileKind"))),
                ("includeKinds", lst(ty("FileKind"))),
            ],
        ),
        input("DraftCreateInput", vec![("title", nn(ty("String"))), ("files", lst(nn(ty("String"))))]),
        input(
            "TicketOpenInput",
            vec![
                ("emoji", nn(ty("String"))),
                ("title", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("client", nn(ty("TicketClient"))),
                ("noIssue", ty("Boolean")),
                ("draft", ty("String")),
                ("goal", nn(ty("String"))),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("issue", ty("String")),
                ("planId", ty("String")),
                ("specId", ty("String")),
            ],
        ),
        input(
            "TicketCloseInput",
            vec![
                ("year", ty("Int")),
                ("month", ty("Int")),
                ("day", ty("Int")),
                ("slug", ty("String")),
                ("summary", ty("String")),
                ("files", lst(nn(ty("String")))),
                ("title", ty("String")),
                ("noManagement", ty("Boolean")),
                ("all", ty("Boolean")),
            ],
        ),
        input(
            "TicketReopenInput",
            vec![
                ("year", nn(ty("Int"))),
                ("month", nn(ty("Int"))),
                ("day", nn(ty("Int"))),
                ("slug", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("client", nn(ty("TicketClient"))),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("title", ty("String")),
                ("draft", ty("String")),
                ("goal", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("planId", ty("String")),
                ("specId", ty("String")),
            ],
        ),
        input(
            "TicketChangeInput",
            vec![
                ("year", nn(ty("Int"))),
                ("month", nn(ty("Int"))),
                ("day", nn(ty("Int"))),
                ("slug", nn(ty("String"))),
                ("title", ty("String")),
                ("prompt", ty("String")),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("client", ty("TicketClient")),
                ("goal", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input("TodoCreateInput", vec![("name", nn(ty("String"))), ("description", nn(ty("String"))), ("parentId", nn(ty("String")))]),
        input("TodoChangeInput", vec![("id", nn(ty("String"))), ("name", ty("String")), ("description", ty("String"))]),
        input(
            "GoalCreateInput",
            vec![
                ("title", nn(ty("String"))),
                ("description", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("dueDate", nn(ty("String"))),
                ("llm", nn(ty("String"))),
                ("effort", ty("String")),
                ("client", nn(ty("String"))),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
                ("milestone", ty("String")),
            ],
        ),
        input(
            "GoalChangeInput",
            vec![
                ("id", nn(ty("String"))),
                ("title", ty("String")),
                ("description", ty("String")),
                ("dueDate", ty("String")),
                ("llm", ty("String")),
                ("effort", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input("GoalCloseInput", vec![("id", nn(ty("String"))), ("summary", nn(ty("String"))), ("noManagement", ty("Boolean"))]),
        input(
            "GoalReopenInput",
            vec![
                ("id", nn(ty("String"))),
                ("prompt", nn(ty("String"))),
                ("client", nn(ty("String"))),
                ("llm", nn(ty("String"))),
                ("effort", ty("String")),
                ("title", ty("String")),
                ("description", ty("String")),
                ("dueDate", ty("String")),
                ("parent", ty("String")),
                ("noManagement", ty("Boolean")),
            ],
        ),
        input(
            "ContributorAddInput",
            vec![
                ("github", nn(ty("String"))),
                ("name", ty("String")),
                ("names", lst(nn(ty("String")))),
                ("email", ty("String")),
                ("emails", lst(nn(ty("String")))),
                ("fingerprint", ty("String")),
                ("fingerprints", lst(nn(ty("String")))),
            ],
        ),
    ]
}
//#endregion 🏗️SchemaInputs

//#region 🏗️SchemaRoots
/// 🔍️ The query root and the node union it discriminates.
pub(super) fn schema_query() -> Vec<NamedType> {
    vec![
        NamedType::Union(UnionType {
            name: "Node".to_string(),
            types: ["Repo", "Bundle", "Folder", "File", "Section", "Definition", "Contributor", "Ticket", "Policy", "Statute", "Breach", "Draft"].iter().map(|name| (*name).to_string()).collect(),
        }),
        NamedType::Object(ObjectType {
            name: "Query".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::resolved("node", nn(ty("Node")), "query.node").arg("id", nn(ty("ID"))),
                Field::resolved("repo", nn(ty("Repo")), "query.repo"),
                Field::resolved("technologies", nn_list(ty("Technology")), "technologies").arg("filter", ty("FilterInput")),
                Field::resolved("bundles", nn_list(ty("Bundle")), "bundles").arg("filter", ty("FilterInput")),
                Field::resolved("folders", nn_list(ty("Folder")), "folders"),
                Field::resolved("files", nn_list(ty("File")), "files"),
                Field::resolved("sections", nn_list(ty("Section")), "sections"),
                Field::resolved("definitions", nn_list(ty("Definition")), "definitions"),
                Field::resolved("contributors", nn_list(ty("Contributor")), "contributors").arg("filter", ty("FilterInput")),
                Field::resolved("todos", nn_list(ty("Todo")), "query.todos").arg("filter", ty("FilterInput")),
                Field::resolved("tickets", nn_list(ty("Ticket")), "tickets")
                    .arg("year", ty("Int"))
                    .arg("month", ty("Int"))
                    .arg("day", ty("Int"))
                    .arg("status", ty("TicketStatus"))
                    .arg("filter", ty("FilterInput")),
                Field::resolved("interactions", nn_list(ty("InteractionResource")), "query.interactions"),
                Field::resolved("drafts", nn_list(ty("Draft")), "query.drafts"),
                Field::resolved("policies", nn_list(ty("Policy")), "policies").arg("filter", ty("FilterInput")),
                Field::resolved("statutes", nn_list(ty("Statute")), "statutes"),
                Field::resolved("breachs", nn_list(ty("Breach")), "breachs").arg("scope", ty("String")),
                Field::resolved("bundle", ty("Bundle"), "query.bundle").arg("name", nn(ty("String"))),
                Field::resolved("folder", ty("Folder"), "query.folder").arg("path", nn(ty("String"))),
                Field::resolved("file", ty("File"), "query.file").arg("path", nn(ty("String"))),
                Field::resolved("section", ty("Section"), "query.section").arg("path", nn(ty("String"))).arg("sectionPath", nn_list(ty("String"))),
                Field::resolved("definition", ty("Definition"), "query.definition").arg("path", nn(ty("String"))).arg("name", nn(ty("String"))),
                Field::resolved("contributor", ty("Contributor"), "query.contributor").arg("id", nn(ty("String"))),
                Field::resolved("ticket", ty("Ticket"), "query.ticket")
                    .arg("year", nn(ty("Int")))
                    .arg("month", nn(ty("Int")))
                    .arg("day", nn(ty("Int")))
                    .arg("slug", nn(ty("String"))),
                Field::resolved("policy", ty("Policy"), "query.policy").arg("id", nn(ty("String"))),
                Field::resolved("statute", ty("Statute"), "query.statute").arg("id", nn(ty("String"))),
                Field::resolved("analyze", nn(ty("AnalyzeResult")), "query.analyze").arg("scope", ty("String")),
            ],
        }),
    ]
}

/// ✍️ The mutation root.
pub(super) fn schema_mutation() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Mutation".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::resolved("syncManagement", nn(ty("Boolean")), "mutation.syncManagement"),
            Field::resolved("goalCreate", ty("Goal"), "mutation.goalCreate").arg("input", nn(ty("GoalCreateInput"))),
            Field::resolved("goalChange", ty("Goal"), "mutation.goalChange").arg("id", nn(ty("ID"))).arg("input", nn(ty("GoalChangeInput"))),
            Field::resolved("goalClose", ty("Goal"), "mutation.goalClose").arg("input", nn(ty("GoalCloseInput"))),
            Field::resolved("goalReopen", ty("Goal"), "mutation.goalReopen").arg("input", nn(ty("GoalReopenInput"))),
            Field::resolved("draftCreate", ty("Draft"), "mutation.draftCreate").arg("input", nn(ty("DraftCreateInput"))),
            Field::resolved("draftDelete", nn(ty("Boolean")), "mutation.draftDelete").arg("id", nn(ty("String"))),
            Field::resolved("todoCreate", ty("Todo"), "mutation.todoCreate").arg("input", nn(ty("TodoCreateInput"))),
            Field::resolved("todoChange", ty("Todo"), "mutation.todoChange").arg("input", nn(ty("TodoChangeInput"))),
            Field::resolved("todoDelete", ty("Boolean"), "mutation.todoDelete").arg("id", nn(ty("ID"))),
            Field::resolved("ticketOpen", ty("Ticket"), "mutation.ticketOpen").arg("input", nn(ty("TicketOpenInput"))),
            Field::resolved("ticketClose", ty("Ticket"), "mutation.ticketClose").arg("input", nn(ty("TicketCloseInput"))),
            Field::resolved("ticketReopen", ty("Ticket"), "mutation.ticketReopen").arg("input", nn(ty("TicketReopenInput"))),
            Field::resolved("ticketChange", ty("Ticket"), "mutation.ticketChange").arg("input", nn(ty("TicketChangeInput"))),
            Field::resolved("contributorAdd", ty("Contributor"), "mutation.contributorAdd").arg("input", nn(ty("ContributorAddInput"))),
            Field::resolved("contributorRemove", nn(ty("Boolean")), "mutation.contributorRemove").arg("github", nn(ty("String"))),
            Field::resolved("folderCreate", ty("Folder"), "mutation.folderCreate").arg("path", nn(ty("String"))),
            Field::resolved("folderMove", ty("Folder"), "mutation.folderMove").arg("src", nn(ty("String"))).arg("dst", nn(ty("String"))),
            Field::resolved("folderDelete", nn(ty("Boolean")), "mutation.folderDelete").arg("path", nn(ty("String"))),
            Field::resolved("fileCreate", ty("File"), "mutation.fileCreate").arg("path", nn(ty("String"))),
            Field::resolved("fileMove", ty("File"), "mutation.fileMove").arg("src", nn(ty("String"))).arg("dst", nn(ty("String"))),
            Field::resolved("fileDelete", nn(ty("Boolean")), "mutation.fileDelete").arg("path", nn(ty("String"))),
            Field::resolved("sectionCreate", ty("Section"), "mutation.sectionCreate").arg("file", nn(ty("String"))).arg("name", nn(ty("String"))).arg("parent", ty("String")),
            Field::resolved("sectionMove", ty("Section"), "mutation.sectionMove").arg("file", nn(ty("String"))).arg("oldName", nn(ty("String"))).arg("newName", nn(ty("String"))),
            Field::resolved("sectionDelete", nn(ty("Boolean")), "mutation.sectionDelete").arg("file", nn(ty("String"))).arg("name", nn(ty("String"))),
            Field::resolved("integrate", ty("File"), "mutation.integrate")
                .arg("source", nn(ty("String")))
                .arg("targetSection", nn(ty("String")))
                .arg("targetFile", nn(ty("String")))
                .arg("targetParent", ty("String")),
            Field::resolved("extract", ty("File"), "mutation.extract").arg("sourceFile", nn(ty("String"))).arg("sourceSection", nn(ty("String"))).arg("targetFile", nn(ty("String"))),
        ],
    })]
}
//#endregion 🏗️SchemaRoots
