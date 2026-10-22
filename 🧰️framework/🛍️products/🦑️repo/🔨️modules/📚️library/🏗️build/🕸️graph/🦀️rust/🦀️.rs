use std::sync::OnceLock;

/// 🗂️ Resolves the neutral graph publication from its declared partition, mounted namespace and filename.
pub fn graph_file() -> &'static str {
    static PATH: OnceLock<String> = OnceLock::new();
    PATH.get_or_init(|| {
        let publication: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/📣️publication.json")).expect("graph publication declaration");
        let policy: serde_json::Value = serde_json::from_str(include_str!("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json")).expect("cache partition declaration");
        let image: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json")).expect("native image declaration");
        let fields = &publication["properties"];
        let partition = fields["partition"]["const"].as_str().expect("graph partition identity");
        format!("{}/{}/{}", policy["partitions"][partition]["directory"].as_str().expect("graph partition directory"), image["definitions"]["ImageProtocol"]["const"]["mountDirectory"].as_str().expect("graph mount directory"), fields["graph"]["const"].as_str().expect("graph publication filename"))
    }).as_str()
}

/// ⚖️ Reads the neutral tool and policy authority definitions without a language-specific default.
fn build_policy_contract() -> &'static serde_json::Value {
    static CONTRACT: OnceLock<serde_json::Value> = OnceLock::new();
    CONTRACT.get_or_init(|| serde_json::from_str(include_str!("../../🧬️schema/🔣️.json")).expect("neutral build policy contract"))
}

/// 🗂️ Resolves the workspace policy from its neutral schema-owned authority path.
pub fn build_policy_path(root: &std::path::Path) -> std::path::PathBuf {
    root.join(build_policy_contract()["$defs"]["BuildPolicyAuthority"]["const"]["path"].as_str().expect("neutral build policy path"))
}

/// 🧭️ Validates a selected backend against the shared neutral tool definition.
pub fn is_build_tool(value: &str) -> bool {
    build_backend_registry()["backends"].as_array().is_some_and(|rows| rows.iter().any(|row| row["id"].as_str() == Some(value)))
}

/// 🔌️ Reads the authored registry of build backends, the only place that lists them.
fn build_backend_registry() -> &'static serde_json::Value {
    static REGISTRY: OnceLock<serde_json::Value> = OnceLock::new();
    REGISTRY.get_or_init(|| serde_json::from_str(include_str!("../../🔌️backends/📋️registry/🔣️.json")).expect("neutral build backend registry"))
}

/// 🚧️ Rejects Windows policy reparse attributes through the neutral authority contract.
pub fn is_build_policy_reparse(attributes: u32) -> bool {
    attributes as u64 & build_policy_contract()["$defs"]["BuildPolicyAuthority"]["const"]["windows"]["reparseAttribute"].as_u64().expect("policy reparse attribute") != 0
}

/// 📖️ Reads the actual workspace policy once through a bounded stable regular-file descriptor.
pub fn default_build_tool(root: &std::path::Path) -> std::io::Result<String> {
    use std::io::Read;
    let authority = &build_policy_contract()["$defs"]["BuildPolicyAuthority"]["const"];
    let maximum = authority["maximumBytes"].as_u64().ok_or_else(|| std::io::Error::other("neutral build policy bound required"))?;
    let path = build_policy_path(root);
    if !root.is_absolute() || path.to_string_lossy().encode_utf16().count() > 256 { return Err(std::io::Error::other("actual bounded workspace policy path required")); }
    let mut ancestor = path.parent();
    while let Some(directory) = ancestor {
        let metadata = std::fs::symlink_metadata(directory)?;
        if !metadata.is_dir() || metadata.is_symlink() { return Err(std::io::Error::other("ordinary workspace policy ancestry required")); }
        #[cfg(windows)] { use std::os::windows::fs::MetadataExt; if is_build_policy_reparse(metadata.file_attributes()) { return Err(std::io::Error::other("workspace policy reparse ancestry refused")); } }
        if directory == root { break; }
        ancestor = directory.parent();
    }
    let before = std::fs::symlink_metadata(&path)?;
    if !before.is_file() || before.is_symlink() || before.len() > maximum { return Err(std::io::Error::other("bounded workspace build policy required")); }
    #[cfg(windows)] let file = {
        use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
        if is_build_policy_reparse(before.file_attributes()) { return Err(std::io::Error::other("workspace policy reparse refused")); }
        let options = &authority["windows"];
        std::fs::OpenOptions::new().read(true).share_mode(options["shareMode"].as_u64().unwrap().try_into().unwrap()).custom_flags(options["openFlags"].as_u64().unwrap().try_into().unwrap()).open(&path)?
    };
    #[cfg(not(windows))] let file = std::fs::File::open(&path)?;
    if !file.metadata()?.is_file() { return Err(std::io::Error::other("ordinary workspace policy descriptor required")); }
    #[cfg(windows)] { use std::os::windows::fs::MetadataExt; if is_build_policy_reparse(file.metadata()?.file_attributes()) { return Err(std::io::Error::other("workspace policy reparse descriptor refused")); } }
    let stamp = |value: &std::fs::Metadata| {
        #[cfg(unix)] { use std::os::unix::fs::MetadataExt; format!("{}:{}:{}:{}:{}:{}:{}:{}:{}", value.dev(), value.ino(), value.mode(), value.nlink(), value.len(), value.mtime(), value.mtime_nsec(), value.ctime(), value.ctime_nsec()) }
        #[cfg(windows)] { use std::os::windows::fs::MetadataExt; format!("{}:{}:{}:{}", value.file_attributes(), value.creation_time(), value.last_write_time(), value.file_size()) }
        #[cfg(not(any(unix, windows)))] { format!("{}:{:?}:{:?}", value.len(), value.modified(), value.created()) }
    };
    if stamp(&file.metadata()?) != stamp(&before) { return Err(std::io::Error::other("workspace build policy descriptor changed")); }
    let mut bytes = Vec::new();
    (&file).take(maximum + 1).read_to_end(&mut bytes)?;
    let named = std::fs::symlink_metadata(&path)?;
    if bytes.len() as u64 > maximum || !named.is_file() || named.is_symlink() || stamp(&file.metadata()?) != stamp(&before) || stamp(&named) != stamp(&before) { return Err(std::io::Error::other("workspace build policy changed")); }
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))?;
    if value["version"].as_f64() != build_policy_contract()["$defs"]["BuildPolicy"]["properties"]["version"]["const"].as_f64() { return Err(std::io::Error::other("workspace build policy version required")); }
    build_backend_registry()["backends"].as_array().and_then(|rows| rows.iter().find(|row| row["default"].as_bool() == Some(true))).and_then(|row| row["id"].as_str()).filter(|tool| is_build_tool(tool)).map(str::to_string).ok_or_else(|| std::io::Error::other("neutral default build backend required"))
}

/// 🚪️ Resolves only schema-owned neutral build launcher paths in the actual workspace.
pub fn build_launcher_paths(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    build_policy_contract()["$defs"]["BuildPolicyAuthority"]["const"]["launchers"].as_array().expect("neutral build launchers").iter().map(|path| root.join(path.as_str().expect("neutral build launcher path"))).collect()
}
