//! 🧬️ `dsl_derive` — compiles `#[dsl(...)]`-annotated struct/enum declarations into
//! `dsl::DslField`/`dsl::DslVariants` bindings (nested usage composes through), so a technology
//! declares its grammar instead of hand-writing a parser/printer. Analyze → IR → emit.
//!
//! P6: `DslArtifact`/`DslOps` no longer emit `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` —
//! those traits are handcrafted per artifact. `DslRecord` stays for field helpers only.
//!
//! Whole crate is sync (E3): a proc-macro entry point's signature is language-fixed to
//! `fn(TokenStream) -> TokenStream` and rustc rejects an `async fn` here outright (a proc macro
//! runs inside rustc at compile time, where there is no executor to poll it). Bounded, no-follow
//! source-authority reads occur during expansion; every helper remains sync because there is no
//! executor to poll.

use proc_macro::TokenStream;
use quote::quote;
use std::{collections::HashSet, fs, path::{Component, Path, PathBuf}};
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

#[cfg(test)]
#[path = "../../🧪️tests/📤️macro-exports/🦀️component.rs"]
mod macro_export_tests;

//#region 🔖️MutationSourceAuthority
#[derive(Debug)]
struct MutationSourceAuthority {
    workspace_root: PathBuf,
    mutation_root: PathBuf,
    owner: String,
    source_path: PathBuf,
    descriptor_path: PathBuf,
    taxonomy_path: PathBuf,
}

#[derive(Debug)]
struct MutationAuthorityCommon {
    workspace_root: PathBuf,
    source_path: PathBuf,
    taxonomy_path: PathBuf,
    mutation_collection: String,
    source_filename: String,
    descriptor_filename: String,
}

#[derive(Debug)]
struct MutationAggregateSourceAuthority {
    workspace_root: PathBuf,
    mutation_root: PathBuf,
    source_path: PathBuf,
    taxonomy_path: PathBuf,
    source_filename: String,
    descriptor_filename: String,
}

fn mutation_authority_common(source: &Path, compiler_cwd: &Path) -> Result<MutationAuthorityCommon, String> {
    let source_path = mutation_authority_normalize(source, compiler_cwd)?;
    let workspace_root = mutation_authority_workspace_root(&source_path)?;
    mutation_authority_no_follow(&workspace_root, &source_path, false)?;
    let project_path = workspace_root.join("📋️project.json");
    mutation_authority_no_follow(&workspace_root, &project_path, false)?;
    let project: serde_json::Value = serde_json::from_slice(&fs::read(&project_path).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    let locator = project.pointer("/metadata/semio/taxonomy").and_then(serde_json::Value::as_str).ok_or_else(|| "missing metadata.semio.taxonomy".to_string())?;
    let taxonomy_path = mutation_authority_locator(&workspace_root, locator)?;
    mutation_authority_no_follow(&workspace_root, &taxonomy_path, false)?;
    let taxonomy: serde_json::Value = serde_json::from_slice(&fs::read(&taxonomy_path).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
    let source_filename = mutation_authority_filename(&taxonomy, taxonomy.get("mutationComponentFileKindId").and_then(serde_json::Value::as_str).ok_or_else(|| "missing mutationComponentFileKindId".to_string())?)?;
    let descriptor_filename = mutation_authority_filename(&taxonomy, taxonomy.get("mutationDescriptorFileKindId").and_then(serde_json::Value::as_str).ok_or_else(|| "missing mutationDescriptorFileKindId".to_string())?)?;
    let mutation_collection = mutation_authority_collection(&taxonomy)?;
    Ok(MutationAuthorityCommon { workspace_root, source_path, taxonomy_path, mutation_collection, source_filename, descriptor_filename })
}

fn mutation_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationSourceAuthority, String> {
    let common = mutation_authority_common(source, compiler_cwd)?;
    let MutationAuthorityCommon { workspace_root, source_path, taxonomy_path, mutation_collection, source_filename, descriptor_filename } = common;
    if source_path.file_name().and_then(|name| name.to_str()) != Some(source_filename.as_str()) { return Err("source is not the taxonomy canonical mutation primary".to_string()); }
    let owner_path = source_path.parent().ok_or_else(|| "source has no owner directory".to_string())?;
    let mutation_root = owner_path.parent().ok_or_else(|| "source owner has no collection parent".to_string())?;
    if mutation_root.file_name().and_then(|name| name.to_str()) != Some(mutation_collection.as_str()) { return Err("source owner is not a direct mutation collection leaf".to_string()); }
    let owner = mutation_authority_relative(&workspace_root, owner_path)?;
    let descriptor_path = owner_path.join(descriptor_filename);
    mutation_authority_no_follow(&workspace_root, &descriptor_path, false)?;
    let descriptor = fs::read(&descriptor_path).map_err(|error| error.to_string())?;
    let authority = MutationSourceAuthority { workspace_root, mutation_root: mutation_root.to_path_buf(), owner, source_path, descriptor_path, taxonomy_path };
    parse_mutation_leaf_descriptor(&descriptor, &authority)?;
    Ok(authority)
}

fn mutation_aggregate_source_authority(source: &Path, compiler_cwd: &Path) -> Result<MutationAggregateSourceAuthority, String> {
    let common = mutation_authority_common(source, compiler_cwd)?;
    if common.source_path.file_name().and_then(|name| name.to_str()) != Some(common.source_filename.as_str()) { return Err("aggregate source is not the taxonomy canonical mutation primary".to_string()); }
    let mutation_root = common.source_path.parent().ok_or_else(|| "aggregate source has no mutation collection directory".to_string())?;
    mutation_authority_no_follow(&common.workspace_root, mutation_root, true)?;
    if mutation_root.file_name().and_then(|name| name.to_str()) != Some(common.mutation_collection.as_str()) { return Err("aggregate source is not directly inside the taxonomy mutation collection".to_string()); }
    Ok(MutationAggregateSourceAuthority { workspace_root: common.workspace_root, mutation_root: mutation_root.to_path_buf(), source_path: common.source_path, taxonomy_path: common.taxonomy_path, source_filename: common.source_filename, descriptor_filename: common.descriptor_filename })
}

fn mutation_authority_normalize(source: &Path, compiler_cwd: &Path) -> Result<PathBuf, String> {
    if !compiler_cwd.is_absolute() { return Err("compiler cwd is not absolute".to_string()); }
    mutation_authority_raw_lexical(compiler_cwd)?;
    let input = if source.is_absolute() { source.to_path_buf() } else { compiler_cwd.join(source) };
    mutation_authority_raw_no_follow(&input)?;
    let mut normalized = PathBuf::new();
    for component in input.components() {
        if let Component::Normal(segment) = component {
            if segment.to_str().map(|value| value.eq_ignore_ascii_case("compose")).unwrap_or(false) { return Err("opaque compose path rejected before I/O".to_string()); }
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
        if segment.to_str().map(|value| value.eq_ignore_ascii_case("compose")).unwrap_or(false) { return Err("opaque compose path rejected before I/O".to_string()); }
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
    if locator.is_empty() || locator.contains('\0') || locator.contains('\\') || locator.starts_with('/') || locator.as_bytes().get(1) == Some(&b':') { return Err("taxonomy locator is not normalized repository-relative path".to_string()); }
    let mut target = root.to_path_buf();
    for segment in locator.split('/') {
        if segment.is_empty() || segment == "." || segment == ".." || segment.eq_ignore_ascii_case("compose") { return Err("taxonomy locator has rejected path component".to_string()); }
        target.push(segment);
    }
    Ok(target)
}

fn mutation_authority_filename(taxonomy: &serde_json::Value, file_kind_id: &str) -> Result<String, String> {
    let file_kind = taxonomy.get("fileKinds").and_then(serde_json::Value::as_object).and_then(|kinds| kinds.get(file_kind_id)).and_then(serde_json::Value::as_object).ok_or_else(|| "taxonomy file kind is missing".to_string())?;
    let emoji = file_kind.get("emoji").and_then(serde_json::Value::as_str).filter(|value| !value.is_empty()).ok_or_else(|| "taxonomy file kind emoji is missing".to_string())?;
    let extension = file_kind.get("extensionChains").and_then(serde_json::Value::as_array).and_then(|values| values.first()).and_then(serde_json::Value::as_str).filter(|value| value.starts_with('.')).ok_or_else(|| "taxonomy file kind canonical extension is missing".to_string())?;
    Ok(format!("{emoji}{extension}"))
}

fn mutation_authority_collection(taxonomy: &serde_json::Value) -> Result<String, String> {
    let collections = taxonomy.get("semanticCollections").and_then(serde_json::Value::as_object).ok_or_else(|| "taxonomy semantic collections are missing".to_string())?;
    let mutations: Vec<&String> = collections.iter().filter_map(|(name, definition)| (definition.get("kind").and_then(serde_json::Value::as_str) == Some("mutation")).then_some(name)).collect();
    if mutations.len() != 1 || mutations[0].contains('/') { return Err("taxonomy mutation collection is ambiguous".to_string()); }
    Ok(mutations[0].to_string())
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
mod mutation_source_authority_tests {
    use super::*;

    pub(super) fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-source-authority/🧫️fixtures/🔣️cases.json")).unwrap() }

    fn link_file(target: &Path, link: &Path) {
        #[cfg(unix)] std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)] std::os::windows::fs::symlink_file(target, link).unwrap();
    }

    fn link_dir(target: &Path, link: &Path) {
        #[cfg(unix)] std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)] std::os::windows::fs::symlink_dir(target, link).unwrap();
    }

    fn fixture_workspace(case: &str) -> PathBuf {
        let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        fs::create_dir_all(&base).unwrap();
        fs::canonicalize(base).unwrap().join(format!("semio-source-authority-{case}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()))
    }

    pub(super) fn materialize(case: &str, fixture: &serde_json::Value) -> (PathBuf, PathBuf, PathBuf) {
        let workspace = fixture_workspace(case);
        let mutation_root = workspace.join("domain/🧬️mutations");
        let owner = mutation_root.join("🆕️insert-page");
        let source = owner.join("🦀️.rs");
        let descriptor = owner.join("🔣️.json");
        let taxonomy = workspace.join("authority/🔣️taxonomy.json");
        fs::create_dir_all(&owner).unwrap();
        fs::create_dir_all(taxonomy.parent().unwrap()).unwrap();
        fs::write(workspace.join("nx.json"), "{}").unwrap();
        fs::write(&source, "pub struct Probe;").unwrap();
        let owner_text = mutation_authority_relative(&workspace, &owner).unwrap();
        let mut descriptor_value = fixture["descriptor"].clone();
        descriptor_value["owner"] = serde_json::Value::String(owner_text);
        fs::write(&descriptor, serde_json::to_vec(&descriptor_value).unwrap()).unwrap();
        fs::write(&taxonomy, r#"{"fileKinds":{"rust":{"emoji":"🦀️","extensionChains":[".rs"]},"json":{"emoji":"🔣️","extensionChains":[".json"]}},"mutationComponentFileKindId":"rust","mutationDescriptorFileKindId":"json","semanticCollections":{"🧬️mutations":{"kind":"mutation"}}}"#).unwrap();
        fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{"taxonomy":"authority/🔣️taxonomy.json"}}}"#).unwrap();
        match case {
            "missing-locator" => fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{}}}"#).unwrap(),
            "malformed-locator" => fs::write(workspace.join("📋️project.json"), r#"{"metadata":{"semio":{"taxonomy":"../authority/🔣️taxonomy.json"}}}"#).unwrap(),
            "wrong-root-pair" => fs::remove_file(workspace.join("nx.json")).unwrap(),
            "wrong-primary-filename" => { let wrong = owner.join("🦀️component.rs"); fs::rename(&source, &wrong).unwrap(); return (workspace.clone(), workspace, wrong); },
            "owner-mismatch" => fs::write(&descriptor, r#"{"owner":"other/🧬️mutations/🆕️insert-page"}"#).unwrap(),
            "symlink-parent" => { let actual = mutation_root.join("actual-owner"); fs::rename(&owner, &actual).unwrap(); link_dir(&actual, &owner); },
            "symlink-source" => { let actual = owner.join("🦀️actual.rs"); fs::rename(&source, &actual).unwrap(); link_file(&actual, &source); },
            "symlink-descriptor" => { let actual = owner.join("🔣️actual.json"); fs::rename(&descriptor, &actual).unwrap(); link_file(&actual, &descriptor); },
            "symlink-taxonomy" => { let actual = workspace.join("authority/🔣️actual.json"); fs::rename(&taxonomy, &actual).unwrap(); link_file(&actual, &taxonomy); },
            "nested-nx-anchor" => { let nested = workspace.join("nested"); fs::create_dir_all(&nested).unwrap(); fs::write(nested.join("nx.json"), "{}").unwrap(); fs::rename(workspace.join("domain"), nested.join("domain")).unwrap(); return (workspace.clone(), workspace, nested.join("domain/🧬️mutations/🆕️insert-page/🦀️.rs")); },
            "symlink-ancestor" => { let link = workspace.join("domain-link"); link_dir(&workspace.join("domain"), &link); return (workspace.clone(), workspace, link.join("🧬️mutations/🆕️insert-page/🦀️.rs")); },
            "symlink-parent-erasure" => { let link = workspace.join("erased"); link_dir(&workspace.join("domain"), &link); return (workspace.clone(), workspace, PathBuf::from("erased/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")); },
            "virtual-compose" => return (workspace.clone(), workspace.clone(), workspace.join("compose/🧬️mutations/🆕️insert-page/🦀️.rs")),
            "raw-case-folded-compose-parent" => return (workspace.clone(), workspace, PathBuf::from("consumer/../COMPOSE/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")),
            "file-parent-erasure" => { fs::write(workspace.join("not-a-directory"), "file").unwrap(); return (workspace.clone(), workspace, PathBuf::from("not-a-directory/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")); },
            "raw-nonutf8" => {
                #[cfg(unix)] { use std::os::unix::ffi::OsStringExt; return (workspace.clone(), workspace, PathBuf::from(std::ffi::OsString::from_vec(b"consumer/\xff".to_vec()))); }
                #[cfg(windows)] { use std::os::windows::ffi::OsStringExt; return (workspace.clone(), workspace, PathBuf::from(std::ffi::OsString::from_wide(&[0xD800]))); }
            },
            "valid-relative-parent" => { fs::create_dir_all(workspace.join("consumer")).unwrap(); return (workspace.clone(), workspace, PathBuf::from("consumer/../domain/🧬️mutations/🆕️insert-page/🦀️.rs")); },
            _ => {},
        }
        (workspace.clone(), workspace, source)
    }

    #[test]
    fn validates_mutation_source_authority_fixture() {
        let fixture = fixture();
        assert_eq!(fixture["schemaVersion"], 1);
        let mut descriptor_keys: Vec<&str> = fixture["descriptor"].as_object().unwrap().keys().map(String::as_str).collect(); descriptor_keys.sort_unstable(); assert_eq!(descriptor_keys, ["aggregateVariant", "binaryTag", "composition", "diffParticipation", "displayName", "emoji", "invertibility", "outcomeClasses", "owner", "payloadSchema", "requiredLanguageSurfaces", "schemaVersion", "semanticKind", "textOpcode"]);
        for vector in fixture["cases"].as_array().unwrap() {
            let name = vector["name"].as_str().unwrap();
            let (workspace, compiler_cwd, source) = materialize(name, &fixture);
            let result = mutation_source_authority(&source, &compiler_cwd);
            assert_eq!(result.is_ok(), vector["accepted"].as_bool().unwrap(), "{name}: {result:?}");
            #[cfg(any(unix, windows))] if name == "raw-nonutf8" { assert!(result.as_ref().unwrap_err().contains("not UTF-8")); }
            if name == "nested-nx-anchor" { assert!(result.as_ref().unwrap_err().contains("lacks paired")); }
            if name == "file-parent-erasure" { assert_eq!(fs::metadata(compiler_cwd.join(&source)).unwrap_err().kind(), std::io::ErrorKind::NotADirectory); }
            if let Ok(facts) = result { assert_eq!(facts.workspace_root, workspace); assert!(facts.mutation_root.ends_with("domain/🧬️mutations")); assert!(facts.owner.ends_with("🆕️insert-page")); assert_eq!(facts.source_path.file_name().and_then(|name| name.to_str()), Some("🦀️.rs")); assert_eq!(facts.descriptor_path.file_name().and_then(|name| name.to_str()), Some("🔣️.json")); assert!(facts.taxonomy_path.ends_with("authority/🔣️taxonomy.json")); }
        }
    }
}

#[cfg(test)]
mod mutation_aggregate_source_authority_tests {
    use super::*;

    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-aggregate-source-authority/🧫️fixtures/🔣️cases.json")).unwrap() }

    fn link_file(target: &Path, link: &Path) {
        #[cfg(unix)] std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)] std::os::windows::fs::symlink_file(target, link).unwrap();
    }

    fn link_dir(target: &Path, link: &Path) {
        #[cfg(unix)] std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)] std::os::windows::fs::symlink_dir(target, link).unwrap();
    }

    fn materialize(name: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
        let leaf_fixture = mutation_source_authority_tests::fixture();
        let (workspace, _, leaf) = mutation_source_authority_tests::materialize("valid", &leaf_fixture);
        let mutation_root = leaf.parent().unwrap().parent().unwrap().to_path_buf();
        let aggregate = mutation_root.join("🦀️.rs");
        fs::write(&aggregate, "pub enum Probe {}").unwrap();
        match name {
            "compiler-relative" => { fs::create_dir_all(workspace.join("consumer")).unwrap(); return (workspace.clone(), workspace.clone(), PathBuf::from("consumer/../domain/🧬️mutations/🦀️.rs"), leaf); },
            "parent-mounted" => { let cwd = workspace.parent().unwrap().to_path_buf(); return (workspace.clone(), cwd, PathBuf::from(workspace.file_name().unwrap()).join("domain/🧬️mutations/🦀️.rs"), leaf); },
            "leaf-primary" => return (workspace.clone(), workspace, leaf.clone(), leaf),
            "historical-primary" => { let historical = mutation_root.join("🦀️component.rs"); fs::rename(&aggregate, &historical).unwrap(); return (workspace.clone(), workspace, historical, leaf); },
            "outside-root" => { let outside_root = workspace.join("outside"); let outside = outside_root.join("🦀️.rs"); fs::create_dir_all(&outside_root).unwrap(); fs::write(&outside, "pub enum Probe {}").unwrap(); return (workspace, outside_root, outside, leaf); },
            "virtual-compose" => { let source = workspace.join("compose/🧬️mutations/🦀️.rs"); return (workspace.clone(), workspace, source, leaf); },
            "nested-nx-anchor" => { let nested = workspace.join("nested"); fs::create_dir_all(&nested).unwrap(); fs::write(nested.join("nx.json"), "{}").unwrap(); fs::rename(workspace.join("domain"), nested.join("domain")).unwrap(); return (workspace.clone(), workspace, nested.join("domain/🧬️mutations/🦀️.rs"), leaf); },
            "taxonomy-filename-change" => { let changed = mutation_root.join("🐹️.go"); fs::rename(&aggregate, &changed).unwrap(); fs::write(workspace.join("authority/🔣️taxonomy.json"), r#"{"fileKinds":{"go":{"emoji":"🐹️","extensionChains":[".go"]},"json":{"emoji":"🔣️","extensionChains":[".json"]}},"mutationComponentFileKindId":"go","mutationDescriptorFileKindId":"json","semanticCollections":{"🧬️mutations":{"kind":"mutation"}}}"#).unwrap(); return (workspace.clone(), workspace, changed, leaf); },
            "symlink-source" => { let actual = mutation_root.join("🦀️actual.rs"); fs::rename(&aggregate, &actual).unwrap(); link_file(&actual, &aggregate); },
            "symlink-root" => { let actual = workspace.join("domain/actual-mutations"); fs::rename(&mutation_root, &actual).unwrap(); link_dir(&actual, &mutation_root); },
            _ => {},
        }
        (workspace.clone(), workspace, aggregate, leaf)
    }

    #[test]
    fn validates_schema_first_aggregate_authority_fixture() {
        let fixture = fixture();
        assert_eq!(fixture["schemaVersion"], 1);
        for vector in fixture["cases"].as_array().unwrap() {
            let name = vector["name"].as_str().unwrap();
            let (workspace, cwd, source, _) = materialize(name);
            let result = mutation_aggregate_source_authority(&source, &cwd);
            assert_eq!(result.is_ok(), vector["accepted"].as_bool().unwrap(), "{name}: {result:?}");
            if let Some(diagnostic) = vector.get("diagnostic").and_then(serde_json::Value::as_str) { assert!(result.as_ref().unwrap_err().contains(diagnostic), "{name}: {result:?}"); }
            if let Ok(facts) = result { assert_eq!(facts.workspace_root, workspace); assert!(facts.mutation_root.ends_with("domain/🧬️mutations")); assert!(facts.source_path.ends_with(facts.source_filename.as_str())); assert!(!facts.descriptor_filename.is_empty()); assert!(facts.taxonomy_path.ends_with("authority/🔣️taxonomy.json")); }
        }
    }

    #[test]
    fn aggregate_and_leaf_authority_share_workspace_taxonomy_names_and_token() {
        let (workspace, cwd, aggregate_source, leaf_source) = materialize("direct-canonical");
        let aggregate = mutation_aggregate_source_authority(&aggregate_source, &cwd).unwrap();
        let leaf = mutation_source_authority(&leaf_source, &cwd).unwrap();
        let leaf_common = mutation_authority_common(&leaf_source, &cwd).unwrap();
        assert_eq!(aggregate.workspace_root, leaf.workspace_root); assert_eq!(aggregate.mutation_root, leaf.mutation_root); assert_eq!(aggregate.taxonomy_path, leaf.taxonomy_path); assert_eq!(aggregate.source_filename, leaf_common.source_filename); assert_eq!(aggregate.descriptor_filename, leaf_common.descriptor_filename);
        assert_eq!(mutation_authority_workspace_token(&aggregate.workspace_root, &aggregate.taxonomy_path).unwrap(), mutation_leaf_workspace_token(&leaf).unwrap());
        let (_, other_cwd, other_aggregate_source, _) = materialize("direct-canonical");
        let other = mutation_aggregate_source_authority(&other_aggregate_source, &other_cwd).unwrap();
        assert_ne!(mutation_authority_workspace_token(&aggregate.workspace_root, &aggregate.taxonomy_path).unwrap(), mutation_authority_workspace_token(&other.workspace_root, &other.taxonomy_path).unwrap());
        assert!(workspace.exists());
    }
}
//#endregion 🔖️MutationSourceAuthority

//#region 🔣️MutationLeafJson
#[derive(Debug, PartialEq, Eq)]
enum MutationLeafInvertibility { SelfInvertible, ExplicitMutation, Plan, NonInvertible }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafDiffParticipation { Detect, ApplyOnly, Plan, None }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafOutcomeClass { Applied, Info, Warning, Error, Fatal }

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
}

const MUTATION_LEAF_DESCRIPTOR_KEYS: [&str; 14] = ["schemaVersion", "owner", "semanticKind", "displayName", "emoji", "aggregateVariant", "payloadSchema", "textOpcode", "binaryTag", "invertibility", "diffParticipation", "outcomeClasses", "composition", "requiredLanguageSurfaces"];

fn parse_mutation_leaf_descriptor(raw: &[u8], authority: &MutationSourceAuthority) -> Result<MutationLeafJson, String> {
    mutation_leaf_reject_duplicate_keys(raw)?;
    let value: serde_json::Value = serde_json::from_slice(raw).map_err(|error| format!("malformed mutation descriptor JSON: {error}"))?;
    let object = value.as_object().ok_or_else(|| "mutation descriptor must be an object".to_string())?;
    if object.len() != MUTATION_LEAF_DESCRIPTOR_KEYS.len() || MUTATION_LEAF_DESCRIPTOR_KEYS.iter().any(|key| !object.contains_key(*key)) || object.keys().any(|key| !MUTATION_LEAF_DESCRIPTOR_KEYS.contains(&key.as_str())) { return Err("mutation descriptor must contain exactly the fourteen schema fields".to_string()); }
    let string = |key| mutation_leaf_string(object.get(key).unwrap(), key);
    let schema_version = mutation_leaf_u32(object.get("schemaVersion").unwrap(), "schemaVersion")?;
    if schema_version != 1 { return Err("schemaVersion must equal 1".to_string()); }
    let owner = string("owner")?;
    if owner != authority.owner { return Err("descriptor owner does not exactly match source owner".to_string()); }
    let semantic_kind = string("semanticKind")?;
    if !mutation_leaf_kebab(&semantic_kind) { return Err("semanticKind must be lowercase kebab-case".to_string()); }
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
    Ok(MutationLeafJson { schema_version, owner, semantic_kind, display_name, emoji, aggregate_variant, payload_schema, text_opcode, binary_tag, invertibility, diff_participation, outcome_classes, composition, required_language_surfaces })
}

fn mutation_leaf_string(value: &serde_json::Value, key: &str) -> Result<String, String> { value.as_str().filter(|value| !value.is_empty()).map(str::to_owned).ok_or_else(|| format!("{key} must be a nonempty string")) }

fn mutation_leaf_u32(value: &serde_json::Value, key: &str) -> Result<u32, String> {
    let number = value.as_f64().ok_or_else(|| format!("{key} must be an integer"))?;
    if !number.is_finite() || number.fract() != 0.0 || number < 0.0 || number > u32::MAX as f64 { return Err(format!("{key} must be a u32 integer")); }
    Ok(number as u32)
}

fn mutation_leaf_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty() && bytes[0].is_ascii_lowercase() && bytes.contains(&b'-') && bytes.split(|byte| *byte == b'-').all(|part| !part.is_empty() && part.iter().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

fn mutation_leaf_pascal(value: &str) -> bool { let bytes = value.as_bytes(); !bytes.is_empty() && bytes[0].is_ascii_uppercase() && bytes.iter().all(|byte| byte.is_ascii_alphanumeric()) }

fn mutation_leaf_outcomes(value: &serde_json::Value) -> Result<Vec<MutationLeafOutcomeClass>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "outcomeClasses must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    values.iter().map(|value| { let value = mutation_leaf_string(value, "outcomeClasses")?; if !seen.insert(value.clone()) { return Err("outcomeClasses must not contain duplicates".to_string()); } match value.as_str() { "applied" => Ok(MutationLeafOutcomeClass::Applied), "info" => Ok(MutationLeafOutcomeClass::Info), "warning" => Ok(MutationLeafOutcomeClass::Warning), "error" => Ok(MutationLeafOutcomeClass::Error), "fatal" => Ok(MutationLeafOutcomeClass::Fatal), _ => Err("outcomeClasses contains a non-schema enum value".to_string()) } }).collect()
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
    match raw.get(index)? { b'\"' => mutation_leaf_string_end(raw, index), b'{' => mutation_leaf_balanced_end(raw, index, b'{', b'}'), b'[' => mutation_leaf_balanced_end(raw, index, b'[', b']'), _ => { let end = raw[index..].iter().position(|byte| matches!(*byte, b',' | b'}' | b']') || byte.is_ascii_whitespace()).map(|offset| index + offset).unwrap_or(raw.len()); (end > index).then_some(end) } }
}
fn mutation_leaf_balanced_end(raw: &[u8], mut index: usize, open: u8, close: u8) -> Option<usize> { let mut depth = 0usize; while let Some(byte) = raw.get(index) { if *byte == b'\"' { index = mutation_leaf_string_end(raw, index)?; continue; } if *byte == open { depth += 1; } else if *byte == close { depth -= 1; if depth == 0 { return Some(index + 1); } } index += 1; } None }

fn emit_mutation_leaf_descriptor(contract: &syn::Path, descriptor: &MutationLeafJson) -> proc_macro2::TokenStream {
    let schema_version = descriptor.schema_version; let owner = &descriptor.owner; let semantic_kind = &descriptor.semantic_kind; let display_name = &descriptor.display_name; let emoji = &descriptor.emoji; let aggregate_variant = &descriptor.aggregate_variant; let payload_schema = &descriptor.payload_schema;
    let text_opcode = descriptor.text_opcode.as_ref().map(|value| quote!(::core::option::Option::Some(#value))).unwrap_or_else(|| quote!(::core::option::Option::None)); let binary_tag = descriptor.binary_tag.map(|value| quote!(::core::option::Option::Some(#value))).unwrap_or_else(|| quote!(::core::option::Option::None));
    let invertibility = match &descriptor.invertibility { MutationLeafInvertibility::SelfInvertible => quote!(#contract::MutationInvertibility::SelfInvertible), MutationLeafInvertibility::ExplicitMutation => quote!(#contract::MutationInvertibility::ExplicitMutation), MutationLeafInvertibility::Plan => quote!(#contract::MutationInvertibility::Plan), MutationLeafInvertibility::NonInvertible => quote!(#contract::MutationInvertibility::NonInvertible) };
    let diff_participation = match &descriptor.diff_participation { MutationLeafDiffParticipation::Detect => quote!(#contract::MutationDiffParticipation::Detect), MutationLeafDiffParticipation::ApplyOnly => quote!(#contract::MutationDiffParticipation::ApplyOnly), MutationLeafDiffParticipation::Plan => quote!(#contract::MutationDiffParticipation::Plan), MutationLeafDiffParticipation::None => quote!(#contract::MutationDiffParticipation::None) };
    let outcome_classes = descriptor.outcome_classes.iter().map(|value| match value { MutationLeafOutcomeClass::Applied => quote!(#contract::MutationOutcomeClass::Applied), MutationLeafOutcomeClass::Info => quote!(#contract::MutationOutcomeClass::Info), MutationLeafOutcomeClass::Warning => quote!(#contract::MutationOutcomeClass::Warning), MutationLeafOutcomeClass::Error => quote!(#contract::MutationOutcomeClass::Error), MutationLeafOutcomeClass::Fatal => quote!(#contract::MutationOutcomeClass::Fatal) });
    let composition = match &descriptor.composition { MutationLeafComposition::Atomic => quote!(#contract::MutationComposition::Atomic), MutationLeafComposition::Composite => quote!(#contract::MutationComposition::Composite) };
    let required_language_surfaces = descriptor.required_language_surfaces.iter().map(|value| match value { MutationLeafLanguageSurface::Rust => quote!(#contract::MutationLanguageSurface::Rust), MutationLeafLanguageSurface::Typescript => quote!(#contract::MutationLanguageSurface::Typescript), MutationLeafLanguageSurface::Graphql => quote!(#contract::MutationLanguageSurface::Graphql), MutationLeafLanguageSurface::Protobuf => quote!(#contract::MutationLanguageSurface::Protobuf), MutationLeafLanguageSurface::JsonSchema => quote!(#contract::MutationLanguageSurface::JsonSchema), MutationLeafLanguageSurface::Text => quote!(#contract::MutationLanguageSurface::Text), MutationLeafLanguageSurface::Binary => quote!(#contract::MutationLanguageSurface::Binary) });
    quote!(#contract::MutationLeafDescriptor { schema_version: #schema_version, owner: #owner, semantic_kind: #semantic_kind, display_name: #display_name, emoji: #emoji, aggregate_variant: #aggregate_variant, payload_schema: #payload_schema, text_opcode: #text_opcode, binary_tag: #binary_tag, invertibility: #invertibility, diff_participation: #diff_participation, outcome_classes: &[#(#outcome_classes),*], composition: #composition, required_language_surfaces: &[#(#required_language_surfaces),*] })
}

#[cfg(test)]
mod mutation_leaf_json_tests {
    use super::*;
    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-leaf-json/🧫️fixtures/🔣️cases.json")).unwrap() }
    fn authority(owner: &str) -> MutationSourceAuthority { MutationSourceAuthority { workspace_root: PathBuf::new(), mutation_root: PathBuf::new(), owner: owner.to_string(), source_path: PathBuf::new(), descriptor_path: PathBuf::new(), taxonomy_path: PathBuf::new() } }
    #[test]
    fn parses_mutation_leaf_json_fixture() {
        let fixture = fixture(); let authority = authority(fixture["authorityOwner"].as_str().unwrap());
        for vector in fixture["cases"].as_array().unwrap() { let result = parse_mutation_leaf_descriptor(vector["raw"].as_str().unwrap().as_bytes(), &authority); assert_eq!(result.is_ok(), vector["parserAccepted"].as_bool().unwrap(), "{}: {result:?}", vector["name"]); if let Err(error) = result { assert!(error.contains(vector["diagnostic"].as_str().unwrap()), "{}: {error}", vector["name"]); } }
    }
    #[test]
    fn emits_all_core_descriptor_fields() {
        let fixture = fixture(); let authority = authority(fixture["authorityOwner"].as_str().unwrap()); let descriptor = parse_mutation_leaf_descriptor(fixture["cases"][0]["raw"].as_str().unwrap().as_bytes(), &authority).unwrap(); let contract: syn::Path = syn::parse_str("::protocol").unwrap(); let emitted = emit_mutation_leaf_descriptor(&contract, &descriptor).to_string();
        for field in ["schema_version", "owner", "semantic_kind", "display_name", "emoji", "aggregate_variant", "payload_schema", "text_opcode", "binary_tag", "invertibility", "diff_participation", "outcome_classes", "composition", "required_language_surfaces"] { assert!(emitted.contains(field), "missing {field}: {emitted}"); }
        assert!(emitted.contains("MutationLeafDescriptor") && emitted.contains("ExplicitMutation") && emitted.contains("JsonSchema"));
    }
}
//#endregion 🔣️MutationLeafJson

//#region 🪪️MutationLeaf
#[derive(Debug)]
struct MutationLeafAttrs { contract: syn::Path }

fn parse_mutation_leaf_attrs(input: &DeriveInput) -> syn::Result<MutationLeafAttrs> {
    let mut contract = None;
    let mut found = false;
    for attribute in &input.attrs {
        if !attribute.path().is_ident("mutation_leaf") { continue; }
        if found { return Err(syn::Error::new_spanned(attribute, "duplicate mutation_leaf attribute")); }
        found = true;
        if matches!(&attribute.meta, syn::Meta::Path(_)) { continue; }
        attribute.parse_nested_meta(|meta| {
            if !meta.path.is_ident("contract") { return Err(meta.error("unsupported mutation_leaf attribute")); }
            if contract.is_some() { return Err(meta.error("duplicate mutation_leaf contract")); }
            let path: syn::Path = meta.value()?.parse()?;
            if path.leading_colon.is_none() || path.segments.is_empty() || path.segments.iter().any(|segment| !matches!(segment.arguments, syn::PathArguments::None) || matches!(segment.ident.to_string().as_str(), "self" | "super" | "crate")) { return Err(meta.error("mutation_leaf contract must be an absolute non-generic Rust path without self, super, or crate segments")); }
            contract = Some(path);
            Ok(())
        })?;
    }
    if !found { return Err(syn::Error::new_spanned(input, "MutationLeaf requires #[mutation_leaf(contract = ::protocol)]")); }
    contract.map(|contract| MutationLeafAttrs { contract }).ok_or_else(|| syn::Error::new_spanned(input, "MutationLeaf requires mutation_leaf contract"))
}

fn mutation_leaf_portable_path(path: &Path) -> Result<String, String> { path.to_str().map(|path| path.replace('\\', "/")).filter(|path| !path.is_empty()).ok_or_else(|| "metadata path is not UTF-8".to_string()) }

fn mutation_authority_workspace_token(workspace_root: &Path, taxonomy_path: &Path) -> Result<[u8; 32], String> {
    let workspace_root = fs::canonicalize(workspace_root).map_err(|error| error.to_string())?;
    let workspace = mutation_leaf_portable_path(&workspace_root)?;
    let taxonomy = mutation_authority_relative(&workspace_root, taxonomy_path)?;
    let mut input = b"semio.mutation-source-provenance/v1\0".to_vec();
    for value in [workspace.as_bytes(), taxonomy.as_bytes()] { let length = u64::try_from(value.len()).map_err(|_| "metadata token component exceeds u64".to_string())?; input.extend_from_slice(&length.to_be_bytes()); input.extend_from_slice(value); }
    Ok(semio_framework_hash::Sha256::digest(&input))
}

fn mutation_leaf_workspace_token(authority: &MutationSourceAuthority) -> Result<[u8; 32], String> { mutation_authority_workspace_token(&authority.workspace_root, &authority.taxonomy_path) }

fn mutation_leaf_include_path(path: &Path) -> Result<String, String> { mutation_leaf_portable_path(path) }

#[proc_macro_derive(MutationLeaf, attributes(mutation_leaf))]
pub fn derive_mutation_leaf(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if matches!(input.data, Data::Union(_)) { return syn::Error::new_spanned(&input, "MutationLeaf does not support unions").to_compile_error().into(); }
    let attrs = match parse_mutation_leaf_attrs(&input) { Ok(attrs) => attrs, Err(error) => return error.to_compile_error().into() };
    let source = match input.ident.span().unwrap().local_file() { Some(source) => source, None => return syn::Error::new_spanned(&input, "MutationLeaf requires a local source file").to_compile_error().into() };
    let compiler_cwd = match std::env::current_dir() { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let authority = match mutation_source_authority(&source, &compiler_cwd) { Ok(authority) => authority, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf source authority failed: {error}")).to_compile_error().into() };
    let raw_descriptor = match fs::read(&authority.descriptor_path) { Ok(raw) => raw, Err(error) => return syn::Error::new_spanned(&input, error.to_string()).to_compile_error().into() };
    let descriptor = match parse_mutation_leaf_descriptor(&raw_descriptor, &authority) { Ok(descriptor) => descriptor, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf descriptor failed: {error}")).to_compile_error().into() };
    let workspace_token = match mutation_leaf_workspace_token(&authority) { Ok(token) => token, Err(error) => return syn::Error::new_spanned(&input, format!("MutationLeaf provenance failed: {error}")).to_compile_error().into() };
    let mutation_root = match mutation_authority_relative(&authority.workspace_root, &authority.mutation_root) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let source_path = match mutation_authority_relative(&authority.workspace_root, &authority.source_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let descriptor_path = match mutation_authority_relative(&authority.workspace_root, &authority.descriptor_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let taxonomy_path = match mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path) { Ok(path) => path, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let dependency_paths = [authority.workspace_root.join("nx.json"), authority.workspace_root.join("📋️project.json"), authority.taxonomy_path.clone(), authority.descriptor_path.clone()];
    let dependency_paths: Result<Vec<_>, _> = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect();
    let dependency_paths = match dependency_paths { Ok(paths) => paths, Err(error) => return syn::Error::new_spanned(&input, error).to_compile_error().into() };
    let [nx_path, project_path, taxonomy_dependency, descriptor_dependency]: [String; 4] = match dependency_paths.try_into() { Ok(paths) => paths, Err(_) => unreachable!() };
    let name = &input.ident;
    let contract = &attrs.contract;
    let owner = &authority.owner;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let descriptor = emit_mutation_leaf_descriptor(contract, &descriptor);
    let workspace_token = workspace_token.iter();
    quote! {
        const _: &str = ::core::include_str!(#nx_path);
        const _: &str = ::core::include_str!(#project_path);
        const _: &str = ::core::include_str!(#taxonomy_dependency);
        const _: &str = ::core::include_str!(#descriptor_dependency);
        impl #impl_generics #contract::MutationLeaf for #name #ty_generics #where_clause {
            const DESCRIPTOR: #contract::MutationLeafDescriptor = #descriptor;
            const PROVENANCE: #contract::MutationSourceProvenance = #contract::MutationSourceProvenance { workspace_token: [#(#workspace_token),*], mutation_root: #mutation_root, owner: #owner, source_path: #source_path, descriptor_path: #descriptor_path, taxonomy_path: #taxonomy_path };
        }
    }.into()
}
//#endregion 🪪️MutationLeaf

#[cfg(test)]
mod mutation_leaf_derive_tests {
    use super::*;
    use sha2::Digest;

    #[test]
    fn parses_strict_mutation_leaf_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-leaf-derive/🧫️fixtures/🔣️cases.json")).unwrap();
        for case in fixture["attributes"].as_array().unwrap() {
            let input: DeriveInput = match syn::parse_str(&format!("{} #[derive(MutationLeaf)] struct Probe;", case["attribute"].as_str().unwrap())) { Ok(input) => input, Err(_) => { assert!(!case["accepted"].as_bool().unwrap(), "{}", case["name"]); continue; } };
            let result = parse_mutation_leaf_attrs(&input);
            assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
            if let Some(diagnostic) = case["diagnostic"].as_str() { assert!(result.unwrap_err().to_string().contains(diagnostic), "{}", case["name"]); }
        }
    }

    #[test]
    fn hashes_workspace_provenance_with_sha2_oracle() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-source-authority/🧫️fixtures/🔣️cases.json")).unwrap();
        let (workspace, compiler_cwd, source) = mutation_source_authority_tests::materialize("valid", &fixture);
        let authority = mutation_source_authority(&source, &compiler_cwd).unwrap();
        let first = mutation_leaf_workspace_token(&authority).unwrap();
        let second = mutation_leaf_workspace_token(&authority).unwrap();
        let workspace = mutation_leaf_portable_path(&fs::canonicalize(workspace).unwrap()).unwrap();
        let taxonomy = mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path).unwrap();
        let mut input = b"semio.mutation-source-provenance/v1\0".to_vec();
        for value in [workspace.as_bytes(), taxonomy.as_bytes()] { input.extend_from_slice(&(value.len() as u64).to_be_bytes()); input.extend_from_slice(value); }
        let expected: [u8; 32] = sha2::Sha256::digest(&input).into();
        assert_eq!(first, second);
        assert_eq!(first, expected);
        let (_, other_cwd, other_source) = mutation_source_authority_tests::materialize("valid", &fixture);
        let other = mutation_leaf_workspace_token(&mutation_source_authority(&other_source, &other_cwd).unwrap()).unwrap();
        assert_ne!(first, other);
    }
}

//#region 🔖️Attrs
#[derive(Default, Clone)]
struct ContainerAttrs {
    extension: Option<String>,
    id: Option<String>,
    keyword: Option<String>,
    lines_layout: bool,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    key: Option<String>,
    positional: bool,
    list: bool,
    tuple: bool,
    statements: bool,
    block: bool,
    base64: bool,
    flatten: bool,
    table: bool,
    /// `#[dsl(unit = "GPa")]` — a scalar `f64`/`f32` field prints/parses as `Shape::Quantity`
    /// (glued unit suffix) instead of plain `Shape::Float`.
    unit: Option<String>,
    /// `#[dsl(angle = "deg")]` — same mechanism as `unit`, `Shape::Angle` instead.
    angle: Option<String>,
    /// `#[dsl(refs = "material")]` — a scalar `String`/`Option<String>` field prints/parses as
    /// `Shape::Ref(kind)` instead of plain `Shape::Text`.
    refs: Option<String>,
    /// `#[dsl(defines = "material")]` — the anchor side of `refs`: this field's `FieldSpec.defines`
    /// is set so `LanguageService::validate` knows which field, in a record of this kind, other
    /// records' `Shape::Ref("material")` fields are expected to resolve against.
    defines: Option<String>,
    /// `#[dsl(lang = "jack")]` — a scalar `String` field prints/parses as `Shape::Embed(lang)`
    /// (fenced verbatim in Document mode) instead of plain `Shape::Text`.
    lang: Option<String>,
    /// `#[dsl(lang_from = "language_id")]` — fence language from a sibling Text field at print/parse time.
    lang_from: Option<String>,
    /// `#[dsl(coord)]` — a `[f64; 3]` (or any `DslField` array) field prints/parses as
    /// `Shape::Coord(3)` (`@x,y,z`) instead of a bare comma tuple.
    coord: bool,
    /// `#[dsl(dir)]` — same mechanism as `coord`, `Shape::Dir` (`^x,y,z`) instead.
    dir: bool,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_container_attrs(input: &DeriveInput) -> ContainerAttrs {
    let mut out = ContainerAttrs::default();
    for attr in &input.attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("extension") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.extension = Some(value.value());
            } else if meta.path.is_ident("id") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.id = Some(value.value());
            } else if meta.path.is_ident("keyword") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.keyword = Some(value.value());
            } else if meta.path.is_ident("layout") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lines_layout = value.value() == "lines";
            }
            Ok(())
        });
    }
    out
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn parse_field_attrs(attrs: &[syn::Attribute]) -> FieldAttrs {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("dsl") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("key") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.key = Some(value.value());
            } else if meta.path.is_ident("positional") {
                out.positional = true;
            } else if meta.path.is_ident("list") {
                out.list = true;
            } else if meta.path.is_ident("tuple") {
                out.tuple = true;
            } else if meta.path.is_ident("statements") {
                out.statements = true;
            } else if meta.path.is_ident("block") {
                out.block = true;
            } else if meta.path.is_ident("base64") {
                out.base64 = true;
            } else if meta.path.is_ident("flatten") {
                out.flatten = true;
            } else if meta.path.is_ident("table") {
                out.table = true;
            } else if meta.path.is_ident("unit") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.unit = Some(value.value());
            } else if meta.path.is_ident("angle") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.angle = Some(value.value());
            } else if meta.path.is_ident("refs") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.refs = Some(value.value());
            } else if meta.path.is_ident("defines") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.defines = Some(value.value());
            } else if meta.path.is_ident("lang") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lang = Some(value.value());
            } else if meta.path.is_ident("lang_from") {
                let value: syn::LitStr = meta.value()?.parse()?;
                out.lang_from = Some(value.value());
            } else if meta.path.is_ident("coord") {
                out.coord = true;
            } else if meta.path.is_ident("dir") {
                out.dir = true;
            }
            Ok(())
        });
    }
    out
}
//#endregion 🔖️Attrs

//#region 🔖️TypeShape
enum FieldKind {
    Scalar,
    OptionScalar(Box<Type>),
    VecList(Box<Type>),
    VecTuple(Box<Type>),
    VecStatements(Box<Type>),
    /// `#[dsl(statements, block)]` — same tagged-variant collection as `VecStatements`, but wrapped
    /// in `{ ... }` so it can sit anywhere in field order (not just as an unbounded trailing field).
    VecBlockStatements(Box<Type>),
    /// `BTreeMap<String, V>` — `V` must itself implement `DslField`; keys print sorted.
    MapField(Box<Type>),
    /// `#[dsl(statements)] Option<T>` — a "sum type" scalar field (`fill: Option<FillStyle>`,
    /// exactly one of several keyword-tagged variants, or none) rather than a collection. Reuses
    /// `Shape::Statements`/`DslVariants` at 0-or-1 length instead of a new shape: a record isn't
    /// allowed more than one *bare* `Statements` field, but two `Option<T>` fields of this kind can
    /// coexist because each is dispatched by its own field key (always paired with `#[dsl(block)]`
    /// in practice, since an un-blocked one would hit that same one-per-record limit).
    OptionStatements(Box<Type>),
    /// `#[dsl(statements)] Box<T>` (or bare `T`) — exactly one required tagged value (`layer:
    /// Box<DrawLayerNode>` on an `AddLayer` operation), the non-optional counterpart of
    /// `OptionStatements`: same `Shape::Statements` reuse, but errors if the count isn't exactly 1
    /// rather than treating 0 as `None`.
    RequiredStatements(Box<Type>),
    Bytes64,
    /// `#[dsl(table)] Vec<T>` (`T: DslRecord`) — Structure-of-Arrays columnar `Shape::Table`.
    /// `to_value`/`from_value` are identical to `VecList` (both produce `FieldValue::List(Vec<
    /// FieldValue::Record>)`) — only the `Shape` differs, so every binder/diff path downstream
    /// keeps working unchanged.
    VecTable(Box<Type>),
}

/// @emoji 🪆️ Strips `macro_rules!`-introduced invisible-delimiter `Type::Group` wrappers so a type
/// captured through a `:ty` metavariable — then re-emitted through another technology-local
/// declarative macro (e.g. an `entity_input!`-style struct-generating macro) before ever reaching
/// this derive — still structurally matches `Type::Path` here exactly like directly-written source.
/// Without this, `Option<T>`/`Vec<T>`/`Box<T>`/`BTreeMap<..>` fields declared through such a wrapping
/// macro silently fall through to plain `FieldKind::Scalar` instead of being classified as
/// optional/list/map, since the wrapper hides the outer `Path` segment from a bare `matches!`.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn strip_groups(ty: &Type) -> &Type {
    let mut ty = ty;
    while let Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn inner_of(ty: &Type, wrapper: &str) -> Option<Type> {
    let Type::Path(path) = strip_groups(ty) else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != wrapper {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    args.args.iter().find_map(|arg| match arg {
        syn::GenericArgument::Type(t) => Some(t.clone()),
        _ => None,
    })
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn is_vec_u8(ty: &Type) -> bool {
    inner_of(ty, "Vec").is_some_and(|inner| matches!(strip_groups(&inner), Type::Path(p) if p.path.is_ident("u8")))
}

/// @emoji 🗺️ Extracts `V` from `BTreeMap<String, V>` — `None` for any other type, including a
/// `BTreeMap` keyed by something other than `String` (the engine's `Shape::Map` is string-keyed
/// only, matching every hand-rolled `{ key=value }` grammar it replaces).
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn btreemap_string_value(ty: &Type) -> Option<Type> {
    let Type::Path(path) = strip_groups(ty) else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != "BTreeMap" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    let types: Vec<&Type> = args
        .args
        .iter()
        .filter_map(|arg| match arg {
            syn::GenericArgument::Type(t) => Some(t),
            _ => None,
        })
        .collect();
    let [key, value] = types.as_slice() else { return None };
    matches!(strip_groups(key), Type::Path(p) if p.path.is_ident("String")).then(|| (*value).clone())
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn classify_field(ty: &Type, attrs: &FieldAttrs) -> (FieldKind, Type) {
    if let Some(inner) = inner_of(ty, "Option") {
        if attrs.statements {
            return (FieldKind::OptionStatements(Box::new(inner.clone())), inner);
        }
        return (FieldKind::OptionScalar(Box::new(inner.clone())), inner);
    }
    if attrs.base64 && is_vec_u8(ty) {
        return (FieldKind::Bytes64, ty.clone());
    }
    if attrs.statements {
        if let Some(inner) = inner_of(ty, "Box") {
            return (FieldKind::RequiredStatements(Box::new(inner.clone())), inner);
        }
    }
    if let Some(value_ty) = btreemap_string_value(ty) {
        return (FieldKind::MapField(Box::new(value_ty.clone())), value_ty);
    }
    if let Some(inner) = inner_of(ty, "Vec") {
        if attrs.statements {
            let kind = if attrs.block { FieldKind::VecBlockStatements(Box::new(inner.clone())) } else { FieldKind::VecStatements(Box::new(inner.clone())) };
            return (kind, inner);
        }
        if attrs.tuple {
            return (FieldKind::VecTuple(Box::new(inner.clone())), inner);
        }
        if attrs.table {
            return (FieldKind::VecTable(Box::new(inner.clone())), inner);
        }
        return (FieldKind::VecList(Box::new(inner.clone())), inner);
    }
    (FieldKind::Scalar, ty.clone())
}
//#endregion 🔖️TypeShape

//#region 🔖️RecordCodegen
struct FieldPlan {
    ident: syn::Ident,
    id: u16,
    key: String,
    positional: Option<u16>,
    optional: bool,
    kind: FieldKind,
    elem_ty: Type,
    /// `#[dsl(block)]` on a field whose `FieldKind` doesn't already imply its own `{ }` wrapping
    /// (`VecBlockStatements` handles that itself) — wraps whatever shape that kind would otherwise
    /// produce in `Shape::Block`, e.g. a single nested `#[derive(DslRecord)]` field printed as a
    /// bare `camera { x=0 y=0 zoom=1 }` line instead of a `camera=...` attribute.
    block: bool,
    /// `#[dsl(unit = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    unit: Option<String>,
    /// `#[dsl(angle = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    angle: Option<String>,
    /// `#[dsl(refs = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    refs: Option<String>,
    /// `#[dsl(defines = "...")]` — sets `FieldSpec.defines`, independent of `Shape`.
    defines: Option<String>,
    /// `#[dsl(lang = "...")]`, only meaningful for `FieldKind::Scalar`/`OptionScalar`.
    lang: Option<String>,
    lang_from: Option<String>,
    /// `#[dsl(coord)]`, only meaningful for `FieldKind::Scalar`/`OptionScalar` on an array type.
    coord: bool,
    /// `#[dsl(dir)]`, ditto.
    dir: bool,
}

// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn plan_fields(fields: &Fields) -> Vec<FieldPlan> {
    let mut positional_counter: u16 = 0;
    let mut out = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        let attrs = parse_field_attrs(&field.attrs);
        let ident = field.ident.clone().expect("dsl_derive only supports named fields");
        let (kind, elem_ty) = classify_field(&field.ty, &attrs);
        let key = attrs.key.clone().unwrap_or_else(|| to_kebab(&ident.to_string()));
        let optional = matches!(kind, FieldKind::OptionScalar(_) | FieldKind::OptionStatements(_));
        let positional = if attrs.positional {
            let p = positional_counter;
            positional_counter += 1;
            Some(p)
        } else {
            None
        };
        let block = attrs.block && !matches!(kind, FieldKind::VecBlockStatements(_));
        out.push(FieldPlan {
            ident,
            id: index as u16,
            key,
            positional,
            optional,
            kind,
            elem_ty,
            block,
            unit: attrs.unit.clone(),
            angle: attrs.angle.clone(),
            refs: attrs.refs.clone(),
            defines: attrs.defines.clone(),
            lang: attrs.lang.clone(),
            lang_from: attrs.lang_from.clone(),
            coord: attrs.coord,
            dir: attrs.dir,
        });
    }
    out
}

/// @emoji 🏗️ Builds the three code fragments shared by `DslRecord`/`DslArtifact`/`DslOps` variant
/// bodies: the `RecordSpec` field-spec expressions, the struct→`RecordValue` conversion, and the
/// `RecordValue`→struct conversion.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn record_codegen(fields: &Fields) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>, Vec<syn::Ident>) {
    let plans = plan_fields(fields);
    let mut spec_exprs = Vec::new();
    let mut to_value_stmts = Vec::new();
    let mut from_value_stmts = Vec::new();
    let mut field_idents = Vec::new();

    for plan in &plans {
        let FieldPlan { ident, id, key, positional, optional, kind, elem_ty, block, unit, angle, refs, defines, lang, lang_from, coord, dir } = plan;
        // A `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` scalar field's Shape is resolved at
        // spec-build time via `dsl::__rt::unit_for_derive` — same lazy-per-call pattern every other
        // `fn() -> RecordSpec`-backed Shape in this engine already uses, so an unknown unit symbol
        // surfaces as a panic the first time the generated spec runs (caught by that app's own
        // RecordSpec-law tests), never silently.
        let quantity_shape_override: Option<proc_macro2::TokenStream> = if let Some(symbol) = unit {
            Some(quote! { ::dsl::Shape::Quantity(::dsl::__rt::unit_for_derive(#symbol)) })
        } else if let Some(symbol) = angle {
            Some(quote! { ::dsl::Shape::Angle(::dsl::__rt::unit_for_derive(#symbol)) })
        } else if let Some(kind) = refs {
            Some(quote! { ::dsl::Shape::Ref(#kind) })
        } else if let Some(from) = lang_from {
            let embed_lang_key = plans.iter().find(|p| p.ident == from.as_str()).map_or_else(|| to_kebab(from), |p| p.key.clone());
            Some(quote! { ::dsl::Shape::EmbedFrom(#embed_lang_key) })
        } else if let Some(l) = lang {
            Some(quote! { ::dsl::Shape::Embed(#l) })
        } else if *coord {
            Some(quote! { ::dsl::Shape::Coord(3) })
        } else if *dir {
            Some(quote! { ::dsl::Shape::Dir })
        } else {
            None
        };
        let defines_expr = match defines {
            Some(kind) => quote! { .defines(#kind) },
            None => quote! {},
        };
        field_idents.push(ident.clone());
        let pos_expr = match positional {
            Some(p) => quote! { .positional(#p as u8) },
            None => quote! {},
        };
        let opt_expr = if *optional {
            quote! { .optional() }
        } else {
            quote! {}
        };

        // `DslField::shape`/`DslVariants::variants` are E4 (sync, fn-pointer transitivity — see
        // R9), so `shape_expr` never needs `.await`. `DslField::to_value`/`from_value` and
        // `DslVariants::to_named_record`/`from_named_record` stay `async`, so every value-level
        // call below is `.await`ed; a `Vec`/`Map` field can't `.await` per-element inside
        // `Iterator::map` (R10 residue shape 1), so those go through a sequential loop instead of
        // `.map().collect()`.
        let (shape_expr, to_value_expr, from_value_expr): (proc_macro2::TokenStream, proc_macro2::TokenStream, proc_macro2::TokenStream) = match kind {
            FieldKind::Scalar => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#elem_ty as ::dsl::DslField>::shape() }),
                quote! { ::dsl::DslField::to_value(&self.#ident) },
                quote! { <#elem_ty as ::dsl::DslField>::from_value(value).map_err(::dsl::__rt::field_error)? },
            ),
            FieldKind::Bytes64 => (
                quote! { ::dsl::Shape::Bytes64 },
                quote! { ::dsl::FieldValue::Bytes64(self.#ident.clone()) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Bytes64(bytes) => bytes.clone(),
                        other => return Err(::dsl::__rt::field_error(format!("expected Bytes64, found {other:?}"))),
                    }
                },
            ),
            FieldKind::OptionScalar(inner) => (
                quantity_shape_override.clone().unwrap_or_else(|| quote! { <#inner as ::dsl::DslField>::shape() }),
                quote! {
                    match &self.#ident {
                        Some(v) => ::dsl::DslField::to_value(v),
                        None => ::dsl::FieldValue::Absent,
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Absent => None,
                        other => Some(<#inner as ::dsl::DslField>::from_value(other).map_err(::dsl::__rt::field_error)?),
                    }
                },
            ),
            FieldKind::VecList(inner) => (
                quote! { ::dsl::Shape::List(Box::new(<#inner as ::dsl::DslField>::shape())) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::List(__items)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::List(items) => {
                            let mut __out = Vec::with_capacity(items.len());
                            for v in items.iter() { __out.push(<#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected List, found {other:?}"))),
                    }
                },
            ),
            // Same `to_value`/`from_value` as `VecList` (both produce `FieldValue::List(Record)`)
            // — only the `Shape` differs (`Table` vs `List(Record)`), which is what makes the
            // printer emit compact SoA instead of verbose AoS for this field.
            FieldKind::VecTable(inner) => (
                quote! { ::dsl::Shape::Table(<#inner>::__dsl_spec as fn() -> ::dsl::RecordSpec) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::List(__items)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::List(items) => {
                            let mut __out = Vec::with_capacity(items.len());
                            for v in items.iter() { __out.push(<#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected List, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecTuple(inner) => (
                quote! { ::dsl::Shape::Tuple(Box::new(<#inner as ::dsl::DslField>::shape()), None) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::Tuple(__items)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Tuple(items) => {
                            let mut __out = Vec::with_capacity(items.len());
                            for v in items.iter() { __out.push(<#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected Tuple, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslVariants::to_named_record(v)); }
                        ::dsl::FieldValue::Statements(__items)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Statements(items) => {
                            let mut __out = Vec::with_capacity(items.len());
                            for (keyword, record) in items.iter() { __out.push(<#inner as ::dsl::DslVariants>::from_named_record(keyword, record)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected Statements, found {other:?}"))),
                    }
                },
            ),
            FieldKind::VecBlockStatements(inner) => (
                quote! { ::dsl::Shape::Block(Box::new(::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()))) },
                quote! {
                    {
                        let mut __items = Vec::with_capacity(self.#ident.len());
                        for v in self.#ident.iter() { __items.push(::dsl::DslVariants::to_named_record(v)); }
                        ::dsl::FieldValue::Block(Box::new(::dsl::FieldValue::Statements(__items)))
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Block(inner_value) => match inner_value.as_ref() {
                            ::dsl::FieldValue::Statements(items) => {
                                let mut __out = Vec::with_capacity(items.len());
                                for (keyword, record) in items.iter() { __out.push(<#inner as ::dsl::DslVariants>::from_named_record(keyword, record)?); }
                                __out
                            }
                            other => return Err(::dsl::__rt::field_error(format!("expected Statements inside Block, found {other:?}"))),
                        },
                        other => return Err(::dsl::__rt::field_error(format!("expected Block, found {other:?}"))),
                    }
                },
            ),
            FieldKind::MapField(inner) => (
                quote! { ::dsl::Shape::Map(Box::new(<#inner as ::dsl::DslField>::shape())) },
                quote! {
                    {
                        let mut __entries = Vec::with_capacity(self.#ident.len());
                        for (k, v) in self.#ident.iter() { __entries.push((k.clone(), ::dsl::DslField::to_value(v))); }
                        ::dsl::FieldValue::Map(__entries)
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Map(entries) => {
                            let mut __out = ::std::collections::BTreeMap::new();
                            for (k, v) in entries.iter() { __out.insert(k.clone(), <#inner as ::dsl::DslField>::from_value(v).map_err(::dsl::__rt::field_error)?); }
                            __out
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected Map, found {other:?}"))),
                    }
                },
            ),
            FieldKind::OptionStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! {
                    ::dsl::FieldValue::Statements(match &self.#ident {
                        Some(v) => vec![::dsl::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Absent => None,
                        ::dsl::FieldValue::Statements(items) if items.is_empty() => None,
                        ::dsl::FieldValue::Statements(items) if items.len() == 1 => {
                            Some(<#inner as ::dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected 0 or 1 tagged values, found {other:?}"))),
                    }
                },
            ),
            FieldKind::RequiredStatements(inner) => (
                quote! { ::dsl::Shape::Statements(<#inner as ::dsl::DslVariants>::variants()) },
                quote! { ::dsl::FieldValue::Statements(vec![::dsl::DslVariants::to_named_record(self.#ident.as_ref())]) },
                quote! {
                    match value {
                        ::dsl::FieldValue::Statements(items) if items.len() == 1 => {
                            Box::new(<#inner as ::dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1)?)
                        }
                        other => return Err(::dsl::__rt::field_error(format!("expected exactly 1 tagged value, found {other:?}"))),
                    }
                },
            ),
        };

        // `#[dsl(block)]` on a field whose own `FieldKind` doesn't already imply `{ }` wrapping
        // (`VecBlockStatements` does that itself) — generically wraps whatever shape the match
        // above produced, e.g. turning a nested `#[derive(DslRecord)]` scalar field into a bare
        // `camera { x=0 y=0 zoom=1 }` line instead of a `camera=...` attribute.
        //
        // `FieldValue::Absent` (an `Option<T>` field's `None`) is deliberately NOT wrapped: an
        // empty `stroke { }` would reparse as "a record whose every field is absent", not "no
        // record at all" — `StrokeStyle`'s own non-optional fields would then fail with "expected
        // a 4-item Tuple, found Absent" instead of the field itself just being omitted, exactly
        // like an ordinary (non-block) optional field already is.
        let (shape_expr, to_value_expr, from_value_expr) = if *block {
            (
                quote! { ::dsl::Shape::Block(Box::new(#shape_expr)) },
                quote! {
                    match #to_value_expr {
                        ::dsl::FieldValue::Absent => ::dsl::FieldValue::Absent,
                        other => ::dsl::FieldValue::Block(Box::new(other)),
                    }
                },
                quote! {
                    match value {
                        ::dsl::FieldValue::Block(inner) => { let value = inner.as_ref(); #from_value_expr },
                        ::dsl::FieldValue::Absent => { let value = &::dsl::FieldValue::Absent; #from_value_expr },
                        other => return Err(::dsl::__rt::field_error(format!("expected Block, found {other:?}"))),
                    }
                },
            )
        } else {
            (shape_expr, to_value_expr, from_value_expr)
        };

        spec_exprs.push(quote! {
            ::dsl::FieldSpec::new(#id, #key, #shape_expr) #pos_expr #opt_expr #defines_expr
        });
        to_value_stmts.push(quote! {
            record.fields.insert(#id, #to_value_expr);
        });
        from_value_stmts.push(quote! {
            let #ident = {
                let value = record.get(#id).ok_or_else(|| ::dsl::__rt::field_error(format!("missing field '{}'", #key)))?;
                #from_value_expr
            };
        });
    }

    (spec_exprs, to_value_stmts, from_value_stmts, field_idents)
}
//#endregion 🔖️RecordCodegen

//#region 🔖️DslRecord
#[proc_macro_derive(DslRecord, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslRecord only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            // 🚫️async: E4 — its VALUE is stored as the fn pointer in `Shape::Record(Self::__dsl_spec)`
            // below (`DslField::shape` is itself E4 for the same reason — see R9), and `spec_exprs`
            // (built from the now-sync `DslField::shape`/`DslVariants::variants`) needs no executor.
            pub fn __dsl_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn __dsl_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::dsl::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
        }

        impl ::dsl::DslField for #name {
            // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Record(Self::__dsl_spec)
            }
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
        }
    };
    expanded.into()
}
//#endregion 🔖️DslRecord

//#region 🔖️DslArtifact
#[proc_macro_derive(DslArtifact, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_document(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let envelope_id = match container.id.clone().or_else(|| container.extension.clone()) {
        Some(id) => id,
        None => {
            return syn::Error::new_spanned(&input, "DslArtifact requires #[dsl(id = \"plugin.artifact\")] or #[dsl(extension = \"...\")]").to_compile_error().into();
        }
    };
    let extension_suffix = container.extension.as_deref().unwrap_or_else(|| envelope_id.rsplit('.').next().unwrap_or(&envelope_id));
    let envelope_id_lit = envelope_id.as_str();
    let extension_suffix_lit = extension_suffix;
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslArtifact only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            // 🚫️async: E4 — see `DslRecord`'s `__dsl_spec` above; identical reasoning.
            pub fn __dsl_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn __dsl_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn __dsl_from_record(record: &::dsl::RecordValue) -> Result<Self, ::store::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
            /// ✉️ Envelope constants for handcrafted ArtifactDsl/ArtifactPack wiring (P6: derive no longer emits those traits).
            pub const __DSL_ENVELOPE_ID: &'static str = #envelope_id_lit;
            pub const __DSL_EXTENSION: &'static str = #extension_suffix_lit;
        }

        // A document type can also be nested as an ordinary field (e.g. a "whole document
        // snapshot" operation variant), so it needs `DslField` too, not just `store::ArtifactDsl`.
        impl ::dsl::DslField for #name {
            // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Record(Self::__dsl_spec)
            }
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Record(self.__dsl_to_record())
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Record(record) => Self::__dsl_from_record(record).map_err(|e| e.message),
                    other => Err(format!("expected Record, found {other:?}")),
                }
            }
        }

    };
    expanded.into()
}
//#endregion 🔖️DslArtifact

//#region 🔖️DslDiff
/// @emoji 🧬️ W1 foundation of the `handcrafted-grammar-for-every-artifact` diff track (design ruling
/// B-R4): emits a `protocol::DiffCodec` impl from the SAME `RecordSpec`-generation machinery
/// `#[derive(DslRecord)]`/`#[derive(DslArtifact)]` already use — a diff is structurally just another
/// record, so this reuses `record_codegen` verbatim rather than reinventing field lowering. Unlike
/// `DslArtifact` there is no `EXTENSION`/file-extension concept (a diff is never opened as its own
/// file) and no `ArtifactPack` (the pack/binary side is `DiffCodec::encode_diff`/`decode_diff`
/// instead, routed through the same `store::pack_rt` the `ArtifactPack` impl above uses — every
/// crate that already derives an operation/document alongside its diff already depends on `store`).
#[proc_macro_derive(DslDiff, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_diff(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let container = parse_container_attrs(&input);
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslDiff only supports structs").to_compile_error().into();
    };
    let (spec_exprs, to_value_stmts, from_value_stmts, field_idents) = record_codegen(&data.fields);
    let keyword_expr = match &container.keyword {
        Some(k) => quote! { Some(#k.to_string()) },
        None => quote! { None },
    };
    let layout_expr = if container.lines_layout {
        quote! { ::dsl::RecordLayout::Lines }
    } else {
        quote! { ::dsl::RecordLayout::Inline }
    };

    let expanded = quote! {
        impl #name {
            // 🚫️async: E1 pure accessor consumed by the same `spec_exprs` shape `DslRecord`'s
            // `__dsl_spec` uses — E4-transitively sync, see R9.
            pub fn __dsl_diff_spec() -> ::dsl::RecordSpec {
                ::dsl::RecordSpec::new_owned(#keyword_expr, #layout_expr, vec![ #(#spec_exprs),* ])
            }
            pub fn __dsl_diff_to_record(&self) -> ::dsl::RecordValue {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts)*
                record
            }
            pub fn __dsl_diff_from_record(record: &::dsl::RecordValue) -> Result<Self, ::dsl::TextError> {
                #(#from_value_stmts)*
                Ok(Self { #(#field_idents),* })
            }
        }

        impl ::semio_framework_os_kernel::DiffCodec for #name {
            fn print_diff(&self) -> String {
                ::dsl::print(&self.__dsl_diff_to_record(), &Self::__dsl_diff_spec(), ::dsl::JoinMode::Inline)
            }
            fn parse_diff(line: &str) -> Result<Self, ::dsl::TextError> {
                let record = ::dsl::parse(line, &Self::__dsl_diff_spec(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;
                Self::__dsl_diff_from_record(&record)
            }
            fn encode_diff(&self) -> Result<Vec<u8>, ::semio_framework_os_kernel::ProtocolError> {
                ::store::pack_rt::encode_document(&Self::__dsl_diff_spec(), &self.__dsl_diff_to_record(), &::store::PackEncodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)
            }
            fn decode_diff(bytes: &[u8]) -> Result<Self, ::semio_framework_os_kernel::ProtocolError> {
                let (record, _report) = ::store::pack_rt::decode_document(bytes, &Self::__dsl_diff_spec(), &::store::PackDecodeOptions::default()).map_err(::semio_framework_os_kernel::ProtocolError::from)?;
                Self::__dsl_diff_from_record(&record).map_err(|error| ::semio_framework_os_kernel::ProtocolError::Malformed { what: "diff record", offset: 0, detail: error.to_string() })
            }
        }
    };
    expanded.into()
}
//#endregion 🔖️DslDiff

//#region 🔖️DslScalar
#[proc_macro_derive(DslScalar, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_scalar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslScalar only supports unit-variant enums").to_compile_error().into();
    };
    let mut variant_tags = Vec::new();
    let mut match_to_ordinal = Vec::new();
    let mut match_from_ordinal = Vec::new();
    for (ordinal, variant) in data.variants.iter().enumerate() {
        if !matches!(variant.fields, Fields::Unit) {
            return syn::Error::new_spanned(variant, "DslScalar only supports unit variants").to_compile_error().into();
        }
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let tag = attrs.key.unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let ordinal = ordinal as u32;
        variant_tags.push(quote! { (#tag.to_string(), #ordinal) });
        match_to_ordinal.push(quote! { #name::#variant_ident => #ordinal });
        match_from_ordinal.push(quote! { #ordinal => Ok(#name::#variant_ident) });
    }

    let expanded = quote! {
        impl ::dsl::DslField for #name {
            // 🚫️async: E4 — see `DslField::shape`'s tag on the trait.
            fn shape() -> ::dsl::Shape {
                ::dsl::Shape::Enum(vec![ #(#variant_tags),* ])
            }
            fn to_value(&self) -> ::dsl::FieldValue {
                ::dsl::FieldValue::Enum(match self { #(#match_to_ordinal),* })
            }
            fn from_value(value: &::dsl::FieldValue) -> Result<Self, String> {
                match value {
                    ::dsl::FieldValue::Enum(ordinal) => match *ordinal {
                        #(#match_from_ordinal,)*
                        other => Err(format!("unknown enum ordinal {other}")),
                    },
                    other => Err(format!("expected Enum, found {other:?}")),
                }
            }
        }
    };
    expanded.into()
}
//#endregion 🔖️DslScalar

//#region 🔖️DslOps
/// @emoji 🌿️ Builds the `impl ::dsl::DslVariants for #name` block shared by `DslEnum` (data-only
/// tagged enums, e.g. a recursive block tree) and `DslOps` (operation enums, which additionally get
/// `store::OpText` on top of this same `DslVariants` foundation).
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn dsl_variants_codegen(name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let mut variants_exprs = Vec::new();
    let mut to_named_arms = Vec::new();
    let mut from_named_arms = Vec::new();

    for variant in &data.variants {
        let attrs = parse_field_attrs(&variant.attrs);
        let variant_ident = variant.ident.clone();
        let keyword = attrs.key.clone().unwrap_or_else(|| to_kebab(&variant_ident.to_string()));
        let fields = &variant.fields;

        // A single-field tuple variant (`Shape(DrawShapeBody)`) delegates entirely to its inner
        // type's own `DslField` impl — its `RecordSpec` IS the inner type's, not a wrapper with one
        // positional field, so a body already declared with `#[derive(DslRecord)]` (its own keyword,
        // its own fields) prints/parses completely unchanged whether reached through the enum or on
        // its own.
        if let Fields::Unnamed(unnamed) = fields {
            if unnamed.unnamed.len() == 1 {
                let inner_ty = &unnamed.unnamed[0].ty;
                variants_exprs.push(quote! {
                    (#keyword.to_string(), ::dsl::__rt::newtype_variant_spec::<#inner_ty> as fn() -> ::dsl::RecordSpec)
                });
                to_named_arms.push(quote! {
                    #name::#variant_ident(inner) => (#keyword.to_string(), ::dsl::__rt::newtype_variant_to_record(inner))
                });
                from_named_arms.push(quote! {
                    #keyword => Ok(#name::#variant_ident(::dsl::__rt::newtype_variant_from_record::<#inner_ty>(record)?))
                });
                continue;
            }
        }

        let (spec_exprs, _to_value_stmts, from_value_stmts, field_idents) = record_codegen(fields);

        variants_exprs.push(quote! {
            (#keyword.to_string(), (|| ::dsl::RecordSpec::new_owned(Some(#keyword.to_string()), ::dsl::RecordLayout::Inline, vec![ #(#spec_exprs),* ])) as fn() -> ::dsl::RecordSpec)
        });

        // Build a per-variant to-record conversion using the field bindings from a `match` on
        // `self`, since (unlike `DslRecord`) the fields live inside an enum variant, not `self.field`.
        // A true unit variant (`Variant`, no braces at all) needs a bare match pattern — `Variant {}`
        // is only valid Rust for a variant that was itself declared with (empty) braces.
        let field_binds: Vec<proc_macro2::TokenStream> = field_idents.iter().map(|f| quote! { #f }).collect();
        let to_value_stmts_for_variant: Vec<proc_macro2::TokenStream> = record_codegen_to_value_from_bindings(fields);
        let is_unit = matches!(fields, Fields::Unit);
        let match_pattern = if is_unit {
            quote! { #name::#variant_ident }
        } else {
            quote! { #name::#variant_ident { #(#field_binds),* } }
        };
        let construct_expr = if is_unit {
            quote! { #name::#variant_ident }
        } else {
            quote! { #name::#variant_ident { #(#field_idents),* } }
        };
        to_named_arms.push(quote! {
            #match_pattern => {
                let mut record = ::dsl::RecordValue::default();
                #(#to_value_stmts_for_variant)*
                (#keyword.to_string(), record)
            }
        });
        from_named_arms.push(quote! {
            #keyword => {
                #(#from_value_stmts)*
                Ok(#construct_expr)
            }
        });
    }

    quote! {
        impl ::dsl::DslVariants for #name {
            // 🚫️async: E4 — see `DslVariants::variants`'s tag on the trait.
            fn variants() -> Vec<(String, fn() -> ::dsl::RecordSpec)> {
                vec![ #(#variants_exprs),* ]
            }
            fn to_named_record(&self) -> (String, ::dsl::RecordValue) {
                match self { #(#to_named_arms),* }
            }
            fn from_named_record(keyword: &str, record: &::dsl::RecordValue) -> Result<Self, ::dsl::TextError> {
                match keyword {
                    #(#from_named_arms,)*
                    other => Err(::dsl::__rt::field_error(format!("unknown keyword '{other}'"))),
                }
            }
        }
    }
}

#[proc_macro_derive(DslOps, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslOps only supports enums").to_compile_error().into();
    };
    let variants_impl = dsl_variants_codegen(&name, data);

    // P6: DslOps emits DslVariants only — OpText/OpBinary must be handcrafted per artifact.
    variants_impl.into()
}
//#endregion 🔖️DslOps

//#region 🔖️DslEnum
/// @emoji 🌳️ Tagged-record enum whose variants are plain data (a recursive block tree, a wire
/// node kind, ...) rather than a `Mutation` — implements `::dsl::DslVariants` only, so it can be
/// used inside `#[dsl(statements)]`/`#[dsl(statements, block)]` collection fields without also
/// gaining (and having to satisfy the bounds of) `store::OpText`.
#[proc_macro_derive(DslEnum, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "DslEnum only supports enums").to_compile_error().into();
    };
    dsl_variants_codegen(&name, data).into()
}
//#endregion 🔖️DslEnum

//#region 🔖️Mutations
/// @emoji 🗣️ `#[mutations(snapshot = ..., diff = ..., schema = "...")]` container attrs for
/// `#[derive(Mutations)]` — see that macro's doc.
#[derive(Default)]
struct MutationsAttrs {
    snapshot: Option<Type>,
    diff: Option<Type>,
    schema: Option<String>,
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
            } else { return Err(meta.error("unsupported mutations attribute")); }
            Ok(())
        })?;
    }
    Ok(out)
}

#[cfg(test)]
mod mutation_attrs_tests {
    use super::*;
    #[test]
    fn parses_mutation_fixture_exactly() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-attributes/🧫️fixtures/🔣️cases.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap().iter().filter(|case| case["attribute"].as_str().unwrap().contains("mutations")) {
            let attribute = case["attribute"].as_str().unwrap();
            let input: DeriveInput = syn::parse_str(&format!("{} #[derive(Mutations)] enum Probe {{ Item(Item) }}", attribute)).unwrap();
            let result = parse_mutations_attrs(&input);
            assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", attribute);
            if let Some(diagnostic) = case["diagnostic"].as_str() { assert!(result.err().unwrap().to_string().contains(diagnostic), "{}", attribute); }
        }
    }
}

/// 🧩️ Derives transparent delegation and full source-validated metadata from direct mutation leaves.
#[proc_macro_derive(Mutations, attributes(mutations))]
pub fn derive_mutations(input: TokenStream) -> TokenStream {
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
    let (Some(snapshot_ty), Some(diff_ty), Some(schema)) = (attrs.snapshot, attrs.diff, attrs.schema) else {
        return Err(syn::Error::new_spanned(input, "#[derive(Mutations)] requires #[mutations(snapshot = YourSnapshot, diff = YourDiff, schema = \"your.doc.schema\")]"));
    };
    let map_error = |error: String| syn::Error::new_spanned(input, error);
    let workspace_token = mutation_authority_workspace_token(&authority.workspace_root, &authority.taxonomy_path).map_err(map_error)?;
    let mutation_root = mutation_authority_relative(&authority.workspace_root, &authority.mutation_root).map_err(map_error)?;
    let taxonomy_path = mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path).map_err(map_error)?;
    let dependency_paths = [authority.workspace_root.join("nx.json"), authority.workspace_root.join("📋️project.json"), authority.taxonomy_path.clone()];
    let dependencies = dependency_paths.iter().map(|path| mutation_leaf_include_path(path)).collect::<Result<Vec<_>, _>>().map_err(map_error)?;
    let source_filename = &authority.source_filename;
    let descriptor_filename = &authority.descriptor_filename;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let aggregate_ty = quote! { #name #ty_generics };
    let mut diff_arms = Vec::new();
    let mut inverse_arms = Vec::new();
    let mut descriptor_arms = Vec::new();
    let mut semantics_arms = Vec::new();
    let mut label_arms = Vec::new();
    let mut target_arms = Vec::new();
    let mut may_emit_foreign_steps_arms = Vec::new();
    let mut foreign_steps_arms = Vec::new();
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
        descriptor_arms.push(quote! { Self::#variant_ident(_) => &<Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS[#index] });
        semantics_arms.push(quote! { Self::#variant_ident(_) => &#kind::SEMANTICS });
        label_arms.push(quote! { Self::#variant_ident(payload) => #kind::label(payload) });
        target_arms.push(quote! { Self::#variant_ident(payload) => #kind::target(payload) });
        may_emit_foreign_steps_arms.push(quote! { Self::#variant_ident(payload) => #kind::may_emit_foreign_steps(payload) });
        foreign_steps_arms.push(quote! { Self::#variant_ident(payload) => #kind::foreign_steps(payload, base) });
        kind_consts.push(quote! { #kind::SEMANTICS });
        leaf_descriptors.push(quote! { #leaf::DESCRIPTOR });
        leaf_checks.push(quote! {
            assert!(::semio_framework_os_kernel::str_eq(#kind::SEMANTICS.kind, #expected_kebab), "Mutations semantic kind must match its variant");
            assert!(::semio_framework_os_kernel::is_approved_verb(#kind::SEMANTICS.verb), "Mutations requires an approved semantic verb");
            assert!(::semio_framework_os_kernel::str_eq(#leaf::DESCRIPTOR.aggregate_variant, #expected_variant), "Mutations descriptor variant must match its wrapped leaf");
            assert!(::semio_framework_os_kernel::str_eq(#leaf::DESCRIPTOR.semantic_kind, #kind::SEMANTICS.kind), "Mutations descriptor and semantic kind must agree");
            match ::semio_framework_os_kernel::validate_mutation_leaf_source(&#leaf::DESCRIPTOR, &#leaf::PROVENANCE, &scope) {
                Ok(()) => (),
                Err(_) => panic!("Mutations leaf source must match its aggregate workspace and direct owner"),
            }
        });
        conversions.push(quote! {
            impl #impl_generics ::core::convert::From<#payload_ty> for #aggregate_ty #where_clause {
                fn from(payload: #payload_ty) -> Self {
                    let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                    Self::#variant_ident(payload)
                }
            }
        });
        register_calls.push(quote! {
            ::semio_framework_os_kernel::register_mutation_descriptor(
                ::semio_framework_os_kernel::MutationDescriptor::new(
                    ::semio_framework_os_kernel::SchemaId(format!("{}#{}", #schema, #kind::SEMANTICS.kind)),
                    ::semio_framework_os_kernel::SchemaVersion(1),
                    ::semio_framework_os_kernel::StateClass::Artifact,
                ).with_semantics(&#kind::SEMANTICS),
            );
        });
    }

    let register_fn_ident = syn::Ident::new(&format!("register_{}_descriptors", to_kebab(&name.to_string()).replace('-', "_")), name.span());
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
                let scope = ::semio_framework_os_kernel::MutationLeafSourceScope {
                    workspace_token: [#(#workspace_token),*], mutation_root: #mutation_root,
                    taxonomy_path: #taxonomy_path, source_filename: #source_filename, descriptor_filename: #descriptor_filename,
                };
                #(#leaf_checks)*
                let descriptors: &'static [::semio_framework_os_kernel::MutationLeafDescriptor] = &[#(#leaf_descriptors),*];
                match ::semio_framework_os_kernel::validate_mutation_leaf_descriptor_roster(#mutation_root, descriptors) {
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
            fn inverse(&self, base: &#snapshot_ty) -> Vec<Self> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#inverse_arms),* }
            }
            fn may_emit_foreign_steps(&self) -> bool {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#may_emit_foreign_steps_arms),* }
            }
            fn foreign_steps(&self, base: &#snapshot_ty) -> Vec<::semio_framework_os_kernel::ForeignStep> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#foreign_steps_arms),* }
            }
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
            fn label(&self) -> String {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#label_arms),* }
            }
            fn target(&self) -> Vec<String> {
                let _ = <Self as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
                match self { #(#target_arms),* }
            }
        }

        /// 🪪️ Registers the validated leaf roster during owner startup.
        pub fn #register_fn_ident #impl_generics () #where_clause {
            let _ = <#aggregate_ty as ::semio_framework_os_kernel::Mutation<#snapshot_ty>>::DESCRIPTORS;
            #(#register_calls)*
        }
    })
}
//#endregion 🔖️Mutations

//#region 🧪️MandatoryMutations
#[cfg(test)]
mod mandatory_mutations_tests {
    use super::*;

    #[test]
    fn expanded_aggregate_matches_neutral_contract_and_syn_ast() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧬️mandatory-mutations/🔣️cases.json")).unwrap();
        let leaf_fixture = mutation_source_authority_tests::fixture();
        let (workspace, cwd, leaf) = mutation_source_authority_tests::materialize("valid", &leaf_fixture);
        let source = leaf.parent().unwrap().parent().unwrap().join("🦀️.rs");
        fs::write(&source, "pub enum Probe {}").unwrap();
        let authority = mutation_aggregate_source_authority(&source, &cwd).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let input: DeriveInput = syn::parse_str(case["source"].as_str().unwrap()).unwrap();
            let result = expand_mutations(&input, &authority);
            assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
            let Ok(tokens) = result else { continue; };
            let syntax: syn::File = syn::parse2(tokens.clone()).unwrap();
            let implementations: Vec<&syn::ItemImpl> = syntax.items.iter().filter_map(|item| if let syn::Item::Impl(item) = item { Some(item) } else { None }).collect();
            let mutation = implementations.iter().find(|item| item.trait_.as_ref().unwrap().1.segments.last().unwrap().ident == "Mutation").unwrap();
            let names: Vec<String> = mutation.items.iter().filter_map(|item| match item {
                syn::ImplItem::Const(item) => Some(item.ident.to_string()),
                syn::ImplItem::Fn(item) => Some(item.sig.ident.to_string()),
                _ => None,
            }).collect();
            assert!(names.iter().any(|name| name == "DESCRIPTORS"));
            assert!(names.iter().any(|name| name == "descriptor"));
            let conversions = implementations.iter().filter(|item| item.trait_.as_ref().unwrap().1.segments.last().unwrap().ident == "From").count();
            assert_eq!(conversions, case["leaves"].as_u64().unwrap() as usize);
            assert_eq!(!mutation.generics.params.is_empty(), case["generic"].as_bool().unwrap());
            if case["generic"] == true { assert!(mutation.generics.where_clause.is_some()); }
            let expanded = tokens.to_string();
            assert_eq!(expanded.matches("validate_mutation_leaf_source").count(), conversions);
            assert_eq!(expanded.matches("validate_mutation_leaf_descriptor_roster").count(), 1);
            assert_eq!(expanded.matches("include_str !").count(), 3);
            assert!(!expanded.contains("include !"));
            assert!(expanded.contains("MutationLeaf > :: DESCRIPTOR"));
            assert!(expanded.contains("MutationLeaf > :: PROVENANCE"));
        }
        assert!(workspace.exists());
    }
}
//#endregion 🧪️MandatoryMutations

//#region 🔖️CompositeMutation
/// @emoji 🌉️ `#[composite(snapshot = ..., op = ...)]` container attrs for
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
mod composite_attrs_tests {
    use super::*;
    #[test]
    fn parses_composite_fixture_exactly() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-attributes/🧫️fixtures/🔣️cases.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap().iter().filter(|case| case["attribute"].as_str().unwrap().contains("composite")) {
            let attribute = case["attribute"].as_str().unwrap();
            let input: DeriveInput = syn::parse_str(&format!("{} #[derive(CompositeMutation)] struct Probe;", attribute)).unwrap();
            let result = parse_composite_attrs(&input);
            assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", attribute);
            if let Some(diagnostic) = case["diagnostic"].as_str() { assert!(result.err().unwrap().to_string().contains(diagnostic), "{}", attribute); }
        }
    }
}

/// @emoji 🌉️ Wires a composite mutation kind's delegating `::semio_framework_os_kernel::MutationKind` impl from its
/// handcrafted `::semio_framework_os_kernel::CompositeMutationKind` impl — `#[composite(snapshot = YourSnapshot, op =
/// YourOpEnum)]` on the payload struct that already `impl CompositeMutationKind<YourSnapshot,
/// YourOpEnum> for` itself. `diff`/`inverse`/`foreign_steps` delegate to the free
/// `::semio_framework_os_kernel::fold_plan_diff`/`fold_plan_inverse`/`plan_foreign_steps` helpers — deliberately NOT
/// a blanket `impl<T: CompositeMutationKind> MutationKind for T`, which coherence rejects against
/// the ~200 concrete `impl MutationKind` in the tree (see
/// `.🦑️repo/🎫️tickets/26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/📋️contract-freeze.md`
/// §1). Emits the same kind/verb `const _: () = assert!(..)` checks `#[derive(Mutations)]` emits,
/// checked against the struct's OWN kebab name (a composite kind is never wrapped in an enum
/// variant the way a handcrafted `MutationKind` payload is).
#[proc_macro_derive(CompositeMutation, attributes(composite))]
// 🚫️async: E3 proc-macro entry
pub fn derive_composite_mutation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let attrs = match parse_composite_attrs(&input) { Ok(attrs) => attrs, Err(error) => return error.to_compile_error().into() };
    let (Some(snapshot_ty), Some(op_ty)) = (attrs.snapshot, attrs.op) else {
        return syn::Error::new_spanned(&input, "#[derive(CompositeMutation)] requires #[composite(snapshot = YourSnapshot, op = YourOp)]").to_compile_error().into();
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
                ::semio_framework_os_kernel::fold_plan_diff(self, base)
            }
            fn inverse(&self, base: &#snapshot_ty) -> Vec<#op_ty> {
                ::semio_framework_os_kernel::fold_plan_inverse(self, base)
            }
            fn label(&self) -> String {
                ::semio_framework_os_kernel::CompositeMutationKind::label(self)
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
    expanded.into()
}
//#endregion 🔖️CompositeMutation

//#region 🔖️VariantHelpers
/// @emoji 🔡️ Converts a Rust identifier (`PascalCase`/`camelCase`/`snake_case`, any mix) into
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

/// @emoji 🏗️ Like the `to_value` half of `record_codegen`, but reading from bare local bindings
/// (`ident`) instead of `self.ident` — what a `match self { Variant { fields... } => ... }` arm
/// needs, since enum variant fields aren't reached through `self.field` syntax.
// 🚫️async: E1 pure accessor consumed by external-trait/E3 proc-macro entry points — see R9
fn record_codegen_to_value_from_bindings(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    let plans = plan_fields(fields);
    plans
        .iter()
        .map(|plan| {
            let FieldPlan { ident, id, kind, block, .. } = plan;
            let to_value_expr: proc_macro2::TokenStream = match kind {
                FieldKind::Scalar => quote! { ::dsl::DslField::to_value(#ident) },
                FieldKind::Bytes64 => quote! { ::dsl::FieldValue::Bytes64(#ident.clone()) },
                FieldKind::OptionScalar(_) => quote! {
                    match #ident {
                        Some(v) => ::dsl::DslField::to_value(v),
                        None => ::dsl::FieldValue::Absent,
                    }
                },
                FieldKind::VecList(_) | FieldKind::VecTable(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::List(__items)
                    }
                },
                FieldKind::VecTuple(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::dsl::DslField::to_value(v)); }
                        ::dsl::FieldValue::Tuple(__items)
                    }
                },
                FieldKind::VecStatements(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::dsl::DslVariants::to_named_record(v)); }
                        ::dsl::FieldValue::Statements(__items)
                    }
                },
                FieldKind::VecBlockStatements(_) => quote! {
                    {
                        let mut __items = Vec::with_capacity(#ident.len());
                        for v in #ident.iter() { __items.push(::dsl::DslVariants::to_named_record(v)); }
                        ::dsl::FieldValue::Block(Box::new(::dsl::FieldValue::Statements(__items)))
                    }
                },
                FieldKind::MapField(_) => quote! {
                    {
                        let mut __entries = Vec::with_capacity(#ident.len());
                        for (k, v) in #ident.iter() { __entries.push((k.clone(), ::dsl::DslField::to_value(v))); }
                        ::dsl::FieldValue::Map(__entries)
                    }
                },
                FieldKind::OptionStatements(_) => quote! {
                    ::dsl::FieldValue::Statements(match #ident {
                        Some(v) => vec![::dsl::DslVariants::to_named_record(v)],
                        None => vec![],
                    })
                },
                FieldKind::RequiredStatements(_) => quote! { ::dsl::FieldValue::Statements(vec![::dsl::DslVariants::to_named_record(#ident.as_ref())]) },
            };
            let to_value_expr = if *block {
                quote! {
                    match #to_value_expr {
                        ::dsl::FieldValue::Absent => ::dsl::FieldValue::Absent,
                        other => ::dsl::FieldValue::Block(Box::new(other)),
                    }
                }
            } else {
                to_value_expr
            };
            quote! { record.fields.insert(#id, #to_value_expr); }
        })
        .collect()
}
//#endregion 🔖️VariantHelpers
