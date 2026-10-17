//! 🌳️ Section, definition, breach and tree aggregates of the codebase context and the folder tree helpers. See [`super`].

use super::*;

//#region 🧱️Aggregates
impl<'a> CodebaseContext<'a> {
    /// 🔺️ Projects every section of every considered file, depth first.
    pub fn build_sections(&self) -> Vec<CodebaseSection> {
        let mut result = Vec::new();
        for file in &self.files {
            let Some(content) = self.read(file) else { continue };
            let sections = parse_sections(&content, file);
            let file_id = self.codebase.build_file_id(file);
            self.add_sections(&mut result, file, &file_id, &content, &sections, "");
        }
        result.sort_by(|left, right| left.path.cmp(&right.path));
        result
    }

    /// ➕️ The recursive half of [`CodebaseContext::build_sections`].
    fn add_sections(&self, result: &mut Vec<CodebaseSection>, file: &str, file_id: &str, content: &str, sections: &[Section], parent_path: &str) {
        for section in sections {
            let section_path = if parent_path.is_empty() { section.name.clone() } else { format!("{parent_path}#{}", section.name) };
            let start = section.start_index.max(0) as usize;
            let end = (section.end_index.max(0) as usize).min(content.len());
            let section_content = if start < content.len() && start <= end && content.is_char_boundary(start) && content.is_char_boundary(end) { &content[start..end] } else { "" };
            let definitions = if language_for_path(file).is_some_and(|language| language.supports_definitions()) { parse_definitions(section_content, file).len() as i64 } else { 0 };
            result.push(CodebaseSection {
                id: format!("{file_id}#{section_path}"),
                path: format!("{file}#{section_path}"),
                uri: format!("{}#{section_path}", self.codebase.file_uri(file)),
                metrics: Some(SectionMetricsInternal { definitions, lines: section.end_line - section.start_line + 1, breachs: 0 }),
            });
            self.add_sections(result, file, file_id, content, &section.children, &section_path);
        }
    }

    /// ▶️ Projects every definition of every considered file.
    pub fn build_definitions(&self) -> Vec<CodebaseDefinition> {
        let mut result = Vec::new();
        for file in &self.files {
            let Some(content) = self.read(file) else { continue };
            if !language_for_path(file).is_some_and(|language| language.supports_definitions()) {
                continue;
            }
            let definitions = parse_definitions(&content, file);
            let sections = parse_sections(&content, file);
            let file_id = self.codebase.build_file_id(file);
            for definition in &definitions {
                let section_path = find_section_for_definition(&sections, definition.start_line, definition.end_line, "");
                let id = if section_path.is_empty() { format!("{file_id}§{}", definition.name) } else { format!("{file_id}#{section_path}§{}", definition.name) };
                result.push(CodebaseDefinition {
                    id: id.clone(),
                    path: id,
                    uri: format!("{}§{}", self.codebase.file_uri(file), definition.name),
                    metrics: Some(DefinitionMetricsInternal { definitions: 0, lines: definition.end_line - definition.start_line + 1, breachs: 0 }),
                });
            }
        }
        result.sort_by(|left, right| left.path.cmp(&right.path));
        result
    }

    /// ⬜️ Projects every breach with the folder and file it points at.
    pub fn build_breachs(&self) -> Vec<CodebaseBreach> {
        self.breachs
            .iter()
            .enumerate()
            .map(|(index, breach)| {
                let file_path = extract_file_path(&breach.scope);
                let bundle_name = self.bundle_for_file(&file_path);
                let mut folders = Vec::new();
                let mut files = Vec::new();
                if !file_path.is_empty() {
                    let folder = normalize_path(&dir_of(&file_path));
                    if folder != "." {
                        let folder_id = if bundle_name.is_empty() { folder.clone() } else { format!("{bundle_name}/{folder}") };
                        folders.push(BreachFolder { id: folder_id, path: folder.clone(), uri: self.codebase.folder_uri(&folder) });
                    }
                    let file_id = if bundle_name.is_empty() { file_path.clone() } else { format!("{bundle_name}/{}", base_of(&file_path)) };
                    files.push(BreachFile {
                        id: file_id,
                        path: file_path.clone(),
                        uri: self.codebase.file_uri(&file_path),
                        range: Some(FileRange { start: RangePosition { line: breach.line, column: breach.column }, end: RangePosition { line: breach.line, column: breach.column } }),
                    });
                }
                CodebaseBreach {
                    id: format!("{}#|{bundle_name}|{file_path}#{index}", breach.kind),
                    folders,
                    files,
                    kind: breach.kind.clone(),
                    priority: breach.lint_priority.unwrap_or(BreachPriority::Low),
                    autofixable: breach.lint_autofixable.unwrap_or_default(),
                    reason: breach.reason.clone(),
                    solution: breach.solution.clone(),
                }
            })
            .collect()
    }

    /// 🌳️ Projects the containment tree of bundles, folders and files.
    pub fn build_tree(&self, bundles: &[CodebaseBundle], files: &[CodebaseFile]) -> BTreeMap<String, CbTreeNode> {
        let mut root = CbTreeNode { kind: CbTreeNodeKind::Repo, children: BTreeMap::new() };
        for bundle in bundles {
            root.children.insert(bundle.id.clone(), CbTreeNode { kind: CbTreeNodeKind::Bundle, children: BTreeMap::new() });
        }
        let mut folder_paths: BTreeSet<String> = BTreeSet::new();
        for file in files {
            let bundle_name = self.bundle_for_file(&file.path);
            let folder = normalize_path(&dir_of(&file.path));
            if folder == "." {
                let parent = if root.children.contains_key(&bundle_name) { root.children.get_mut(&bundle_name).expect("bundle node") } else { &mut root };
                parent.children.insert(file.id.clone(), CbTreeNode { kind: CbTreeNodeKind::File, children: BTreeMap::new() });
                continue;
            }
            let parts: Vec<&str> = folder.split('/').collect();
            for index in 0..parts.len() {
                let folder_path = parts[..index + 1].join("/");
                if folder_paths.contains(&folder_path) {
                    continue;
                }
                folder_paths.insert(folder_path.clone());
                let branch = if root.children.contains_key(&bundle_name) { root.children.get_mut(&bundle_name).expect("bundle node") } else { &mut root };
                let node = descend(branch, &parts[..index]);
                node.children.insert(parts[index].to_string(), CbTreeNode { kind: CbTreeNodeKind::Folder, children: BTreeMap::new() });
            }
            let branch = if root.children.contains_key(&bundle_name) { root.children.get_mut(&bundle_name).expect("bundle node") } else { &mut root };
            let node = descend(branch, &parts);
            node.children.insert(file.id.clone(), CbTreeNode { kind: CbTreeNodeKind::File, children: BTreeMap::new() });
        }
        BTreeMap::from([("compose".to_string(), root)])
    }

    /// 🟥️ Assembles the whole codebase snapshot from this context.
    pub fn build(&self) -> CodebaseSnapshot {
        let bundles = self.build_bundles();
        let files = self.build_files();
        let tree = self.build_tree(&bundles, &files);
        CodebaseSnapshot {
            bundles: Some(bundles),
            folders: Some(self.build_folders()),
            files: Some(files),
            sections: Some(self.build_sections()),
            definitions: Some(self.build_definitions()),
            contributors: None,
            tickets: None,
            policies: None,
            breachs: Some(self.build_breachs()),
            tree: Some(tree),
        }
    }
}
//#endregion 🧱️Aggregates

//#region 📚️Context
/// 🌳️ Walks a chain of folder segments, creating nothing.
fn descend<'node>(node: &'node mut CbTreeNode, parts: &[&str]) -> &'node mut CbTreeNode {
    let mut current = node;
    for part in parts {
        if !current.children.contains_key(*part) {
            current.children.insert((*part).to_string(), CbTreeNode { kind: CbTreeNodeKind::Folder, children: BTreeMap::new() });
        }
        current = current.children.get_mut(*part).expect("folder node");
    }
    current
}

/// 🔖️ The first section with a given name, searched depth first.
pub(super) fn find_section<'a>(sections: &'a [Section], name: &str) -> Option<&'a Section> {
    for section in sections {
        if section.name == name {
            return Some(section);
        }
        if let Some(found) = find_section(&section.children, name) {
            return Some(found);
        }
    }
    None
}
//#endregion 📚️Context
