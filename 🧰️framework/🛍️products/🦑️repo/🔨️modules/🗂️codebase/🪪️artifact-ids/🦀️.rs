//! 🪪️ Artifact ids of the codebase: the folder kind derivation and the folder, file and section id and uri builders. See [`super`].

use super::*;

//#region 🗂️Codebase
//#region 🪪️Artifact Ids
impl Codebase {
    /// 📁️ Infers whether a folder is required by a toolchain or purely organisational.
    pub fn derive_folder_kind(&self, path: &str) -> FolderKind {
        if base_of(path).starts_with('.') {
            return FolderKind::Required;
        }
        if let Some(cached) = self.folder_kinds.lock().expect("folder kind cache").get(path) {
            return *cached;
        }
        let mut kind = FolderKind::Organization;
        for indicator in ["package.json", "pyproject.toml", "go.mod", "Cargo.toml"] {
            if self.exists(&format!("{path}/{indicator}")) {
                kind = FolderKind::Required;
                break;
            }
        }
        if kind == FolderKind::Organization {
            for name in read_dir_names(&self.root_dir.join(path), false) {
                if self.root_dir.join(path).join(&name).is_dir() {
                    continue;
                }
                let extension = ext_of(&name);
                if extension == ".csproj" || extension == ".sln" {
                    kind = FolderKind::Required;
                    break;
                }
            }
        }
        self.folder_kinds.lock().expect("folder kind cache").insert(path.to_string(), kind);
        kind
    }

    /// 🛤️ Builds the id of the folder chain leading to a directory.
    fn resolve_parent_id_from_path(&self, dir_path: &str) -> String {
        let normalized = normalize_path(dir_path);
        if normalized == "." || normalized.is_empty() {
            return self.bundle_by_path(".").map(|bundle| self.bundle_id(&bundle)).unwrap_or_default();
        }
        if let Some(bundle) = self.bundle_by_path(&normalized) {
            let bundle_root = normalize_path(&bundle.root);
            if normalized == bundle_root {
                return self.bundle_id(&bundle);
            }
            if let Some(relative) = normalized.strip_prefix(&format!("{bundle_root}/")) {
                let mut parent_id = self.bundle_id(&bundle);
                let mut current = bundle_root;
                for part in relative.split('/') {
                    current = format!("{current}/{part}");
                    parent_id = format!("{parent_id}{}{}", folder_kind_emoji(self.derive_folder_kind(&current)), flat(part));
                }
                return parent_id;
            }
        }
        let mut parent_id = String::new();
        let mut current = String::new();
        for part in normalized.split('/') {
            current = if current.is_empty() { part.to_string() } else { format!("{current}/{part}") };
            parent_id = format!("{parent_id}{}{}", folder_kind_emoji(self.derive_folder_kind(&current)), flat(part));
        }
        parent_id
    }

    /// 🔨️ Builds the emoji id of a folder.
    pub fn build_folder_id(&self, path: &str) -> String {
        let normalized = normalize_path(path);
        if normalized == "." || normalized.is_empty() {
            return self.bundle_by_path(".").map(|bundle| self.bundle_id(&bundle)).unwrap_or_default();
        }
        let dir = dir_of(&normalized);
        let name = base_of(&normalized);
        let bundle = self.bundle_by_path(&normalized);
        let parent_id = match &bundle {
            Some(bundle) => {
                let bundle_root = normalize_path(&bundle.root);
                if normalized == bundle_root {
                    return self.bundle_id(bundle);
                }
                if dir == bundle_root {
                    self.bundle_id(bundle)
                } else {
                    self.resolve_parent_id_from_path(&dir)
                }
            }
            None if dir != "." && !dir.is_empty() => self.resolve_parent_id_from_path(&dir),
            None => self.bundle_by_path(".").map(|bundle| self.bundle_id(&bundle)).unwrap_or_default(),
        };
        let kind = self.derive_folder_kind(&normalized);
        if kind == FolderKind::Root {
            if let Some(bundle) = &bundle {
                return self.bundle_id(bundle);
            }
        }
        format!("{parent_id}{}{}", folder_kind_emoji(kind), flat(&name))
    }

    /// 📄️ Builds the emoji id of a file.
    pub fn build_file_id(&self, path: &str) -> String {
        let normalized = normalize_path(path);
        let name = base_of(&normalized);
        let kind = derive_file_kind(&name);
        let dir = dir_of(&normalized);
        let bundle = self.bundle_by_path(&normalized);
        let parent_id = if dir != "." && !dir.is_empty() {
            match &bundle {
                Some(bundle) if normalize_path(&bundle.root) == normalize_path(&dir) => self.bundle_id(bundle),
                _ => self.resolve_parent_id_from_path(&dir),
            }
        } else {
            bundle.as_ref().map(|bundle| self.bundle_id(bundle)).unwrap_or_default()
        };
        let extension = ext_of(&name);
        let stem = name.strip_suffix(&extension).unwrap_or(&name);
        format!("{parent_id}{}{}", file_kind_emoji(kind), flat(stem))
    }

    /// 🔗️ The `repo://file/…` uri of a path.
    pub fn file_uri(&self, path: &str) -> String {
        let id = self.build_file_id(path);
        if id.is_empty() {
            return String::new();
        }
        format!("repo://file/{id}")
    }

    /// 🦑️ The uri a folder is addressed by. The Go twin routes folders through the same builder.
    pub fn folder_uri(&self, path: &str) -> String {
        self.file_uri(&normalize_path(path))
    }
}
//#endregion 🪪️Artifact Ids
//#endregion 🗂️Codebase
