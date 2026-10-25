//! 🖥️ The codebase aggregate: bundles, folders, files, sections, definitions, breaches and contributor rollups.

use super::*;

//#region 🖥️Codebase Aggregate

/// 💿️ Entity counts of one bundle in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BundleMetricsInternal {
    #[serde(rename = "folders")]
    pub folders: i64,
    #[serde(rename = "files")]
    pub files: i64,
    #[serde(rename = "sections")]
    pub sections: i64,
    #[serde(rename = "definitions")]
    pub definitions: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "breachs")]
    pub breachs: i64,
}

/// 📁️ Entity counts of one folder in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FolderMetricsInternal {
    #[serde(rename = "files")]
    pub files: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "breachs")]
    pub breachs: i64,
}

/// 📄️ Entity counts of one file in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileMetricsInternal {
    #[serde(rename = "sections")]
    pub sections: i64,
    #[serde(rename = "definitions")]
    pub definitions: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
}

/// 📑️ Entity counts of one section in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SectionMetricsInternal {
    #[serde(rename = "definitions")]
    pub definitions: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "breachs")]
    pub breachs: i64,
}

/// 📖️ Entity counts of one definition in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DefinitionMetricsInternal {
    #[serde(rename = "definitions")]
    pub definitions: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "breachs")]
    pub breachs: i64,
}

/// 🔷️ A line and column position.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RangePosition {
    #[serde(rename = "line")]
    pub line: i64,
    #[serde(rename = "column")]
    pub column: i64,
}

/// 🔶️ A start/end position pair inside a file.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct FileRange {
    #[serde(rename = "start")]
    pub start: RangePosition,
    #[serde(rename = "end")]
    pub end: RangePosition,
}

/// 🔹️ The file a codebase breach points at.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BreachFile {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
    pub range: Option<FileRange>,
}

/// 🔸️ The folder a codebase breach points at.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BreachFolder {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
}

/// 🔺️ A breach as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseBreach {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<BreachFolder>,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<BreachFile>,
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "priority")]
    pub priority: BreachPriority,
    #[serde(rename = "autofixable")]
    pub autofixable: bool,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "solution")]
    pub solution: String,
}

/// 📦️ A bundle as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseBundle {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "folder")]
    pub folder: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "contributors", default, skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<String>,
    #[serde(rename = "tickets", default, skip_serializing_if = "Vec::is_empty")]
    pub tickets: Vec<String>,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<BundleMetricsInternal>,
}

/// 🔻️ A folder as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseFolder {
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
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<FolderMetricsInternal>,
}

/// 🔗️ The statute a file-level breach reference carries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileBreachRef {
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "priority")]
    pub priority: BreachPriority,
    #[serde(rename = "autofixable")]
    pub autofixable: bool,
    #[serde(rename = "solution")]
    pub solution: String,
}

/// ⬛️ A file as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseFile {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "folderId", default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "bundleId", default, skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<FileMetricsInternal>,
    #[serde(rename = "breachs", default, skip_serializing_if = "Vec::is_empty")]
    pub breachs: Vec<FileBreachRef>,
}

/// ⬜️ A section as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseSection {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<SectionMetricsInternal>,
}

/// 🟥️ A definition as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseDefinition {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<DefinitionMetricsInternal>,
}

/// 🤝️ A contributor's count metrics against one bundle in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorBundleContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// 🟧️ A contributor's count metrics against one folder in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorFolderContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// 🟨️ A contributor's line metrics against one file in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorFileContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<LineMetrics>,
}

/// 🟩️ A contributor's line metrics against one section in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorSectionContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<LineMetrics>,
}

/// 🟦️ A contributor's line metrics against one definition in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContributorDefinitionContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<LineMetrics>,
}

/// 🟪️ The full contribution index of a contributor in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorContributionsInternal {
    #[serde(rename = "bundles", default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<ContributorBundleContrib>,
    #[serde(rename = "folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<ContributorFolderContrib>,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<ContributorFileContrib>,
    #[serde(rename = "sections", default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<ContributorSectionContrib>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<ContributorDefinitionContrib>,
}

/// 🟫️ Aggregate counts of a contributor in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ContributorMetricsInternal {
    #[serde(rename = "checkpoints")]
    pub checkpoints: i64,
    #[serde(rename = "tickets")]
    pub tickets: i64,
    #[serde(rename = "bundles")]
    pub bundles: i64,
    #[serde(rename = "folders")]
    pub folders: i64,
    #[serde(rename = "files")]
    pub files: i64,
    #[serde(rename = "lines")]
    pub lines: i64,
    #[serde(rename = "sections")]
    pub sections: i64,
    #[serde(rename = "definitions")]
    pub definitions: i64,
}

/// 💠️ A contributor as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseContributor {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "name", default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(rename = "icons", default, skip_serializing_if = "Option::is_none")]
    pub icons: Option<ContributorIcons>,
    #[serde(rename = "emails", default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    #[serde(rename = "links", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, String>,
    #[serde(rename = "contributions", default, skip_serializing_if = "Option::is_none")]
    pub contributions: Option<ContributorContributionsInternal>,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<ContributorMetricsInternal>,
}

/// 🎫️ The created/finished dates of a ticket in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketDateInfo {
    #[serde(rename = "created", default, skip_serializing_if = "String::is_empty")]
    pub created: String,
    #[serde(rename = "finished", default, skip_serializing_if = "String::is_empty")]
    pub finished: String,
}

/// 🔳️ A ticket's count metrics against one bundle in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketBundleContribInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// 🔲️ A ticket's count metrics against one folder in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFolderContribInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// ▪️ A ticket's count metrics against one file in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketFileContribInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// ▫️ A ticket's count metrics against one section in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketSectionContribInfo {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<CountMetrics>,
}

/// ◾ A ticket's line metrics against one definition in a snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketDefinitionContrib {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metrics", default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<LineMetrics>,
}

/// 🗃️ A ticket as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebaseTicket {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<TicketDateInfo>,
    #[serde(rename = "checkpoint", default, skip_serializing_if = "String::is_empty")]
    pub checkpoint: String,
    #[serde(rename = "year")]
    pub year: String,
    #[serde(rename = "month")]
    pub month: String,
    #[serde(rename = "day")]
    pub day: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "prompt", default, skip_serializing_if = "String::is_empty")]
    pub prompt: String,
    #[serde(rename = "llm", default, skip_serializing_if = "String::is_empty")]
    pub llm: String,
    #[serde(rename = "author", default, skip_serializing_if = "String::is_empty")]
    pub author: String,
    #[serde(rename = "status")]
    pub status: TicketStatus,
    #[serde(rename = "bundles", default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<TicketBundleContribInfo>,
    #[serde(rename = "folders", default, skip_serializing_if = "Vec::is_empty")]
    pub folders: Vec<TicketFolderContribInfo>,
    #[serde(rename = "files", default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<TicketFileContribInfo>,
    #[serde(rename = "sections", default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<TicketSectionContribInfo>,
    #[serde(rename = "definitions", default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<TicketDefinitionContrib>,
}

/// 📜️ The statute a policy-level breach reference carries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PolicyBreachRef {
    #[serde(rename = "kind")]
    pub kind: Statute,
    #[serde(rename = "priority")]
    pub priority: BreachPriority,
    #[serde(rename = "autofixable")]
    pub autofixable: bool,
    #[serde(rename = "solution")]
    pub solution: String,
}

/// ◽ A policy as stored in a codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CodebasePolicy {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "scopes", default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "breachs", default, skip_serializing_if = "Vec::is_empty")]
    pub breachs: Vec<PolicyBreachRef>,
}

/// 🌳️ The entity level of a codebase tree node.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CbTreeNodeKind {
    #[serde(rename = "repo")]
    Repo,
    #[serde(rename = "bundle")]
    Bundle,
    #[serde(rename = "folder")]
    Folder,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "definition")]
    Definition,
}

impl CbTreeNodeKind {
    /// 🔤️ The canonical wire string.
    pub fn as_str(&self) -> &'static str {
        match self {
            CbTreeNodeKind::Repo => "repo",
            CbTreeNodeKind::Bundle => "bundle",
            CbTreeNodeKind::Folder => "folder",
            CbTreeNodeKind::File => "file",
            CbTreeNodeKind::Section => "section",
            CbTreeNodeKind::Definition => "definition",
        }
    }
}

impl fmt::Display for CbTreeNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// 🌿️ One node of the codebase containment tree.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CbTreeNode {
    #[serde(rename = "kind")]
    pub kind: CbTreeNodeKind,
    #[serde(rename = "children", default, skip_serializing_if = "BTreeMap::is_empty")]
    pub children: BTreeMap<String, CbTreeNode>,
}

/// ◻ A complete codebase snapshot.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Codebase {
    #[serde(rename = "bundles")]
    pub bundles: Option<Vec<CodebaseBundle>>,
    #[serde(rename = "folders")]
    pub folders: Option<Vec<CodebaseFolder>>,
    #[serde(rename = "files")]
    pub files: Option<Vec<CodebaseFile>>,
    #[serde(rename = "sections")]
    pub sections: Option<Vec<CodebaseSection>>,
    #[serde(rename = "definitions")]
    pub definitions: Option<Vec<CodebaseDefinition>>,
    #[serde(rename = "contributors")]
    pub contributors: Option<Vec<CodebaseContributor>>,
    #[serde(rename = "tickets")]
    pub tickets: Option<Vec<CodebaseTicket>>,
    #[serde(rename = "policies")]
    pub policies: Option<Vec<CodebasePolicy>>,
    #[serde(rename = "breachs")]
    pub breachs: Option<Vec<CodebaseBreach>>,
    #[serde(rename = "tree")]
    pub tree: Option<BTreeMap<String, CbTreeNode>>,
}

//#endregion 🖥️Codebase Aggregate
