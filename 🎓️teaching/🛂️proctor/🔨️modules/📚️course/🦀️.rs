//! 📚️ The course a proctor serves: the course `🔣️.json`, every quiz it lists (resolved relative
//! to the course file), each quiz's revision (lowercase hex SHA-256 of the quiz file bytes as
//! loaded) and the course fingerprint the projections were built against.
//!
//! A course with any structural or semantic issue (`course::course_issues`, plus every quiz's
//! `course::quiz_issues` nested under `/quizzes/<index>`) is refused as a whole: a proctor never
//! boots on a partially valid course.
//!
//! @see ../../../../🧰️framework/🛍️products/📚️course/🧬️schema/🔣️.json — `Course` and `Quiz`
//! @see ../../../../🧰️framework/🔨️modules/🔏️hash/🦀️.rs — the owned SHA-256

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use course::{Course, CourseView, LoadedQuiz, Quiz};
use semio_framework_hash::{sha256_hex, Sha256};

/// 📄️ One quiz of the course as loaded: the path the course lists, the file it resolved to, the
/// parsed quiz and its revision.
#[derive(Clone, Debug, PartialEq)]
pub struct CourseEntry {
    pub entry: String,
    pub source: PathBuf,
    pub quiz: Quiz,
    pub revision: String,
}

/// 📚️ A validated course with every quiz it lists.
#[derive(Clone, Debug, PartialEq)]
pub struct LoadedCourse {
    pub source: PathBuf,
    pub course: Course,
    pub entries: Vec<CourseEntry>,
    pub current: BTreeMap<String, LoadedQuiz>,
    pub view: CourseView,
    pub fingerprint: String,
}

/// 🧯️ Why a course was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CourseError {
    Unreadable { path: PathBuf, detail: String },
    Malformed { path: PathBuf, detail: String },
    Misplaced { entry: String },
    Invalid { issues: Vec<CourseIssue> },
}

/// 📍️ One validation issue: a JSON pointer into the course (issues of a quiz are nested under
/// `/quizzes/<index>`), its code, and the quiz file the pointer lies in, if any.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CourseIssue {
    pub path: String,
    pub code: String,
    pub quiz: Option<String>,
}

impl fmt::Display for CourseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unreadable { path, detail } => write!(formatter, "cannot read {}: {detail}", path.display()),
            Self::Malformed { path, detail } => write!(formatter, "{} is not a course/quiz document: {detail}", path.display()),
            Self::Misplaced { entry } => write!(formatter, "quiz path {entry:?} must be relative to the course file"),
            Self::Invalid { issues } => write!(formatter, "{} course issue(s): {}", issues.len(), issues.iter().map(|issue| format!("{} {}", issue.path, issue.code)).collect::<Vec<_>>().join(", ")),
        }
    }
}

impl std::error::Error for CourseError {}

impl LoadedCourse {
    /// 🆔️ The course id — the tenant and scope of every command and query this proctor serves.
    pub fn id(&self) -> &str {
        &self.course.id
    }

    /// 🔎️ The course entry of the quiz with `id`.
    pub fn entry(&self, id: &str) -> Option<&CourseEntry> {
        self.entries.iter().find(|entry| entry.quiz.id == id)
    }

    /// 🗺️ Quiz id → `(quiz, revision)`, the shape the quiz lifecycle decides against.
    pub fn current(&self) -> &BTreeMap<String, LoadedQuiz> {
        &self.current
    }

    /// 🪟️ The solution-free course view clients render and every learner view is totalled over.
    pub fn view(&self) -> &CourseView {
        &self.view
    }
}

/// 📥️ Load and validate the course at `path` and every quiz it lists.
pub fn load_course(path: &Path) -> Result<LoadedCourse, CourseError> {
    let bytes = read(path)?;
    let course: Course = parse(path, &bytes)?;
    let base = path.parent().unwrap_or(Path::new("."));
    let mut fingerprint = Sha256::new();
    fingerprint.update(b"semio/teaching/proctor/course/v1\0");
    fingerprint.update(&bytes);
    let mut entries = Vec::with_capacity(course.quizzes.len());
    for entry in &course.quizzes {
        let relative = Path::new(entry);
        if relative.is_absolute() || relative.has_root() {
            return Err(CourseError::Misplaced { entry: entry.clone() });
        }
        let source = base.join(relative);
        let quiz_bytes = read(&source)?;
        let quiz: Quiz = parse(&source, &quiz_bytes)?;
        let revision = sha256_hex(&quiz_bytes);
        fingerprint.update(entry.as_bytes());
        fingerprint.update(b"\0");
        fingerprint.update(revision.as_bytes());
        entries.push(CourseEntry { entry: entry.clone(), source, quiz, revision });
    }
    let listed: Vec<Quiz> = entries.iter().map(|entry: &CourseEntry| entry.quiz.clone()).collect();
    let nested = listed.iter().enumerate().flat_map(|(index, document)| course::quiz_issues(document).into_iter().map(move |issue| (format!("/quizzes/{index}{}", issue.path), issue.code)));
    let mut found: Vec<(String, course::IssueCode)> = course::course_issues(&course, &listed).into_iter().map(|issue| (issue.path, issue.code)).chain(nested).collect();
    found.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.as_str().cmp(right.1.as_str())));
    found.dedup();
    let issues: Vec<CourseIssue> = found.into_iter().map(|(path, code)| CourseIssue { quiz: quiz_entry(&course, &path), path, code: code.as_str().to_string() }).collect();
    if !issues.is_empty() {
        return Err(CourseError::Invalid { issues });
    }
    let current = entries.iter().map(|entry| (entry.quiz.id.clone(), LoadedQuiz { quiz: entry.quiz.clone(), revision: entry.revision.clone() })).collect();
    let view = course::course_view(&course, &listed);
    Ok(LoadedCourse { source: path.to_path_buf(), course, entries, current, view, fingerprint: semio_framework_hash::hex_lower(&fingerprint.finalize()) })
}

fn quiz_entry(course: &Course, pointer: &str) -> Option<String> {
    let index: usize = pointer.strip_prefix("/quizzes/")?.split('/').next()?.parse().ok()?;
    course.quizzes.get(index).cloned()
}

fn read(path: &Path) -> Result<Vec<u8>, CourseError> {
    std::fs::read(path).map_err(|error| CourseError::Unreadable { path: path.to_path_buf(), detail: error.to_string() })
}

fn parse<T: serde::de::DeserializeOwned>(path: &Path, bytes: &[u8]) -> Result<T, CourseError> {
    serde_json::from_slice(bytes).map_err(|error| CourseError::Malformed { path: path.to_path_buf(), detail: error.to_string() })
}

#[cfg(any(test, feature = "testkit"))]
#[doc(hidden)]
#[path = "🧰️fixtures/🦀️.rs"]
pub mod fixtures;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
