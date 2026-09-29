//! 🧾️ Subject adapter of the learner-lifecycle case: the `quiz` crate replays every committed sequence through its deciders.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🧾️lifecycle/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{
        catalog_view, decide_learner, decide_roster, empty_learner_state, empty_roster_state, evolve_learner, evolve_roster, learner_view, normalize_handle, run_view, Catalog, Command, Decision, Event, LearnerContext, LearnerState,
        LoadedQuiz, Quiz,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use std::collections::BTreeMap;

    const VECTORS: &str = "shared://🧾️learner-lifecycle/🔣️.json";

    /// 🔓️ A committed document decoded into its typed twin; a document the twin refuses is an error.
    macro_rules! decode {
        ($value:expr, $type:ty) => {
            serde_json::from_value::<$type>($value.clone()).map_err(|error| format!("{error} in {}", $value))
        };
    }

    /// 🧫️ The committed vectors.
    fn committed(ctx: &Context) -> Result<Value, String> {
        serde_json::from_slice(&ctx.fixture_bytes(VECTORS)?).map_err(|error| error.to_string())
    }

    /// 🔣️ Any serializable answer as the owned protocol projection.
    fn projected(value: Map<String, Value>) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&Value::Object(value).to_string())?))
    }

    /// 📚️ The loaded quizzes at the given revisions.
    fn loaded(quizzes: &[Quiz], revisions: &BTreeMap<String, String>) -> BTreeMap<String, LoadedQuiz> {
        quizzes.iter().map(|quiz| (quiz.id.clone(), LoadedQuiz { quiz: quiz.clone(), revision: revisions.get(&quiz.id).cloned().unwrap_or_default() })).collect()
    }

    /// 🎞️ Folds the given events, then decides and folds every step.
    fn replay(vectors: &Value, sequence: &Value) -> Result<(Vec<Decision>, LearnerState, BTreeMap<String, LoadedQuiz>), String> {
        let catalog = decode!(vectors["catalog"], Catalog)?;
        let quizzes = decode!(vectors["quizzes"], Vec<Quiz>)?;
        let mut revisions = decode!(vectors["revisions"], BTreeMap<String, String>)?;
        let mut state = empty_learner_state(sequence["learner"].as_str().unwrap_or_default());
        for event in decode!(sequence["given"], Vec<Event>)? {
            evolve_learner(&mut state, &event);
        }
        let mut decisions = Vec::new();
        for step in sequence["steps"].as_array().ok_or("the sequence carries no steps")? {
            if step.get("revisions").is_some() {
                revisions.extend(decode!(step["revisions"], BTreeMap<String, String>)?);
            }
            let current = loaded(&quizzes, &revisions);
            let context = LearnerContext { now: step["now"].as_u64().ok_or("the step carries no now")?, catalog: &catalog, quizzes: &current };
            let decision = decide_learner(&state, &decode!(step["command"], Command)?, &context);
            if let Decision::Events(events) = &decision {
                for event in events {
                    evolve_learner(&mut state, event);
                }
            }
            decisions.push(decision);
        }
        Ok((decisions, state, loaded(&quizzes, &revisions)))
    }

    /// 🪪️ `normalize_handle` of every committed handle.
    pub fn handles(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let mut projection = Map::new();
        for vector in vectors["handles"].as_array().ok_or("the vectors carry no handles")? {
            projection.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(normalize_handle(vector["handle"].as_str().unwrap_or_default())).map_err(|error| error.to_string())?);
        }
        projected(projection)
    }

    /// 🗂️ Every committed identify-learner sequence decided and folded from an empty roster.
    pub fn roster(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let mut projection = Map::new();
        for sequence in vectors["roster"].as_array().ok_or("the vectors carry no roster")? {
            let mut state = empty_roster_state();
            let mut decisions = Vec::new();
            for step in sequence["steps"].as_array().ok_or("the sequence carries no steps")? {
                let decision = decide_roster(&state, &decode!(step["command"], Command)?, step["now"].as_u64().ok_or("the step carries no now")?);
                if let Decision::Events(events) = &decision {
                    for event in events {
                        evolve_roster(&mut state, event);
                    }
                }
                decisions.push(decision);
            }
            projection.insert(sequence["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(decisions).map_err(|error| error.to_string())?);
        }
        projected(projection)
    }

    /// ⚖️ The decisions of every committed learner sequence.
    pub fn learner_decisions(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let mut projection = Map::new();
        for sequence in vectors["learners"].as_array().ok_or("the vectors carry no learners")? {
            projection.insert(sequence["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(replay(&vectors, sequence)?.0).map_err(|error| error.to_string())?);
        }
        projected(projection)
    }

    /// 👤️ The learner view and the committed run views after every sequence of a registered learner.
    pub fn learner_views(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let view = catalog_view(&decode!(vectors["catalog"], Catalog)?, &decode!(vectors["quizzes"], Vec<Quiz>)?);
        let mut projection = Map::new();
        for sequence in vectors["learners"].as_array().ok_or("the vectors carry no learners")?.iter().filter(|sequence| sequence.get("views").is_some()) {
            let (_, state, quizzes) = replay(&vectors, sequence)?;
            let mut runs = Map::new();
            for run in sequence["views"]["runs"].as_array().ok_or("the views name no runs")? {
                let run = run.as_str().unwrap_or_default();
                runs.insert(run.to_string(), serde_json::to_value(run_view(&state, run, &quizzes)).map_err(|error| error.to_string())?);
            }
            let learner = serde_json::to_value(learner_view(&state, &view)).map_err(|error| error.to_string())?;
            projection.insert(sequence["id"].as_str().unwrap_or_default().to_string(), serde_json::json!({ "learner": learner, "runs": runs }));
        }
        projected(projection)
    }
}

/// 🧭️ Subject role only — the oracle is the Python second implementation in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("handles", subject::handles).subject("roster", subject::roster).subject("learner-decisions", subject::learner_decisions).subject("learner-views", subject::learner_views);
    built
}
