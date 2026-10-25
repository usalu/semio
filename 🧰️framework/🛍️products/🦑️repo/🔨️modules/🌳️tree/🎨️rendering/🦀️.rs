//! 🎨️ Rendering: text and markdown tree output, the Mermaid treemap and the lines-of-code Mermaid views.
//!
//! Holds the `🎨️Rendering`, `🧜️Mermaid`, `🔢️MermaidLoc` regions of the crate root; the root re-exports its items.

use super::*;

//#region 🎨️Rendering

/// 🖌️ Renders a monorepo tree as indented text, one line per node.
pub fn render_monorepo_tree(root: &TreeNode, renderer: &dyn EntityRenderer) -> String {
    let mut out = String::new();
    let children = children_of(root);
    let last = children.len().saturating_sub(1);
    for (index, child) in children.iter().enumerate() {
        render_tree_node_text(&mut out, child, "", index == last, true, renderer);
    }
    out
}

/// 📰️ Renders a monorepo tree as a markdown list.
pub fn render_monorepo_tree_markdown(root: &TreeNode, renderer: &dyn EntityRenderer) -> String {
    let mut out = String::new();
    for child in children_of(root) {
        render_tree_node_markdown(&mut out, child, "", renderer);
    }
    out
}

/// 🧾️ The pre-order outline of a tree: one `<depth>|<kind>|<id>|<label>` row per node, the
/// implementation-neutral projection every structural assertion compares.
pub fn tree_outline(root: &TreeNode) -> Vec<String> {
    fn walk(node: &TreeNode, depth: usize, rows: &mut Vec<String>) {
        rows.push(format!("{depth}|{}|{}|{}", node.kind.as_str(), node.id, node.label));
        for child in children_of(node) {
            walk(child, depth + 1, rows);
        }
    }
    let mut rows = Vec::new();
    walk(root, 0, &mut rows);
    rows
}

/// 🔺️ Renders one node and its subtree as text.
fn render_tree_node_text(out: &mut String, node: &TreeNode, prefix: &str, is_last: bool, is_root: bool, renderer: &dyn EntityRenderer) {
    let connector = if is_root {
        ""
    } else if is_last {
        TEXT_LAST_CONNECTOR
    } else {
        TEXT_BRANCH_CONNECTOR
    };
    let label = tree_node_text_label(node, renderer);
    out.push_str(prefix);
    out.push_str(connector);
    out.push_str(&label);
    out.push('\n');
    let new_prefix = if is_root {
        prefix.to_string()
    } else if is_last {
        format!("{prefix}{TEXT_BLANK_INDENT}")
    } else {
        format!("{prefix}{TEXT_PIPE_INDENT}")
    };
    let children = children_of(node);
    let last = children.len().saturating_sub(1);
    for (index, child) in children.iter().enumerate() {
        render_tree_node_text(out, child, &new_prefix, index == last, false, renderer);
    }
}

/// 🏷️ The text label of one node: a link for a category, the entity line for everything else.
pub fn tree_node_text_label(node: &TreeNode, renderer: &dyn EntityRenderer) -> String {
    if node.kind == TreeNodeKind::Category {
        if node.uri.is_empty() {
            return node.label.clone();
        }
        return format!("[{}]({})", node.label, node.uri);
    }
    let Some(data) = node.data.as_ref() else { return node.label.clone() };
    let entity_kind = entity_kind_of(node);
    let mut blanked = data.clone();
    blanked.insert("parentId".to_string(), text(""));
    let rendered = renderer.human(&entity_kind, &blanked);
    if rendered.is_empty() {
        node.label.clone()
    } else {
        rendered
    }
}

/// 🔻️ Renders one node and its subtree as markdown.
fn render_tree_node_markdown(out: &mut String, node: &TreeNode, indent: &str, renderer: &dyn EntityRenderer) {
    let line = match (node.kind, node.data.as_ref()) {
        (TreeNodeKind::Category, _) | (_, None) => {
            if node.uri.is_empty() {
                format!("{indent}- {}", node.label)
            } else {
                format!("{indent}- [{}]({})", node.label, node.uri)
            }
        }
        (_, Some(data)) => format!("{indent}{}", renderer.markdown(&entity_kind_of(node), data)),
    };
    out.push_str(&line);
    out.push('\n');
    let child_indent = format!("{indent}  ");
    for child in children_of(node) {
        render_tree_node_markdown(out, child, &child_indent, renderer);
    }
}

/// 🧱️ The entity kind a node renders as, falling back to its own kind slug.
fn entity_kind_of(node: &TreeNode) -> String {
    let entity_kind = tree_node_kind_to_entity_kind(node.kind);
    if entity_kind.is_empty() {
        node.kind.as_str().to_string()
    } else {
        entity_kind.to_string()
    }
}

//#endregion 🎨️Rendering

//#region 🧜️Mermaid

/// 🧜️ One node of a Mermaid `treemap-beta` diagram: a labelled group, or a labelled leaf value.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MermaidNode {
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MermaidNode>,
}

/// 🐚️ A Mermaid `treemap-beta` diagram: a title and its node forest.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MermaidTreemap {
    pub title: String,
    #[serde(default)]
    pub nodes: Vec<MermaidNode>,
}

impl MermaidTreemap {
    /// 📥️ Decodes a treemap from its JSON document.
    pub fn from_json(document: &str) -> Result<MermaidTreemap, String> {
        serde_json::from_str(document).map_err(|error| error.to_string())
    }
}

/// 🔤️ Escapes a Mermaid label: a double quote would close the quoted label, so it becomes an
/// apostrophe, exactly as the Go twin does.
pub fn mermaid_escape_label(value: &str) -> String {
    value.replace('"', "'")
}

/// 🖍️ Renders a `treemap-beta` diagram: the header, the quoted title, then four spaces of indent
/// per level, a quoted label and, for a leaf, `: <value>`.
pub fn render_mermaid_treemap(treemap: &MermaidTreemap) -> String {
    let mut out = String::from("treemap-beta\n");
    out.push_str(&format!("\"{}\"\n", mermaid_escape_label(&treemap.title)));
    for node in &treemap.nodes {
        render_mermaid_node(&mut out, node, 1);
    }
    out
}

/// ✏️ Renders one treemap node at a depth.
fn render_mermaid_node(out: &mut String, node: &MermaidNode, depth: usize) {
    let indent = TEXT_BLANK_INDENT.repeat(depth);
    match node.value {
        Some(value) if node.children.is_empty() => out.push_str(&format!("{indent}\"{}\": {value}\n", mermaid_escape_label(&node.label))),
        _ => out.push_str(&format!("{indent}\"{}\"\n", mermaid_escape_label(&node.label))),
    }
    for child in &node.children {
        render_mermaid_node(out, child, depth + 1);
    }
}

/// 🌳️ Projects a monorepo tree into a treemap: every node becomes a group, and a node carrying a
/// numeric `value` in its data becomes a leaf of that weight.
pub fn mermaid_treemap_from_tree(root: &TreeNode, title: &str, weight_key: &str) -> MermaidTreemap {
    MermaidTreemap {
        title: title.to_string(),
        nodes: children_of(root).iter().map(|child| mermaid_node_from_tree(child, weight_key)).collect(),
    }
}

/// 🌿️ Projects one tree node into a treemap node.
fn mermaid_node_from_tree(node: &TreeNode, weight_key: &str) -> MermaidNode {
    MermaidNode {
        label: node.label.clone(),
        value: node.data.as_ref().and_then(|data| data.get(weight_key)).and_then(Value::as_i64),
        children: children_of(node).iter().map(|child| mermaid_node_from_tree(child, weight_key)).collect(),
    }
}

//#endregion 🧜️Mermaid

//#region 🔢️MermaidLoc
// 🔢️ The three LOC treemaps the `mermaid` verb renders. Every one is a pure function over already
// gathered rows, so the walk, the line counting and the `git blame` stay outside this crate.

/// 👤️ The emoji a technology kind carries in a LOC treemap, raw and without a presentation
/// selector — the Go twin writes the constant, not its text form.
fn mermaid_technology_emoji(kind: TechnologyKind) -> &'static str {
    match kind {
        TechnologyKind::Infrastructure => "🧰️",
        TechnologyKind::Research => "🔬️",
        _ => "👤️",
    }
}

/// 📦️ The emoji a bundle kind carries in a LOC treemap.
fn mermaid_bundle_emoji(kind: BundleKind) -> &'static str {
    match kind {
        BundleKind::Schema => "🛂️",
        BundleKind::Binary => "⌨️",
        BundleKind::Ui => "🖱️",
        BundleKind::Site => "🌐️",
        BundleKind::Assets => "🏪️",
        _ => "📚️",
    }
}

/// 📄️ The emoji a file kind carries in a LOC treemap.
fn mermaid_file_emoji(kind: FileKind) -> &'static str {
    match kind {
        FileKind::Lab => "🥼️",
        FileKind::Script => "📜️",
        FileKind::Docs => "📃️",
        FileKind::Config => "⚙️",
        FileKind::Resource => "💾️",
        FileKind::Template => "📋️",
        FileKind::License => "⚖️",
        _ => "💻️",
    }
}

/// 🗃️ The emoji an organization folder carries in a LOC treemap.
const MERMAID_FOLDER_EMOJI: &str = "🗃️";

/// 🧰️ One counted file, already attributed to the bundle that owns it.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MermaidLocFile {
    /// 🎁️ `technology/bundle` name of the owning bundle.
    pub bundle_name: String,
    /// 📁️ Bundle-relative folder, empty for a file at the bundle root.
    pub folder: String,
    /// 🔖️ The file's own name, as the treemap labels it.
    pub name: String,
    /// 🗂️ Its kind, which decides the emoji.
    pub kind: FileKind,
    /// 🔢️ Its physical line count; a file counted as zero is dropped by the caller.
    pub loc: i64,
}

/// 📂️ The `loc-by-technologies-bundles-folders-files` treemap.
///
/// Twin of `MermaidLocByTechnologiesBundlesFoldersFiles` in `📦️packages/🐹️go/🐹️.go`: technologies,
/// bundles and folders in name order, an empty branch dropped, a bundle-root file promoted to the
/// folder level, and the bundle label being the segment after the technology.
pub fn mermaid_loc_by_technologies_bundles_folders_files(technologies: &[Technology], files: &[MermaidLocFile]) -> String {
    let mut rows: BTreeMap<String, BTreeMap<String, BTreeMap<String, Vec<MermaidLocFile>>>> = BTreeMap::new();
    let mut technology_of: BTreeMap<String, (String, TechnologyKind)> = BTreeMap::new();
    let mut bundle_kind: BTreeMap<String, BundleKind> = BTreeMap::new();
    for technology in technologies {
        rows.entry(technology.name.clone()).or_default();
        technology_of.insert(technology.name.clone(), (technology.name.clone(), technology.kind));
        for bundle in technology.bundles.as_deref().unwrap_or_default() {
            rows.entry(technology.name.clone()).or_default().entry(bundle.name.clone()).or_default();
            bundle_kind.insert(bundle.name.clone(), bundle.kind);
        }
    }
    for file in files {
        let technology_name = file.bundle_name.split('/').next().unwrap_or_default().to_string();
        let Some(bundles) = rows.get_mut(&technology_name) else { continue };
        let Some(folders) = bundles.get_mut(&file.bundle_name) else { continue };
        folders.entry(file.folder.clone()).or_default().push(file.clone());
    }
    let mut out = String::from("treemap-beta\n\"Lines of Code\"\n");
    for (technology_name, bundles) in &rows {
        let technology_loc: i64 = bundles.values().flat_map(|folders| folders.values()).flatten().map(|file| file.loc).sum();
        if technology_loc == 0 {
            continue;
        }
        let kind = technology_of.get(technology_name).map_or(TechnologyKind::User, |entry| entry.1);
        out.push_str(&format!("    \"{}{}\"\n", mermaid_technology_emoji(kind), mermaid_escape_label(&semio_framework_repo_identity::flat(technology_name))));
        for (bundle_name, folders) in bundles {
            let bundle_loc: i64 = folders.values().flatten().map(|file| file.loc).sum();
            if bundle_loc == 0 {
                continue;
            }
            let label = bundle_name.split_once('/').map_or_else(|| bundle_name.clone(), |(_, tail)| tail.to_string());
            let kind = bundle_kind.get(bundle_name).copied().unwrap_or(BundleKind::Library);
            out.push_str(&format!("        \"{}{}\"\n", mermaid_bundle_emoji(kind), mermaid_escape_label(&semio_framework_repo_identity::flat(&label))));
            for (folder_name, folder_files) in folders {
                let folder_loc: i64 = folder_files.iter().map(|file| file.loc).sum();
                if folder_loc == 0 {
                    continue;
                }
                if folder_name.is_empty() {
                    for file in folder_files {
                        out.push_str(&format!("            \"{}{}\": {}\n", mermaid_file_emoji(file.kind), mermaid_escape_label(&file.name), file.loc));
                    }
                    continue;
                }
                let leaf = folder_name.rsplit('/').next().unwrap_or(folder_name);
                out.push_str(&format!("            \"{MERMAID_FOLDER_EMOJI}{}\"\n", mermaid_escape_label(leaf)));
                let mut sorted = folder_files.clone();
                sorted.sort_by(|left, right| left.name.cmp(&right.name));
                for file in sorted {
                    out.push_str(&format!("                \"{}{}\": {}\n", mermaid_file_emoji(file.kind), mermaid_escape_label(&file.name), file.loc));
                }
            }
        }
    }
    out
}

/// 📏️ A flat `label: value` treemap under a title, ordered by descending value then by label.
///
/// Twin of the tail of `MermaidLocByContributors` and `MermaidLocByLanguage`, which differ only in
/// their title and in how their rows were gathered.
pub fn mermaid_loc_flat(title: &str, rows: &BTreeMap<String, i64>) -> String {
    let mut ordered: Vec<(&String, &i64)> = rows.iter().collect();
    ordered.sort_by(|left, right| right.1.cmp(left.1).then_with(|| left.0.cmp(right.0)));
    let mut out = format!("treemap-beta\n\"{title}\"\n");
    for (label, value) in ordered {
        out.push_str(&format!("    \"{}\": {value}\n", mermaid_escape_label(label)));
    }
    out
}

/// 👥️ The `loc-by-contributors` treemap title.
pub const MERMAID_TITLE_BY_CONTRIBUTORS: &str = "Lines of Code by Contributor";

/// 🌐️ The `loc-by-language` treemap title.
pub const MERMAID_TITLE_BY_LANGUAGE: &str = "Lines of Code by Language";

//#endregion 🔢️MermaidLoc
