//! 🧾️ Subject adapter of the learner-lifecycle case: the `quiz` crate replays every committed sequence through its deciders and plays the site catalog.
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/🧾️lifecycle/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, Map, Value};
    use quiz::{
        catalog_view, decide_handle, decide_learner, empty_handle_state, empty_learner_state, evolve_handle, evolve_learner, learner_view, registration_rejection, run_seed, run_view, sheet_of, Answer, Catalog, ClassificationAnswer, Command, Decision, Event,
        Identity, LearnerContext, LearnerState, Limits, LoadedQuiz, MatchingAnswer, Quiz, SheetTask, SortingAnswer, Task, DEFAULT_LIMITS,
    };
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use std::collections::BTreeMap;
    use std::path::Path;

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

    /// 🧺️ Every sequence of a group: the well-formed ones and those whose commands carry malformed ids.
    fn group<'a>(vectors: &'a Value, name: &str) -> Result<Vec<&'a Value>, String> {
        let listed = |value: &'a Value| value[name].as_array().ok_or_else(|| format!("the vectors carry no {name}"));
        Ok(listed(vectors)?.iter().chain(listed(&vectors["malformed"])?).collect())
    }

    /// 🔣️ Any serializable answer as the owned protocol projection.
    fn projected(value: Map<String, Value>) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&Value::Object(value).to_string())?))
    }

    /// 📚️ The loaded quizzes at the given revisions.
    fn loaded(quizzes: &[Quiz], revisions: &BTreeMap<String, String>) -> BTreeMap<String, LoadedQuiz> {
        quizzes.iter().map(|quiz| (quiz.id.clone(), LoadedQuiz { quiz: quiz.clone(), revision: revisions.get(&quiz.id).cloned().unwrap_or_default() })).collect()
    }

    /// 🎞️ Folds the given events, then decides and folds every step under the sequence's caps.
    fn replay(vectors: &Value, sequence: &Value) -> Result<(Vec<Decision>, LearnerState, BTreeMap<String, LoadedQuiz>), String> {
        let catalog = decode!(vectors["catalog"], Catalog)?;
        let quizzes = decode!(vectors["quizzes"], Vec<Quiz>)?;
        let mut revisions = decode!(vectors["revisions"], BTreeMap<String, String>)?;
        let limits = decode!(sequence.get("limits").unwrap_or(&vectors["limits"]), Limits)?;
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
            let context = LearnerContext { now: step["now"].as_u64().ok_or("the step carries no now")?, catalog: &catalog, quizzes: &current, limits: &limits };
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

    /// 🗂️ Every committed identify-learner sequence of a handle key decided and folded from the unclaimed key, and every committed learner count against its cap.
    pub fn registrations(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let mut handles = Map::new();
        for sequence in group(&vectors, "registrations")? {
            let mut state = empty_handle_state(sequence["key"].as_str().unwrap_or_default());
            let mut decisions = Vec::new();
            for step in sequence["steps"].as_array().ok_or("the sequence carries no steps")? {
                let decision = decide_handle(&state, &decode!(step["command"], Command)?, step["now"].as_u64().ok_or("the step carries no now")?);
                if let Decision::Events(events) = &decision {
                    for event in events {
                        evolve_handle(&mut state, event);
                    }
                }
                decisions.push(decision);
            }
            handles.insert(sequence["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(decisions).map_err(|error| error.to_string())?);
        }
        let mut quotas = Map::new();
        for vector in vectors["quotas"].as_array().ok_or("the vectors carry no quotas")? {
            let rejection = registration_rejection(vector["learners"].as_u64().ok_or("the quota carries no learners")?, &decode!(vector["limits"], Limits)?);
            quotas.insert(vector["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(rejection).map_err(|error| error.to_string())?);
        }
        projected(Map::from_iter([("handles".to_string(), Value::Object(handles)), ("quotas".to_string(), Value::Object(quotas))]))
    }

    /// ⚖️ The decisions of every committed learner sequence.
    pub fn learner_decisions(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let mut projection = Map::new();
        for sequence in group(&vectors, "learners")? {
            projection.insert(sequence["id"].as_str().unwrap_or_default().to_string(), serde_json::to_value(replay(&vectors, sequence)?.0).map_err(|error| error.to_string())?);
        }
        projected(projection)
    }

    /// 👤️ The learner view and the committed run views after every sequence of a registered learner.
    pub fn learner_views(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let view = catalog_view(&decode!(vectors["catalog"], Catalog)?, &decode!(vectors["quizzes"], Vec<Quiz>)?);
        let mut projection = Map::new();
        for sequence in group(&vectors, "learners")?.into_iter().filter(|sequence| sequence.get("views").is_some()) {
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

    /// 📄️ A JSON document from disk.
    fn read(path: &Path) -> Result<Value, String> {
        serde_json::from_slice(&std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?).map_err(|error| format!("{}: {error}", path.display()))
    }

    /// 💯️ The answer that scores a sheet task 1: every item in its category, ascending by value (ties in definition order), every item on a card of its own value.
    fn perfect_answer(task: &Task, sheet_task: &SheetTask) -> Result<Answer, String> {
        match (task, sheet_task) {
            (Task::Classification(task), SheetTask::Classification(sheet)) => Ok(Answer::Classification(ClassificationAnswer { assignments: sheet.items.iter().filter_map(|item| task.items.iter().find(|candidate| candidate.id == item.id).map(|candidate| (item.id.clone(), candidate.category.clone()))).collect() })),
            (Task::Sorting(task), SheetTask::Sorting(sheet)) => {
                let mut order: Vec<(usize, f64, &String)> = task.items.iter().enumerate().filter(|(_, item)| sheet.items.iter().any(|presented| presented.id == item.id)).map(|(index, item)| (index, item.value, &item.id)).collect();
                order.sort_by(|left, right| left.1.total_cmp(&right.1).then(left.0.cmp(&right.0)));
                Ok(Answer::Sorting(SortingAnswer { order: order.into_iter().map(|(_, _, id)| id.clone()).collect(), guesses: BTreeMap::new() }))
            }
            (Task::Matching(task), SheetTask::Matching(sheet)) => {
                let mut assignments = BTreeMap::new();
                for dimension in &sheet.dimensions {
                    let mut free: Vec<usize> = (0..dimension.cards.len()).collect();
                    let mut cards = BTreeMap::new();
                    for item in &sheet.items {
                        let value = task.items.iter().find(|candidate| candidate.id == item.id).and_then(|candidate| candidate.values.get(&dimension.id)).ok_or_else(|| format!("{} has no value of {}", item.id, dimension.id))?;
                        let position = free.iter().position(|&card| dimension.cards[card] == *value).ok_or_else(|| format!("{} has no card of its value", item.id))?;
                        cards.insert(item.id.clone(), free.remove(position));
                    }
                    assignments.insert(dimension.id.clone(), cards);
                }
                Ok(Answer::Matching(MatchingAnswer { assignments }))
            }
            _ => Err(format!("{} is not presented as its own kind", task.id())),
        }
    }

    /// 🩹️ The perfect answer with exactly one mistake: the first item in the first wrong category, the smallest and the largest item exchanged, or the cards of the first item and of the first later item of another value exchanged in the first dimension.
    fn flawed_answer(task: &Task, sheet_task: &SheetTask) -> Result<Answer, String> {
        let flawless = perfect_answer(task, sheet_task)?;
        let unsuitable = || format!("{} cannot carry a single mistake", task.id());
        match (task, sheet_task, flawless) {
            (Task::Classification(task), SheetTask::Classification(sheet), Answer::Classification(mut answer)) => {
                let first = sheet.items.first().ok_or_else(unsuitable)?;
                let correct = &task.items.iter().find(|candidate| candidate.id == first.id).ok_or_else(unsuitable)?.category;
                answer.assignments.insert(first.id.clone(), sheet.categories.iter().find(|category| category.id != *correct).ok_or_else(unsuitable)?.id.clone());
                Ok(Answer::Classification(answer))
            }
            (_, _, Answer::Sorting(mut answer)) => {
                let last = answer.order.len().checked_sub(1).ok_or_else(unsuitable)?;
                answer.order.swap(0, last);
                Ok(Answer::Sorting(answer))
            }
            (Task::Matching(task), SheetTask::Matching(sheet), Answer::Matching(mut answer)) => {
                let dimension = &sheet.dimensions.first().ok_or_else(unsuitable)?.id;
                let value = |id: &String| task.items.iter().find(|candidate| candidate.id == *id).and_then(|candidate| candidate.values.get(dimension)).copied();
                let first = &sheet.items.first().ok_or_else(unsuitable)?.id;
                let other = &sheet.items.iter().skip(1).find(|item| value(&item.id) != value(first)).ok_or_else(unsuitable)?.id;
                let cards = answer.assignments.get_mut(dimension).ok_or_else(unsuitable)?;
                let (mine, theirs) = (*cards.get(first).ok_or_else(unsuitable)?, *cards.get(other).ok_or_else(unsuitable)?);
                cards.insert(first.clone(), theirs);
                cards.insert(other.clone(), mine);
                Ok(Answer::Matching(answer))
            }
            _ => Err(unsuitable()),
        }
    }

    /// 🎮️ One registered learner playing the committed runs: per run its score and the badges it awards, and every badge held at the end.
    fn play(catalog: &Catalog, quizzes: &BTreeMap<String, LoadedQuiz>, scenario: &Value) -> Result<Value, String> {
        let learner = scenario["learner"].as_str().unwrap_or_default().to_string();
        let mut state = empty_learner_state(&learner);
        evolve_learner(&mut state, &Event::LearnerRegistered { learner: learner.clone(), identity: Identity::Anonymous, at: 0 });
        let mut now = 0u64;
        let mut decided = |state: &mut LearnerState, command: Command| -> Result<Vec<Event>, String> {
            now += 1;
            match decide_learner(state, &command, &LearnerContext { now, catalog, quizzes, limits: &DEFAULT_LIMITS }) {
                Decision::Events(events) => {
                    for event in &events {
                        evolve_learner(state, event);
                    }
                    Ok(events)
                }
                Decision::Rejection(rejection) => Err(format!("site/{}: {} is refused with {}", scenario["id"], command.type_name(), rejection.as_str())),
            }
        };
        let (mut scores, mut awards) = (Vec::new(), Vec::new());
        for (number, run) in scenario["runs"].as_array().ok_or("the play carries no runs")?.iter().enumerate() {
            let (run_id, quiz_id) = (run["run"].as_str().unwrap_or_default().to_string(), run["quiz"].as_str().unwrap_or_default().to_string());
            let quiz = &quizzes.get(&quiz_id).ok_or_else(|| format!("the site has no quiz {quiz_id}"))?.quiz;
            let mut issued = 0u128;
            let mut id = || {
                issued += 1;
                format!("{:032x}", ((number as u128) << 64) + (issued << 32))
            };
            decided(&mut state, Command::StartRun { id: id(), learner: learner.clone(), run: run_id.clone(), quiz: quiz_id.clone() })?;
            for sheet_task in &sheet_of(quiz, run_seed(&run_id)).tasks {
                let task = quiz.tasks.iter().find(|candidate| candidate.id() == sheet_task.id()).ok_or("a sheet task has no task")?;
                let answer = if run["flaw"].as_str() == Some(sheet_task.id().as_str()) { flawed_answer(task, sheet_task)? } else { perfect_answer(task, sheet_task)? };
                decided(&mut state, Command::RecordAnswer { id: id(), learner: learner.clone(), run: run_id.clone(), task: sheet_task.id().clone(), answer })?;
            }
            let events = decided(&mut state, Command::SubmitRun { id: id(), learner: learner.clone(), run: run_id.clone() })?;
            scores.push(events.iter().find_map(|event| if let Event::RunSubmitted { result, .. } = event { Some(result.score) } else { None }).ok_or("a submission carries no result")?);
            awards.push(events.iter().filter_map(|event| if let Event::BadgeAwarded { badge, .. } = event { Some(badge.clone()) } else { None }).collect::<Vec<_>>());
        }
        Ok(serde_json::json!({ "scores": scores, "awards": awards, "held": state.badges.iter().map(|award| award.badge.clone()).collect::<Vec<_>>() }))
    }

    /// 🏅️ Every committed play of the site catalog read from the repository.
    pub fn site_catalog(ctx: &Context) -> Result<Outcome, String> {
        let site = committed(ctx)?["site"].clone();
        let path = ctx.repo_root.join(site["catalog"].as_str().ok_or("the site names no catalog")?);
        let document = read(&path)?;
        let catalog = decode!(document, Catalog)?;
        let directory = path.parent().ok_or("the catalog has no directory")?;
        let mut quizzes = BTreeMap::new();
        for entry in &catalog.quizzes {
            let document = read(&directory.join(entry))?;
            let quiz = decode!(document, Quiz)?;
            quizzes.insert(quiz.id.clone(), LoadedQuiz { quiz, revision: "0".repeat(64) });
        }
        let mut projection = Map::new();
        for scenario in site["plays"].as_array().ok_or("the site carries no plays")? {
            projection.insert(scenario["id"].as_str().unwrap_or_default().to_string(), play(&catalog, &quizzes, scenario)?);
        }
        projected(projection)
    }
}

/// 🧭️ Subject role only — the oracle is the Python second implementation in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("registrations", subject::registrations).subject("learner-decisions", subject::learner_decisions).subject("learner-views", subject::learner_views).subject("site-catalog", subject::site_catalog);
    built
}
