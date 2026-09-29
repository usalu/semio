//! ❓️ The four quiz reads (design §9a): `quiz.catalog`, `quiz.learner`, `quiz.run` and
//! `quiz.leaderboard`, version `1`, arguments = the quiz `Query` JSON, answered as a `snapshot`
//! whose value is the view JSON. The catalog view is derived from the loaded catalog; every other
//! view is read from the projections, never from an actor.
//!
//! @see ../🔭️projections/🦀️.rs — where the views are kept
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs — `QueryHandler`

use std::sync::Arc;

use quiz::{Leaderboard, Query};
use server::contract::{QueryEnvelope, QueryResult};
use server::gateway::{QueryHandler, ServerError};
use server::storage::ProjectionStore;

use crate::actors::WIRE_VERSION;
use crate::projections::{LEADERBOARD, LEADERBOARD_KEY, LEARNERS, RUNS};

/// 🔭️ Which of the four reads a handler answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryKind {
    Catalog,
    Learner,
    Run,
    Leaderboard,
}

impl QueryKind {
    /// 📋️ All four, in registration order.
    pub const ALL: [QueryKind; 4] = [Self::Catalog, Self::Learner, Self::Run, Self::Leaderboard];

    /// 🏷️ The framework query kind, `quiz.<type>`.
    pub fn wire(self) -> &'static str {
        match self {
            Self::Catalog => "quiz.catalog",
            Self::Learner => "quiz.learner",
            Self::Run => "quiz.run",
            Self::Leaderboard => "quiz.leaderboard",
        }
    }
}

/// ❓️ One registered quiz read.
pub struct QuizQuery {
    pub kind: QueryKind,
    pub tenant: String,
    pub catalog: Arc<Vec<u8>>,
}

impl QueryHandler for QuizQuery {
    async fn kind(&self) -> &str {
        self.kind.wire()
    }

    async fn handle<P: ProjectionStore>(&self, envelope: &QueryEnvelope, projections: &P) -> Result<QueryResult, ServerError> {
        if envelope.version != WIRE_VERSION {
            return Err(ServerError::BadRequest(format!("{} speaks version {WIRE_VERSION}, not {}", envelope.kind, envelope.version)));
        }
        if envelope.scope.0 != self.tenant {
            return Err(ServerError::NotFound(format!("this proctor serves catalog {:?}, not {:?}", self.tenant, envelope.scope.0)));
        }
        let query: Query = serde_json::from_slice(&envelope.arguments).map_err(|error| ServerError::BadRequest(format!("query-malformed: {error}")))?;
        let value = match (self.kind, &query) {
            (QueryKind::Catalog, Query::Catalog) => self.catalog.to_vec(),
            (QueryKind::Learner, Query::Learner { learner }) => projections.get(LEARNERS, learner).await.ok_or_else(|| ServerError::NotFound(format!("unknown-learner {learner}")))?,
            (QueryKind::Run, Query::Run { run }) => projections.get(RUNS, run).await.ok_or_else(|| ServerError::NotFound(format!("unknown-run {run}")))?,
            (QueryKind::Leaderboard, Query::Leaderboard) => match projections.get(LEADERBOARD, LEADERBOARD_KEY).await {
                Some(board) => board,
                None => serde_json::to_vec(&Leaderboard { rows: Vec::new() }).map_err(|error| ServerError::Internal(error.to_string()))?,
            },
            _ => return Err(ServerError::BadRequest(format!("{} cannot answer a {} query", envelope.kind, query.type_name()))),
        };
        Ok(QueryResult::Snapshot { value, frontier: None })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
