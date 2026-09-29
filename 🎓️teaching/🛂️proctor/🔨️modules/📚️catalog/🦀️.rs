//! 📚️ The catalog a proctor serves: the catalog `🔣️.json`, every quiz it lists (resolved relative
//! to the catalog file), each quiz's revision (lowercase hex SHA-256 of the quiz file bytes as
//! loaded) and the catalog fingerprint the projections were built against.
//!
//! A catalog with any structural or semantic issue (`quiz::catalog_issues`, plus every quiz's
//! `quiz::quiz_issues` nested under `/quizzes/<index>`) is refused as a whole: a proctor never
//! boots on a partially valid catalog.
//!
//! @see ../../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json — `Catalog` and `Quiz`
//! @see ../../../../🧰️framework/🔨️modules/🔏️hash/🦀️.rs — the owned SHA-256

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use quiz::{Catalog, CatalogView, LoadedQuiz, Quiz};
use semio_framework_hash::{sha256_hex, Sha256};

/// 📄️ One quiz of the catalog as loaded: the path the catalog lists, the file it resolved to, the
/// parsed quiz and its revision.
#[derive(Clone, Debug, PartialEq)]
pub struct CatalogEntry {
    pub entry: String,
    pub source: PathBuf,
    pub quiz: Quiz,
    pub revision: String,
}

/// 📚️ A validated catalog with every quiz it lists.
#[derive(Clone, Debug, PartialEq)]
pub struct LoadedCatalog {
    pub source: PathBuf,
    pub catalog: Catalog,
    pub entries: Vec<CatalogEntry>,
    pub current: BTreeMap<String, LoadedQuiz>,
    pub view: CatalogView,
    pub fingerprint: String,
}

/// 🧯️ Why a catalog was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CatalogError {
    Unreadable { path: PathBuf, detail: String },
    Malformed { path: PathBuf, detail: String },
    Misplaced { entry: String },
    Invalid { issues: Vec<CatalogIssue> },
}

/// 📍️ One validation issue: a JSON pointer into the catalog (issues of a quiz are nested under
/// `/quizzes/<index>`), its code, and the quiz file the pointer lies in, if any.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CatalogIssue {
    pub path: String,
    pub code: String,
    pub quiz: Option<String>,
}

impl fmt::Display for CatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unreadable { path, detail } => write!(formatter, "cannot read {}: {detail}", path.display()),
            Self::Malformed { path, detail } => write!(formatter, "{} is not a catalog/quiz document: {detail}", path.display()),
            Self::Misplaced { entry } => write!(formatter, "quiz path {entry:?} must be relative to the catalog file"),
            Self::Invalid { issues } => write!(formatter, "{} catalog issue(s): {}", issues.len(), issues.iter().map(|issue| format!("{} {}", issue.path, issue.code)).collect::<Vec<_>>().join(", ")),
        }
    }
}

impl std::error::Error for CatalogError {}

impl LoadedCatalog {
    /// 🆔️ The catalog id — the tenant and scope of every command and query this proctor serves.
    pub fn id(&self) -> &str {
        &self.catalog.id
    }

    /// 🔎️ The catalog entry of the quiz with `id`.
    pub fn entry(&self, id: &str) -> Option<&CatalogEntry> {
        self.entries.iter().find(|entry| entry.quiz.id == id)
    }

    /// 🗺️ Quiz id → `(quiz, revision)`, the shape the quiz lifecycle decides against.
    pub fn current(&self) -> &BTreeMap<String, LoadedQuiz> {
        &self.current
    }

    /// 🪟️ The solution-free catalog view clients render and every learner view is totalled over.
    pub fn view(&self) -> &CatalogView {
        &self.view
    }
}

/// 📥️ Load and validate the catalog at `path` and every quiz it lists.
pub fn load_catalog(path: &Path) -> Result<LoadedCatalog, CatalogError> {
    let bytes = read(path)?;
    let catalog: Catalog = parse(path, &bytes)?;
    let base = path.parent().unwrap_or(Path::new("."));
    let mut fingerprint = Sha256::new();
    fingerprint.update(b"semio/teaching/proctor/catalog/v1\0");
    fingerprint.update(&bytes);
    let mut entries = Vec::with_capacity(catalog.quizzes.len());
    for entry in &catalog.quizzes {
        let relative = Path::new(entry);
        if relative.is_absolute() || relative.has_root() {
            return Err(CatalogError::Misplaced { entry: entry.clone() });
        }
        let source = base.join(relative);
        let quiz_bytes = read(&source)?;
        let quiz: Quiz = parse(&source, &quiz_bytes)?;
        let revision = sha256_hex(&quiz_bytes);
        fingerprint.update(entry.as_bytes());
        fingerprint.update(b"\0");
        fingerprint.update(revision.as_bytes());
        entries.push(CatalogEntry { entry: entry.clone(), source, quiz, revision });
    }
    let listed: Vec<Quiz> = entries.iter().map(|entry: &CatalogEntry| entry.quiz.clone()).collect();
    let nested = listed.iter().enumerate().flat_map(|(index, document)| quiz::quiz_issues(document).into_iter().map(move |issue| (format!("/quizzes/{index}{}", issue.path), issue.code)));
    let mut found: Vec<(String, quiz::IssueCode)> = quiz::catalog_issues(&catalog, &listed).into_iter().map(|issue| (issue.path, issue.code)).chain(nested).collect();
    found.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.as_str().cmp(right.1.as_str())));
    found.dedup();
    let issues: Vec<CatalogIssue> = found.into_iter().map(|(path, code)| CatalogIssue { quiz: quiz_entry(&catalog, &path), path, code: code.as_str().to_string() }).collect();
    if !issues.is_empty() {
        return Err(CatalogError::Invalid { issues });
    }
    let current = entries.iter().map(|entry| (entry.quiz.id.clone(), LoadedQuiz { quiz: entry.quiz.clone(), revision: entry.revision.clone() })).collect();
    let view = quiz::catalog_view(&catalog, &listed);
    Ok(LoadedCatalog { source: path.to_path_buf(), catalog, entries, current, view, fingerprint: semio_framework_hash::hex_lower(&fingerprint.finalize()) })
}

fn quiz_entry(catalog: &Catalog, pointer: &str) -> Option<String> {
    let index: usize = pointer.strip_prefix("/quizzes/")?.split('/').next()?.parse().ok()?;
    catalog.quizzes.get(index).cloned()
}

fn read(path: &Path) -> Result<Vec<u8>, CatalogError> {
    std::fs::read(path).map_err(|error| CatalogError::Unreadable { path: path.to_path_buf(), detail: error.to_string() })
}

fn parse<T: serde::de::DeserializeOwned>(path: &Path, bytes: &[u8]) -> Result<T, CatalogError> {
    serde_json::from_slice(bytes).map_err(|error| CatalogError::Malformed { path: path.to_path_buf(), detail: error.to_string() })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
