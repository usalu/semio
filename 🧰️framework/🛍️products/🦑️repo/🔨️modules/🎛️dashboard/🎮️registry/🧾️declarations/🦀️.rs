//! 🧾️ Declarations of the registry: the parameter kind, the wire-shaped target, tool, compound, group, project and ticket declarations and the problem record. See [`super`].

use super::*;

// #region 🔖️Declarations
/// 🎚️ The three kinds of things chosen at launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterKind { Choice, Text, Flag }

/// 🔎️ Which current physical source values a text field can suggest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SuggestionSource { Projects, Scripts, Tests, Configs }

/// 🧭️ Related editable fields filled by accepting an authored source suggestion.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SuggestionSpec { pub source: SuggestionSource, #[serde(skip_serializing_if = "Option::is_none")] pub project_parameter: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] pub directory_parameter: Option<String> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct AppliesTo { pub(super) all: Option<bool>, pub(super) verbs: Option<Vec<String>>, pub(super) playground: Option<bool> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ValueDeclaration { pub(super) id: String, pub(super) env: Option<BTreeMap<String, String>>, pub(super) build_flags: Option<Vec<String>>, pub(super) args: Option<Vec<String>> }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ParameterDeclaration {
    #[serde(default, deserialize_with = "localized_declaration")] pub(super) title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] pub(super) description: Option<LocalizedText>,
    pub(super) id: String,
    pub(super) kind: ParameterKind,
    pub(super) applies_to: Option<AppliesTo>,
    pub(super) default: Option<serde_json::Value>,
    pub(super) required: Option<bool>,
    pub(super) values: Option<Vec<ValueDeclaration>>,
    pub(super) env: Option<BTreeMap<String, String>>,
    pub(super) build_flags: Option<Vec<String>>,
    pub(super) args: Option<Vec<String>>,
    pub(super) value_env: Option<String>,
    pub(super) value_flag: Option<String>,
    pub(super) value_positional: Option<bool>,
    pub(super) suggestions: Option<SuggestionSpec>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ReadyDeclaration { pub(super) port: Option<u32>, pub(super) port_env: Option<String>, pub(super) path: Option<String>, pub(super) printed: Option<bool> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct TargetDeclaration { #[serde(default, deserialize_with = "localized_declaration")] pub(super) title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] pub(super) description: Option<LocalizedText>, pub(super) verb: Option<String>, pub(super) ready: Option<ReadyDeclaration>, #[serde(default)] pub(super) requires: Vec<MemberDeclaration>, #[serde(default)] pub(super) parameters: Vec<ParameterDeclaration> }

/// 🔌️ `#/$defs/McpExposure`: the tool is an MCP stdio server the listed agent clients start.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mcp { pub server: String, pub clients: BTreeMap<String, McpClient> }

/// 🤖️ What one agent client states for the server: the value of each parameter it passes.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct McpClient { #[serde(default, skip_serializing_if = "BTreeMap::is_empty")] pub parameters: BTreeMap<String, String> }

/// 🧑‍💻️ The agent clients whose MCP configuration the workspace derives (`#/$defs/AgentClientId`).
pub const AGENT_CLIENTS: [&str; 2] = ["claude-code", "codex"];

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ToolDeclaration { #[serde(default, deserialize_with = "localized_declaration")] pub(super) title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] pub(super) description: Option<LocalizedText>, pub(super) id: String, pub(super) verb: Option<String>, pub(super) command: Vec<String>, #[serde(default)] pub(super) cwd: String, #[serde(default)] pub(super) env: BTreeMap<String, String>, #[serde(default)] pub(super) continuous: bool, pub(super) ready: Option<ReadyDeclaration>, #[serde(default)] pub(super) requires: Vec<MemberDeclaration>, #[serde(default)] pub(super) parameters: Vec<ParameterDeclaration>, pub(super) mcp: Option<Mcp> }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MemberObject { pub(super) run: String, #[serde(default)] pub(super) parameters: BTreeMap<String, serde_json::Value>, #[serde(default)] pub(super) env: BTreeMap<String, String> }

/// 🔗️ A reference to another command, plain or with the parameters and environment it is pinned to:
/// the one shape of `requires` entries and compound members.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct MemberDeclaration { pub(super) run: String, pub(super) parameters: BTreeMap<String, serde_json::Value>, pub(super) env: BTreeMap<String, String> }

impl<'de> Deserialize<'de> for MemberDeclaration {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::String(run) => Ok(Self { run, parameters: BTreeMap::new(), env: BTreeMap::new() }),
            other => serde_json::from_value::<MemberObject>(other).map(|object| Self { run: object.run, parameters: object.parameters, env: object.env }).map_err(serde::de::Error::custom),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct CompoundDeclaration { #[serde(default, deserialize_with = "localized_declaration")] pub(super) title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] pub(super) description: Option<LocalizedText>, pub(super) id: String, pub(super) verb: Option<String>, pub(super) stop: Option<Stop>, pub(super) members: Vec<MemberDeclaration> }

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct GroupDeclaration { #[serde(default, deserialize_with = "localized_declaration")] pub(super) title: Option<LocalizedText>, #[serde(default, deserialize_with = "localized_declaration")] pub(super) description: Option<LocalizedText>, pub(super) id: String, pub(super) targets: Vec<String>, pub(super) projects: Vec<String> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct ProjectDeclaration { #[serde(default)] pub(super) parameters: Vec<ParameterDeclaration>, #[serde(default)] pub(super) tools: Vec<ToolDeclaration>, #[serde(default)] pub(super) compounds: Vec<CompoundDeclaration>, #[serde(default)] pub(super) groups: Vec<GroupDeclaration> }

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TicketDeclaration { #[serde(rename = "$schema")] _schema: Option<String>, #[serde(default)] pub(super) tools: Vec<ToolDeclaration>, #[serde(default)] pub(super) compounds: Vec<CompoundDeclaration> }

/// 🚨️ One defect of a declaration or of a reference, with the file that declares it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Problem { pub file: String, pub at: String, pub message: String }

impl std::fmt::Display for Problem {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts = [self.file.as_str(), self.at.as_str(), self.message.as_str()];
        formatter.write_str(&parts.iter().filter(|part| !part.is_empty()).copied().collect::<Vec<_>>().join(": "))
    }
}
// #endregion 🔖️Declarations
