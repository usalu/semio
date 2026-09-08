
use std::path::{Path, PathBuf};

const EXAMPLES_DIR_NAME: &str = "📚️examples";
const ASSETS_DIR_NAME: &str = "🖼️assets";
// 🎓️ P2-PW: local copy of `m5_auto_discovery::STANDARDS_DIR` — that constant is private to its
// own sibling module and this module intentionally stays free-standing (same reasoning as
// `EXAMPLES_DIR_NAME`/`ASSETS_DIR_NAME` above already being local copies rather than cross-module
// imports); both name the same literal `🏅️standards` directory segment by construction.
const STANDARDS_DIR: &str = "🏅️standards";

/// 🏠️ Ascends from `CARGO_MANIFEST_DIR` looking for `nx.json`.
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

async fn skip_dir_name(name: &str) -> bool {
    name == "node_modules" || name == "target" || name.starts_with('.') || name == "🦑️repo"
}

async fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if skip_dir_name(&name).await {
                continue;
            }
            Box::pin(collect_files(&path, out)).await;
        } else if path.is_file() {
            out.push(path);
        }
    }
}

async fn name_matches_kind(path: &Path, kind_suffix: &str) -> bool {
    path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(kind_suffix))
}

/// 🖼️ Finds one example `.semio` under `examples_dir` (a `📚️examples` directory) matching
/// `kind_suffix` (e.g. `.dsl.semio`, `.pack.semio`, `.spr.semio`). Assets-dir hits win over
/// legacy nested hits. Extracted from the old single-slot `find_example_semio` so the
/// (artifact, standard)-aware wrapper below can try more than one `examples_dir` candidate.
async fn find_example_semio_under(examples: &Path, kind_suffix: &str) -> Option<PathBuf> {
    if !examples.is_dir() {
        return None;
    }
    let mut preferred = Vec::new();
    let mut fallback = Vec::new();
    let entries = match std::fs::read_dir(examples) {
        Ok(entries) => entries,
        Err(_) => return None,
    };
    for entry in entries.flatten() {
        let slug = entry.path();
        if !slug.is_dir() {
            continue;
        }
        let assets = slug.join(ASSETS_DIR_NAME);
        let mut files = Vec::new();
        if assets.is_dir() {
            collect_files(&assets, &mut files).await;
            for file in files {
                if name_matches_kind(&file, kind_suffix).await {
                    preferred.push(file);
                }
            }
        } else {
            collect_files(&slug, &mut files).await;
            for file in files {
                if name_matches_kind(&file, kind_suffix).await {
                    fallback.push(file);
                }
            }
        }
    }
    preferred.sort();
    fallback.sort();
    preferred.into_iter().next().or_else(|| fallback.into_iter().next())
}

/// 🖼️ Finds one example `.semio` for `artifact_rel` (repo-relative artifact dir) matching
/// `kind_suffix` (e.g. `.dsl.semio`, `.pack.semio`, `.spr.semio`).
///
/// 🎓️ P2-PW m5 fixture-slot widening: when `standard` is `Some`, first tries the PER-STANDARD
/// fixture slot at `<artifact_rel>/🏅️standards/<standard>/📚️examples/...` — real and shipped for
/// any multi-standard artifact whose standards each landed their OWN fixtures there (gif 87a/89a,
/// pdf 1.4/1.7; see `p2-fg2-closer-report.md`/`p2-fg3-closer-report.md` for the exact citations
/// this widening fixes). Falls back to the original artifact-level slot
/// (`<artifact_rel>/📚️examples/...`) whenever the per-standard slot doesn't exist or has no
/// matching fixture, so every single-standard artifact (the overwhelming majority, and every
/// non-stdio caller which never has a `standard`) keeps resolving byte-for-byte as before —
/// additive/widening, never a narrowing of what used to resolve.
pub async fn find_example_semio(artifact_rel: &str, standard: Option<&str>, kind_suffix: &str) -> Option<PathBuf> {
    if let Some(standard) = standard {
        let per_standard = repo_root().await.join(artifact_rel).join(STANDARDS_DIR).join(standard).join(EXAMPLES_DIR_NAME);
        if let Some(found) = find_example_semio_under(&per_standard, kind_suffix).await {
            return Some(found);
        }
    }
    find_example_semio_under(&repo_root().await.join(artifact_rel).join(EXAMPLES_DIR_NAME), kind_suffix).await
}

/// 📄️ Reads example fixture text; `None` soft-skips the pilot when missing mid-migration.
pub async fn read_example_text(artifact_rel: &str, standard: Option<&str>, kind_suffix: &str) -> Option<String> {
    let path = find_example_semio(artifact_rel, standard, kind_suffix).await?;
    std::fs::read_to_string(&path).ok()
}

/// 🎒️ Reads example binary/text bytes; `None` soft-skips the pilot when missing mid-migration.
pub async fn read_example_bytes(artifact_rel: &str, standard: Option<&str>, kind_suffix: &str) -> Option<Vec<u8>> {
    let path = find_example_semio(artifact_rel, standard, kind_suffix).await?;
    std::fs::read(&path).ok()
}
