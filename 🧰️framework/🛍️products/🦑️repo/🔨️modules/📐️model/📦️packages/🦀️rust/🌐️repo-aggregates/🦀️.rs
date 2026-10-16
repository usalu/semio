//! 🌐️ The repo, contributor, checkpoint and interaction aggregates with their metrics.

use super::*;

//#region 🔢️Metrics

/// 💿️ A closed line range inside a file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Range {
    #[serde(rename = "start")]
    pub start: i64,
    #[serde(rename = "end")]
    pub end: i64,
}

/// 🟨️ Added and removed line counts.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LineMetrics {
    #[serde(rename = "added")]
    pub added: i64,
    #[serde(rename = "removed")]
    pub removed: i64,
}

/// 🟩️ The concrete line numbers a diff touched.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DiffLines {
    #[serde(rename = "Added")]
    pub added: Option<Vec<i64>>,
    #[serde(rename = "Removed")]
    pub removed: Option<Vec<i64>>,
}

/// 🟦️ Added, updated and removed entity counts.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CountMetrics {
    #[serde(rename = "added")]
    pub added: i64,
    #[serde(rename = "updated")]
    pub updated: i64,
    #[serde(rename = "removed")]
    pub removed: i64,
}

/// 🟫️ Breach counts split by priority.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PriorityCount {
    #[serde(rename = "high")]
    pub high: i64,
    #[serde(rename = "medium")]
    pub medium: i64,
    #[serde(rename = "low")]
    pub low: i64,
}

/// 🔬️ Aggregate breach statistics of an analyze run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AnalyzeMetrics {
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "byPriority")]
    pub by_priority: Option<PriorityCount>,
    #[serde(rename = "autofixable")]
    pub autofixable: i64,
}

//#endregion 🔢️Metrics

//#region 🌐️Repo Aggregate

/// 💠️ The repository itself: the root of every artifact id.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Repo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "technologies")]
    pub technologies: Option<Vec<Technology>>,
    #[serde(rename = "bundles")]
    pub bundles: Option<Vec<Bundle>>,
}

/// 📜️ A technology: the first grouping level under the repo root.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Technology {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "root")]
    pub root: String,
    #[serde(rename = "kind")]
    pub kind: TechnologyKind,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "bundles")]
    pub bundles: Option<Vec<Bundle>>,
}

/// 🔵️ A bundle: one buildable unit inside a technology.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Bundle {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "root")]
    pub root: String,
    #[serde(rename = "sourceRoot", default, skip_serializing_if = "String::is_empty")]
    pub source_root: String,
    #[serde(rename = "technologyName")]
    pub technology_name: String,
    #[serde(rename = "tags", default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "kind")]
    pub kind: BundleKind,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "packages", default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<Package>,
}

/// 🔴️ One published package produced by a bundle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Package {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "kind")]
    pub kind: String,
}

/// 🩷️ A directory inside the repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Folder {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "bundleId", default, skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "kind")]
    pub kind: FolderKind,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "ignored")]
    pub ignored: bool,
    #[serde(rename = "generated")]
    pub generated: bool,
}

/// 📄️ A file inside the repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct File {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "extension")]
    pub extension: String,
    #[serde(rename = "folderId", default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "bundleId", default, skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "ignored")]
    pub ignored: bool,
    #[serde(rename = "generated")]
    pub generated: bool,
}

/// 💗️ A named region inside a file, possibly nested.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Section {
    #[serde(rename = "id", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "path", default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(rename = "filePath", default, skip_serializing_if = "String::is_empty")]
    pub file_path: String,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[serde(rename = "endLine")]
    pub end_line: i64,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "endIndex")]
    pub end_index: i64,
    #[serde(rename = "children", default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Section>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<Definition>,
}

/// 💕️ A single declaration inside a section.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Definition {
    #[serde(rename = "id", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "kind")]
    pub kind: DefinitionKind,
    #[serde(rename = "filePath", default, skip_serializing_if = "String::is_empty")]
    pub file_path: String,
    #[serde(rename = "sectionPath", default, skip_serializing_if = "String::is_empty")]
    pub section_path: String,
    #[serde(rename = "emoji")]
    pub emoji: String,
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[serde(rename = "endLine")]
    pub end_line: i64,
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[serde(rename = "endIndex")]
    pub end_index: i64,
}

//#endregion 🌐️Repo Aggregate

//#region 🧑️Contributor Aggregate

/// 🤝️ Rendered avatar links of a contributor.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorIcons {
    #[serde(rename = "avatar", default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "avatarRound", default, skip_serializing_if = "Option::is_none")]
    pub avatar_round: Option<String>,
    #[serde(rename = "github", default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
}

/// 🔗️ A named external link of a contributor.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorLink {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
}

/// 🤍️ A ticket a contributor worked on, as stored in the contributor record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorTicket {
    #[serde(rename = "year")]
    pub year: i64,
    #[serde(rename = "month")]
    pub month: i64,
    #[serde(rename = "day")]
    pub day: i64,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "status")]
    pub status: TicketStatus,
    #[serde(rename = "filePath", default, skip_serializing_if = "String::is_empty")]
    pub file_path: String,
}

/// 🔀️ A checkpoint a contributor authored, as stored in the contributor record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorCheckpoint {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "sha")]
    pub sha: String,
}

/// 🖤️ The persisted contribution index of a contributor.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorContributionsStorage {
    #[serde(rename = "bundles", default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<String>,
    #[serde(rename = "folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<String>,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(rename = "regions", default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<String>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<String>,
    #[serde(rename = "tickets", default, skip_serializing_if = "Vec::is_empty")]
    pub tickets: Vec<ContributorTicket>,
    #[serde(rename = "checkpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoints: Vec<ContributorCheckpoint>,
    #[serde(rename = "lines", default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<LineMetrics>,
}

/// 🧑️‍💻️ A person or agent that contributes to the repository.
///
/// The four members Go writes unconditionally (`alias`, `github`, `name`, `email`) carry
/// `default` on the way in: Go's `encoding/json` leaves a missing member at its zero value rather
/// than refusing the document, and three of the committed `🧑️‍💻️contributor.json` in this
/// repository predate `email` being written at all. Encoding is unchanged — all four are still
/// emitted, matching the `json:"…"` tags that carry no `omitempty`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Contributor {
    #[serde(rename = "alias", default)]
    pub alias: String,
    #[serde(rename = "aliases", default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(rename = "emoji", default, skip_serializing_if = "String::is_empty")]
    pub emoji: String,
    #[serde(rename = "github", default)]
    pub github: String,
    #[serde(rename = "githubs", default, skip_serializing_if = "Vec::is_empty")]
    pub githubs: Vec<String>,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "names", default, skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "emails", default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    #[serde(rename = "links", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, String>,
    #[serde(rename = "fingerprint", default, skip_serializing_if = "String::is_empty")]
    pub fingerprint: String,
    #[serde(rename = "fingerprints", default, skip_serializing_if = "Vec::is_empty")]
    pub fingerprints: Vec<String>,
    #[serde(rename = "contributions", default)]
    pub contributions: ContributorContributionsStorage,
}

/// 🔖️ Line totals a contributor owns inside one definition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorDefinition {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
}

/// 🔖️ Line totals a contributor owns inside one section.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorSection {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
    #[serde(rename = "Definitions")]
    pub definitions: Option<Vec<ContributorDefinition>>,
}

/// 🔖️ Line totals a contributor owns inside one file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorFile {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
    #[serde(rename = "Sections")]
    pub sections: Option<Vec<ContributorSection>>,
}

/// 🔖️ Line totals a contributor owns inside one folder.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorFolder {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
    #[serde(rename = "Files")]
    pub files: Option<Vec<ContributorFile>>,
}

/// 🔖️ Line totals a contributor owns inside one bundle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorBundle {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Lines")]
    pub lines: LineMetrics,
    #[serde(rename = "Folders")]
    pub folders: Option<Vec<ContributorFolder>>,
}

/// 🌳️ The expanded contribution tree of a contributor.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorContributionsTree {
    #[serde(rename = "Checkpoints")]
    pub checkpoints: Option<Vec<Checkpoint>>,
    #[serde(rename = "Tickets")]
    pub tickets: Option<Vec<Ticket>>,
    #[serde(rename = "Bundles")]
    pub bundles: Option<Vec<ContributorBundle>>,
}

/// 🔖️ Count metrics of a contributor against one bundle.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionBundle {
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<CountMetrics>,
}

/// 🔖️ Count metrics of a contributor against one folder.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionFolder {
    #[serde(rename = "folderId")]
    pub folder_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<CountMetrics>,
}

/// 🔖️ Line metrics of a contributor against one file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionFile {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<LineMetrics>,
}

/// 🔖️ Line metrics of a contributor against one section.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionSection {
    #[serde(rename = "sectionId")]
    pub section_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<LineMetrics>,
}

/// 🔖️ Line metrics of a contributor against one definition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributionDefinition {
    #[serde(rename = "definitionId")]
    pub definition_id: String,
    #[serde(rename = "metrics")]
    pub metrics: Option<LineMetrics>,
}

/// 🔖️ The contribution index of a contributor, keyed by artifact id.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorContributions {
    #[serde(rename = "bundles")]
    pub bundles: Option<Vec<ContributionBundle>>,
    #[serde(rename = "folders")]
    pub folders: Option<Vec<ContributionFolder>>,
    #[serde(rename = "files")]
    pub files: Option<Vec<ContributionFile>>,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<ContributionSection>>,
    #[serde(rename = "definitions")]
    pub definitions: Option<Vec<ContributionDefinition>>,
}

//#endregion 🧑️Contributor Aggregate

//#region 🔀️Checkpoint Aggregate

/// ✔️ A committed checkpoint of the repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Checkpoint {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "authorId", default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(rename = "date")]
    pub date: String,
}

/// 💾️ A from/to pair for a renamed entity in a checkpoint diff.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CheckpointDiffRename {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
}

/// 🆕️ Deleted, renamed, modified and created entities of one diff category.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CheckpointDiffStats {
    #[serde(rename = "deleted", default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<String>,
    #[serde(rename = "renamed", default, skip_serializing_if = "Vec::is_empty")]
    pub renamed: Vec<CheckpointDiffRename>,
    #[serde(rename = "modified", default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<String>,
    #[serde(rename = "created", default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<String>,
}

/// 📁️ The full entity diff of a checkpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CheckpointDiff {
    #[serde(rename = "technologies", default)]
    pub technologies: CheckpointDiffStats,
    #[serde(rename = "bundles", default)]
    pub bundles: CheckpointDiffStats,
    #[serde(rename = "folders", default)]
    pub folders: CheckpointDiffStats,
    #[serde(rename = "files", default)]
    pub files: CheckpointDiffStats,
    #[serde(rename = "sections", default)]
    pub sections: CheckpointDiffStats,
    #[serde(rename = "definitions", default)]
    pub definitions: CheckpointDiffStats,
}

//#endregion 🔀️Checkpoint Aggregate

//#region 💬️Interaction Aggregate

/// 📄️ A file an interaction touched.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InteractionFile {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "id", default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(rename = "uri", default, skip_serializing_if = "String::is_empty")]
    pub uri: String,
}

/// 🟫️ One recorded exchange between a contributor and the repository.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Interaction {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "system")]
    pub system: String,
    #[serde(rename = "client")]
    pub client: String,
    #[serde(rename = "checkpoint")]
    pub checkpoint: String,
    #[serde(rename = "prompt", default, skip_serializing_if = "String::is_empty")]
    pub prompt: String,
    #[serde(rename = "summary", default, skip_serializing_if = "String::is_empty")]
    pub summary: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "effort", default, skip_serializing_if = "String::is_empty")]
    pub effort: String,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<InteractionFile>,
}

/// 🎁️ An interaction flattened together with the artifact it was recorded on.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InteractionResource {
    #[serde(flatten)]
    pub interaction: Interaction,
    #[serde(rename = "sourceKind")]
    pub source_kind: String,
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "goalId", default, skip_serializing_if = "String::is_empty")]
    pub goal_id: String,
    #[serde(rename = "ticketId", default, skip_serializing_if = "String::is_empty")]
    pub ticket_id: String,
}

//#endregion 💬️Interaction Aggregate
