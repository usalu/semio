use std::path::{Path, PathBuf};

pub const EXAMPLES_DIR_NAME: &str = "📚️examples";
pub const ASSETS_DIR_NAME: &str = "🖼️assets";

/// @emoji 🏠️ Ascends from `CARGO_MANIFEST_DIR` to the repo root (`nx.json`).
pub async fn repo_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("nx.json").is_file() {
            return dir;
        }
        if !dir.pop() {
            panic!("could not locate repo root (nx.json) ascending from {}", env!("CARGO_MANIFEST_DIR"));
        }
    }
}

async fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            Box::pin(collect_files(&path, out)).await;
        } else {
            out.push(path);
        }
    }
}

/// @emoji 🔎 Finds the first `.semio` under an artifact's examples whose file name ends with `suffix`
/// (e.g. `.dsl.semio`, `.pack.semio`). Assets-first, then legacy walk.
pub async fn find_example_asset(artifact_dir: &Path, suffix: &str) -> Option<PathBuf> {
    let examples = artifact_dir.join(EXAMPLES_DIR_NAME);
    if !examples.is_dir() {
        return None;
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    let entries = match std::fs::read_dir(&examples) {
        Ok(entries) => entries,
        Err(_) => return None,
    };
    for entry in entries.flatten() {
        let slug = entry.path();
        if !slug.is_dir() {
            continue;
        }
        let assets = slug.join(ASSETS_DIR_NAME);
        if assets.is_dir() {
            collect_files(&assets, &mut candidates).await;
        } else {
            collect_files(&slug, &mut candidates).await;
        }
    }
    candidates.retain(|path| path.file_name().and_then(|n| n.to_str()).is_some_and(|name| name.ends_with(suffix)));
    // Prefer the largest match so handcrafted fixtures win over 64-byte / preamble-only stubs
    // that still sit beside them under legacy placeholder slug dirs during migration.
    candidates.sort_by(|a, b| {
        let size = |path: &PathBuf| std::fs::metadata(path).map_or(0, |m| m.len());
        size(b).cmp(&size(a)).then_with(|| a.cmp(b))
    });
    candidates.into_iter().next()
}

/// @emoji 📄️ Reads UTF-8 text for the first matching example asset under `artifact_dir`.
pub async fn read_example_asset_text(artifact_dir: &Path, suffix: &str) -> Option<String> {
    let path = find_example_asset(artifact_dir, suffix).await?;
    std::fs::read_to_string(&path).ok()
}

/// @emoji 📒️ Reads bytes for the first matching example asset under `artifact_dir`.
pub async fn read_example_asset_bytes(artifact_dir: &Path, suffix: &str) -> Option<Vec<u8>> {
    let path = find_example_asset(artifact_dir, suffix).await?;
    std::fs::read(&path).ok()
}

/// @emoji 🗺️ Resolves `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>`.
pub async fn artifact_dir(plugin: &str, artifact: &str) -> PathBuf {
    repo_root().await.join("✏️s").join("🔌️plugins").join(plugin).join("🗿️artifacts").join(artifact)
}
