//! 🗂️ Projects owned source transitions using only the standard library.
use std::collections::BTreeMap;
use std::fmt;

/// 📄️ An exact source transition; None represents absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceChange<'a> {
    pub path: &'a str,
    pub before: Option<&'a str>,
    pub after: Option<&'a str>,
}

/// 🧯️ An owned source projection refusal category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceProjectionCode { InvalidPath, DuplicateChange, EmptyChange, PredecessorMismatch, DeletedReference, MissingReference }

impl SourceProjectionCode {
    /// 🪪️ Returns the language-neutral category spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidPath => "invalid-path", Self::DuplicateChange => "duplicate-change",
            Self::EmptyChange => "empty-change", Self::PredecessorMismatch => "predecessor-mismatch",
            Self::DeletedReference => "deleted-reference", Self::MissingReference => "missing-reference",
        }
    }
}

/// ⚠️ An owned refusal retaining the exact source identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceProjectionError { pub code: SourceProjectionCode, pub path: String }

impl SourceProjectionError {
    fn new(code: SourceProjectionCode, path: &str) -> Self { Self { code, path: path.to_owned() } }
}
impl fmt::Display for SourceProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}: {}", self.code.as_str(), self.path) }
}
impl std::error::Error for SourceProjectionError {}

fn validate_path(path: &str) -> Result<(), SourceProjectionError> {
    if path.is_empty() || path.contains(['\\', ':', '\0']) || path.split('/').any(|part| part.is_empty() || part == "." || part == "..") {
        return Err(SourceProjectionError::new(SourceProjectionCode::InvalidPath, path));
    }
    Ok(())
}

/// 📷️ Borrows source authorities and prevents fallback for explicit deletions.
pub struct SourceProjection<'a> { changes: &'a [SourceChange<'a>], by_path: BTreeMap<&'a str, SourceChange<'a>> }

impl<'a> SourceProjection<'a> {
    /// 🏗️ Checks unique canonical paths and every exact predecessor body.
    pub fn new(changes: &'a [SourceChange<'a>], mut read: impl FnMut(&str) -> Option<&'a str>) -> Result<Self, SourceProjectionError> {
        let mut by_path = BTreeMap::new();
        for change in changes {
            validate_path(change.path)?;
            if by_path.contains_key(change.path) { return Err(SourceProjectionError::new(SourceProjectionCode::DuplicateChange, change.path)); }
            if change.before.is_none() && change.after.is_none() { return Err(SourceProjectionError::new(SourceProjectionCode::EmptyChange, change.path)); }
            by_path.insert(change.path, *change);
        }
        for change in changes {
            if read(change.path) != change.before { return Err(SourceProjectionError::new(SourceProjectionCode::PredecessorMismatch, change.path)); }
        }
        Ok(Self { changes, by_path })
    }
    /// 🪦️ Identifies a retirement without consulting physical sources.
    pub fn is_deleted(&self, path: &str) -> Result<bool, SourceProjectionError> {
        validate_path(path)?;
        Ok(self.by_path.get(path).is_some_and(|change| change.after.is_none()))
    }
    /// 🔎️ Reads a fallback only when no authority owns the path.
    pub fn resolve(&self, path: &str, read: impl FnOnce(&str) -> Option<&'a str>) -> Result<Option<&'a str>, SourceProjectionError> {
        validate_path(path)?;
        Ok(match self.by_path.get(path) { Some(change) => change.after, None => read(path) })
    }
    /// 🔗️ Requires a source and distinguishes retired from unavailable identities.
    pub fn require(&self, path: &str, read: impl FnOnce(&str) -> Option<&'a str>) -> Result<&'a str, SourceProjectionError> {
        self.resolve(path, read)?.ok_or_else(|| SourceProjectionError::new(if self.by_path.contains_key(path) { SourceProjectionCode::DeletedReference } else { SourceProjectionCode::MissingReference }, path))
    }
    /// ↩️ Returns inverse transitions in the original authority order.
    pub fn inverse(&self) -> Vec<SourceChange<'a>> {
        self.changes.iter().map(|change| SourceChange { path: change.path, before: change.after, after: change.before }).collect()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
