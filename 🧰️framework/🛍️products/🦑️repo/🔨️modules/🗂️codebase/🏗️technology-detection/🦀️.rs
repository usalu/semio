//! 🏗️ Technology detection of the codebase: the technology and bundle detection that gives every path its owning bundle. See [`super`].

use super::*;

//#region 🗂️Codebase
//#region 🏗️Technologies
impl Codebase {
    /// 📁️ Reports whether a root-level directory is a technology directory.
    fn is_technology_dir(&self, name: &str) -> bool {
        let Some(content) = self.read_text(&format!("{name}/README.md")) else { return false };
        if !content.starts_with("---") {
            return false;
        }
        if name.starts_with('.') {
            return false;
        }
        !matches!(name, "node_modules" | "target" | "temp")
    }

    /// 🧾️ Decodes the leading `---` front matter block of a document.
    fn front_matter(&self, relative: &str) -> Option<serde_json::Value> {
        let content = self.read_text(relative)?;
        if !content.starts_with("---") {
            return None;
        }
        let end = content[3..].find("---")?;
        if end == 0 {
            return None;
        }
        semio_framework_repo_yaml::unmarshal(&content[3..3 + end]).ok()
    }

    /// 🧬️ Infers a bundle kind from a bundle's own `package.json` or `project.json`.
    pub fn derive_bundle_kind(&self, root: &str) -> BundleKind {
        for config in ["package.json", "project.json"] {
            let Some(content) = self.read_text(&format!("{root}/{config}")) else { continue };
            let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else { continue };
            let Some(raw) = value.get("bundleKind").and_then(serde_json::Value::as_str) else { continue };
            if let Ok(kind) = serde_json::from_value::<BundleKind>(serde_json::Value::String(raw.to_lowercase())) {
                return kind;
            }
        }
        BundleKind::Library
    }

    /// 📦️ Reads a bundle's declared packages from its `package.json`.
    fn load_packages(&self, root: &str) -> Vec<Package> {
        let Some(content) = self.read_text(&format!("{root}/package.json")) else { return Vec::new() };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else { return Vec::new() };
        let name = value.get("name").and_then(serde_json::Value::as_str).unwrap_or_default();
        if name.is_empty() {
            return Vec::new();
        }
        vec![Package {
            name: name.to_string(),
            version: value.get("version").and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
            path: root.to_string(),
            kind: "npm".to_string(),
        }]
    }

    /// 🧱️ Reads the `sourceRoot` and `tags` a bundle's project or package manifest declares.
    fn apply_manifest(&self, bundle: &mut Bundle) {
        let mut config = format!("{}/project.json", bundle.root);
        if !self.exists(&config) {
            config = format!("{}/package.json", bundle.root);
        }
        let Some(content) = self.read_text(&config) else { return };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) else { return };
        bundle.source_root = value.get("sourceRoot").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        bundle.tags = value
            .get("tags")
            .and_then(serde_json::Value::as_array)
            .map(|items| items.iter().filter_map(serde_json::Value::as_str).map(str::to_string).collect())
            .unwrap_or_default();
    }

    /// 🎨️ Reads a bundle's declared emoji from its `AGENTS.md` front matter.
    fn bundle_emoji(&self, root: &str) -> String {
        self.front_matter(&format!("{root}/AGENTS.md"))
            .and_then(|value| value.get("bundle").and_then(|bundle| bundle.get("emoji")).and_then(serde_json::Value::as_str).map(str::to_string))
            .unwrap_or_default()
    }

    /// 🏠️ Walks the repository root and returns every technology with its bundles.
    pub fn technologies(&self) -> Vec<Technology> {
        let mut cache = self.technologies.lock().expect("technology cache");
        if let Some(cached) = cache.as_ref() {
            return cached.clone();
        }
        let loaded = self.load_technologies();
        *cache = Some(loaded.clone());
        loaded
    }

    /// 🔃️ Every bundle of every technology, in technology order.
    pub fn bundles(&self) -> Vec<Bundle> {
        self.technologies().into_iter().flat_map(|technology| technology.bundles.unwrap_or_default()).collect()
    }

    /// 📖️ The uncached technology walk.
    fn load_technologies(&self) -> Vec<Technology> {
        let mut technologies = Vec::new();
        for name in read_dir_names(&self.root_dir, true) {
            if !self.is_technology_dir(&name) {
                continue;
            }
            let raw_name = name.strip_prefix('@').unwrap_or(&name).to_string();
            let mut technology_name = raw_name.clone();
            let mut technology_kind = derive_technology_kind(&raw_name);
            if let Some(matter) = self.front_matter(&format!("{name}/README.md")) {
                if let Some(declared) = matter.get("name").and_then(serde_json::Value::as_str) {
                    if !declared.is_empty() {
                        technology_name = declared.to_string();
                    }
                }
                if let Some(declared) = matter.get("kind").and_then(serde_json::Value::as_str) {
                    match declared.to_lowercase().as_str() {
                        "user" => technology_kind = TechnologyKind::User,
                        "infrastructure" => technology_kind = TechnologyKind::Infrastructure,
                        "research" => technology_kind = TechnologyKind::Research,
                        _ => {}
                    }
                }
            }
            let emoji = self
                .front_matter(&format!("{name}/AGENTS.md"))
                .and_then(|matter| matter.get("emoji").and_then(serde_json::Value::as_str).map(str::to_string))
                .unwrap_or_default();
            let mut bundles = Vec::new();
            for sub in read_dir_names(&self.root_dir.join(&name), true) {
                if sub.starts_with('.') || sub == "node_modules" {
                    continue;
                }
                if sub == "sites" {
                    for site in read_dir_names(&self.root_dir.join(&name).join(&sub), true) {
                        if site.starts_with('.') || site == "node_modules" {
                            continue;
                        }
                        let root = format!("{name}/{sub}/{site}");
                        let mut bundle = Bundle {
                            name: format!("{technology_name}/{site}"),
                            root: root.clone(),
                            source_root: String::new(),
                            technology_name: technology_name.clone(),
                            tags: Vec::new(),
                            kind: BundleKind::Site,
                            emoji: self.bundle_emoji(&root),
                            packages: self.load_packages(&root),
                        };
                        self.apply_manifest(&mut bundle);
                        bundles.push(bundle);
                    }
                    continue;
                }
                let root = format!("{name}/{sub}");
                let mut bundle = Bundle {
                    name: format!("{technology_name}/{sub}"),
                    root: root.clone(),
                    source_root: String::new(),
                    technology_name: technology_name.clone(),
                    tags: Vec::new(),
                    kind: self.derive_bundle_kind(&root),
                    emoji: self.bundle_emoji(&root),
                    packages: self.load_packages(&root),
                };
                self.apply_manifest(&mut bundle);
                bundles.push(bundle);
            }
            technologies.push(Technology { name: technology_name, root: name, kind: technology_kind, emoji, bundles: Some(bundles) });
        }
        technologies
    }

    /// 🔎️ The emoji a technology code resolves to, falling back to its derived kind.
    fn resolve_technology_emoji(&self, technology_code: &str) -> String {
        for technology in self.technologies() {
            if technology.name == technology_code {
                if !technology.emoji.is_empty() {
                    return technology.emoji;
                }
                break;
            }
        }
        derive_technology_kind(technology_code).as_str().to_string()
    }

    /// 🆔️ The emoji id of a technology.
    pub fn technology_id(&self, technology: &Technology) -> String {
        format!("{}{}", technology_kind_emoji(technology), flat(&technology.name))
    }

    /// 🎁️ The emoji id of a bundle.
    pub fn bundle_id(&self, bundle: &Bundle) -> String {
        let (technology_code, bundle_code) = split_bundle_name(&bundle.name);
        let technology_emoji = self.resolve_technology_emoji(&technology_code);
        format!("{}{}{}{}", emoji_text(&technology_emoji), flat(&technology_code), bundle_kind_emoji(bundle), flat(&bundle_code))
    }

    /// 📂️ The bundle owning a path — the deepest matching bundle root.
    pub fn bundle_by_path(&self, path: &str) -> Option<Bundle> {
        let normalized = normalize_path(path);
        let mut best: Option<Bundle> = None;
        let mut matched = 0usize;
        for bundle in self.bundles() {
            let root = normalize_path(&bundle.root);
            if (normalized.starts_with(&format!("{root}/")) || normalized == root) && root.len() > matched {
                matched = root.len();
                best = Some(bundle);
            }
        }
        best
    }
}
//#endregion 🏗️Technologies
//#endregion 🗂️Codebase
