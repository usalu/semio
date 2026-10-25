//! 🗂️ Bundle, folder and file aggregates of the codebase context. See [`super`].

use super::*;

//#region 🧱️Aggregates
impl<'a> CodebaseContext<'a> {
    /// 🧱️ Projects every bundle with its folder, file, line, section and definition counts.
    pub fn build_bundles(&self) -> Vec<CodebaseBundle> {
        let mut file_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut line_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut section_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut definition_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut folder_sets: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut contributor_sets: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut breach_counts: BTreeMap<String, i64> = BTreeMap::new();

        for bundle in &self.bundles {
            let name = normalize_bundle_label(&bundle.name);
            folder_sets.entry(name.clone()).or_default();
            contributor_sets.entry(name).or_default();
        }
        folder_sets.entry("repo/repo".to_string()).or_default();
        contributor_sets.entry("repo/repo".to_string()).or_default();

        for file in &self.files {
            if file == "README.md" || file == "AGENTS.md" {
                continue;
            }
            let bundle_name = self.bundle_for_file(file);
            if bundle_name.is_empty() {
                continue;
            }
            *file_counts.entry(bundle_name.clone()).or_default() += 1;
            let folder = normalize_path(&dir_of(file));
            if folder != "." {
                folder_sets.entry(bundle_name.clone()).or_default().insert(folder);
            }
            let Some(content) = self.read(file) else { continue };
            *line_counts.entry(bundle_name.clone()).or_default() += content.matches('\n').count() as i64 + 1;
            let sections = parse_sections(&content, file);
            *section_counts.entry(bundle_name.clone()).or_default() += count_sections(&sections);
            if language_for_path(file).is_some_and(|language| language.supports_definitions()) {
                *definition_counts.entry(bundle_name.clone()).or_default() += parse_definitions(&content, file).len() as i64;
            }
            if let Some(header) = find_section(&sections, "Header") {
                let start = header.start_index.max(0) as usize;
                let end = (header.end_index.max(0) as usize).min(content.len());
                if start <= end && content.is_char_boundary(start) && content.is_char_boundary(end) {
                    for line in content[start..end].split('\n') {
                        if let Some((_, email)) = parse_contributor_identity(line) {
                            contributor_sets.entry(bundle_name.clone()).or_default().insert(email);
                        }
                    }
                }
            }
        }

        for breach in &self.breachs {
            let bundle_name = self.bundle_for_file(&breach.scope);
            if !bundle_name.is_empty() {
                *breach_counts.entry(bundle_name).or_default() += 1;
            }
        }

        folder_sets
            .keys()
            .map(|name| {
                let bundle_root = self.bundles.iter().find(|bundle| &normalize_bundle_label(&bundle.name) == name).map(|bundle| bundle.root.clone()).unwrap_or_default();
                CodebaseBundle {
                    id: name.clone(),
                    folder: bundle_root.clone(),
                    uri: self.codebase.file_uri(&bundle_root),
                    contributors: contributor_sets.get(name).map(|set| set.iter().cloned().collect()).unwrap_or_default(),
                    tickets: Vec::new(),
                    metrics: Some(BundleMetricsInternal {
                        folders: folder_sets.get(name).map(BTreeSet::len).unwrap_or_default() as i64,
                        files: file_counts.get(name).copied().unwrap_or_default(),
                        sections: section_counts.get(name).copied().unwrap_or_default(),
                        definitions: definition_counts.get(name).copied().unwrap_or_default(),
                        lines: line_counts.get(name).copied().unwrap_or_default(),
                        breachs: breach_counts.get(name).copied().unwrap_or_default(),
                    }),
                }
            })
            .collect()
    }

    /// 📦️ Projects every folder that holds at least one considered file.
    pub fn build_folders(&self) -> Vec<CodebaseFolder> {
        let mut folder_set: BTreeSet<String> = BTreeSet::new();
        let mut file_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut line_counts: BTreeMap<String, i64> = BTreeMap::new();
        let mut breach_counts: BTreeMap<String, i64> = BTreeMap::new();

        for file in &self.files {
            let folder = normalize_path(&dir_of(file));
            if folder == "." {
                continue;
            }
            folder_set.insert(folder.clone());
            *file_counts.entry(folder.clone()).or_default() += 1;
            if let Some(content) = self.read(file) {
                *line_counts.entry(folder).or_default() += content.matches('\n').count() as i64 + 1;
            }
        }
        for breach in &self.breachs {
            let file_path = extract_file_path(&breach.scope);
            if file_path.is_empty() {
                continue;
            }
            let folder = normalize_path(&dir_of(&file_path));
            if folder != "." {
                *breach_counts.entry(folder).or_default() += 1;
            }
        }

        let mut result: Vec<CodebaseFolder> = folder_set
            .iter()
            .map(|folder| CodebaseFolder {
                id: self.codebase.build_folder_id(folder),
                path: folder.clone(),
                uri: self.codebase.file_uri(folder),
                name: base_of(folder),
                parent_id: match dir_of(folder) {
                    parent if parent != "." && !parent.is_empty() => Some(self.codebase.build_folder_id(&parent)),
                    _ => None,
                },
                bundle_id: self.codebase.bundle_by_path(folder).map(|bundle| self.codebase.bundle_id(&bundle)),
                metrics: Some(FolderMetricsInternal {
                    files: file_counts.get(folder).copied().unwrap_or_default(),
                    lines: line_counts.get(folder).copied().unwrap_or_default(),
                    breachs: breach_counts.get(folder).copied().unwrap_or_default(),
                }),
            })
            .collect();
        result.sort_by(|left, right| left.path.cmp(&right.path));
        result
    }

    /// 🔸️ Projects every considered file with its metrics and breach references.
    pub fn build_files(&self) -> Vec<CodebaseFile> {
        let mut breachs_by_file: BTreeMap<String, Vec<&Breach>> = BTreeMap::new();
        for breach in &self.breachs {
            let file_path = extract_file_path(&breach.scope);
            if !file_path.is_empty() {
                breachs_by_file.entry(file_path).or_default().push(breach);
            }
        }
        let mut result: Vec<CodebaseFile> = self
            .files
            .iter()
            .map(|file| {
                let metrics = self.read(file).map(|content| {
                    let sections = parse_sections(&content, file);
                    let definitions = if language_for_path(file).is_some_and(|language| language.supports_definitions()) { parse_definitions(&content, file).len() as i64 } else { 0 };
                    FileMetricsInternal { sections: count_sections(&sections), definitions, lines: content.split('\n').count() as i64 }
                });
                CodebaseFile {
                    id: self.codebase.build_file_id(file),
                    path: file.clone(),
                    uri: self.codebase.file_uri(file),
                    folder_id: match dir_of(file) {
                        parent if parent != "." && !parent.is_empty() => Some(self.codebase.build_folder_id(&parent)),
                        _ => None,
                    },
                    bundle_id: self.codebase.bundle_by_path(file).map(|bundle| self.codebase.bundle_id(&bundle)),
                    metrics,
                    breachs: breachs_by_file
                        .get(file)
                        .map(|breachs| {
                            breachs
                                .iter()
                                .map(|breach| FileBreachRef {
                                    kind: breach.kind.clone(),
                                    priority: breach.lint_priority.unwrap_or(BreachPriority::Low),
                                    autofixable: breach.lint_autofixable.unwrap_or_default(),
                                    solution: breach.solution.clone(),
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                }
            })
            .collect();
        result.sort_by(|left, right| left.path.cmp(&right.path));
        result
    }
}
//#endregion 🧱️Aggregates
