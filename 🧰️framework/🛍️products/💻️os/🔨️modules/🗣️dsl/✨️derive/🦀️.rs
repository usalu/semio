//! 🏭️ Product artifact envelopes, OS diff codecs, and source-authorized mutation derives.

use proc_macro::TokenStream;
use quote::quote;
use std::{collections::{BTreeMap, HashSet}, fs, path::{Component, Path, PathBuf}};
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[cfg(test)]
#[path = "🧪️tests/📤️macro-exports/🦀️.rs"]
mod macro_export_tests;

#[cfg(test)]
#[path = "🧪️tests/🪪️mandatory-mutation-descriptor/🦀️.rs"]
mod mandatory_mutation_descriptor_tests;

//#region 🔖️MutationSourceAuthority
#[derive(Debug)]
struct MutationSourceAuthority {
    workspace_root: PathBuf,
    mutation_root: PathBuf,
    owner: String,
    expected_semantic_kind: Option<String>,
    source_path: PathBuf,
    descriptor_path: PathBuf,
    taxonomy_path: PathBuf,
}

type MutationDomainOperations = Vec<(String, String)>;

/// 🧭️ Workspace-relative locator of the taxonomy's generated mutation-source-authority projection (`bun nx run
/// @semio-tech/dsl-derive-rs:generate`), the only authority file an expansion reads and tracks, so an edit elsewhere in the taxonomy,
/// `nx.json` or any `📋️project.json` never invalidates a crate that derives mutations.
const MUTATION_AUTHORITY_LOCATOR: &str = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json";

/// 🪪️ Schema id the projection must declare (`MutationSourceAuthorityProjectionV1` in `🧬️schema/🔣️.json`).
const MUTATION_AUTHORITY_SCHEMA: &str = "semio.dsl.mutation-source-authority/v1";

#[derive(Debug)]
struct MutationAuthorityCommon {
    workspace_root: PathBuf,
    source_path: PathBuf,
    taxonomy_path: PathBuf,
    mutation_collection: String,
    mutation_payload_facet: String,
    source_filename: String,
    descriptor_filename: String,
    domain_owners: BTreeMap<String, MutationDomainOperations>,
    aggregate_sources: BTreeMap<String, Vec<String>>,
}

#[derive(Debug)]
struct MutationAggregateSourceAuthority {
    workspace_root: PathBuf,
    mutation_root: PathBuf,
    #[cfg(test)]
    source_path: PathBuf,
    taxonomy_path: PathBuf,
    mutation_payload_facet: String,
    source_filename: String,
    descriptor_filename: String,
    domain_operations: Option<MutationDomainOperations>,
    component_roots: Vec<(String, Option<MutationDomainOperations>)>,
}

fn mutation_authority_common(source: &Path, compiler_cwd: &Path) -> Result<MutationAuthorityCommon, String> {
    let source_path = mutation_authority_normalize(source, compiler_cwd)?;
    let workspace_root = mutation_authority_workspace_root(&source_path)?;
    mutation_authority_no_follow(&workspace_root, &source_path, false)?;
    let taxonomy_path = mutation_authority_locator(&workspace_root, MUTATION_AUTHORITY_LOCATOR)?;
    mutation_authority_no_follow(&workspace_root, &taxonomy_path, false).map_err(|error| format!("mutation authority projection {MUTATION_AUTHORITY_LOCATOR}: {error}; run bun nx run @semio-tech/dsl-derive-rs:generate"))?;
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

fn mutation_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationSourceAuthority, String> {
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

fn mutation_aggregate_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationAggregateSourceAuthority, String> {
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
        #[cfg(test)]
        source_path: common.source_path,
        taxonomy_path: common.taxonomy_path,
        mutation_payload_facet: common.mutation_payload_facet,
        source_filename: common.source_filename,
        descriptor_filename: common.descriptor_filename,
        domain_operations,
        component_roots,
    })
}

fn mutation_authority_aggregate_sources(taxonomy: &serde_json::Value, collection: &str) -> Result<BTreeMap<String, Vec<String>>, String> {
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

fn mutation_authority_domain_owners(taxonomy: &serde_json::Value, collection: &str) -> Result<BTreeMap<String, MutationDomainOperations>, String> {
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

fn mutation_authority_workspace_root(source: &Path) -> Result<PathBuf, String> {
    for ancestor in source.parent().into_iter().flat_map(Path::ancestors) {
        let nx = ancestor.join("nx.json");
        let project = ancestor.join("📋️project.json");
        match mutation_authority_node(&nx) {
            "missing" => continue,
            "symlink" => return Err("workspace nx.json marker is a symlink".to_string()),
            "file" => {},
            _ => return Err("workspace nx.json marker is not a regular file".to_string()),
        }
        match mutation_authority_node(&project) {
            "file" => return Ok(ancestor.to_path_buf()),
            "missing" => return Err("workspace nx.json marker lacks paired 📋️project.json".to_string()),
            "symlink" => return Err("workspace 📋️project.json marker is a symlink".to_string()),
            _ => return Err("workspace 📋️project.json marker is not a regular file".to_string()),
        }
    }
    Err("source has no exact nx.json and 📋️project.json workspace pair".to_string())
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

fn mutation_authority_no_follow(root: &Path, target: &Path, directory: bool) -> Result<(), String> {
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

fn mutation_authority_locator(root: &Path, locator: &str) -> Result<PathBuf, String> {
    if locator.is_empty() || locator.contains('\0') || locator.contains('\\') || locator.starts_with('/') || locator.as_bytes().get(1) == Some(&b':') { return Err("mutation authority locator is not a normalized repository-relative path".to_string()); }
    let mut target = root.to_path_buf();
    for segment in locator.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.eq_ignore_ascii_case("compose") { return Err("mutation authority locator has a rejected path component".to_string()); }
        target.push(segment);
    }
    Ok(target)
}

fn mutation_authority_segment(authority: &serde_json::Value, key: &str) -> Result<String, String> {
    authority.get(key).and_then(serde_json::Value::as_str).filter(|value| mutation_authority_owner_segment(value)).map(str::to_string).ok_or_else(|| format!("mutation authority projection {key} is not one portable path segment"))
}

fn mutation_authority_relative(root: &Path, path: &Path) -> Result<String, String> {
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

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-source-authority/🦀️.rs"]
mod mutation_source_authority_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-aggregate-source-authority/🦀️.rs"]
mod mutation_aggregate_source_authority_tests;
//#endregion 🔖️MutationSourceAuthority

//#region 🔣️MutationLeafJson
#[derive(Debug, PartialEq, Eq)]
enum MutationLeafInvertibility { SelfInvertible, ExplicitMutation, Plan, NonInvertible }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafDiffParticipation { Detect, ApplyOnly, Plan, None }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafOutcomeClass { Applied, NoOp, Empty, Disjoint, Rejected }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafComposition { Atomic, Composite }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafLanguageSurface { Rust, Typescript, Graphql, Protobuf, JsonSchema, Text, Binary }

#[derive(Debug, PartialEq, Eq)]
struct MutationLeafJson {
    schema_version: u32,
    owner: String,
    semantic_kind: String,
    display_name: String,
    emoji: String,
    aggregate_variant: String,
    payload_schema: String,
    text_opcode: Option<String>,
    binary_tag: Option<u32>,
    invertibility: MutationLeafInvertibility,
    diff_participation: MutationLeafDiffParticipation,
    outcome_classes: Vec<MutationLeafOutcomeClass>,
    composition: MutationLeafComposition,
    required_language_surfaces: Vec<MutationLeafLanguageSurface>,
    editable: bool,
}

const MUTATION_LEAF_DESCRIPTOR_KEYS: [&str; 14] = ["schemaVersion", "owner", "semanticKind", "displayName", "emoji", "aggregateVariant", "payloadSchema", "textOpcode", "binaryTag", "invertibility", "diffParticipation", "outcomeClasses", "composition", "requiredLanguageSurfaces"];

/// ✏️ The one optional descriptor key: `false` declares the leaf withdraw-only (design §22.20); absent means editable.
const MUTATION_LEAF_EDITABLE_KEY: &str = "editable";

fn parse_mutation_leaf_descriptor(raw: &[u8], authority: &MutationSourceAuthority) -> Result<MutationLeafJson, String> {
    mutation_leaf_reject_duplicate_keys(raw)?;
    let value: serde_json::Value = serde_json::from_slice(raw).map_err(|error| format!("malformed mutation descriptor JSON: {error}"))?;
    let object = value.as_object().ok_or_else(|| "mutation descriptor must be an object".to_string())?;
    if MUTATION_LEAF_DESCRIPTOR_KEYS.iter().any(|key| !object.contains_key(*key)) || object.keys().any(|key| !MUTATION_LEAF_DESCRIPTOR_KEYS.contains(&key.as_str()) && key != MUTATION_LEAF_EDITABLE_KEY) { return Err("mutation descriptor must contain exactly the fourteen schema fields, and beside them only the optional editable".to_string()); }
    let string = |key| mutation_leaf_string(object.get(key).unwrap(), key);
    let schema_version = mutation_leaf_u32(object.get("schemaVersion").unwrap(), "schemaVersion")?;
    if schema_version != 1 { return Err("schemaVersion must equal 1".to_string()); }
    let owner = string("owner")?;
    if owner != authority.owner { return Err("descriptor owner does not exactly match source owner".to_string()); }
    let semantic_kind = string("semanticKind")?;
    if !mutation_leaf_kebab(&semantic_kind) { return Err("semanticKind must be lowercase kebab-case".to_string()); }
    if authority.expected_semantic_kind.as_ref().is_some_and(|expected| expected != &semantic_kind) { return Err("descriptor semanticKind does not match its exact registered domain owner".to_string()); }
    let display_name = string("displayName")?;
    let emoji = string("emoji")?;
    let aggregate_variant = string("aggregateVariant")?;
    if !mutation_leaf_pascal(&aggregate_variant) { return Err("aggregateVariant must be ASCII PascalCase".to_string()); }
    let payload_schema = string("payloadSchema")?;
    let text_opcode = match object.get("textOpcode").unwrap() { serde_json::Value::Null => None, value => { let value = mutation_leaf_string(value, "textOpcode")?; if !mutation_leaf_kebab(&value) { return Err("textOpcode must be lowercase kebab-case or null".to_string()); } Some(value) } };
    let binary_tag = match object.get("binaryTag").unwrap() { serde_json::Value::Null => None, value => Some(mutation_leaf_u32(value, "binaryTag")?) };
    let invertibility = match string("invertibility")?.as_str() { "self" => MutationLeafInvertibility::SelfInvertible, "explicit-mutation" => MutationLeafInvertibility::ExplicitMutation, "plan" => MutationLeafInvertibility::Plan, "non-invertible" => MutationLeafInvertibility::NonInvertible, _ => return Err("invertibility is not a schema enum value".to_string()) };
    let diff_participation = match string("diffParticipation")?.as_str() { "detect" => MutationLeafDiffParticipation::Detect, "apply-only" => MutationLeafDiffParticipation::ApplyOnly, "plan" => MutationLeafDiffParticipation::Plan, "none" => MutationLeafDiffParticipation::None, _ => return Err("diffParticipation is not a schema enum value".to_string()) };
    let outcome_classes = mutation_leaf_outcomes(object.get("outcomeClasses").unwrap())?;
    let composition = match string("composition")?.as_str() { "atomic" => MutationLeafComposition::Atomic, "composite" => MutationLeafComposition::Composite, _ => return Err("composition is not a schema enum value".to_string()) };
    let required_language_surfaces = mutation_leaf_surfaces(object.get("requiredLanguageSurfaces").unwrap())?;
    let editable = match object.get(MUTATION_LEAF_EDITABLE_KEY) { None => true, Some(serde_json::Value::Bool(value)) => *value, Some(_) => return Err("editable must be a boolean".to_string()) };
    Ok(MutationLeafJson { schema_version, owner, semantic_kind, display_name, emoji, aggregate_variant, payload_schema, text_opcode, binary_tag, invertibility, diff_participation, outcome_classes, composition, required_language_surfaces, editable })
}

fn mutation_leaf_string(value: &serde_json::Value, key: &str) -> Result<String, String> { value.as_str().filter(|value| !value.is_empty()).map(str::to_owned).ok_or_else(|| format!("{key} must be a nonempty string")) }

fn mutation_leaf_u32(value: &serde_json::Value, key: &str) -> Result<u32, String> {
    let number = value.as_f64().ok_or_else(|| format!("{key} must be an integer"))?;
    if !number.is_finite() || number.fract() != 0.0 || number < 0.0 || number > u32::MAX as f64 { return Err(format!("{key} must be a u32 integer")); }
    Ok(number as u32)
}

/// 🔤️ Lowercase kebab-case with at least one hyphen — a mutation kind is always multi-word.
///
/// 🧭️This mirrors `mutation_leaf_descriptor_kebab` in the kernel
/// (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`), whose final expression IS
/// `hyphen`: a kind without one is rejected there no matter what this copy says. The two must agree.
///
/// 🐛️This once dropped the hyphen clause, on the reasoning that "kebab-case includes a single word"
/// and that no consumer splits the kind on `-`. Both are true and both are beside the point — the
/// kernel is the authority, and relaxing only this copy made single-word kinds pass the derive and
/// then panic in const-eval at `validate_mutation_leaf_source`, which reports every field failure
/// through one message ("Mutations leaf source must match its aggregate workspace and direct owner")
/// and so pointed nowhere near the kind. `✳️drawing`'s `rotate`/`scale`/`group`/`ungroup`/`flatten`/
/// `unflatten` were renamed to `rotate-node`/`scale-node`/`group-nodes`/… instead; the VERB stays the
/// single word, which is a separate descriptor field and is what `APPROVED_VERBS` gates.
fn mutation_leaf_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty() && bytes[0].is_ascii_lowercase() && bytes.contains(&b'-') && bytes.split(|byte| *byte == b'-').all(|part| !part.is_empty() && part.iter().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

fn mutation_leaf_pascal(value: &str) -> bool { let bytes = value.as_bytes(); !bytes.is_empty() && bytes[0].is_ascii_uppercase() && bytes.iter().all(|byte| byte.is_ascii_alphanumeric()) }

fn mutation_leaf_outcomes(value: &serde_json::Value) -> Result<Vec<MutationLeafOutcomeClass>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "outcomeClasses must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    values.iter().map(|value| { let value = mutation_leaf_string(value, "outcomeClasses")?; if !seen.insert(value.clone()) { return Err("outcomeClasses must not contain duplicates".to_string()); } match value.as_str() { "applied" => Ok(MutationLeafOutcomeClass::Applied), "no-op" => Ok(MutationLeafOutcomeClass::NoOp), "empty" => Ok(MutationLeafOutcomeClass::Empty), "disjoint" => Ok(MutationLeafOutcomeClass::Disjoint), "rejected" => Ok(MutationLeafOutcomeClass::Rejected), _ => Err("outcomeClasses contains a non-schema enum value".to_string()) } }).collect()
}

fn mutation_leaf_surfaces(value: &serde_json::Value) -> Result<Vec<MutationLeafLanguageSurface>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "requiredLanguageSurfaces must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    let surfaces: Vec<_> = values.iter().map(|value| { let value = mutation_leaf_string(value, "requiredLanguageSurfaces")?; if !seen.insert(value.clone()) { return Err("requiredLanguageSurfaces must not contain duplicates".to_string()); } match value.as_str() { "rust" => Ok(MutationLeafLanguageSurface::Rust), "typescript" => Ok(MutationLeafLanguageSurface::Typescript), "graphql" => Ok(MutationLeafLanguageSurface::Graphql), "protobuf" => Ok(MutationLeafLanguageSurface::Protobuf), "json-schema" => Ok(MutationLeafLanguageSurface::JsonSchema), "text" => Ok(MutationLeafLanguageSurface::Text), "binary" => Ok(MutationLeafLanguageSurface::Binary), _ => Err("requiredLanguageSurfaces contains a non-schema enum value".to_string()) } }).collect::<Result<_, _>>()?;
    if !surfaces.iter().any(|surface| matches!(surface, MutationLeafLanguageSurface::Rust)) { return Err("requiredLanguageSurfaces must contain rust".to_string()); }
    Ok(surfaces)
}

fn mutation_leaf_reject_duplicate_keys(raw: &[u8]) -> Result<(), String> {
    let mut index = mutation_leaf_skip_ws(raw, 0);
    if raw.get(index) != Some(&b'{') { return Err("mutation descriptor must be a JSON object".to_string()); }
    index += 1;
    let mut keys = HashSet::new();
    loop {
        index = mutation_leaf_skip_ws(raw, index);
        if raw.get(index) == Some(&b'}') { return Ok(()); }
        let key_start = index;
        index = mutation_leaf_string_end(raw, index).ok_or_else(|| "malformed mutation descriptor JSON key".to_string())?;
        let key: String = serde_json::from_slice(&raw[key_start..index]).map_err(|_| "malformed mutation descriptor JSON key".to_string())?;
        if !keys.insert(key) { return Err("mutation descriptor has a duplicate key".to_string()); }
        index = mutation_leaf_skip_ws(raw, index);
        if raw.get(index) != Some(&b':') { return Err("malformed mutation descriptor JSON key separator".to_string()); }
        index = mutation_leaf_json_value_end(raw, mutation_leaf_skip_ws(raw, index + 1)).ok_or_else(|| "malformed mutation descriptor JSON value".to_string())?;
        index = mutation_leaf_skip_ws(raw, index);
        match raw.get(index) { Some(b',') => index += 1, Some(b'}') => return Ok(()), _ => return Err("malformed mutation descriptor JSON object".to_string()) }
    }
}

fn mutation_leaf_skip_ws(raw: &[u8], mut index: usize) -> usize { while raw.get(index).is_some_and(|byte| byte.is_ascii_whitespace()) { index += 1; } index }
fn mutation_leaf_string_end(raw: &[u8], mut index: usize) -> Option<usize> { if raw.get(index) != Some(&b'\"') { return None; } index += 1; while let Some(byte) = raw.get(index) { match byte { b'\"' => return Some(index + 1), b'\\' => index += 2, 0..=0x1f => return None, _ => index += 1 } } None }
fn mutation_leaf_json_value_end(raw: &[u8], index: usize) -> Option<usize> {
    match raw.get(index)? { b'\"' => mutation_leaf_string_end(raw, index), b'{' => mutation_leaf_balanced_end(raw, index, b'{', b'}'), b'[' => mutation_leaf_balanced_end(raw, index, b'[', b']'), _ => { let end = raw[index..].iter().position(|byte| matches!(*byte, b',' | b'}' | b']') || byte.is_ascii_whitespace()).map_or(raw.len(), |offset| index + offset); (end > index).then_some(end) } }
}
fn mutation_leaf_balanced_end(raw: &[u8], mut index: usize, open: u8, close: u8) -> Option<usize> { let mut depth = 0usize; while let Some(byte) = raw.get(index) { if *byte == b'\"' { index = mutation_leaf_string_end(raw, index)?; continue; }
        if *byte == open { depth += 1; } else if *byte == close { depth -= 1; if depth == 0 { return Some(index + 1); } } index += 1; } None }

fn emit_mutation_leaf_descriptor(contract: &syn::Path, descriptor: &MutationLeafJson) -> proc_macro2::TokenStream {
    let schema_version = descriptor.schema_version; let owner = &descriptor.owner; let semantic_kind = &descriptor.semantic_kind; let display_name = &descriptor.display_name; let emoji = &descriptor.emoji; let aggregate_variant = &descriptor.aggregate_variant; let payload_schema = &descriptor.payload_schema;
    let text_opcode = descriptor.text_opcode.as_ref().map_or_else(|| quote!(::core::option::Option::None), |value| quote!(::core::option::Option::Some(#value))); let binary_tag = descriptor.binary_tag.map_or_else(|| quote!(::core::option::Option::None), |value| quote!(::core::option::Option::Some(#value)));
    let invertibility = match &descriptor.invertibility { MutationLeafInvertibility::SelfInvertible => quote!(#contract::MutationInvertibility::SelfInvertible), MutationLeafInvertibility::ExplicitMutation => quote!(#contract::MutationInvertibility::ExplicitMutation), MutationLeafInvertibility::Plan => quote!(#contract::MutationInvertibility::Plan), MutationLeafInvertibility::NonInvertible => quote!(#contract::MutationInvertibility::NonInvertible) };
    let diff_participation = match &descriptor.diff_participation { MutationLeafDiffParticipation::Detect => quote!(#contract::MutationDiffParticipation::Detect), MutationLeafDiffParticipation::ApplyOnly => quote!(#contract::MutationDiffParticipation::ApplyOnly), MutationLeafDiffParticipation::Plan => quote!(#contract::MutationDiffParticipation::Plan), MutationLeafDiffParticipation::None => quote!(#contract::MutationDiffParticipation::None) };
    let outcome_classes = descriptor.outcome_classes.iter().map(|value| match value { MutationLeafOutcomeClass::Applied => quote!(#contract::MutationOutcomeClass::Applied), MutationLeafOutcomeClass::NoOp => quote!(#contract::MutationOutcomeClass::NoOp), MutationLeafOutcomeClass::Empty => quote!(#contract::MutationOutcomeClass::Empty), MutationLeafOutcomeClass::Disjoint => quote!(#contract::MutationOutcomeClass::Disjoint), MutationLeafOutcomeClass::Rejected => quote!(#contract::MutationOutcomeClass::Rejected) });
    let composition = match &descriptor.composition { MutationLeafComposition::Atomic => quote!(#contract::MutationComposition::Atomic), MutationLeafComposition::Composite => quote!(#contract::MutationComposition::Composite) };
    let required_language_surfaces = descriptor.required_language_surfaces.iter().map(|value| match value { MutationLeafLanguageSurface::Rust => quote!(#contract::MutationLanguageSurface::Rust), MutationLeafLanguageSurface::Typescript => quote!(#contract::MutationLanguageSurface::Typescript), MutationLeafLanguageSurface::Graphql => quote!(#contract::MutationLanguageSurface::Graphql), MutationLeafLanguageSurface::Protobuf => quote!(#contract::MutationLanguageSurface::Protobuf), MutationLeafLanguageSurface::JsonSchema => quote!(#contract::MutationLanguageSurface::JsonSchema), MutationLeafLanguageSurface::Text => quote!(#contract::MutationLanguageSurface::Text), MutationLeafLanguageSurface::Binary => quote!(#contract::MutationLanguageSurface::Binary) });
    quote!(#contract::MutationLeafDescriptor { schema_version: #schema_version, owner: #owner, semantic_kind: #semantic_kind, display_name: #display_name, emoji: #emoji, aggregate_variant: #aggregate_variant, payload_schema: #payload_schema, text_opcode: #text_opcode, binary_tag: #binary_tag, invertibility: #invertibility, diff_participation: #diff_participation, outcome_classes: &[#(#outcome_classes),*], composition: #composition, required_language_surfaces: &[#(#required_language_surfaces),*] })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-leaf-json/🦀️.rs"]
mod mutation_leaf_json_tests;
//#endregion 🔣️MutationLeafJson

//#region 🪪️MutationLeaf
#[derive(Debug)]
struct MutationLeafAttrs { contract: syn::Path, payload: Option<syn::Ident>, input_schema: Option<syn::Path> }

fn parse_mutation_leaf_attrs(input: &DeriveInput) -> syn::Result<MutationLeafAttrs> {
    let mut contract = None;
    let mut payload = None;
    let mut input_schema = None;
    let mut found = false;
    for attribute in &input.attrs {
        if !attribute.path().is_ident("mutation_leaf") { continue; }
        if found { return Err(syn::Error::new_spanned(attribute, "duplicate mutation_leaf attribute")); }
        found = true;
        if matches!(&attribute.meta, syn::Meta::Path(_)) { continue; }
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("payload") {
                if payload.is_some() { return Err(meta.error("duplicate mutation_leaf payload")); }
                payload = Some(meta.value()?.parse::<syn::Ident>()?);
                return Ok(());
            }
            if meta.path.is_ident("input_schema") {
                if input_schema.is_some() { return Err(meta.error("duplicate mutation_leaf input_schema")); }
                input_schema = Some(meta.value()?.parse::<syn::Path>()?);
                return Ok(());
            }
            if !meta.path.is_ident("contract") { return Err(meta.error("unsupported mutation_leaf attribute")); }
            if contract.is_some() { return Err(meta.error("duplicate mutation_leaf contract")); }
            let path: syn::Path = meta.value()?.parse()?;
            if path.leading_colon.is_none() || path.segments.is_empty() || path.segments.iter().any(|segment| !matches!(segment.arguments, syn::PathArguments::None) || matches!(segment.ident.to_string().as_str(), "self" | "super" | "crate")) { return Err(meta.error("mutation_leaf contract must be an absolute non-generic Rust path without self, super, or crate segments")); }
            contract = Some(path);
            Ok(())
        })?;
    }
    if !found { return Err(syn::Error::new_spanned(input, "MutationLeaf requires #[mutation_leaf(contract = ::protocol)]")); }
    let contract = contract.ok_or_else(|| syn::Error::new_spanned(input, "MutationLeaf requires mutation_leaf contract"))?;
    if let Some(variant) = &payload {
        let Data::Enum(data) = &input.data else { return Err(syn::Error::new_spanned(variant, "mutation_leaf payload names a variant of an enum leaf")); };
        let wraps_one = data.variants.iter().find(|candidate| candidate.ident == *variant).is_some_and(|candidate| matches!(&candidate.fields, Fields::Unnamed(fields) if fields.unnamed.len() == 1));
        if !wraps_one { return Err(syn::Error::new_spanned(variant, "mutation_leaf payload names a variant that wraps exactly one payload")); }
        if let Some(path) = &input_schema { return Err(syn::Error::new_spanned(path, "mutation_leaf input_schema and payload are exclusive: a wrapped leaf's input schema is its payload variant's")); }
    }
    Ok(MutationLeafAttrs { contract, payload, input_schema })
}

fn mutation_leaf_portable_path(path: &Path) -> Result<String, String> { path.to_str().map(|path| path.replace('\\', "/")).filter(|path| !path.is_empty()).ok_or_else(|| "metadata path is not UTF-8".to_string()) }

/// 🧭️ Canonical path without the Windows verbatim prefix, so a canonical root and a canonical member share one prefix on every platform.
fn mutation_authority_canonical(path: &Path) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(path).map_err(|error| error.to_string())?;
    Ok(match canonical.to_str().and_then(|value| value.strip_prefix(r"\\?\")) { Some(plain) => PathBuf::from(plain), None => canonical })
}

fn mutation_authority_workspace_token(workspace_root: &Path, taxonomy_path: &Path) -> Result<[u8; 32], String> {
    let workspace_root = mutation_authority_canonical(workspace_root)?;
    let taxonomy_path = mutation_authority_canonical(taxonomy_path)?;
    let workspace = mutation_leaf_portable_path(&workspace_root)?;
    let taxonomy = mutation_authority_relative(&workspace_root, &taxonomy_path)?;
    let mut input = b"semio.mutation-source-provenance/v1\0".to_vec();
    for value in [workspace.as_bytes(), taxonomy.as_bytes()] { let length = u64::try_from(value.len()).map_err(|_| "metadata token component exceeds u64".to_string())?; input.extend_from_slice(&length.to_be_bytes()); input.extend_from_slice(value); }
    Ok(semio_framework_hash::Sha256::digest(&input))
}

fn mutation_leaf_workspace_token(authority: &MutationSourceAuthority) -> Result<[u8; 32], String> { mutation_authority_workspace_token(&authority.workspace_root, &authority.taxonomy_path) }

fn mutation_leaf_include_path(path: &Path) -> Result<String, String> { mutation_leaf_portable_path(path) }

/// 🚪️ The editing surface of a withdraw-only leaf (descriptor `editable: false`, design §22.20): `input_schema` is always `None`, so
/// the history editor never opens on it, and `with_input_value` refuses every edited payload — the editable-payload law
/// (`mutation_payload_round_trip_failures`) holds an inert leaf to that. `from_input_value` stays the trait's: feature rows and
/// fixtures still decode the leaf from its payload. Such a leaf declares neither `payload` nor `input_schema` — both name an
/// editable payload.
fn mutation_leaf_withdraw_only(name: &syn::Ident, descriptor: &MutationLeafJson, attrs: &MutationLeafAttrs) -> syn::Result<Option<proc_macro2::TokenStream>> {
    if descriptor.editable { return Ok(None); }
    if let Some(variant) = &attrs.payload { return Err(syn::Error::new_spanned(variant, "a withdraw-only leaf (descriptor editable: false) declares no mutation_leaf payload")); }
    if let Some(path) = &attrs.input_schema { return Err(syn::Error::new_spanned(path, "a withdraw-only leaf (descriptor editable: false) declares no mutation_leaf input_schema")); }
    Ok(Some(quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            ::core::option::Option::None
        }
        fn with_input_value(&self, _value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, ::std::format!("{} is withdraw-only", ::core::stringify!(#name))))
        }
    }))
}

pub fn expand_mutation_leaf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if matches!(input.data, Data::Union(_)) { return syn::Error::new_spanned(&input, "MutationLeaf does not support unions").to_compile_error().into(); }
    let attrs = match parse_mutation_leaf_attrs(&input) { Ok(attrs) => attrs, Err(error) => return error.to_compile_error().into() };
    let source = match input.ident.span().unwrap().local_file() { Some(source) => source, None => return syn::Error::new_spanned(&input, "MutationLeaf requires a local source file").to_compile_error().into() };
    let compiler_cwd = match std::env::current_dir() { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let authority = match mutation_source_authority(&source, &compiler_cwd) { Ok(authority) => authority, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf source authority failed: {error}")).to_compile_error().into() };
    let raw_descriptor = match crate::compiler_resources::read(&authority.descriptor_path) { Ok(raw) => raw, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let descriptor = match parse_mutation_leaf_descriptor(&raw_descriptor, &authority) { Ok(descriptor) => descriptor, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf descriptor failed: {error}")).to_compile_error().into() };
    let workspace_token = match mutation_leaf_workspace_token(&authority) { Ok(token) => token, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf provenance failed: {error}")).to_compile_error().into() };
    let mutation_root = match mutation_authority_relative(&authority.workspace_root, &authority.mutation_root) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let source_path = match mutation_authority_relative(&authority.workspace_root, &authority.source_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let descriptor_path = match mutation_authority_relative(&authority.workspace_root, &authority.descriptor_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let taxonomy_path = match mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let payload_schema_path = match mutation_leaf_payload_schema_path(&authority, &descriptor.payload_schema) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf payload schema failed: {error}")).to_compile_error().into() };
    let inverse_rows = match mutation_leaf_inverse_rows(&payload_schema_path) { Ok(rows) => rows, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf x-semio-inverse-rows failed: {error}")).to_compile_error().into() };
    let referenced_documents = match mutation_leaf_referenced_documents(&payload_schema_path).and_then(|paths| paths.iter().map(|path| mutation_leaf_include_path(path)).collect::<Result<Vec<_>, _>>()) { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf referenced schema documents failed: {error}")).to_compile_error().into() };
    let dependency_paths = [authority.taxonomy_path.clone(), authority.descriptor_path.clone(), payload_schema_path];
    let dependency_paths: Result<Vec<_>, _> = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect();
    let dependency_paths = match dependency_paths { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let [taxonomy_dependency, descriptor_dependency, payload_schema_dependency]: [String; 3] = match dependency_paths.try_into() { Ok(paths) => paths, Err(_) => unreachable!() };
    let name = &input.ident;
    let contract = &attrs.contract;
    let editable = attrs.payload.as_ref().map(|variant| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            match self { Self::#variant(_) => ::core::option::Option::Some(<Self as #contract::MutationLeaf>::PAYLOAD_SCHEMA), _ => ::core::option::Option::None }
        }
        fn input_value(&self) -> ::semio_framework_value::DslValue {
            match self { Self::#variant(payload) => ::semio_framework_value::ToValue::to_value(payload), _ => ::semio_framework_value::ToValue::to_value(self) }
        }
        fn with_input_value(&self, value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            match self {
                Self::#variant(_) => ::semio_framework_value::FromValue::from_value(value).map(Self::#variant),
                _ => ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,::std::format!("{} is editable only as {}", ::core::stringify!(#name), ::core::stringify!(#variant)))),
            }
        }
        fn from_input_value(value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
            ::semio_framework_value::FromValue::from_value(value).map(Self::#variant)
        }
    });
    let instance_schema = attrs.input_schema.as_ref().map(|path| quote! {
        fn input_schema(&self) -> ::core::option::Option<&'static str> {
            #path(self)
        }
    });
    let withdraw_only = match mutation_leaf_withdraw_only(&input.ident, &descriptor, &attrs) { Ok(tokens) => tokens, Err(error) => return error.to_compile_error().into() };
    let inverse_rows = mutation_leaf_inverse_rows_body(&inverse_rows, attrs.payload.as_ref());
    let owner = &authority.owner;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let descriptor = emit_mutation_leaf_descriptor(contract, &descriptor);
    let workspace_token = workspace_token.iter();
    quote! {
        const _: &str = ::core::include_str!(#taxonomy_dependency);
        const _: &str = ::core::include_str!(#descriptor_dependency);
        impl #impl_generics #contract::MutationLeaf for #name #ty_generics #where_clause {
            const DESCRIPTOR: #contract::MutationLeafDescriptor = #descriptor;
            const PROVENANCE: #contract::MutationSourceProvenance = #contract::MutationSourceProvenance { workspace_token: [#(#workspace_token),*], mutation_root: #mutation_root, owner: #owner, source_path: #source_path, descriptor_path: #descriptor_path, taxonomy_path: #taxonomy_path };
            const PAYLOAD_SCHEMA: &'static str = ::core::include_str!(#payload_schema_dependency);
            const PAYLOAD_SCHEMA_DOCUMENTS: &'static [&'static str] = &[#(::core::include_str!(#referenced_documents)),*];
            #editable
            #instance_schema
            #withdraw_only
            fn inverse_rows(&self) -> usize {
                #inverse_rows
            }
        }
    }.into()
}

/// 🧾️ A leaf payload schema's `x-semio-inverse-rows` (design §20.5): `fixed` rows plus `perTarget` rows per item of each named
/// array field, or one `bounded` constant; absent, one row.
struct MutationLeafInverseRows {
    fixed: usize,
    per_target: Vec<(String, usize)>,
}

/// 📖️ Reads `x-semio-inverse-rows` from the root of the payload schema at `payload_schema`.
fn mutation_leaf_inverse_rows(payload_schema: &Path) -> Result<MutationLeafInverseRows, String> {
    let raw = crate::compiler_resources::read(payload_schema).map_err(|error| error.to_string())?;
    let value: serde_json::Value = serde_json::from_slice(&raw).map_err(|error| format!("malformed payload schema: {error}"))?;
    let Some(rows) = value.get("x-semio-inverse-rows") else { return Ok(MutationLeafInverseRows { fixed: 1, per_target: Vec::new() }) };
    let object = rows.as_object().filter(|object| !object.is_empty()).ok_or_else(|| "x-semio-inverse-rows must be a nonempty object".to_string())?;
    if object.keys().any(|key| !["fixed", "perTarget", "bounded"].contains(&key.as_str())) { return Err("x-semio-inverse-rows admits only fixed, perTarget and bounded".to_string()); }
    if object.contains_key("bounded") && object.len() != 1 { return Err("x-semio-inverse-rows bounded excludes fixed and perTarget".to_string()); }
    let count = |key: &str| object.get(key).map(|value| value.as_u64().map(|count| count as usize).ok_or_else(|| format!("x-semio-inverse-rows {key} must be a nonnegative integer"))).transpose();
    let fixed = count("bounded")?.or(count("fixed")?).unwrap_or(0);
    let per_target = match object.get("perTarget") {
        None => Vec::new(),
        Some(fields) => fields
            .as_object()
            .filter(|fields| !fields.is_empty())
            .ok_or_else(|| "x-semio-inverse-rows perTarget must be a nonempty object".to_string())?
            .iter()
            .map(|(field, rows)| rows.as_u64().filter(|rows| *rows >= 1).map(|rows| (to_snake_ascii(field), rows as usize)).ok_or_else(|| format!("x-semio-inverse-rows perTarget {field} must be a positive integer")))
            .collect::<Result<_, _>>()?,
    };
    Ok(MutationLeafInverseRows { fixed, per_target })
}

/// 🐍️ The Rust field a camelCase payload property names.
fn to_snake_ascii(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 4);
    for character in value.chars() {
        if character.is_ascii_uppercase() {
            out.push('_');
            out.push(character.to_ascii_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

/// 🧮️ The generated `MutationLeaf::inverse_rows` body: the fixed rows plus each `perTarget` field's length times its rows, read
/// from the leaf itself or from its `payload` variant (any other variant declares the fixed rows).
fn mutation_leaf_inverse_rows_body(rows: &MutationLeafInverseRows, payload: Option<&syn::Ident>) -> proc_macro2::TokenStream {
    let fixed = proc_macro2::Literal::usize_unsuffixed(rows.fixed);
    if rows.per_target.is_empty() {
        return quote! { #fixed };
    }
    let terms = rows.per_target.iter().map(|(field, count)| {
        let field = syn::Ident::new(field, proc_macro2::Span::call_site());
        let count = proc_macro2::Literal::usize_unsuffixed(*count);
        quote! { + #count * payload.#field.len() }
    });
    match payload {
        None => quote! { let payload = self; #fixed #(#terms)* },
        Some(variant) => quote! { match self { Self::#variant(payload) => #fixed #(#terms)*, _ => #fixed } },
    }
}

/// 🧬️ The descriptor's `payloadSchema`, resolved beside the descriptor: a normalized relative path of portable
/// segments that stays inside the leaf and names a regular file reached without a symlink.
fn mutation_leaf_payload_schema_path(authority: &MutationSourceAuthority, payload_schema: &str) -> Result<PathBuf, String> {
    let leaf = authority.descriptor_path.parent().ok_or_else(|| "descriptor has no leaf directory".to_string())?;
    if payload_schema.starts_with('/') || payload_schema.contains('\\') || payload_schema.contains('\0') { return Err("payloadSchema is not a relative portable path".to_string()); }
    let mut path = leaf.to_path_buf();
    for segment in payload_schema.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.eq_ignore_ascii_case("compose") { return Err("payloadSchema has a rejected path segment".to_string()); }
        path.push(segment);
    }
    mutation_authority_no_follow(&authority.workspace_root, &path, false)?;
    Ok(path)
}

/// 🔗️ Every schema document the payload schema at `payload_schema` references by an absolute `$id` (fragment stripped),
/// transitively, among the `🧬️schema` JSON documents of its search tree ([`mutation_schema_search_root`]) — what the runtime
/// publishes beside the leaf (`MutationLeaf::PAYLOAD_SCHEMA_DOCUMENTS`), so an input that `$ref`s another facet of its scope
/// resolves. A reference the tree does not hold is left to the OS-wide registry, which framework scopes publish themselves.
/// Sorted, never the payload schema itself.
fn mutation_leaf_referenced_documents(payload_schema: &Path) -> Result<Vec<PathBuf>, String> {
    let index = mutation_schema_document_index(&mutation_schema_search_root(payload_schema));
    let (own, mut pending) = mutation_schema_document_references(payload_schema)?;
    let mut seen: HashSet<String> = own.into_iter().collect();
    let mut found = Vec::new();
    while let Some(id) = pending.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        let Some(path) = index.get(&id) else { continue };
        pending.extend(mutation_schema_document_references(path)?.1);
        found.push(path.clone());
    }
    found.sort();
    Ok(found)
}

/// 🧭️ The tree a leaf's referenced documents are looked up in: its plugin (the directory under `🔌️plugins`, which bundles every
/// artifact a leaf may reference across), else its framework module (the nearest directory under `🔨️modules`), else the module
/// owning its `🧬️schema/🧬️mutations` root.
fn mutation_schema_search_root(payload_schema: &Path) -> PathBuf {
    let under = |parent: &str| payload_schema.ancestors().find(|ancestor| ancestor.parent().and_then(Path::file_name).is_some_and(|name| name == parent));
    let owner = payload_schema.ancestors().find(|ancestor| ancestor.file_name().is_some_and(|name| name == "🧬️schema") && ancestor.join("🧬️mutations").is_dir()).and_then(Path::parent);
    under("🔌️plugins").or_else(|| under("🔨️modules")).or(owner).or_else(|| payload_schema.parent()).unwrap_or(payload_schema).to_path_buf()
}

/// 🧫️ Recognizes example collections while preserving domain modules whose names match collection roles.
fn mutation_schema_example_collection(path: &Path) -> bool {
    path.ancestors().any(|directory| directory.file_name().is_some_and(|name| ["🧫️fixtures", "🧪️fixtures", "🧪️tests"].iter().any(|collection| name == *collection)) && !directory.parent().and_then(Path::file_name).is_some_and(|name| name == "🔨️modules"))
}

/// 🗂️ Indexes production `$id` documents within schema scopes, excluding example collections before any directory read.
fn mutation_schema_document_index(root: &Path) -> std::rc::Rc<BTreeMap<String, PathBuf>> {
    if mutation_schema_example_collection(root) {
        return std::rc::Rc::new(BTreeMap::new());
    }
    thread_local! {
        static INDEX: std::cell::RefCell<BTreeMap<PathBuf, std::rc::Rc<BTreeMap<String, PathBuf>>>> = const { std::cell::RefCell::new(BTreeMap::new()) };
    }
    fn walk(directory: &Path, schema: bool, index: &mut BTreeMap<String, PathBuf>) {
        let Ok(entries) = crate::compiler_resources::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let (path, name) = (entry.path(), entry.file_name().to_string_lossy().into_owned());
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_dir() && !name.starts_with('.') && !mutation_schema_example_collection(&path) && !["target", "node_modules", "dist", "🗑️generated", "📦️packages"].contains(&name.as_str()) {
                walk(&path, schema || name == "🧬️schema", index);
            } else if schema && kind.is_file() && name.ends_with(".json") {
                let id = crate::compiler_resources::read(&path).ok().and_then(|raw| serde_json::from_slice::<serde_json::Value>(&raw).ok()).and_then(|value| value.get("$id").and_then(serde_json::Value::as_str).map(str::to_string));
                if let Some(id) = id {
                    index.entry(id).or_insert(path);
                }
            }
        }
    }
    if let Some(index) = INDEX.with(|cache| cache.borrow().get(root).cloned()) {
        return index;
    }
    let mut index = BTreeMap::new();
    walk(root, root.components().any(|component| component.as_os_str() == "🧬️schema"), &mut index);
    let index = std::rc::Rc::new(index);
    INDEX.with(|cache| cache.borrow_mut().insert(root.to_path_buf(), index.clone()));
    index
}

/// 🔗️ A schema document's own `$id` and every absolute document id its `$ref`s name (fragments stripped).
fn mutation_schema_document_references(path: &Path) -> Result<(Option<String>, Vec<String>), String> {
    fn collect(value: &serde_json::Value, into: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(entries) => {
                for (key, value) in entries {
                    match (key.as_str(), value) {
                        ("$ref", serde_json::Value::String(reference)) if !reference.starts_with('#') => into.push(reference.split('#').next().unwrap_or_default().to_string()),
                        _ => collect(value, into),
                    }
                }
            }
            serde_json::Value::Array(items) => items.iter().for_each(|item| collect(item, into)),
            _ => {}
        }
    }
    let raw = crate::compiler_resources::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let document: serde_json::Value = serde_json::from_slice(&raw).map_err(|error| format!("{}: {error}", path.display()))?;
    let mut references = Vec::new();
    collect(&document, &mut references);
    references.retain(|reference| !reference.is_empty());
    Ok((document.get("$id").and_then(serde_json::Value::as_str).map(str::to_string), references))
}
//#endregion 🪪️MutationLeaf

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-leaf-derive/🦀️.rs"]
mod mutation_leaf_derive_tests;

/// ✉️ Owns artifact envelope metadata independently from canonical Record fields.
pub fn expand_dsl_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if !matches!(input.data, Data::Struct(_)) {
        return syn::Error::new_spanned(&input, "DslArtifact only supports structs").to_compile_error().into();
    }
    let mut id: Option<String> = None;
    let mut extension: Option<String> = None;
    for attribute in &input.attrs {
        if !attribute.path().is_ident("artifact") { continue; }
        if let Err(error) = attribute.parse_nested_meta(|meta| {
            let target = if meta.path.is_ident("id") { &mut id } else if meta.path.is_ident("extension") { &mut extension } else { return Err(meta.error("unsupported artifact attribute")); };
            if target.is_some() { return Err(meta.error("duplicate artifact attribute")); }
            let value: syn::LitStr = meta.value()?.parse()?;
            *target = Some(value.value());
            Ok(())
        }) { return error.to_compile_error().into(); }
    }
    let envelope_id = match id.or_else(|| extension.clone()) {
        Some(id) => id,
        None => return syn::Error::new_spanned(&input, "DslArtifact requires #[artifact(id = \"plugin.artifact\")] or #[artifact(extension = \"...\")]").to_compile_error().into(),
    };
    let suffix = extension.as_deref().unwrap_or_else(|| envelope_id.rsplit('.').next().unwrap_or(&envelope_id));
    let name = &input.ident;
    quote! {
        impl #name {
            pub const __DSL_ENVELOPE_ID: &'static str = #envelope_id;
            pub const __DSL_EXTENSION: &'static str = #suffix;
        }
    }.into()
}

/// 📝️ Generates diff text I/O at its representation owner.
pub fn expand_diff_text(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as syn::Type);
    diff_text_tokens(&name).into()
}

fn diff_text_tokens(name: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        impl ::semio_framework_os_kernel::DiffText for #name {
            fn print_diff(&self) -> String {
                ::semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), ::semio_framework_dsl_record::JoinMode::Inline)
            }
            fn parse_diff(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
                let record = ::semio_framework_dsl_record::parse_exact(line, &Self::__dsl_spec(), &::semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: ::semio_framework_dsl_record::SourceMode::Inline })?;
                Self::__dsl_from_record(&record)
            }
        }
    }
}

/// 💾️ Generates diff binary I/O at its representation owner.
pub fn expand_diff_binary(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as syn::Type);
    diff_binary_tokens(&name).into()
}

fn diff_binary_tokens(name: &syn::Type) -> proc_macro2::TokenStream {
    quote! {
        impl ::semio_framework_os_kernel::DiffBinary for #name {
            fn encode_diff(&self) -> Result<Vec<u8>, ::semio_framework_os_kernel::ProtocolError> {
                ::semio_framework_os_kernel::os_store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), &::semio_framework_os_kernel::os_store::PackEncodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)
            }
            fn decode_diff(bytes: &[u8]) -> Result<Self, ::semio_framework_os_kernel::ProtocolError> {
                let (record, _report) = ::semio_framework_os_kernel::os_store::pack_rt::decode_document(bytes, &Self::__dsl_spec(), &::semio_framework_os_kernel::os_store::PackDecodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)?;
                Self::__dsl_from_record(&record).map_err(|error| ::semio_framework_os_kernel::ProtocolError::Malformed { what: "diff record", offset: 0, detail: error.to_string() })
            }
        }
    }
}

//#region 🔖️Mutations
/// 🗣️ `#[mutations(snapshot = ..., diff = ..., schema = "..." [, retire_cold = path])]` container
/// attrs for `#[derive(Mutations)]` — see that macro's doc. `retire_cold` names a `fn(Self)` that
/// disposes an operation owning fail-closed roots; the generated `Mutation::retire_cold` calls it.
#[derive(Default)]
struct MutationsAttrs {
    snapshot: Option<Type>,
    diff: Option<Type>,
    schema: Option<String>,
    retire_cold: Option<syn::Path>,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_mutations_attrs(input: &DeriveInput) -> syn::Result<MutationsAttrs> {
    let mut out = MutationsAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("mutations") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("snapshot") {
                if out.snapshot.is_some() { return Err(meta.error("duplicate mutations snapshot")); }
                out.snapshot = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("diff") {
                if out.diff.is_some() { return Err(meta.error("duplicate mutations diff")); }
                out.diff = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("schema") {
                if out.schema.is_some() { return Err(meta.error("duplicate mutations schema")); }
                let value: syn::LitStr = meta.value()?.parse()?;
                out.schema = Some(value.value());
            } else if meta.path.is_ident("retire_cold") {
                if out.retire_cold.is_some() { return Err(meta.error("duplicate mutations retire_cold")); }
                out.retire_cold = Some(meta.value()?.parse()?);
            } else { return Err(meta.error("unsupported mutations attribute")); }
            Ok(())
        })?;
    }
    Ok(out)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-attrs/🦀️.rs"]
mod mutation_attrs_tests;

pub fn expand_derive_mutations(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let source = match input.ident.span().unwrap().local_file() {
        Some(source) => source,
        None => return syn::Error::new_spanned(&input, "Mutations requires a local source file").to_compile_error().into(),
    };
    let compiler_cwd = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into(),
    };
    let authority = match mutation_aggregate_source_authority(&source, &compiler_cwd) {
        Ok(authority) => authority,
        Err(error) => return syn::Error::new_spanned(&input, format!("Mutations source authority failed: {error}")).to_compile_error().into(),
    };
    match expand_mutations(&input, &authority) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_mutations(input: &DeriveInput, authority: &MutationAggregateSourceAuthority) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "#[derive(Mutations)] only supports enums"));
    };
    if data.variants.is_empty() {
        return Err(syn::Error::new_spanned(input, "Mutations requires at least one concrete leaf"));
    }
    if input.attrs.iter().any(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr")) {
        return Err(syn::Error::new_spanned(input, "Mutations does not permit conditional aggregate metadata"));
    }
    let attrs = parse_mutations_attrs(input)?;
    let retire_cold = attrs.retire_cold.map(|retire| quote! {
        fn retire_cold(self) {
            #retire(self)
        }
    });
    let (Some(snapshot_ty), Some(diff_ty), Some(schema)) = (attrs.snapshot, attrs.diff, attrs.schema) else {
        return Err(syn::Error::new_spanned(input, "#[derive(Mutations)] requires #[mutations(snapshot = YourSnapshot, diff = YourDiff, schema = \"your.doc.schema\")]"));
    };
    let map_error = |error: String| syn::Error::new_spanned(input, error);
    let workspace_token = mutation_authority_workspace_token(&authority.workspace_root, &authority.taxonomy_path).map_err(map_error)?;
    let mutation_root = mutation_authority_relative(&authority.workspace_root, &authority.mutation_root).map_err(map_error)?;
    let taxonomy_path = mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path).map_err(map_error)?;
    let dependency_paths = [authority.taxonomy_path.clone()];
    let dependencies = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect::<Result<Vec<_>, _>>().map_err(map_error)?;
    let source_filename = &authority.source_filename;
    let descriptor_filename = &authority.descriptor_filename;
    let mutation_payload_facet = &authority.mutation_payload_facet;
    let owner_layout = if let Some(operations) = &authority.domain_operations {
        let entries = operations.iter().map(|(owner, identity)| quote! { ::semio_framework_os_kernel::MutationDomainOperation { owner: #owner, semantic_kind: #identity } });
        quote! { ::semio_framework_os_kernel::MutationOwnerLayout::DomainOperations(&[#(#entries),*]) }
    } else {
        quote! { ::semio_framework_os_kernel::MutationOwnerLayout::Flat }
    };
    let components = if authority.component_roots.is_empty() { vec![(mutation_root.clone(), authority.domain_operations.clone())] } else { authority.component_roots.clone() };
    let mut component_completeness = Vec::new();
    let scopes = components.iter().map(|(root, operations)| {
        let layout = if let Some(operations) = operations {
            let entries = operations.iter().map(|(owner, identity)| quote! { ::semio_framework_os_kernel::MutationDomainOperation { owner: #owner, semantic_kind: #identity } });
            for (owner, _) in operations {
                component_completeness.push(quote! {
                    let mut found = false;
                    let mut index = 0;
                    while index < descriptors.len() {
                        if ::semio_framework_os_kernel::str_eq(descriptors[index].owner, #owner) { found = true; }
                        index += 1;
                    }
                    assert!(found, "Mutations requires every explicitly registered component domain operation");
                });
            }
            quote! { ::semio_framework_os_kernel::MutationOwnerLayout::DomainOperations(&[#(#entries),*]) }
        } else { quote! { ::semio_framework_os_kernel::MutationOwnerLayout::Flat } };
        quote! {
            match (::semio_framework_os_kernel::MutationLeafSourceScope {
                workspace_token: [#(#workspace_token),*], mutation_root: #root, owner_layout: #layout,
                taxonomy_path: #taxonomy_path, mutation_payload_facet: #mutation_payload_facet,
                source_filename: #source_filename, descriptor_filename: #descriptor_filename,
            }).validate() {
                Ok(scope) => scope,
                Err(_) => panic!("Mutations requires a valid aggregate component source scope"),
            }
        }
    }).collect::<Vec<_>>();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let aggregate_ty = quote! { #name #ty_generics };
    let mut diff_arms = Vec::new();
    let mut inverse_arms = Vec::new();
    let mut timestamp_arms = Vec::new();
    let mut descriptor_arms = Vec::new();
    let mut semantics_arms = Vec::new();
    let mut label_arms = Vec::new();
    let mut target_arms = Vec::new();
    let mut may_emit_foreign_steps_arms = Vec::new();
    let mut foreign_steps_arms = Vec::new();
    let mut input_schema_arms = Vec::new();
    let mut inverse_rows_arms = Vec::new();
    let mut payload_value_arms = Vec::new();
    let mut with_payload_value_arms = Vec::new();
    let mut from_payload_value_arms = Vec::new();
    let mut input_schemas = Vec::new();
    let mut input_schema_documents = Vec::new();
    let mut kind_consts = Vec::new();
    let mut leaf_descriptors = Vec::new();
    let mut leaf_checks = Vec::new();
    let mut conversions = Vec::new();
    let mut register_calls = Vec::new();

    for (index, variant) in data.variants.iter().enumerate() {
        let variant_ident = &variant.ident;
        let Fields::Unnamed(unnamed) = &variant.fields else {
            return Err(syn::Error::new_spanned(variant, "Mutations requires every variant to wrap exactly one direct MutationKind payload"));
        };
        if unnamed.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(variant, "Mutations requires every variant to wrap exactly one direct MutationKind payload"));
        }
        if variant.attrs.iter().chain(unnamed.unnamed[0].attrs.iter()).any(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr")) {
            return Err(syn::Error::new_spanned(variant, "Mutations does not permit conditional leaf metadata"));
        }
        let payload_ty = &unnamed.unnamed[0].ty;
        if !matches!(payload_ty, Type::Path(path) if path.qself.is_none()) {
            return Err(syn::Error::new_spanned(payload_ty, "Mutations requires a direct leaf type path"));
        }
        let expected_variant = variant_ident.to_string();
        let expected_kebab = to_kebab(&expected_variant);
        let kind = quote! { <#payload_ty as ::semio_framework_os_kernel::MutationKind<#snapshot_ty, #aggregate_ty>> };
        let leaf = quote! { <#payload_ty as ::semio_framework_os_kernel::MutationLeaf> };
        diff_arms.push(quote! { Self::#variant_ident(payload) => #kind::diff(payload, base) });
        inverse_arms.push(quote! { Self::#variant_ident(payload) => #kind::inverse(payload, base) });
        timestamp_arms.push(quote! { Self::#variant_ident(payload) => #kind::timestamp(payload) });
        descriptor_arms.push(quote! { Self::#variant_ident(_) => &<Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS[#index] });
        semantics_arms.push(quote! { Self::#variant_ident(_) => &#kind::SEMANTICS });
        label_arms.push(quote! { Self::#variant_ident(payload) => #kind::label(payload) });
        target_arms.push(quote! { Self::#variant_ident(payload) => #kind::target(payload) });
        may_emit_foreign_steps_arms.push(quote! { Self::#variant_ident(payload) => #kind::may_emit_foreign_steps(payload) });
        foreign_steps_arms.push(quote! { Self::#variant_ident(payload) => #kind::foreign_steps(payload, base) });
        input_schema_arms.push(quote! { Self::#variant_ident(payload) => #leaf::input_schema(payload) });
        inverse_rows_arms.push(quote! { Self::#variant_ident(payload) => #leaf::inverse_rows(payload) });
        payload_value_arms.push(quote! { Self::#variant_ident(payload) => #leaf::input_value(payload) });
        with_payload_value_arms.push(quote! { Self::#variant_ident(payload) => #leaf::with_input_value(payload, value).map(Self::#variant_ident) });
        from_payload_value_arms.push(quote! { if kind == #kind::SEMANTICS.kind { return #leaf::from_input_value(value).map(Self::#variant_ident); } });
        input_schemas.push(quote! { #leaf::PAYLOAD_SCHEMA });
        input_schema_documents.push(quote! { #leaf::PAYLOAD_SCHEMA_DOCUMENTS });
        kind_consts.push(quote! { #kind::SEMANTICS });
        leaf_descriptors.push(quote! { #leaf::DESCRIPTOR });
        leaf_checks.push(quote! { const {
            assert!(::semio_framework_os_kernel::str_eq(#kind::SEMANTICS.kind, #expected_kebab), "Mutations semantic kind must match its variant");
            assert!(::semio_framework_os_kernel::is_approved_verb(#kind::SEMANTICS.verb), "Mutations requires an approved semantic verb");
            assert!(::semio_framework_os_kernel::str_eq(#leaf::DESCRIPTOR.aggregate_variant, #expected_variant), "Mutations descriptor variant must match its wrapped leaf");
            assert!(::semio_framework_os_kernel::str_eq(#leaf::DESCRIPTOR.semantic_kind, #kind::SEMANTICS.kind), "Mutations descriptor and semantic kind must agree");
            let mut scope_index = 0;
            let mut matched = false;
            while scope_index < SCOPES.len() {
                if let Ok(()) = SCOPES[scope_index].validate_leaf(&#leaf::DESCRIPTOR, &#leaf::PROVENANCE) { matched = true; break; }
                scope_index += 1;
            }
            assert!(matched, "Mutations leaf source must match its aggregate workspace and declared component owner");
        }; });
        conversions.push(quote! {
            impl #impl_generics ::core::convert::From<#payload_ty> for #aggregate_ty #where_clause {
                fn from(payload: #payload_ty) -> Self {
                    let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                    Self::#variant_ident(payload)
                }
            }
        });
        register_calls.push(quote! {
            ::semio_framework_os_kernel::MutationDescriptor::new(
                ::semio_framework_os_kernel::SchemaId(format!("{}#{}", #schema, #kind::SEMANTICS.kind)),
                ::semio_framework_os_kernel::SchemaVersion(1),
                state_class,
                #leaf::DESCRIPTOR,
                #kind::SEMANTICS,
            )?
        });
    }

    let register_fn_ident = syn::Ident::new(&format!("register_{}_descriptors", to_kebab(&name.to_string()).replace('-', "_")), name.span());
    let payload_law = mutation_payload_law_test(name, &snapshot_ty, authority, &input.generics);
    let eager_check = input.generics.params.is_empty().then(|| quote! {
        const _: () = { let _ = <#name as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS; };
    });
    Ok(quote! {
        #(const _: &str = ::core::include_str!(#dependencies);)*
        #eager_check
        #(#conversions)*

        impl #impl_generics ::semio_framework_os_kernel::Mutation<#snapshot_ty> for #aggregate_ty #where_clause {
            type Diff = #diff_ty;
            const DESCRIPTORS: &'static [::semio_framework_os_kernel::MutationLeafDescriptor] = {
                const SCOPES: &[::semio_framework_os_kernel::ValidatedMutationLeafSourceScope] = &[#(#scopes),*];
                #(#leaf_checks)*
                let descriptors: &'static [::semio_framework_os_kernel::MutationLeafDescriptor] = &[#(#leaf_descriptors),*];
                #(#component_completeness)*
                match ::semio_framework_os_kernel::validate_mutation_leaf_descriptor_roster_uniqueness(#mutation_root, descriptors, #owner_layout) {
                    Ok(()) => descriptors,
                    Err(_) => panic!("Mutations requires a unique and complete direct leaf descriptor roster"),
                }
            };
            fn descriptor(&self) -> &'static ::semio_framework_os_kernel::MutationLeafDescriptor {
                match self { #(#descriptor_arms),* }
            }
            fn diff(&self, base: &#snapshot_ty) -> ::semio_framework_os_kernel::MutationOutcome<Self::Diff> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#diff_arms),* }
            }
            fn inverse(&self, base: &#snapshot_ty) -> Result<Vec<Self>, ::semio_framework_value::ValueError> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#inverse_arms),* }
            }
            fn timestamp(&self) -> Option<::semio_framework_os_kernel::HybridLogicalTimestamp> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#timestamp_arms),* }
            }
            fn conflict_target(&self) -> Vec<String> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#target_arms),* }
            }
            fn may_emit_foreign_steps(&self) -> bool {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#may_emit_foreign_steps_arms),* }
            }
            fn foreign_steps(&self, base: &#snapshot_ty) -> Vec<::semio_framework_os_kernel::ForeignStep> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#foreign_steps_arms),* }
            }
            fn inverse_rows(&self) -> usize {
                match self { #(#inverse_rows_arms),* }
            }
            const INPUT_SCHEMAS: &'static [&'static str] = &[#(#input_schemas),*];
            const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = &[#(#input_schema_documents),*];
            fn input_schema(&self) -> ::core::option::Option<&'static str> {
                match self { #(#input_schema_arms),* }
            }
            fn payload_value(&self) -> ::semio_framework_value::DslValue {
                match self { #(#payload_value_arms),* }
            }
            fn with_payload_value(&self, value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
                match self { #(#with_payload_value_arms),* }
            }
            fn from_payload_value(kind: &str, value: ::semio_framework_value::DslValue) -> ::core::result::Result<Self, ::semio_framework_value::ValueError> {
                #(#from_payload_value_arms)*
                ::core::result::Result::Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,::std::format!("{kind} is no leaf kind of {}", ::core::stringify!(#name))))
            }
            #retire_cold
        }

        impl #impl_generics ::semio_framework_os_kernel::SemanticMutation<#snapshot_ty> for #aggregate_ty #where_clause {
            fn kinds() -> &'static [::semio_framework_os_kernel::SemanticDescriptor] {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                &[#(#kind_consts),*]
            }
            fn semantics(&self) -> &'static ::semio_framework_os_kernel::SemanticDescriptor {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#semantics_arms),* }
            }
            fn label(&self) -> ::semio_framework_ui_locale::LocalizedLabel {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#label_arms),* }
            }
            fn target(&self) -> Vec<String> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#target_arms),* }
            }
        }

        /// 🪪️ Registers the validated leaf roster during owner startup.
        pub fn #register_fn_ident #impl_generics (
            state_class: ::semio_framework_schema_state::StateClass,
        ) -> ::core::result::Result<(), ::semio_framework_os_kernel::MutationDescriptorError> #where_clause {
            let _ = <#aggregate_ty as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
            let descriptors = [#(#register_calls),*];
            ::semio_framework_os_kernel::register_mutation_descriptors(descriptors)
        }

        #payload_law
    })
}

/// ⚖️ The `#[cfg(test)]` editable-payload law `#[derive(Mutations)]` emits for a non-generic aggregate: every leaf publishes one
/// payload schema (`::semio_framework_os_kernel::mutation_input_schema_failures`), and every committed fixture under the
/// aggregate's owner directory (the one holding its `🧬️schema`) that decodes as the aggregate and, when the aggregate's own file
/// defines a top-level `demo_mutation_cases() -> Vec<Aggregate>`, every demo case is labelled in every locale
/// (`mutation_label_failures`) and rebuilds itself from its own editable payload or, when inert, refuses to
/// (`mutation_payload_round_trip_failures`), every operation answers its schema-declared inverse rows and every `perTarget` leaf
/// is admitted at the one-item ceiling and refused past it as `mutation.too-large` (`mutation_inverse_rows_declaration_failures`),
/// and every fixture case's inverse fits the leaf's declared rows
/// (`mutation_inverse_rows_failures`, design §20.5) — walked over every subset of the owner's standard, since a subset's
/// fixtures replay through the aggregate too. Nothing is emitted when the owner directory cannot be addressed from the
/// compiling crate.
fn mutation_payload_law_test(name: &syn::Ident, snapshot_ty: &syn::Type, authority: &MutationAggregateSourceAuthority, generics: &syn::Generics) -> proc_macro2::TokenStream {
    if !generics.params.is_empty() {
        return quote! {};
    }
    let parent = authority.mutation_root.parent();
    let owner = match parent {
        Some(schema) if schema.file_name().and_then(|segment| segment.to_str()) == Some("🧬️schema") => schema.parent(),
        other => other,
    };
    let (Some(owner), Ok(manifest)) = (owner, std::env::var("CARGO_MANIFEST_DIR")) else { return quote! {} };
    let Some(relative) = mutation_relative_path(Path::new(&manifest), owner) else { return quote! {} };
    let subsets = owner.parent().filter(|subsets| subsets.file_name().and_then(|segment| segment.to_str()) == Some("🪆️subsets")).unwrap_or(owner);
    let Some(footprint_relative) = mutation_relative_path(Path::new(&manifest), subsets) else { return quote! {} };
    let source = crate::compiler_resources::read_to_string(authority.mutation_root.join(&authority.source_filename)).unwrap_or_default();
    let signature = format!("demo_mutation_cases() -> Vec<{name}>");
    let demo_cases = source.lines().any(|line| ["fn ", "pub fn ", "pub(crate) fn "].iter().any(|prefix| line.strip_prefix(prefix).is_some_and(|rest| rest.starts_with(&signature)))).then(|| quote! { ops.extend(demo_mutation_cases()); });
    let test_ident = syn::Ident::new(&format!("semio_payload_law_{}", to_kebab(&name.to_string()).replace('-', "_")), name.span());
    quote! {
        #[cfg(test)]
        #[test]
        fn #test_ident() {
            let root = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join(#relative);
            let (mut ops, _) = ::semio_framework_os_kernel::mutation_fixture_ops::<#name>(&root);
            #demo_cases
            let count = ops.len();
            let mut failures = ::semio_framework_os_kernel::mutation_input_schema_failures::<#snapshot_ty, #name>();
            failures.extend(::semio_framework_os_kernel::mutation_label_failures::<#snapshot_ty, #name>(&ops));
            failures.extend(::semio_framework_os_kernel::mutation_inverse_rows_declaration_failures::<#snapshot_ty, #name>(&ops));
            failures.extend(::semio_framework_os_kernel::mutation_payload_round_trip_failures::<#snapshot_ty, #name>(ops));
            let (footprint_failures, cases) = ::semio_framework_os_kernel::os_spr::fold::mutation_inverse_rows_failures::<#snapshot_ty, #name>(&::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join(#footprint_relative));
            failures.extend(footprint_failures);
            assert!(failures.is_empty(), "{} breaches of the editable-payload and fold-footprint laws over {} {} operations and {} fixture cases: {:#?}", failures.len(), count, ::core::stringify!(#name), cases, failures);
        }
    }
}

/// 🧭️ `to` relative to `from` (both canonical), in `/` segments — `None` when they share no root or a segment is not UTF-8.
fn mutation_relative_path(from: &Path, to: &Path) -> Option<String> {
    let (from, to) = (mutation_authority_canonical(from).ok()?, mutation_authority_canonical(to).ok()?);
    let (from, to): (Vec<_>, Vec<_>) = (from.components().collect(), to.components().collect());
    let shared = from.iter().zip(&to).take_while(|(left, right)| left == right).count();
    if shared == 0 {
        return None;
    }
    let mut segments: Vec<String> = std::iter::repeat_n("..".to_string(), from.len() - shared).collect();
    for component in &to[shared..] {
        segments.push(component.as_os_str().to_str()?.to_string());
    }
    Some(segments.join("/"))
}
//#endregion 🔖️Mutations

//#region 🧪️MandatoryMutations
#[cfg(test)]
#[path = "🧪️tests/🔬️mandatory-mutations/🦀️.rs"]
mod mandatory_mutations_tests;
//#endregion 🧪️MandatoryMutations

//#region 🔖️CompositeMutation
/// 🌉️ `#[composite(snapshot = ..., op = ...)]` container attrs for
/// `#[derive(CompositeMutation)]` — see that macro's doc.
#[derive(Default)]
struct CompositeAttrs {
    snapshot: Option<Type>,
    op: Option<Type>,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_composite_attrs(input: &DeriveInput) -> syn::Result<CompositeAttrs> {
    let mut out = CompositeAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("composite") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("snapshot") {
                if out.snapshot.is_some() { return Err(meta.error("duplicate composite snapshot")); }
                out.snapshot = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("op") {
                if out.op.is_some() { return Err(meta.error("duplicate composite op")); }
                out.op = Some(meta.value()?.parse()?);
            } else { return Err(meta.error("unsupported composite attribute")); }
            Ok(())
        })?;
    }
    Ok(out)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️composite-attrs/🦀️.rs"]
mod composite_attrs_tests;

pub fn expand_derive_composite_mutation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_composite_mutation(&input) {
        Ok(expanded) => expanded.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_composite_mutation(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = input.ident.clone();
    let attrs = parse_composite_attrs(input)?;
    let (Some(snapshot_ty), Some(op_ty)) = (attrs.snapshot, attrs.op) else {
        return Err(syn::Error::new_spanned(input, "#[derive(CompositeMutation)] requires #[composite(snapshot = YourSnapshot, op = YourOp)]"));
    };

    let expected_kebab = to_kebab(&name.to_string());
    let assert_kind_message = format!("#[derive(CompositeMutation)]: {}'s CompositeMutationKind::SEMANTICS.kind must equal \"{}\" (its own kebab form)", name, expected_kebab);
    let assert_verb_message = format!("#[derive(CompositeMutation)]: {}'s CompositeMutationKind::SEMANTICS.verb must be one of protocol::APPROVED_VERBS", name);

    let expanded = quote! {
        const _: () = assert!(::semio_framework_os_kernel::str_eq(<#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS.kind, #expected_kebab), #assert_kind_message);
        const _: () = assert!(::semio_framework_os_kernel::is_approved_verb(<#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS.verb), #assert_verb_message);

        impl ::semio_framework_os_kernel::MutationKind<#snapshot_ty, #op_ty> for #name {
            const SEMANTICS: ::semio_framework_os_kernel::SemanticDescriptor = <#name as ::semio_framework_os_kernel::CompositeMutationKind<#snapshot_ty, #op_ty>>::SEMANTICS;
            fn diff(&self, base: &#snapshot_ty) -> ::semio_framework_os_kernel::MutationOutcome<<#op_ty as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::Diff> {
                ::semio_framework_os_kernel::os_spr::fold::fold_plan_diff(self, base)
            }
            fn inverse(&self, base: &#snapshot_ty) -> Result<Vec<#op_ty>, ::semio_framework_value::ValueError> {
                ::semio_framework_os_kernel::os_spr::fold::fold_plan_inverse(self, base)
            }
            fn label(&self) -> ::semio_framework_ui_locale::LocalizedLabel {
                ::semio_framework_os_kernel::CompositeMutationKind::label(self)
            }
            fn timestamp(&self) -> Option<::semio_framework_os_kernel::HybridLogicalTimestamp> {
                ::semio_framework_os_kernel::CompositeMutationKind::timestamp(self)
            }
            fn target(&self) -> Vec<String> {
                ::semio_framework_os_kernel::CompositeMutationKind::target(self)
            }
            fn may_emit_foreign_steps(&self) -> bool {
                true
            }
            fn foreign_steps(&self, base: &#snapshot_ty) -> Vec<::semio_framework_os_kernel::ForeignStep> {
                ::semio_framework_os_kernel::plan_foreign_steps(self, base)
            }
        }
    };
    Ok(expanded)
}
//#endregion 🔖️CompositeMutation

//#region 🧪️CompositeTimestamp
#[cfg(test)]
#[path = "🧪️tests/🔬️composite-timestamp/🦀️.rs"]
mod composite_timestamp_tests;
//#endregion 🧪️CompositeTimestamp

//#region 🔖️VariantHelpers
/// 🔡️ Converts a Rust identifier (`PascalCase`/`camelCase`/`snake_case`, any mix) into
/// lowercase `kebab-case` — the unified syntax law's key/keyword/tag convention. Falls back to
/// this whenever no explicit `#[dsl(key = "...")]` override is given, for variant keywords,
/// record field keys, and `DslScalar` variant tags alike, so `SetCamera` -> `set-camera`,
/// `airtightness_n50` -> `airtightness-n50`, `HTTPServer` -> `http-server`.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn to_kebab(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == '-' {
            if !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            continue;
        }
        if c.is_uppercase() {
            let prev = if i == 0 { None } else { chars.get(i - 1).copied() };
            let next = chars.get(i + 1).copied();
            // A new word starts at an uppercase letter that follows a lowercase/digit
            // (`SetCamera` -> boundary before `C`) OR that follows another uppercase letter but
            // is itself followed by a lowercase one (`HTTPServer` -> boundary before the `S` that
            // starts "Server", not between every letter of the "HTTP" acronym).
            let boundary = match prev {
                Some(p) if p.is_lowercase() || p.is_ascii_digit() => true,
                Some(p) if p.is_uppercase() => next.is_some_and(|n| n.is_lowercase()),
                _ => false,
            };
            if boundary && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

//#endregion 🔖️VariantHelpers

#[cfg(test)]
#[path = "🧪️tests/🪆️record-owner/🦀️.rs"]
mod canonical_product_macro_tests;

#[cfg(test)]
#[path = "🧪️tests/🚪️diff-codecs/🦀️.rs"]
mod diff_codec_tests;
