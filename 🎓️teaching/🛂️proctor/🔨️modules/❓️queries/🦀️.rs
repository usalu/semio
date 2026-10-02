//! ❓️ The six quiz reads (design §9a, §17): `quiz.catalog`, `quiz.learner`, `quiz.run`,
//! `quiz.leaderboard`, `quiz.crowd` and `quiz.handle`, version `1`, arguments = the quiz `Query`
//! JSON, answered as a `snapshot` whose value is the view JSON. The catalog view is derived from the
//! loaded catalog; every other view is read from the projections, never from an actor.
//!
//! Every query is held to its shapes first (`id-invalid`, `handle-invalid`, answered `400`), so no
//! malformed id is looked up. `quiz.leaderboard` answers from the [`Board`] the leaderboard of the
//! asked period — daily, weekly, monthly or all-time, of every quiz or of the one the query names —
//! as it stands at the proctor's clock: the top rows, the count of ranked learners, the count of
//! submissions and — when the query names a ranked `learner` — that caller's own row; the caller is
//! named in the query because the proctor resolves every principal as anonymous, and a quiz the
//! catalog does not list is `unknown-quiz` (`404`), as for `quiz.crowd`. `quiz.handle` is how a
//! claimed handle is recalled: it normalizes the handle and answers its holder, and it writes
//! nothing.
//!
//! @see ../🔭️projections/🦀️.rs — where the views are kept
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs — `QueryHandler`

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use quiz::{HandleHolder, HandleView, Query, Timestamp};
use server::contract::{QueryEnvelope, QueryResult};
use server::gateway::{QueryHandler, ServerError};
use server::storage::ProjectionStore;

use crate::actors::WIRE_VERSION;
use crate::projections::{decoded, Board, CROWDS, HANDLES, LEARNERS, RUNS};

/// 🔭️ Which of the six reads a handler answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryKind {
    Catalog,
    Learner,
    Run,
    Leaderboard,
    Crowd,
    Handle,
}

impl QueryKind {
    /// 📋️ All six, in registration order.
    pub const ALL: [QueryKind; 6] = [Self::Catalog, Self::Learner, Self::Run, Self::Leaderboard, Self::Crowd, Self::Handle];

    /// 🏷️ The framework query kind, `quiz.<type>`.
    pub fn wire(self) -> &'static str {
        match self {
            Self::Catalog => "quiz.catalog",
            Self::Learner => "quiz.learner",
            Self::Run => "quiz.run",
            Self::Leaderboard => "quiz.leaderboard",
            Self::Crowd => "quiz.crowd",
            Self::Handle => "quiz.handle",
        }
    }
}

/// 🕰️ The wall clock in milliseconds since the Unix epoch: the instant a leaderboard is asked at.
pub fn wall_clock() -> Timestamp {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as Timestamp)
}

/// 🙋️ One registered quiz read; `now` is the clock the periods of a leaderboard turn by.
pub struct QuizQuery {
    pub kind: QueryKind,
    pub tenant: String,
    pub catalog: Arc<Vec<u8>>,
    pub board: Arc<Board>,
    pub now: fn() -> Timestamp,
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
        if let Some(malformed) = quiz::query_rejection(&query) {
            return Err(ServerError::BadRequest(format!("{}: the {} query is refused", malformed.as_str(), query.type_name())));
        }
        let value = match (self.kind, &query) {
            (QueryKind::Catalog, Query::Catalog) => self.catalog.to_vec(),
            (QueryKind::Learner, Query::Learner { learner }) => projections.get(LEARNERS, learner).await.ok_or_else(|| ServerError::NotFound(format!("unknown-learner {learner}")))?,
            (QueryKind::Run, Query::Run { run }) => projections.get(RUNS, run).await.ok_or_else(|| ServerError::NotFound(format!("unknown-run {run}")))?,
            (QueryKind::Leaderboard, Query::Leaderboard { period, quiz, learner }) => {
                self.board.load(projections).await.map_err(internal)?;
                let board = self.board.view(*period, quiz.as_deref(), (self.now)(), learner.as_deref()).ok_or_else(|| ServerError::NotFound(format!("unknown-quiz {}", quiz.as_deref().unwrap_or_default())))?;
                encoded(&board)?
            }
            (QueryKind::Crowd, Query::Crowd { quiz }) => projections.get(CROWDS, quiz).await.ok_or_else(|| ServerError::NotFound(format!("unknown-quiz {quiz}")))?,
            (QueryKind::Handle, Query::Handle { handle }) => {
                let normalized = quiz::normalize_handle(handle).ok_or_else(|| ServerError::BadRequest(quiz::Rejection::HandleInvalid.as_str().to_string()))?;
                let holder = match projections.get(HANDLES, &normalized.key).await {
                    Some(bytes) => Some(decoded::<HandleHolder>(HANDLES, &normalized.key, &bytes).map_err(internal)?),
                    None => None,
                };
                encoded(&HandleView { display: normalized.display, holder })?
            }
            _ => return Err(ServerError::BadRequest(format!("{} cannot answer a {} query", envelope.kind, query.type_name()))),
        };
        Ok(QueryResult::Snapshot { value, frontier: None })
    }
}

fn internal(error: impl std::fmt::Display) -> ServerError {
    ServerError::Internal(error.to_string())
}

fn encoded<T: serde::Serialize>(view: &T) -> Result<Vec<u8>, ServerError> {
    serde_json::to_vec(view).map_err(internal)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
