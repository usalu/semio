//! 🎟️ Ticket and contributor schemas of the repo GraphQL schema: the ticket, checkpoint and contributor types. See [`super`].

use super::*;

//#region 🏗️SchemaTicket
/// 🎫️ The ticket type and the date pair it carries.
pub(super) fn schema_ticket() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "TicketDate".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("started", nn(ty("DateTime"))), Field::new("finished", ty("DateTime"))],
        }),
        NamedType::Object(ObjectType {
            name: "Ticket".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("year", nn(ty("Int"))),
                Field::new("month", nn(ty("Int"))),
                Field::new("day", nn(ty("Int"))),
                Field::new("slug", nn(ty("String"))),
                Field::new("path", nn(ty("String"))),
                Field::new("llm", ty("String")),
                Field::new("effort", ty("String")),
                Field::new("client", ty("TicketClient")),
                Field::new("checkpoint", ty("String")),
                Field::new("uri", nn(ty("String"))),
                Field::new("title", nn(ty("String"))),
                Field::new("emoji", ty("String")),
                Field::new("prompt", nn(ty("String"))),
                Field::new("summary", ty("String")),
                Field::new("status", nn(ty("TicketStatus"))),
                Field::new("interactions", nn_list(ty("Interaction"))),
                Field::resolved("author", ty("Contributor"), "ticket.author"),
                Field::new("dates", nn(ty("TicketDate"))),
                Field::new("goal", ty("String")),
                Field::new("parent", ty("String")),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("files", nn_list(ty("File"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "TicketDay".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("day", nn(ty("Int"))), Field::new("tickets", nn_list(ty("Ticket")))],
        }),
        NamedType::Object(ObjectType {
            name: "TicketMonth".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("month", nn(ty("Int"))), Field::new("days", nn_list(ty("TicketDay")))],
        }),
        NamedType::Object(ObjectType {
            name: "TicketYear".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("year", nn(ty("Int"))), Field::new("months", nn_list(ty("TicketMonth")))],
        }),
    ]
}
//#endregion 🏗️SchemaTicket

//#region 🏗️SchemaCheckpoint
/// ✔️ The checkpoint type.
pub(super) fn schema_checkpoint() -> Vec<NamedType> {
    vec![NamedType::Object(ObjectType {
        name: "Checkpoint".to_string(),
        interfaces: Vec::new(),
        fields: vec![
            Field::new("id", nn(ty("ID"))),
            Field::new("sha", nn(ty("String"))),
            Field::new("title", nn(ty("String"))),
            Field::new("date", nn(ty("DateTime"))),
        ],
    })]
}
//#endregion 🏗️SchemaCheckpoint

//#region 🏗️SchemaContributor
/// 🧑️‍💻️ The contributor type and its contribution tree.
pub(super) fn schema_contributor() -> Vec<NamedType> {
    let named = |name: &str, child: &str, plural: &str| {
        NamedType::Object(ObjectType {
            name: name.to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String"))), Field::new(plural, nn_list(ty(child)))],
        })
    };
    vec![
        NamedType::Object(ObjectType {
            name: "ContributorIcons".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("avatar", ty("String")), Field::new("avatarRound", ty("String")), Field::new("github", ty("String"))],
        }),
        NamedType::Object(ObjectType {
            name: "ContributorLink".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String"))), Field::new("url", nn(ty("String")))],
        }),
        NamedType::Object(ObjectType {
            name: "ContributorDefinition".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("name", nn(ty("String")))],
        }),
        named("ContributorSection", "ContributorDefinition", "definitions"),
        named("ContributorFile", "ContributorSection", "sections"),
        named("ContributorFolder", "ContributorFile", "files"),
        named("ContributorBundle", "ContributorFolder", "folders"),
        NamedType::Object(ObjectType {
            name: "ContributorContributions".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("checkpoints", nn_list(ty("Checkpoint"))),
                Field::new("tickets", nn_list(ty("TicketYear"))),
                Field::new("bundles", nn_list(ty("ContributorBundle"))),
            ],
        }),
        NamedType::Object(ObjectType {
            name: "Contributor".to_string(),
            interfaces: Vec::new(),
            fields: vec![
                Field::new("id", nn(ty("ID"))),
                Field::new("github", nn(ty("String"))),
                Field::new("emoji", ty("String")),
                Field::new("name", nn(ty("String"))),
                Field::new("names", nn_list(ty("String"))),
                Field::new("email", nn(ty("String"))),
                Field::new("emails", nn_list(ty("String"))),
                Field::new("fingerprint", ty("String")),
                Field::new("fingerprints", nn_list(ty("String"))),
                Field::new("links", nn_list(ty("ContributorLink"))),
                Field::resolved("contributions", ty("ContributorContributions"), "contributor.contributions"),
                Field::new("icons", ty("ContributorIcons")),
                Field::new("bundles", nn_list(ty("Bundle"))),
                Field::new("files", nn_list(ty("File"))),
                Field::new("tickets", nn_list(ty("Ticket"))),
            ],
        }),
    ]
}
//#endregion 🏗️SchemaContributor
