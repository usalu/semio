//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//! 📝️ Todos and drafts of the `🦑️repo` product. A todo has no store of its own: it IS a line of
//! source, either a `- TODO Name: description` item of a directory's `.todos.md` or a
//! `// TODO Name: description` comment in a scanned file, and every operation on one is a rewrite
//! of that line in place.
//!
//! Every effect is a port. The scanned tree is [`TodoTree`], the ticket a todo is promoted into
//! comes from [`TicketOpener`], and drafts live behind [`DraftStore`] — each with an in-memory
//! implementation, so a scenario needs no repository on disk.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use semio_framework_repo_identity as identity;

pub use semio_framework_repo_model::{Draft, Location, Todo, TodoChangeInput, TodoCreateInput};

//#region ❗️Errors

/// ❗️ Everything this crate can refuse, as a class rather than a rendered string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TodoError {
    /// 🔍️ No todo carries that identifier.
    NotFound,
    /// 📍️ The todo was found but nothing says which line it is.
    NoLocation,
    /// 🚫️ The parent is neither a directory nor a file that exists.
    InvalidParent,
    /// 📛️ A draft already exists under that identifier.
    DraftExists(String),
    /// 🔤️ The draft title carries no slug.
    InvalidTitle,
    /// 🗄️ The tree or a port refused.
    Port(String),
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::NotFound => write!(formatter, "todo not found"),
            TodoError::NoLocation => write!(formatter, "todo has no location"),
            TodoError::InvalidParent => write!(formatter, "invalid parent id (must be path to folder or file)"),
            TodoError::DraftExists(id) => write!(formatter, "draft already exists: {id}"),
            TodoError::InvalidTitle => write!(formatter, "invalid draft title"),
            TodoError::Port(message) => write!(formatter, "{message}"),
        }
    }
}

impl TodoError {
    /// 🏷️ The stable class of the refusal, for a projection that must not depend on wording.
    pub fn class(&self) -> &'static str {
        match self {
            TodoError::NotFound => "not-found",
            TodoError::NoLocation => "no-location",
            TodoError::InvalidParent => "invalid-parent",
            TodoError::DraftExists(_) => "draft-exists",
            TodoError::InvalidTitle => "invalid-title",
            TodoError::Port(_) => "port",
        }
    }
}

//#endregion ❗️Errors

//#region 🔍️Parsing

/// 📄️ The file a directory records its own todos in.
pub const TODO_MARKDOWN_NAME: &str = ".todos.md";

/// 🏷️ The item prefix a `.todos.md` entry carries.
pub const TODO_MARKDOWN_PREFIX: &str = "- TODO ";

/// 🗂️ The file extensions a scan reads todo comments out of.
pub const TODO_SCAN_EXTENSIONS: &[&str] = &[".ts", ".js", ".tsx", ".jsx", ".go", ".cs", ".py", ".md", ".json"];

/// 💬️ The comment openers a todo comment may start with.
pub const TODO_COMMENT_OPENERS: &[&str] = &["//", "#", "--"];

/// 🗑️ The directory names a scan never descends into.
pub const TODO_SKIPPED_DIRECTORIES: &[&str] = &["node_modules", "dist", "build"];

/// 📰️ Parses a `.todos.md` document. Every `- TODO ` item becomes a todo whose name is the text
/// before the first `:` and whose description is everything after it; an item without a `:` has a
/// blank description.
pub fn parse_todo_markdown(content: &str, parent_path: &str) -> Vec<Todo> {
    content
        .split('\n')
        .filter_map(|line| {
            let rest = line.trim().strip_prefix(TODO_MARKDOWN_PREFIX)?;
            let (name, description) = match rest.split_once(':') {
                Some((name, description)) => (name.trim(), description.trim()),
                None => (rest.trim(), ""),
            };
            Some(Todo {
                id: identity::slugify(name),
                name: name.to_string(),
                description: description.to_string(),
                parent_id: parent_path.to_string(),
                location: Some(Location { file_path: join_path(parent_path, TODO_MARKDOWN_NAME), line: 0, column: 0 }),
            })
        })
        .collect()
}

/// 💬️ Parses todo comments out of a source file. A line qualifies when, after leading whitespace,
/// it opens a comment with `//`, `#` or `--`, then `TODO`, then a name that carries no `:`, then a
/// `:` and the description. The line number is one based.
pub fn parse_todo_comments(content: &str, file_path: &str) -> Vec<Todo> {
    content
        .split('\n')
        .enumerate()
        .filter_map(|(index, line)| {
            let (name, description) = split_todo_comment(line)?;
            Some(Todo {
                id: identity::slugify(&name),
                name,
                description,
                parent_id: file_path.to_string(),
                location: Some(Location { file_path: file_path.to_string(), line: index as i64 + 1, column: 1 }),
            })
        })
        .collect()
}

/// ✂️ The `(name, description)` of a todo comment line, and the untouched prefix that precedes the
/// name, so a rewrite can put a new name and description back without disturbing the indentation
/// or the comment opener.
pub fn split_todo_comment_parts(line: &str) -> Option<(String, String, String)> {
    let rest = line.trim_start();
    let indent_len = line.len() - rest.len();
    let opener = TODO_COMMENT_OPENERS.iter().find(|opener| rest.starts_with(**opener))?;
    let after_opener = &rest[opener.len()..];
    let after_opener_trimmed = after_opener.trim_start();
    let keyword = after_opener_trimmed.strip_prefix("TODO")?;
    if !keyword.starts_with(char::is_whitespace) {
        return None;
    }
    let after_keyword = keyword.trim_start();
    if after_keyword.is_empty() {
        return None;
    }
    let (name, description) = after_keyword.split_once(':')?;
    if name.contains(':') {
        return None;
    }
    let prefix_len = line.len() - after_keyword.len();
    let prefix = line[..prefix_len].to_string();
    debug_assert!(indent_len <= prefix_len);
    Some((prefix, name.trim().to_string(), description.trim().to_string()))
}

fn split_todo_comment(line: &str) -> Option<(String, String)> {
    split_todo_comment_parts(line).map(|(_, name, description)| (name, description))
}

/// 💬️ The comment opener a todo written into a file gets, by file extension.
pub fn todo_comment_opener(path: &str) -> &'static str {
    match extension_of(path).as_str() {
        ".py" | ".sh" | ".yaml" | ".yml" => "#",
        ".sql" | ".lua" => "--",
        _ => "//",
    }
}

fn extension_of(path: &str) -> String {
    let name = path.rsplit(['/', '\\']).next().unwrap_or(path);
    match name.rfind('.') {
        Some(index) if index > 0 => name[index..].to_string(),
        _ => String::new(),
    }
}

fn join_path(parent: &str, name: &str) -> String {
    if parent.is_empty() || parent == "." {
        name.to_string()
    } else {
        format!("{}/{name}", parent.trim_end_matches('/'))
    }
}

//#endregion 🔍️Parsing

//#region 🌲️Tree

/// 📍️ One entry of a scanned tree.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TreeEntry {
    /// 🛤️ The path, `/` separated and relative to the scan root.
    pub path: String,
    /// 📁️ Whether the entry is a directory.
    #[serde(rename = "isDir", default)]
    pub is_dir: bool,
}

/// 🌲️ The tree a scan reads and a rewrite writes back to.
pub trait TodoTree {
    /// 🚶️ Every entry, in pre-order: a directory before everything it contains.
    fn walk(&self) -> Vec<TreeEntry>;
    /// 📖️ The content of one file.
    fn read(&self, path: &str) -> Option<String>;
    /// 💾️ Replaces or creates one file.
    fn write(&self, path: &str, content: &str) -> Result<(), TodoError>;
    /// 📁️ Whether a path is a directory that exists.
    fn is_dir(&self, path: &str) -> bool;
    /// 🔍️ Whether a path exists at all.
    fn exists(&self, path: &str) -> bool;

    /// ➕️ Appends to a file, creating it when it does not exist.
    fn append(&self, path: &str, text: &str) -> Result<(), TodoError> {
        let existing = self.read(path).unwrap_or_default();
        self.write(path, &format!("{existing}{text}"))
    }
}

/// 🧠️ The in-memory tree: a set of directories and a map of file contents.
#[derive(Debug, Default)]
pub struct MemoryTodoTree {
    directories: RefCell<Vec<String>>,
    files: RefCell<BTreeMap<String, String>>,
}

impl MemoryTodoTree {
    /// 🌱️ A tree of `(path, content)` files. Every ancestor directory of every file exists.
    pub fn seeded<I: IntoIterator<Item = (String, String)>>(files: I) -> Self {
        let tree = Self::default();
        for (path, content) in files {
            let mut prefix = String::new();
            let segments: Vec<&str> = path.split('/').collect();
            for segment in &segments[..segments.len().saturating_sub(1)] {
                prefix = if prefix.is_empty() { (*segment).to_string() } else { format!("{prefix}/{segment}") };
                let mut directories = tree.directories.borrow_mut();
                if !directories.contains(&prefix) {
                    directories.push(prefix.clone());
                }
            }
            tree.files.borrow_mut().insert(path, content);
        }
        tree
    }

    /// 📸️ Every `(path, content)` pair, in path order.
    pub fn snapshot(&self) -> Vec<(String, String)> {
        self.files.borrow().iter().map(|(path, content)| (path.clone(), content.clone())).collect()
    }
}

impl TodoTree for MemoryTodoTree {
    fn walk(&self) -> Vec<TreeEntry> {
        let mut entries: Vec<TreeEntry> = vec![TreeEntry { path: String::new(), is_dir: true }];
        entries.extend(self.directories.borrow().iter().map(|path| TreeEntry { path: path.clone(), is_dir: true }));
        entries.extend(self.files.borrow().keys().map(|path| TreeEntry { path: path.clone(), is_dir: false }));
        entries.sort_by(|left, right| left.path.cmp(&right.path).then(right.is_dir.cmp(&left.is_dir)));
        entries
    }

    fn read(&self, path: &str) -> Option<String> {
        self.files.borrow().get(path).cloned()
    }

    fn write(&self, path: &str, content: &str) -> Result<(), TodoError> {
        self.files.borrow_mut().insert(path.to_string(), content.to_string());
        Ok(())
    }

    fn is_dir(&self, path: &str) -> bool {
        path.is_empty() || self.directories.borrow().iter().any(|candidate| candidate == path)
    }

    fn exists(&self, path: &str) -> bool {
        self.is_dir(path) || self.files.borrow().contains_key(path)
    }
}

/// 💽️ The tree over a real checkout, applying the scan's own skip rules: no dotted directory
/// except the repository meta root, and none of [`TODO_SKIPPED_DIRECTORIES`].
#[derive(Debug, Clone)]
pub struct FsTodoTree {
    root: PathBuf,
}

impl FsTodoTree {
    /// 🆕️ The tree rooted at a directory.
    pub fn new(root: &Path) -> Self {
        Self { root: root.to_path_buf() }
    }

    fn absolute(&self, path: &str) -> PathBuf {
        path.split('/').filter(|segment| !segment.is_empty()).fold(self.root.clone(), |current, segment| current.join(segment))
    }

    fn descend(&self, relative: &str, found: &mut Vec<TreeEntry>) {
        let Ok(entries) = std::fs::read_dir(self.absolute(relative)) else { return };
        let mut names: Vec<(String, bool)> = entries
            .flatten()
            .map(|entry| (entry.file_name().to_string_lossy().to_string(), entry.path().is_dir()))
            .collect();
        names.sort();
        for (name, is_dir) in names {
            let path = join_path(relative, &name);
            if !is_dir {
                found.push(TreeEntry { path, is_dir: false });
                continue;
            }
            if TODO_SKIPPED_DIRECTORIES.contains(&name.as_str()) {
                continue;
            }
            if name.starts_with('.') && name != semio_framework_repo_workspace::SEMIO_DIR_NAME {
                continue;
            }
            found.push(TreeEntry { path: path.clone(), is_dir: true });
            self.descend(&path, found);
        }
    }
}

impl TodoTree for FsTodoTree {
    fn walk(&self) -> Vec<TreeEntry> {
        let mut found = vec![TreeEntry { path: String::new(), is_dir: true }];
        self.descend("", &mut found);
        found
    }

    fn read(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(self.absolute(path)).ok()
    }

    fn write(&self, path: &str, content: &str) -> Result<(), TodoError> {
        let target = self.absolute(path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|error| TodoError::Port(error.to_string()))?;
        }
        std::fs::write(target, content).map_err(|error| TodoError::Port(error.to_string()))
    }

    fn is_dir(&self, path: &str) -> bool {
        self.absolute(path).is_dir()
    }

    fn exists(&self, path: &str) -> bool {
        self.absolute(path).exists()
    }
}

/// 🎯️ Every todo the tree carries: each directory's `.todos.md` items when the directory is
/// entered, and each scanned file's todo comments when the file is reached.
pub fn scan_todos(tree: &dyn TodoTree) -> Vec<Todo> {
    let mut found: Vec<Todo> = Vec::new();
    for entry in tree.walk() {
        if entry.is_dir {
            let markdown = join_path(&entry.path, TODO_MARKDOWN_NAME);
            if let Some(content) = tree.read(&markdown) {
                found.extend(parse_todo_markdown(&content, &entry.path));
            }
            continue;
        }
        if entry.path.ends_with(TODO_MARKDOWN_NAME) {
            continue;
        }
        let extension = extension_of(&entry.path);
        if !TODO_SCAN_EXTENSIONS.contains(&extension.as_str()) {
            continue;
        }
        if let Some(content) = tree.read(&entry.path) {
            found.extend(parse_todo_comments(&content, &entry.path));
        }
    }
    found
}

/// 🔎️ Every todo whose name or description contains the term, case-insensitively.
pub fn search_todos(tree: &dyn TodoTree, term: &str) -> Vec<Todo> {
    let needle = term.to_lowercase();
    scan_todos(tree)
        .into_iter()
        .filter(|todo| needle.is_empty() || todo.name.to_lowercase().contains(&needle) || todo.description.to_lowercase().contains(&needle))
        .collect()
}

//#endregion 🌲️Tree

//#region ✏️Rewrites

/// ✏️ Rewrites the `- TODO <old name>:` item of a markdown document with a new name and
/// description, leaving every other line untouched.
pub fn replace_in_markdown(document: &str, old_name: &str, new_name: &str, new_description: &str) -> Result<String, TodoError> {
    let prefix = format!("{TODO_MARKDOWN_PREFIX}{old_name}:");
    let mut lines: Vec<String> = document.split('\n').map(str::to_string).collect();
    let Some(index) = lines.iter().position(|line| line.trim_start().starts_with(&prefix)) else {
        return Err(TodoError::NotFound);
    };
    lines[index] = format!("{TODO_MARKDOWN_PREFIX}{new_name}: {new_description}");
    Ok(lines.join("\n"))
}

/// 🗑️ Removes the `- TODO <name>:` item of a markdown document.
pub fn remove_from_markdown(document: &str, name: &str) -> String {
    let prefix = format!("{TODO_MARKDOWN_PREFIX}{name}:");
    document.split('\n').filter(|line| !line.trim_start().starts_with(&prefix)).collect::<Vec<_>>().join("\n")
}

/// ✏️ Rewrites the todo comment on a one-based line, keeping its indentation and comment opener.
pub fn replace_in_file(document: &str, line_number: i64, new_name: &str, new_description: &str) -> Result<String, TodoError> {
    let mut lines: Vec<String> = document.split('\n').map(str::to_string).collect();
    if line_number <= 0 || line_number as usize > lines.len() {
        return Err(TodoError::NoLocation);
    }
    let index = line_number as usize - 1;
    let Some((prefix, _, _)) = split_todo_comment_parts(&lines[index]) else {
        return Err(TodoError::NotFound);
    };
    lines[index] = format!("{prefix}{new_name}: {new_description}");
    Ok(lines.join("\n"))
}

/// 🗑️ Removes the one-based line from a document.
pub fn remove_from_file(document: &str, line_number: i64) -> String {
    let mut lines: Vec<String> = document.split('\n').map(str::to_string).collect();
    if line_number > 0 && line_number as usize <= lines.len() {
        lines.remove(line_number as usize - 1);
    }
    lines.join("\n")
}

//#endregion ✏️Rewrites

//#region 🎫️TicketOpener

/// 🎫️ The port that turns a promoted todo into a ticket and returns the ticket identifier.
pub trait TicketOpener {
    /// 🆕️ Opens a ticket with a title and a prompt.
    fn open(&self, title: &str, prompt: &str) -> Result<String, TodoError>;
}

/// 🧠️ The in-memory opener: hands out `ticket-<n>` and keeps every `title\tprompt` it was given.
#[derive(Debug, Default)]
pub struct RecordingTicketOpener {
    opened: RefCell<Vec<String>>,
}

impl RecordingTicketOpener {
    /// 🆕️ An opener that has opened nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📜️ Every `title\tprompt` opened so far, in order.
    pub fn opened(&self) -> Vec<String> {
        self.opened.borrow().clone()
    }
}

impl TicketOpener for RecordingTicketOpener {
    fn open(&self, title: &str, prompt: &str) -> Result<String, TodoError> {
        let mut opened = self.opened.borrow_mut();
        opened.push(format!("{title}\t{prompt}"));
        Ok(format!("ticket-{}", opened.len()))
    }
}

//#endregion 🎫️TicketOpener

//#region 🔓️Lifecycle

/// 📝️ The todo aggregate bound to its ports.
pub struct Todos<'a> {
    tree: &'a dyn TodoTree,
    emitter: &'a dyn semio_framework_repo_events::Emitter,
    author: String,
}

impl<'a> Todos<'a> {
    /// 🆕️ Binds the aggregate to a tree, an emitter and an author alias.
    pub fn new(tree: &'a dyn TodoTree, emitter: &'a dyn semio_framework_repo_events::Emitter, author: &str) -> Self {
        Self { tree, emitter, author: author.to_string() }
    }

    /// 📋️ Every todo the tree carries.
    pub fn list(&self) -> Vec<Todo> {
        scan_todos(self.tree)
    }

    /// 🔍️ The first todo carrying an identifier.
    pub fn find(&self, id: &str) -> Result<Todo, TodoError> {
        self.list().into_iter().find(|todo| todo.id == id).ok_or(TodoError::NotFound)
    }

    /// 🆕️ Writes a todo: an item appended to the parent directory's `.todos.md`, or a comment
    /// appended to the parent file in that file's comment style.
    pub fn create(&self, input: &TodoCreateInput) -> Result<Todo, TodoError> {
        let todo = if self.tree.is_dir(&input.parent_id) {
            let path = join_path(&input.parent_id, TODO_MARKDOWN_NAME);
            self.tree.append(&path, &format!("{TODO_MARKDOWN_PREFIX}{}: {}\n", input.name, input.description))?;
            Todo {
                id: identity::slugify(&input.name),
                name: input.name.clone(),
                description: input.description.clone(),
                parent_id: input.parent_id.clone(),
                location: None,
            }
        } else if self.tree.exists(&input.parent_id) {
            let opener = todo_comment_opener(&input.parent_id);
            self.tree.append(&input.parent_id, &format!("\n{opener} TODO {}: {}\n", input.name, input.description))?;
            Todo {
                id: identity::slugify(&input.name),
                name: input.name.clone(),
                description: input.description.clone(),
                parent_id: input.parent_id.clone(),
                location: Some(Location { file_path: input.parent_id.clone(), line: 0, column: 0 }),
            }
        } else {
            return Err(TodoError::InvalidParent);
        };
        self.emit(
            semio_framework_repo_events::TODO_CREATE_ENDED,
            &semio_framework_repo_events::TodoPayload { id: todo.id.clone(), parent_id: todo.parent_id.clone(), name: todo.name.clone(), author: self.author.clone() },
        );
        Ok(todo)
    }

    /// ♻️ Rewrites the source line a todo is, in place.
    pub fn change(&self, input: &TodoChangeInput) -> Result<Todo, TodoError> {
        let todo = self.find(&input.id)?;
        let Some(location) = todo.location.clone() else { return Err(TodoError::NoLocation) };
        let name = input.name.clone().unwrap_or_else(|| todo.name.clone());
        let description = input.description.clone().unwrap_or_else(|| todo.description.clone());
        let document = self.tree.read(&location.file_path).ok_or(TodoError::NotFound)?;
        let rewritten = if location.file_path.ends_with(TODO_MARKDOWN_NAME) {
            replace_in_markdown(&document, &todo.name, &name, &description)?
        } else if location.line > 0 {
            replace_in_file(&document, location.line, &name, &description)?
        } else {
            return Err(TodoError::NoLocation);
        };
        self.tree.write(&location.file_path, &rewritten)?;
        let updated = Todo { id: identity::slugify(&name), name: name.clone(), description, parent_id: todo.parent_id.clone(), location: Some(location) };
        self.emit(
            semio_framework_repo_events::TODO_CHANGE_ENDED,
            &semio_framework_repo_events::TodoChangePayload {
                id: updated.id.clone(),
                parent_id: todo.parent_id,
                author: self.author.clone(),
                name: input.name.clone(),
                description: input.description.clone(),
            },
        );
        Ok(updated)
    }

    /// 🗑️ Removes the source line a todo is.
    pub fn delete(&self, id: &str) -> Result<bool, TodoError> {
        let todo = self.find(id)?;
        let Some(location) = todo.location.clone() else { return Err(TodoError::NoLocation) };
        let document = self.tree.read(&location.file_path).ok_or(TodoError::NotFound)?;
        let rewritten = if location.file_path.ends_with(TODO_MARKDOWN_NAME) {
            remove_from_markdown(&document, &todo.name)
        } else if location.line > 0 {
            remove_from_file(&document, location.line)
        } else {
            return Err(TodoError::NoLocation);
        };
        self.tree.write(&location.file_path, &rewritten)?;
        self.emit(
            semio_framework_repo_events::TODO_DELETE_ENDED,
            &semio_framework_repo_events::TodoPayload { id: todo.id.clone(), parent_id: todo.parent_id.clone(), name: todo.name, author: self.author.clone() },
        );
        Ok(true)
    }

    /// 🎫️ Promotes a todo into a ticket: the todo's description becomes the head of the ticket
    /// prompt, and the todo is removed once the ticket exists.
    pub fn to_ticket(&self, id: &str, opener: &dyn TicketOpener, title: &str, prompt: &str) -> Result<String, TodoError> {
        let todo = self.find(id)?;
        let ticket = opener.open(title, &format!("{}\n\n{prompt}", todo.description))?;
        self.delete(id)?;
        Ok(ticket)
    }

    fn emit<T: serde::Serialize>(&self, kind: &str, payload: &T) {
        let Ok(value) = serde_json::to_value(payload) else { return };
        self.emitter.emit(kind, "repo-cli", &value);
    }
}

//#endregion 🔓️Lifecycle

//#region 🎨️Drafts

/// 🪪️ The artifact identifier of a draft.
pub fn draft_id(draft: &Draft) -> String {
    format!("{}{}", identity::emoji_text(identity::entity("draft")), identity::flat(&draft.id))
}

/// 🔗️ The artifact URI of a draft.
pub fn draft_uri(draft: &Draft) -> String {
    format!("repo://draft/{}", draft_id(draft))
}

/// 🗄️ Where drafts live: one directory of copied files per draft identifier.
pub trait DraftStore {
    /// 📋️ Every draft identifier, in ascending order.
    fn ids(&self) -> Vec<String>;
    /// 🔍️ Whether a draft exists.
    fn exists(&self, id: &str) -> bool;
    /// 🆕️ Creates a draft holding these `(file name, content)` pairs.
    fn create(&self, id: &str, files: &[(String, String)]) -> Result<(), TodoError>;
    /// 📋️ The `(file name, content)` pairs of one draft, in file name order.
    fn files(&self, id: &str) -> Vec<(String, String)>;
    /// 🗑️ Removes a draft, which is a no-operation when it does not exist.
    fn delete(&self, id: &str) -> Result<(), TodoError>;
}

/// 🧠️ The in-memory draft store.
#[derive(Debug, Default)]
pub struct MemoryDraftStore {
    drafts: RefCell<BTreeMap<String, BTreeMap<String, String>>>,
}

impl MemoryDraftStore {
    /// 🆕️ An empty store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl DraftStore for MemoryDraftStore {
    fn ids(&self) -> Vec<String> {
        self.drafts.borrow().keys().cloned().collect()
    }

    fn exists(&self, id: &str) -> bool {
        self.drafts.borrow().contains_key(id)
    }

    fn create(&self, id: &str, files: &[(String, String)]) -> Result<(), TodoError> {
        self.drafts.borrow_mut().insert(id.to_string(), files.iter().cloned().collect());
        Ok(())
    }

    fn files(&self, id: &str) -> Vec<(String, String)> {
        self.drafts.borrow().get(id).map(|files| files.iter().map(|(name, content)| (name.clone(), content.clone())).collect()).unwrap_or_default()
    }

    fn delete(&self, id: &str) -> Result<(), TodoError> {
        self.drafts.borrow_mut().remove(id);
        Ok(())
    }
}

/// 💽️ The store over a real `.🧬semio/🦑️repo/✍️notes` tree.
#[derive(Debug, Clone)]
pub struct FsDraftStore {
    root: PathBuf,
}

impl FsDraftStore {
    /// 🆕️ The store of the notes directory of a repository root.
    pub fn new(repo_root: &Path) -> Self {
        Self { root: semio_framework_repo_workspace::repo_meta_path_for_root(repo_root, "✍️notes") }
    }
}

impl DraftStore for FsDraftStore {
    fn ids(&self) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(&self.root) else { return Vec::new() };
        let mut ids: Vec<String> = entries.flatten().filter(|entry| entry.path().is_dir()).map(|entry| entry.file_name().to_string_lossy().to_string()).collect();
        ids.sort();
        ids
    }

    fn exists(&self, id: &str) -> bool {
        self.root.join(id).is_dir()
    }

    fn create(&self, id: &str, files: &[(String, String)]) -> Result<(), TodoError> {
        let directory = self.root.join(id);
        std::fs::create_dir_all(&directory).map_err(|error| TodoError::Port(error.to_string()))?;
        for (name, content) in files {
            std::fs::write(directory.join(name), content).map_err(|error| TodoError::Port(error.to_string()))?;
        }
        Ok(())
    }

    fn files(&self, id: &str) -> Vec<(String, String)> {
        let Ok(entries) = std::fs::read_dir(self.root.join(id)) else { return Vec::new() };
        let mut files: Vec<(String, String)> = entries
            .flatten()
            .filter(|entry| entry.path().is_file())
            .filter_map(|entry| Some((entry.file_name().to_string_lossy().to_string(), std::fs::read_to_string(entry.path()).ok()?)))
            .collect();
        files.sort();
        files
    }

    fn delete(&self, id: &str) -> Result<(), TodoError> {
        if !self.exists(id) {
            return Ok(());
        }
        std::fs::remove_dir_all(self.root.join(id)).map_err(|error| TodoError::Port(error.to_string()))
    }
}

/// 📋️ Every draft the store carries.
pub fn list_drafts(store: &dyn DraftStore) -> Vec<Draft> {
    store.ids().into_iter().map(|id| Draft { id }).collect()
}

/// 📝️ Creates a draft from a title and a set of `(source path, content)` files. The identifier is
/// the title's slug, a title carrying no slug is refused, an existing identifier is refused, and
/// each file is copied under its base name.
pub fn create_draft(store: &dyn DraftStore, title: &str, files: &[(String, String)]) -> Result<Draft, TodoError> {
    let id = identity::slugify(title);
    if id.is_empty() {
        return Err(TodoError::InvalidTitle);
    }
    if store.exists(&id) {
        return Err(TodoError::DraftExists(id));
    }
    let copied: Vec<(String, String)> = files
        .iter()
        .map(|(path, content)| (path.rsplit(['/', '\\']).next().unwrap_or(path).to_string(), content.clone()))
        .collect();
    store.create(&id, &copied)?;
    Ok(Draft { id })
}

/// 🗑️ Removes a draft. Removing one that does not exist is not an error.
pub fn delete_draft(store: &dyn DraftStore, id: &str) -> Result<(), TodoError> {
    store.delete(id)
}

//#endregion 🎨️Drafts

//#region 🧾️Recording

/// 🧠️ An emitter that keeps `kind\tsource\tpayload` lines instead of reaching a coordinator.
#[derive(Debug, Default)]
pub struct MemoryEmitter {
    envelopes: RefCell<Vec<String>>,
}

impl MemoryEmitter {
    /// 🆕️ An emitter with an empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// 📜️ Every envelope emitted so far, in order.
    pub fn envelopes(&self) -> Vec<String> {
        self.envelopes.borrow().clone()
    }
}

impl semio_framework_repo_events::Emitter for MemoryEmitter {
    fn emit(&self, kind: &str, source: &str, payload: &serde_json::Value) {
        self.envelopes.borrow_mut().push(format!("{kind}\t{source}\t{payload}"));
    }
}

//#endregion 🧾️Recording
