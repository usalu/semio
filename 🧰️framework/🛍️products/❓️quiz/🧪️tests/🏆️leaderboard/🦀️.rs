//! 🏆️ Subject adapter of the leaderboard case: the `quiz` crate folds every committed stream and answers with its views.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/👁️views/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{catalog_view, empty_learner_state, evolve_learner, leaderboard, learner_view, period_window, transcript, Catalog, CatalogView, Event, Leaderboard, LeaderboardPeriod, LeaderboardRow, LearnerState, Quiz, Transcript, LEADERBOARD_PERIODS};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://🏆️leaderboard/🔣️.json";

    /// 🔓️ A committed document decoded into its typed twin; a document the twin refuses is an error.
    macro_rules! decode {
        ($value:expr, $type:ty) => {
            serde_json::from_value::<$type>($value.clone()).map_err(|error| format!("{error} in {}", $value))
        };
    }

    /// 🧫️ The committed vectors and the catalog view they are viewed against.
    fn committed(ctx: &Context) -> Result<(Value, CatalogView), String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let view = catalog_view(&decode!(vectors["catalog"], Catalog)?, &decode!(vectors["quizzes"], Vec<Quiz>)?);
        Ok((vectors, view))
    }

    /// 🗂️ Every learner of a vector folded from its committed events.
    fn folded(vector: &Value) -> Result<Vec<LearnerState>, String> {
        let mut states = Vec::new();
        for learner in vector["learners"].as_array().ok_or("the vector carries no learners")? {
            let mut state = empty_learner_state(learner["learner"].as_str().unwrap_or_default());
            for event in decode!(learner["events"], Vec<Event>)? {
                evolve_learner(&mut state, &event);
            }
            states.push(state);
        }
        Ok(states)
    }

    /// 🎯️ The projection keyed by the id of every entry of `vectors`.
    fn keyed(vectors: &Value, answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for vector in vectors.as_array().ok_or("the vectors carry no such group")? {
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), answer(vector)?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 🙋️ One leaderboard as every committed caller is answered, keyed by the caller's label.
    fn asked(transcripts: &[Transcript], view: &CatalogView, board: &Value, callers: &Value, shape: impl Fn(Leaderboard) -> Result<Value, String>) -> Result<Value, String> {
        let period = decode!(board["period"], LeaderboardPeriod)?;
        let at = board["at"].as_u64().ok_or("the board names no instant")?;
        let mut answers = Map::new();
        for caller in callers.as_array().ok_or("the vector names no callers")? {
            answers.insert(caller["id"].as_str().unwrap_or_default().to_string(), shape(leaderboard(transcripts, view, period, board["quiz"].as_str(), at, caller["learner"].as_str()))?);
        }
        Ok(Value::Object(answers))
    }

    /// 🪧️ A leaderboard reduced to what a cut decides: `rank:tag` of every row, the number of ranked learners and `rank:tag` of the own row.
    fn outline(board: Leaderboard) -> Result<Value, String> {
        let place = |row: &LeaderboardRow| format!("{}:{}", row.rank, row.tag);
        Ok(serde_json::json!({ "rows": board.rows.iter().map(place).collect::<Vec<_>>(), "learners": board.learners, "own": board.own.as_ref().map(place) }))
    }

    /// 🗺️ The catalog view of every committed catalog with its quizzes.
    pub fn catalog_views(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        keyed(&vectors["catalogs"], |vector| serde_json::to_value(catalog_view(&decode!(vector["catalog"], Catalog)?, &decode!(vector["quizzes"], Vec<Quiz>)?)).map_err(|error| error.to_string()))
    }

    /// 👤️ The learner view of every committed learner stream.
    pub fn learner_views(ctx: &Context) -> Result<Outcome, String> {
        let (vectors, view) = committed(ctx)?;
        keyed(&vectors["vectors"], |vector| {
            let mut views = Map::new();
            for state in folded(vector)? {
                views.insert(state.learner.clone(), serde_json::to_value(learner_view(&state, &view)).map_err(|error| error.to_string())?);
            }
            Ok(Value::Object(views))
        })
    }

    /// 🪟️ The window of every period around every committed instant; `null` for all-time.
    pub fn windows(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        keyed(&vectors["windows"], |vector| {
            let at = vector["at"].as_u64().ok_or("the vector names no instant")?;
            let mut windows = Map::new();
            for period in LEADERBOARD_PERIODS {
                windows.insert(serde_json::to_value(period).ok().and_then(|name| name.as_str().map(str::to_string)).unwrap_or_default(), serde_json::to_value(period_window(period, at)).map_err(|error| error.to_string())?);
            }
            Ok(Value::Object(windows))
        })
    }

    /// 🗃️ Every committed leaderboard of every committed set of learner streams, as every committed caller is answered.
    pub fn rankings(ctx: &Context) -> Result<Outcome, String> {
        let (vectors, view) = committed(ctx)?;
        keyed(&vectors["vectors"], |vector| {
            let transcripts: Vec<Transcript> = folded(vector)?.iter().filter_map(transcript).collect();
            let mut boards = Map::new();
            for board in vector["boards"].as_array().ok_or("the vector asks for no boards")? {
                boards.insert(board["id"].as_str().unwrap_or_default().to_string(), asked(&transcripts, &view, board, &vector["callers"], |board| serde_json::to_value(board).map_err(|error| error.to_string()))?);
            }
            Ok(Value::Object(boards))
        })
    }

    /// 🪜️ The outline of the committed board over the first transcripts of the committed crowd, per committed cut and caller.
    pub fn cuts(ctx: &Context) -> Result<Outcome, String> {
        let (vectors, view) = committed(ctx)?;
        let crowd = decode!(vectors["crowd"]["transcripts"], Vec<Transcript>)?;
        keyed(&vectors["crowd"]["cuts"], |vector| asked(&crowd[..(vector["learners"].as_u64().unwrap_or(0) as usize).min(crowd.len())], &view, &vectors["crowd"]["board"], &vector["callers"], outline))
    }
}

/// 🧭️ Subject role only — the oracle is the Python second implementation in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("catalog-views", subject::catalog_views).subject("learner-views", subject::learner_views).subject("windows", subject::windows).subject("rankings", subject::rankings).subject("cuts", subject::cuts);
    built
}
