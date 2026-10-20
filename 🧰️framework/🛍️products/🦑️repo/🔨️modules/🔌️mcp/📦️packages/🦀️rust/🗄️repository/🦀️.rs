//! 🗄️ Repository: the per-client profile, the repository port and the tool, resource and prompt surface.
//!
//! Holds the `🗄️Repository` region of the crate root; the root re-exports its items.

use super::*;

//#region 🗄️Repository

/// 🪪️ The per-IDE surface selected by `SEMIO_REPO_MCP_CLIENT`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Profile {
    #[default]
    Generic,
    Cursor,
    Kiro,
    Copilot,
    Claude,
    Codex,
}

impl Profile {
    /// 🔤️ Parses a profile slug; the empty string and `client` both mean the generic surface.
    pub fn parse(raw: &str) -> Result<Profile, Error> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "" | "generic" | "client" => Ok(Profile::Generic),
            "cursor" => Ok(Profile::Cursor),
            "kiro" => Ok(Profile::Kiro),
            "copilot" => Ok(Profile::Copilot),
            "claude" => Ok(Profile::Claude),
            "codex" => Ok(Profile::Codex),
            other => Err(Error::Invalid(format!("unknown mcp kind {other:?} (expected client, cursor, copilot, claude, codex, or kiro)"))),
        }
    }

    /// 🏷️ The stable kind slug used as the description-table column.
    pub fn kind(self) -> &'static str {
        match self {
            Profile::Generic => "generic",
            Profile::Cursor => "cursor",
            Profile::Kiro => "kiro",
            Profile::Copilot => "copilot",
            Profile::Claude => "claude",
            Profile::Codex => "codex",
        }
    }

    /// 🖥️ The server name reported in `initialize`.
    pub fn server_name(self) -> &'static str {
        match self {
            Profile::Generic | Profile::Cursor | Profile::Kiro | Profile::Copilot | Profile::Claude | Profile::Codex => "repo",
        }
    }
}

const DESCRIPTIONS_JSON: &str = include_str!("../../../🖼️assets/🔣️descriptions.json");

static DESCRIPTIONS: OnceLock<BTreeMap<String, BTreeMap<String, String>>> = OnceLock::new();

/// 🗣️ The per-profile description table, authored once at `🔌️mcp/🖼️assets/🔣️descriptions.json`.
pub fn descriptions() -> &'static BTreeMap<String, BTreeMap<String, String>> {
    DESCRIPTIONS.get_or_init(|| {
        let parsed: serde_json::Value = serde_json::from_str(DESCRIPTIONS_JSON).expect("description table must parse");
        let mut table = BTreeMap::new();
        if let Some(members) = parsed.get("descriptions").and_then(serde_json::Value::as_object) {
            for (key, value) in members {
                let mut column = BTreeMap::new();
                if let Some(kinds) = value.as_object() {
                    for (kind, text) in kinds {
                        column.insert(kind.clone(), text.as_str().unwrap_or_default().to_string());
                    }
                }
                table.insert(key.clone(), column);
            }
        }
        table
    })
}

/// 🗣️ Picks one description for one profile, falling back to the generic wording.
pub fn describe(profile: Profile, key: &str) -> String {
    let Some(column) = descriptions().get(key) else { return String::new() };
    if let Some(text) = column.get(profile.kind()) {
        if !text.is_empty() {
            return text.clone();
        }
    }
    column.get("generic").cloned().unwrap_or_default()
}

/// 🧰️ What a domain tool returns to the protocol layer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RepositoryResult {
    pub text: String,
    pub structured: Option<String>,
    pub is_error: bool,
}

/// 🔗️ The domain boundary. Everything the repo MCP server can do that is not protocol lives behind
/// this trait; the `⌨️cli` crate supplies the production implementation backed by the tickets, goals
/// and move crates.
pub trait Repository: Send + Sync {
    /// 🧰️ Runs one tool. `arguments` is the raw, already-validated-as-object `arguments` member.
    fn call(&self, context: &Context, name: &str, arguments: &serde_json::Value) -> Result<RepositoryResult, HandlerError>;
    /// 📚️ Reads one resource URI.
    fn read(&self, context: &Context, uri: &str) -> Result<ResourceContent, HandlerError>;
    /// 💬️ Renders one prompt.
    fn prompt(&self, context: &Context, name: &str, arguments: &BTreeMap<String, String>) -> Result<GetPromptResult, HandlerError>;
}

/// 🧪️ An in-memory repository that records every call and answers deterministically.
#[derive(Debug, Default)]
pub struct RecordingRepository {
    calls: Mutex<Vec<(String, String)>>,
}

impl RecordingRepository {
    pub fn new() -> RecordingRepository {
        RecordingRepository::default()
    }

    /// 📋️ Every `(surface, argument)` pair observed so far, in order.
    pub fn calls(&self) -> Vec<(String, String)> {
        self.calls.lock().unwrap_or_else(|poison| poison.into_inner()).clone()
    }

    fn record(&self, surface: &str, argument: &str) {
        self.calls.lock().unwrap_or_else(|poison| poison.into_inner()).push((surface.to_string(), argument.to_string()));
    }
}

impl Repository for RecordingRepository {
    fn call(&self, _context: &Context, name: &str, arguments: &serde_json::Value) -> Result<RepositoryResult, HandlerError> {
        if !TOOL_NAMES.contains(&name) {
            return Err(HandlerError::tool("tool not found"));
        }
        self.record("call", name);
        Ok(RepositoryResult { text: name.to_string(), structured: Some(serde_json::to_string(arguments).unwrap_or_else(|_| "null".to_string())), is_error: false })
    }

    fn read(&self, _context: &Context, uri: &str) -> Result<ResourceContent, HandlerError> {
        if !RESOURCE_URIS.contains(&uri) {
            return Err(HandlerError::tool("resource not found"));
        }
        self.record("read", uri);
        Ok(ResourceContent { uri: uri.to_string(), mime_type: "text/plain".to_string(), text: uri.to_string(), blob: String::new() })
    }

    fn prompt(&self, _context: &Context, name: &str, arguments: &BTreeMap<String, String>) -> Result<GetPromptResult, HandlerError> {
        let Some(instruction) = prompt_instruction(name) else { return Err(HandlerError::tool("prompt not found")) };
        self.record("prompt", name);
        let given = arguments.get("prompt").cloned().unwrap_or_default();
        Ok(GetPromptResult { description: instruction.to_string(), messages: vec![("user".to_string(), Content::text(&format!("{instruction}\n\n{given}")))] })
    }
}

/// 💬️ The fixed instruction each prompt prefixes to the caller's own prompt.
pub fn prompt_instruction(name: &str) -> Option<&'static str> {
    match name {
        "enhance" => Some("Enhance the request while preserving its intent and constraints."),
        "refactor" => Some("Refactor the requested scope completely and preserve observable behavior."),
        "test" => Some("Test the requested scope with executable success and hostile cases."),
        "comply" => Some("Apply the repository instructions and resolve every in-scope breach."),
        _ => None,
    }
}

/// 🧰️ Every tool name, in the order `tools/list` sorts them.
pub const TOOL_NAMES: [&str; 6] = ["goal_close", "goal_open", "goal_reopen", "ticket_close", "ticket_open", "ticket_reopen"];

/// 📚️ Every resource URI, in the order `resources/list` sorts them.
pub const RESOURCE_URIS: [&str; 8] = ["repo://", "repo://bundles", "repo://contributors", "repo://files", "repo://folders", "repo://goals", "repo://policies", "repo://tickets"];

/// 💬️ Every prompt name, in the order `prompts/list` sorts them.
pub const PROMPT_NAMES: [&str; 4] = ["comply", "enhance", "refactor", "test"];

/// 🧰️ The advertised tool table for one profile.
pub fn tool_schemas(profile: Profile) -> Vec<Tool> {
    let mut open = vec![
        ("emoji", Schema::string("Ticket emoji.")),
        ("title", Schema::string("Ticket title.")),
        ("prompt", Schema::string("Task description.")),
        ("goal", Schema::string("Goal id.")),
        ("client", Schema::string("Agent client.")),
        ("llm", Schema::string("Model.")),
        ("effort", Schema::string("Reasoning effort.")),
        ("draft", Schema::string("Draft id.")),
        ("parent", Schema::string("Parent ticket.")),
        ("issue", Schema::string("Existing issue URL.")),
        ("no_issue", Schema::boolean("Skip issue creation.")),
        ("no_management", Schema::boolean("Skip management integration.")),
    ];
    let mut reopen = vec![
        ("path", Schema::string("YY/MM/DD/SLUG path.")),
        ("prompt", Schema { min_length: 1, pattern: r"\S".to_string(), ..Schema::string("Additional task description.") }),
        ("client", Schema::string("Agent client.")),
        ("llm", Schema::string("Model.")),
        ("effort", Schema::string("Reasoning effort.")),
        ("draft", Schema::string("Draft id.")),
        ("title", Schema::string("Updated title.")),
        ("goal", Schema::string("Goal id.")),
        ("parent", Schema::string("Parent ticket.")),
        ("no_management", Schema::boolean("Skip management integration.")),
    ];
    match profile {
        Profile::Cursor | Profile::Copilot | Profile::Claude | Profile::Codex => {
            open.push(("plan_id", Schema::string(&describe(profile, "arg_plan_id"))));
            reopen.push(("plan_id", Schema::string(&describe(profile, "arg_plan_id"))));
        }
        Profile::Kiro => {
            open.push(("spec_id", Schema::string(&describe(profile, "arg_spec_id"))));
            reopen.push(("spec_id", Schema::string(&describe(profile, "arg_spec_id"))));
        }
        Profile::Generic => {}
    }
    let tool = |name: &str, schema: Schema| Tool { name: name.to_string(), title: String::new(), description: describe(profile, &format!("tool_{name}")), input_schema: schema, output_schema: Some(Tool::result_output_schema()) };
    vec![
        tool("ticket_open", Schema::object(open, &["emoji", "title", "prompt", "goal"])),
        tool(
            "ticket_close",
            Schema::object(
                vec![
                    ("path", Schema::string("YY/MM/DD/SLUG path.")),
                    ("summary", Schema::string("Completion summary.")),
                    ("files", Schema::array("Changed files.")),
                    ("title", Schema::string("Updated title.")),
                    ("no_management", Schema::boolean("Skip management integration.")),
                ],
                &["summary"],
            ),
        ),
        tool("ticket_reopen", Schema::object(reopen, &["prompt"])),
        tool(
            "goal_open",
            Schema::object(
                vec![
                    ("title", Schema::string("Goal title.")),
                    ("description", Schema::string("Goal description.")),
                    ("prompt", Schema::string("Goal prompt.")),
                    ("due_date", Schema::string("YYYY-MM-DD due date.")),
                    ("llm", Schema::string("Model.")),
                    ("client", Schema::string("Agent client.")),
                    ("parent", Schema::string("Parent goal id.")),
                    ("milestone", Schema::string("Management milestone.")),
                    ("no_management", Schema::boolean("Skip management integration.")),
                ],
                &["title", "prompt"],
            ),
        ),
        tool(
            "goal_close",
            Schema::object(vec![("id", Schema::string("Goal id.")), ("summary", Schema::string("Completion summary.")), ("no_management", Schema::boolean("Skip management integration."))], &["id", "summary"]),
        ),
        tool(
            "goal_reopen",
            Schema::object(
                vec![
                    ("id", Schema::string("Goal id.")),
                    ("prompt", Schema::string("Additional goal prompt.")),
                    ("llm", Schema::string("Model.")),
                    ("client", Schema::string("Agent client.")),
                    ("title", Schema::string("Updated title.")),
                    ("description", Schema::string("Updated description.")),
                    ("due_date", Schema::string("YYYY-MM-DD due date.")),
                    ("no_management", Schema::boolean("Skip management integration.")),
                ],
                &["id", "prompt", "llm", "client"],
            ),
        ),
    ]
}

/// 📚️ The advertised resource table for one profile.
pub fn resource_schemas(profile: Profile) -> Vec<Resource> {
    [("repo://", "repo", "res_root"), ("repo://bundles", "bundles", "res_bundles"), ("repo://folders", "folders", "res_folders"), ("repo://files", "files", "res_files"), ("repo://tickets", "tickets", "res_tickets"), ("repo://goals", "goals", "res_goals"), ("repo://policies", "policies", "res_policies"), ("repo://contributors", "contributors", "res_contributors")]
        .into_iter()
        .map(|(uri, name, key)| Resource { uri: uri.to_string(), name: name.to_string(), title: String::new(), description: describe(profile, key), mime_type: "text/plain".to_string() })
        .collect()
}

/// 💬️ The advertised prompt table for one profile.
pub fn prompt_schemas(profile: Profile) -> Vec<Prompt> {
    ["enhance", "refactor", "test", "comply"]
        .into_iter()
        .map(|name| Prompt {
            name: name.to_string(),
            title: String::new(),
            description: describe(profile, &format!("prompt_{name}")),
            arguments: vec![PromptArgument { name: "prompt".to_string(), description: String::new(), required: true }],
        })
        .collect()
}

/// 🏭️ Builds the production server for one repository and one profile.
pub fn repository_server(repository: impl Repository + 'static, profile: Profile, limits: Limits) -> Result<Server, Error> {
    let repository: Arc<dyn Repository> = Arc::new(repository);
    let mut server = Server::new(Config {
        server_name: profile.server_name().to_string(),
        server_version: "1.0.0".to_string(),
        instructions: "Use repository tools and resources through their owned schemas. Run build tasks with repo run <project>:<target> and provision tools with repo tools setup all. Nx remains the default; select Bazel with --tool=bazel. Nested tasks inherit SEMIO_BUILD_TOOL.".to_string(),
        limits,
    })?;
    for schema in tool_schemas(profile) {
        let name = schema.name.clone();
        let declared = schema.output_schema.is_some();
        let owned = Arc::clone(&repository);
        server.register_tool(
            schema,
            Box::new(move |context, params, progress| {
                progress.report(context, 0.0, None, "started").map_err(|_| HandlerError::tool("progress failed"))?;
                let result = owned.call(context, &name, &params.arguments)?;
                progress.report(context, 1.0, None, "completed").map_err(|_| HandlerError::tool("progress failed"))?;
                Ok(CallToolResult { content: vec![Content::text(&result.text)], structured_content: result.structured, is_error: result.is_error, output_schema_declared: declared })
            }),
        )?;
    }
    for schema in resource_schemas(profile) {
        let uri = schema.uri.clone();
        let owned = Arc::clone(&repository);
        server.register_resource(
            schema,
            Box::new(move |context, _requested, progress| {
                progress.report(context, 0.0, None, "started").map_err(|_| HandlerError::tool("progress failed"))?;
                Ok(vec![owned.read(context, &uri)?])
            }),
        )?;
    }
    for schema in prompt_schemas(profile) {
        let name = schema.name.clone();
        let owned = Arc::clone(&repository);
        server.register_prompt(schema, Box::new(move |context, _requested, arguments| owned.prompt(context, &name, arguments)))?;
    }
    Ok(server)
}

/// 🧪️ The synthetic server the `2️⃣g2-contract.json` golden vectors are recorded against: one `echo`
/// tool, one JSON `repo://goals` resource, one ticket template and one `review` prompt.
pub fn contract_server(limits: Limits) -> Result<Server, Error> {
    let mut server = Server::new(Config { server_name: "repo".to_string(), server_version: "1.0.0".to_string(), instructions: "owned".to_string(), limits })?;
    server.register_tool(
        Tool { name: "echo".to_string(), title: String::new(), description: String::new(), input_schema: Schema { kind: "object".to_string(), ..Schema::default() }, output_schema: None },
        Box::new(|context, params, progress| {
            let text = params.arguments.get("text").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
            progress.report(context, 1.0, None, "echo").map_err(|_| HandlerError::tool("progress failed"))?;
            Ok(CallToolResult { content: vec![Content::text(&text)], structured_content: None, is_error: false, output_schema_declared: false })
        }),
    )?;
    server.register_resource(
        Resource { uri: "repo://goals".to_string(), name: "goals".to_string(), title: String::new(), description: String::new(), mime_type: "application/json".to_string() },
        Box::new(|_context, _uri, _progress| Ok(vec![ResourceContent { uri: "repo://goals".to_string(), mime_type: "application/json".to_string(), text: "[]".to_string(), blob: String::new() }])),
    )?;
    server.register_resource_template(ResourceTemplate { uri_template: "repo://ticket/{id}".to_string(), name: "ticket".to_string(), title: String::new(), description: String::new(), mime_type: String::new() })?;
    server.register_prompt(
        Prompt { name: "review".to_string(), title: String::new(), description: String::new(), arguments: vec![PromptArgument { name: "scope".to_string(), description: String::new(), required: true }] },
        Box::new(|_context, _name, arguments| {
            let scope = arguments.get("scope").cloned().unwrap_or_default();
            Ok(GetPromptResult { description: format!("Review {scope}"), messages: vec![("user".to_string(), Content::text(&scope))] })
        }),
    )?;
    Ok(server)
}

//#endregion 🗄️Repository
