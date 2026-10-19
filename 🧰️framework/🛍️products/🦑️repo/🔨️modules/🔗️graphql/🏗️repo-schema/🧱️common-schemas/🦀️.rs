//! 🧱️ Common schemas of the repo GraphQL schema: the shared enums and metrics. See [`super`].

use super::*;

//#region 🏗️SchemaCommon
/// 🔢️ The enum types the repo schema declares.
pub(super) fn schema_enums() -> Vec<NamedType> {
    let enumeration = |name: &str, values: &[(&str, &str)]| {
        NamedType::Enum(EnumType { name: name.to_string(), values: values.iter().map(|(member, wire)| ((*member).to_string(), (*wire).to_string())).collect() })
    };
    vec![
        enumeration("DefinitionKind", &[("IMPLEMENTATION", "implementation"), ("INTERFACE", "interface"), ("CONSTANT", "constant")]),
        enumeration("BundleKind", &[("LIBRARY", "library"), ("SCHEMA", "schema"), ("BINARY", "binary"), ("UI", "ui"), ("SITE", "site"), ("ASSETS", "assets"), ("REPO", "repo")]),
        enumeration("FolderKind", &[("ORGANIZATION", "organization"), ("REQUIRED", "required")]),
        enumeration("TicketStatus", &[("OPEN", "open"), ("CLOSED", "closed")]),
        enumeration(
            "TicketClient",
            &[
                ("COPILOT_CHAT", "copilot-chat"),
                ("WINDSURF", "windsurf"),
                ("WINDSURF_CHAT", "windsurf-chat"),
                ("ANTIGRAVITY", "antigravity"),
                ("ANTIGRAVITY_CHAT", "antigravity-chat"),
                ("CURSOR", "cursor"),
                ("CURSOR_CHAT", "cursor-chat"),
                ("VSCODE", "vscode"),
                ("CLAUDE_CODE", "claude-code"),
                ("CODEX", "codex"),
                ("DROID", "droid"),
                ("KIRO_CLI", "kiro-cli"),
            ],
        ),
        enumeration("BreachPriority", &[("HIGH", "high"), ("MEDIUM", "medium"), ("LOW", "low")]),
        enumeration(
            "FileKind",
            &[
                ("CODE", "code"),
                ("SCRIPT", "script"),
                ("CONFIG", "config"),
                ("LAB", "lab"),
                ("DOCS", "docs"),
                ("RESOURCE", "resource"),
                ("TEMPLATE", "template"),
                ("LICENSE", "license"),
            ],
        ),
    ]
}

/// 🔢️ The metric objects every aggregate shares.
pub(super) fn schema_metrics() -> Vec<NamedType> {
    vec![
        NamedType::Object(ObjectType {
            name: "Range".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("start", nn(ty("Int"))), Field::new("end", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "CountMetrics".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("added", nn(ty("Int"))), Field::new("updated", nn(ty("Int"))), Field::new("removed", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "PriorityCount".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("high", nn(ty("Int"))), Field::new("medium", nn(ty("Int"))), Field::new("low", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "AnalyzeMetrics".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("total", nn(ty("Int"))), Field::new("byPriority", ty("PriorityCount")), Field::new("autofixable", nn(ty("Int")))],
        }),
        NamedType::Object(ObjectType {
            name: "AnalyzeResult".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("breachs", nn_list(ty("Breach"))), Field::new("metrics", nn(ty("AnalyzeMetrics")))],
        }),
        NamedType::Object(ObjectType {
            name: "FixResult".to_string(),
            interfaces: Vec::new(),
            fields: vec![Field::new("fixed", nn(ty("Int"))), Field::new("remaining", nn(ty("Int"))), Field::new("breachs", nn_list(ty("Breach")))],
        }),
    ]
}
//#endregion 🏗️SchemaCommon
