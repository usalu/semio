//! 🔖️ Mutation source authority of the dsl derive expansion: the source authority records, the authority locators and schemas, the common and aggregate authority readers, the domain owners and the path normalisation, no-follow reads and workspace root lookup. See [`super`].

use super::*;

//#region 🔖️MutationSourceAuthority
#[derive(Debug)]
#[doc(hidden)] pub struct MutationSourceAuthority {
    #[doc(hidden)] pub workspace_root: PathBuf,
    #[doc(hidden)] pub mutation_root: PathBuf,
    #[doc(hidden)] pub owner: String,
    #[doc(hidden)] pub expected_semantic_kind: Option<String>,
    #[doc(hidden)] pub source_path: PathBuf,
    #[doc(hidden)] pub descriptor_path: PathBuf,
    #[doc(hidden)] pub taxonomy_path: PathBuf,
}

type MutationDomainOperations = Vec<(String, String)>;

/// 🧭️ Workspace-relative locator of the taxonomy's generated mutation-source-authority projection (`repo run
/// @semio-tech/dsl-derive-rs:generate`), the only authority file an expansion reads and tracks, so an edit elsewhere in the taxonomy,
/// `package.json` or any `📋️project.json` never invalidates a crate that derives mutations.
#[doc(hidden)] pub const MUTATION_AUTHORITY_LOCATOR: &str = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json";

/// 🪪️ Schema id the projection must declare (`MutationSourceAuthorityProjectionV1` in `🧬️schema/🔣️.json`).
#[doc(hidden)] pub const MUTATION_AUTHORITY_SCHEMA: &str = "semio.dsl.mutation-source-authority/v1";

/// 🗂️ Workspace-relative locator of the generated schema-document index (`repo run @semio-tech/dsl-derive-rs:generate-schema-index`).
#[doc(hidden)] pub const MUTATION_SCHEMA_INDEX_LOCATOR: &str = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json";

/// 🪪️ Schema id the index must declare (`MutationSchemaDocumentIndexV1` in `🧬️schema/🔣️.json`).
pub(super) const MUTATION_SCHEMA_INDEX_SCHEMA: &str = "semio.dsl.mutation-schema-document-index/v1";

#[derive(Debug)]
#[doc(hidden)]
pub struct MutationAuthorityCommon {
    pub(super) workspace_root: PathBuf,
    pub(super) source_path: PathBuf,
    pub(super) taxonomy_path: PathBuf,
    mutation_collection: String,
    pub(super) mutation_payload_facet: String,
    #[doc(hidden)] pub source_filename: String,
    #[doc(hidden)] pub descriptor_filename: String,
    domain_owners: BTreeMap<String, MutationDomainOperations>,
    aggregate_sources: BTreeMap<String, Vec<String>>,
}

#[derive(Debug)]
#[doc(hidden)]
pub struct MutationAggregateSourceAuthority {
    #[doc(hidden)] pub workspace_root: PathBuf,
    #[doc(hidden)] pub mutation_root: PathBuf,
    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)] pub source_path: PathBuf,
    #[doc(hidden)] pub taxonomy_path: PathBuf,
    pub(super) mutation_payload_facet: String,
    #[doc(hidden)] pub source_filename: String,
    #[doc(hidden)] pub descriptor_filename: String,
    #[doc(hidden)] pub domain_operations: Option<MutationDomainOperations>,
    pub(super) component_roots: Vec<(String, Option<MutationDomainOperations>)>,
}

#[doc(hidden)] pub fn mutation_authority_common(source: &Path, compiler_cwd: &Path) -> Result<MutationAuthorityCommon, String> {
    let source_path = mutation_authority_normalize(source, compiler_cwd)?;
    let workspace_root = mutation_authority_workspace_root(&source_path)?;
    mutation_authority_no_follow(&workspace_root, &source_path, false)?;
    let taxonomy_path = mutation_authority_locator(&workspace_root, MUTATION_AUTHORITY_LOCATOR)?;
    mutation_authority_no_follow(&workspace_root, &taxonomy_path, false).map_err(|error| format!("mutation authority projection {MUTATION_AUTHORITY_LOCATOR}: {error}; run repo run @semio-tech/dsl-derive-rs:generate"))?;
    let authority: serde_json::Value = serde_json::from_slice(&crate::compiler_resources::read(&taxonomy_path).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    if authority.get("schema").and_then(serde_json::Value::as_str) != Some(MUTATION_AUTHORITY_SCHEMA) { return Err("mutation authority projection declares another schema".to_string()); }
    let source_filename = mutation_authority_segment(&authority, "sourceFilename")?;
    let descriptor_filename = mutation_authority_segment(&authority, "descriptorFilename")?;
    let mutation_collection = mutation_authority_segment(&authority, "mutationCollection")?;
    let mutation_payload_facet = mutation_authority_segment(&authority, "mutationPayloadFacet")?;
    let domain_owners = mutation_authority_domain_owners(&authority, &mutation_collection)?;
    let aggregate_sources = mutation_authority_aggregate_sources(&authority, &mutation_collection)?;
    Ok(MutationAuthorityCommon { workspace_root, source_path, taxonomy_path, mutation_collection, mutation_payload_facet, source_filename, descriptor_filename, domain_owners, aggregate_sources })
}

#[doc(hidden)] pub fn mutation_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationSourceAuthority, String> {
    let common = mutation_authority_common(source, compiler_cwd)?;
    let MutationAuthorityCommon { workspace_root, source_path, taxonomy_path, mutation_collection, mutation_payload_facet, source_filename, descriptor_filename, domain_owners, .. } = common;
    if source_path.file_name().and_then(|name| name.to_str()) != Some(source_filename.as_str()) { return Err("source is not the taxonomy canonical mutation primary".to_string()); }
    let source_parent = source_path.parent().ok_or_else(|| "source has no owner directory".to_string())?;
    let owner_path = if source_parent.file_name().and_then(|name| name.to_str()) == Some(mutation_payload_facet.as_str()) {
        source_parent.parent().ok_or_else(|| "mutation payload facet has no semantic owner".to_string())?
    } else {
        source_parent
    };
    let owner = mutation_authority_relative(&workspace_root, owner_path)?;
    let parent = owner_path.parent().ok_or_else(|| "source owner has no collection parent".to_string())?;
    let (mutation_root, expected_semantic_kind) = if parent.file_name().and_then(|name| name.to_str()) == Some(mutation_collection.as_str()) {
        let root = mutation_authority_relative(&workspace_root, parent)?;
        if domain_owners.contains_key(&root) { return Err("flat source owner is not registered under its domain-operation root".to_string()); }
        (parent.to_path_buf(), None)
    } else {
        let root = parent.parent().ok_or_else(|| "source owner has no domain-operation root".to_string())?;
        if root.file_name().and_then(|name| name.to_str()) != Some(mutation_collection.as_str()) { return Err("source is neither a flat mutation owner nor one registered domain operation".to_string()); }
        let root_name = mutation_authority_relative(&workspace_root, root)?;
        let operations = domain_owners.get(&root_name).ok_or_else(|| "domain-operation root is not explicitly registered".to_string())?;
        let identity = operations.iter().find(|(registered, _)| registered == &owner).map(|(_, identity)| identity.clone()).ok_or_else(|| "source owner is not an exact registered domain operation".to_string())?;
        (root.to_path_buf(), Some(identity))
    };
    let descriptor_path = owner_path.join(descriptor_filename);
    mutation_authority_no_follow(&workspace_root, &descriptor_path, false)?;
    let descriptor = crate::compiler_resources::read(&descriptor_path).map_err(|error| error.to_string())?;
    let authority = MutationSourceAuthority { workspace_root, mutation_root, owner, expected_semantic_kind, source_path, descriptor_path, taxonomy_path };
    parse_mutation_leaf_descriptor(&descriptor, &authority)?;
    Ok(authority)
}

#[doc(hidden)] pub fn mutation_aggregate_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationAggregateSourceAuthority, String> {
    let common = mutation_authority_common(source, compiler_cwd)?;
    if common.source_path.file_name().and_then(|name| name.to_str()) != Some(common.source_filename.as_str()) { return Err("aggregate source is not the taxonomy canonical mutation primary".to_string()); }
    let mutation_root = common.source_path.parent().ok_or_else(|| "aggregate source has no mutation collection directory".to_string())?;
    mutation_authority_no_follow(&common.workspace_root, mutation_root, true)?;
    if mutation_root.file_name().and_then(|name| name.to_str()) != Some(common.mutation_collection.as_str()) { return Err("aggregate source is not directly inside the taxonomy mutation collection".to_string()); }
    let root_name = mutation_authority_relative(&common.workspace_root, mutation_root)?;
    let domain_operations = common.domain_owners.get(&root_name).cloned();
    let mut component_roots = Vec::new();
    if let Some(roots) = common.aggregate_sources.get(&root_name) {
        if domain_operations.is_some() { return Err("a mutation aggregate must declare either direct domain owners or component sources".to_string()); }
        for root in roots {
            mutation_authority_no_follow(&common.workspace_root, &common.workspace_root.join(root), true)?;
            component_roots.push((root.clone(), common.domain_owners.get(root).cloned()));
        }
    }
    Ok(MutationAggregateSourceAuthority {
        workspace_root: common.workspace_root,
        mutation_root: mutation_root.to_path_buf(),
        #[cfg(any(test, feature = "testkit"))]
        source_path: common.source_path,
        taxonomy_path: common.taxonomy_path,
        mutation_payload_facet: common.mutation_payload_facet,
        source_filename: common.source_filename,
        descriptor_filename: common.descriptor_filename,
        domain_operations,
        component_roots,
    })
}

#[doc(hidden)] pub fn mutation_authority_aggregate_sources(taxonomy: &serde_json::Value, collection: &str) -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut result = BTreeMap::new();
    let Some(registry) = taxonomy.get("mutationAggregateSources") else { return Ok(result); };
    let roots = registry.as_object().ok_or_else(|| "mutationAggregateSources must be an exact-root object".to_string())?;
    let valid = |root: &str| root.ends_with(&format!("/{collection}")) && root.split('/').all(|part| mutation_authority_owner_segment(part) && !part.eq_ignore_ascii_case("compose"));
    for (root, sources) in roots {
        if !valid(root) { return Err("mutationAggregateSources contains an unsafe or non-mutation aggregate root".to_string()); }
        let sources = sources.as_array().filter(|sources| !sources.is_empty()).ok_or_else(|| "mutationAggregateSources requires non-empty source arrays".to_string())?;
        let mut members = Vec::new();
        for source in sources {
            let source = source.as_str().filter(|source| valid(source)).ok_or_else(|| "mutationAggregateSources contains an unsafe or non-mutation component source".to_string())?;
            if members.iter().any(|member| member == source) { return Err("mutationAggregateSources contains a duplicate component source".to_string()); }
            members.push(source.to_string());
        }
        result.insert(root.clone(), members);
    }
    Ok(result)
}

#[doc(hidden)] pub fn mutation_authority_domain_owners(taxonomy: &serde_json::Value, collection: &str) -> Result<BTreeMap<String, MutationDomainOperations>, String> {
    let mut result = BTreeMap::new();
    let Some(registry) = taxonomy.get("mutationDomainOwners") else { return Ok(result); };
    let roots = registry.as_object().ok_or_else(|| "mutationDomainOwners must be an exact-root object".to_string())?;
    for (root, domains) in roots {
        if !root.ends_with(&format!("/{collection}")) || root.split('/').any(|part| !mutation_authority_owner_segment(part)) { return Err("mutationDomainOwners contains an unsafe or non-mutation root".to_string()); }
        let domains = domains.as_object().filter(|value| !value.is_empty()).ok_or_else(|| "registered mutation root must declare non-empty domains".to_string())?;
        let mut owners = Vec::new();
        let mut identities = HashSet::new();
        for (domain, operations) in domains {
            if !mutation_authority_owner_segment(domain) { return Err("registered mutation domain must be one safe basename".to_string()); }
            let operations = operations.as_object().filter(|value| !value.is_empty()).ok_or_else(|| "registered domain must declare non-empty operations".to_string())?;
            for (operation, identity) in operations {
                if !mutation_authority_owner_segment(operation) { return Err("registered mutation operation must be one safe basename".to_string()); }
                let identity = identity.as_str().filter(|value| mutation_leaf_kebab(value)).ok_or_else(|| "registered mutation identity must be full kebab-case".to_string())?;
                if !identities.insert(identity.to_string()) { return Err("registered mutation root has a duplicate semantic identity".to_string()); }
                owners.push((format!("{root}/{domain}/{operation}"), identity.to_string()));
            }
        }
        result.insert(root.clone(), owners);
    }
    Ok(result)
}

fn mutation_authority_owner_segment(value: &str) -> bool {
    !value.is_empty() && value != "." && value != ".." && !value.eq_ignore_ascii_case("compose") && !value.chars().any(|character| character.is_control() || matches!(character, '/' | '\\' | ':' | '*' | '?' | '{' | '}' | '\u{2028}' | '\u{2029}'))
}

fn mutation_authority_normalize(source: &Path, compiler_cwd: &Path) -> Result<PathBuf, String> {
    if !compiler_cwd.is_absolute() { return Err("compiler cwd is not absolute".to_string()); }
    mutation_authority_raw_lexical(compiler_cwd)?;
    let input = if source.is_absolute() { source.to_path_buf() } else { compiler_cwd.join(source) };
    mutation_authority_raw_no_follow(&input)?;
    let mut normalized = PathBuf::new();
    for component in input.components() {
        if let Component::Normal(segment) = component {
            if segment.to_str().is_some_and(|value| value.eq_ignore_ascii_case("compose")) { return Err("opaque compose path rejected before I/O".to_string()); }
        }
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {},
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir => { if !normalized.pop() { return Err("source escapes filesystem root".to_string()); } },
        }
    }
    if !normalized.is_absolute() { return Err("source is not absolute after compiler cwd resolution".to_string()); }
    Ok(normalized)
}

fn mutation_authority_raw_no_follow(path: &Path) -> Result<(), String> {
    if !path.is_absolute() { return Err("raw source path is not absolute".to_string()); }
    mutation_authority_raw_lexical(path)?;
    let mut current = PathBuf::new();
    let components: Vec<Component<'_>> = path.components().collect();
    for (index, component) in components.iter().enumerate() {
        match component {
            Component::Prefix(prefix) => current.push(prefix.as_os_str()),
            Component::RootDir => { current.push(component.as_os_str()); let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?; if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() { return Err("raw source root is not a regular directory".to_string()); } },
            Component::CurDir => {},
            Component::Normal(segment) => {
                current.push(segment);
                let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
                if metadata.file_type().is_symlink() { return Err("raw source symlink component rejected before normalization".to_string()); }
                if components[index + 1..].iter().any(|next| matches!(next, Component::Normal(_) | Component::ParentDir)) && !metadata.file_type().is_dir() { return Err("raw source intermediate component is not a directory".to_string()); }
                if !components[index + 1..].iter().any(|next| matches!(next, Component::Normal(_) | Component::ParentDir)) && !metadata.file_type().is_file() { return Err("raw source terminal component is not a regular file".to_string()); }
            },
            Component::ParentDir => { if !current.pop() { return Err("source escapes filesystem root".to_string()); } },
        }
    }
    Ok(())
}

fn mutation_authority_raw_lexical(path: &Path) -> Result<(), String> {
    for component in path.components() {
        if let Component::Normal(segment) = component {
            let segment = segment.to_str().ok_or_else(|| "raw source path component is not UTF-8".to_string())?;
            if segment.contains('\0') || segment.contains('\\') { return Err("raw source path component is not a portable owner segment".to_string()); }
            if segment.eq_ignore_ascii_case("compose") { return Err("opaque compose path rejected before I/O".to_string()); }
        }
    }
    Ok(())
}

#[doc(hidden)] pub fn mutation_authority_workspace_root(source: &Path) -> Result<PathBuf, String> {
    for ancestor in source.parent().into_iter().flat_map(Path::ancestors) {
        let package = ancestor.join("package.json");
        match mutation_authority_node(&package) {
            "missing" => continue,
            "symlink" => return Err("workspace package.json marker is a symlink".to_string()),
            "file" => {},
            _ => return Err("workspace package.json marker is not a regular file".to_string()),
        }
        let package: serde_json::Value = serde_json::from_slice(&fs::read(&package).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
        if !matches!(package.get("workspaces"), Some(serde_json::Value::Array(_) | serde_json::Value::Object(_))) { continue; }
        match mutation_authority_node(&ancestor.join("📋️project.json")) {
            "file" => return Ok(ancestor.to_path_buf()),
            "missing" if mutation_authority_node(&ancestor.join("bun.lock")) == "file" => continue,
            "missing" => return Err("workspace package.json declaration lacks paired 📋️project.json".to_string()),
            "symlink" => return Err("workspace 📋️project.json marker is a symlink".to_string()),
            _ => return Err("workspace 📋️project.json marker is not a regular file".to_string()),
        }
    }
    Err("source has no exact package workspace declaration and 📋️project.json pair".to_string())
}

fn mutation_authority_node(path: &Path) -> &'static str {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => "symlink",
        Ok(metadata) if metadata.file_type().is_file() => "file",
        Ok(_) => "other",
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => "missing",
        Err(_) => "other",
    }
}

pub(super) fn mutation_authority_no_follow(root: &Path, target: &Path, directory: bool) -> Result<(), String> {
    let relative = target.strip_prefix(root).map_err(|_| "path escapes workspace root".to_string())?;
    let mut current = root.to_path_buf();
    let root_metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
    if root_metadata.file_type().is_symlink() || !root_metadata.file_type().is_dir() { return Err("workspace root is not a regular directory".to_string()); }
    for component in relative.components() {
        let Component::Normal(segment) = component else { return Err("path is not lexically normalized".to_string()); };
        if segment.to_str().is_some_and(|value| value.eq_ignore_ascii_case("compose")) { return Err("opaque compose path rejected before I/O".to_string()); }
        current.push(segment);
        let metadata = fs::symlink_metadata(&current).map_err(|error| error.to_string())?;
        if metadata.file_type().is_symlink() { return Err("symlink path component rejected".to_string()); }
    }
    let metadata = fs::symlink_metadata(target).map_err(|error| error.to_string())?;
    if directory && !metadata.file_type().is_dir() { return Err("expected regular directory".to_string()); }
    if !directory && !metadata.file_type().is_file() { return Err("expected regular file".to_string()); }
    Ok(())
}

#[doc(hidden)] pub fn mutation_authority_locator(root: &Path, locator: &str) -> Result<PathBuf, String> {
    if locator.is_empty() || locator.contains('\0') || locator.contains('\\') || locator.starts_with('/') || locator.as_bytes().get(1) == Some(&b':') { return Err("mutation authority locator is not a normalized repository-relative path".to_string()); }
    let mut target = root.to_path_buf();
    for segment in locator.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.eq_ignore_ascii_case("compose") { return Err("mutation authority locator has a rejected path component".to_string()); }
        target.push(segment);
    }
    Ok(target)
}

#[doc(hidden)] pub fn mutation_authority_segment(authority: &serde_json::Value, key: &str) -> Result<String, String> {
    authority.get(key).and_then(serde_json::Value::as_str).filter(|value| mutation_authority_owner_segment(value)).map(str::to_string).ok_or_else(|| format!("mutation authority projection {key} is not one portable path segment"))
}

#[doc(hidden)] pub fn mutation_authority_relative(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path.strip_prefix(root).map_err(|_| "owner escapes workspace root".to_string())?;
    let mut segments = Vec::new();
    for component in relative.components() {
        let Component::Normal(segment) = component else { return Err("owner is not normalized".to_string()); };
        let segment = segment.to_str().ok_or_else(|| "owner is not UTF-8".to_string())?;
        if segment.contains('\0') || segment.contains('\\') || segment.eq_ignore_ascii_case("compose") { return Err("owner has rejected cross-platform path component".to_string()); }
        segments.push(segment);
    }
    if segments.is_empty() { return Err("owner is workspace root".to_string()); }
    Ok(segments.join("/"))
}
//#endregion 🔖️MutationSourceAuthority
