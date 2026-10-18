//! 🚫️ Database open rejections: the retained activity rejection and the document open rejection with its conversions. See [`super`].

use super::*;

/// 🚪️ A Database-owned retained admission either refuses before it copies any owner, or returns
/// the exact operation-specific rejection that must still be driven to terminal cleanup.
pub enum DatabaseRetainedActivityRejected<R> {
    Closed(DbError),
    Retained(R),
}

impl<R: std::fmt::Debug> std::fmt::Debug for DatabaseRetainedActivityRejected<R> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed(error) => formatter.debug_tuple("Closed").field(error).finish(),
            Self::Retained(rejected) => formatter.debug_tuple("Retained").field(rejected).finish(),
        }
    }
}

/// 🚧️ Document mount rejection retaining any writer acquired by WAL or engine construction.
#[must_use = "document mount rejection must reach terminal retained cleanup"]
pub enum DatabaseDocumentOpenRejected {
    Database(DbError),
    Engine(db_artifact_engine::ArtifactEngineOpenRejected),
}

impl DatabaseDocumentOpenRejected {
    pub fn error(&self) -> &DbError {
        match self {
            Self::Database(error) => error,
            Self::Engine(rejected) => rejected.error(),
        }
    }

    pub fn cleanup_error(&self) -> Option<&DbError> {
        match self {
            Self::Database(_) => None,
            Self::Engine(rejected) => rejected.cleanup_error(),
        }
    }

    pub fn has_retained_writer(&self) -> bool {
        matches!(self, Self::Engine(rejected) if rejected.has_retained_writer())
    }

    pub async fn retry_close(self) -> Result<DbError, DatabaseDocumentOpenRejected> {
        match self {
            Self::Database(error) => Ok(error),
            Self::Engine(rejected) => rejected.retry_close().await.map_err(Self::Engine),
        }
    }
}

impl From<DbError> for DatabaseDocumentOpenRejected {
    fn from(error: DbError) -> Self {
        Self::Database(error)
    }
}

impl From<db_artifact_engine::ArtifactEngineOpenRejected> for DatabaseDocumentOpenRejected {
    fn from(rejected: db_artifact_engine::ArtifactEngineOpenRejected) -> Self {
        Self::Engine(rejected)
    }
}

impl std::fmt::Debug for DatabaseDocumentOpenRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseDocumentOpenRejected").field("cause", self.error()).field("cleanup_error", &self.cleanup_error()).field("has_retained_writer", &self.has_retained_writer()).finish()
    }
}
