//! 🏆️ Subject adapter of the leaderboard case: the `quiz` crate folds every committed stream and answers with its views.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/👁️views/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{catalog_view, empty_learner_state, evolve_learner, leaderboard, learner_view, Catalog, CatalogView, Event, LearnerState, Quiz};
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

    /// 🎯️ The projection keyed by vector id.
    fn keyed(vectors: &Value, answer: impl Fn(&Value) -> Result<Value, String>) -> Result<Outcome, String> {
        let mut projection = Map::new();
        for vector in vectors["vectors"].as_array().ok_or("the vectors carry no vectors")? {
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), answer(vector)?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 🗺️ The catalog view of every committed catalog with its quizzes.
    pub fn catalog_views(ctx: &Context) -> Result<Outcome, String> {
        let vectors: Value = serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())?;
        let mut projection = Map::new();
        for vector in vectors["catalogs"].as_array().ok_or("the vectors carry no catalogs")? {
            let view = catalog_view(&decode!(vector["catalog"], Catalog)?, &decode!(vector["quizzes"], Vec<Quiz>)?);
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(view).map_err(|error| error.to_string())?);
        }
        Ok(Outcome::projection(parse_json(&Value::Object(projection).to_string())?))
    }

    /// 👤️ The learner view of every committed learner stream.
    pub fn learner_views(ctx: &Context) -> Result<Outcome, String> {
        let (vectors, view) = committed(ctx)?;
        keyed(&vectors, |vector| {
            let mut views = Map::new();
            for state in folded(vector)? {
                views.insert(state.learner.clone(), serde_json::to_value(learner_view(&state, &view)).map_err(|error| error.to_string())?);
            }
            Ok(Value::Object(views))
        })
    }

    /// 🗃️ The leaderboard of every committed set of learner streams.
    pub fn rankings(ctx: &Context) -> Result<Outcome, String> {
        let (vectors, view) = committed(ctx)?;
        keyed(&vectors, |vector| serde_json::to_value(leaderboard(&folded(vector)?, &view)).map_err(|error| error.to_string()))
    }
}

/// 🧭️ Subject role only — the oracle is the Python second implementation in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("catalog-views", subject::catalog_views).subject("learner-views", subject::learner_views).subject("rankings", subject::rankings);
    built
}
