//! ⛰️ Subject adapter of the challenge-rules case: the rule table, `points`, `reach`, `misses`, `task_seconds`, `acted` and `hints_of` of the `quiz` crate — the hints over three vector groups (compare hints, classification hints, hints on the authored quizzes).
//!
//! @see ./🥒️.feature
//! @see ../../🔨️modules/⛰️challenge/🦀️.rs

use semio_repo_test_host::Adapter;

#[cfg(feature = "sut")]
mod subject {
    use quiz::serde_json::{self, json, Map, Value};
    use quiz::{acted, challenge_meets, challenge_rank, challenge_rules, hints_of, misses, points, reach, task_seconds, Answer, Challenge, Scale, SheetTask, Task, TaskKind};
    use semio_repo_test_host::{parse_json, Context, Outcome};

    const VECTORS: &str = "shared://⛰️challenge-rules/🔣️.json";

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

    /// 📜️ One committed vector group.
    fn group<'a>(vectors: &'a Value, name: &str) -> Result<&'a Vec<Value>, String> {
        vectors[name].as_array().ok_or_else(|| format!("the vectors carry no {name}"))
    }

    /// 🏷️ The id of a vector.
    fn id(vector: &Value) -> String {
        vector["id"].as_str().unwrap_or_default().to_string()
    }

    /// 🔣️ A projection as the owned protocol value.
    fn projected(value: Value) -> Result<Outcome, String> {
        Ok(Outcome::projection(parse_json(&value.to_string())?))
    }

    /// ♾️ A reach as the vectors carry it: `null` where it is unbounded.
    fn bounded(found: f64) -> Value {
        if found.is_finite() {
            json!(found)
        } else {
            Value::Null
        }
    }

    /// 📏️ A committed list of numbers.
    fn numbers(value: &Value) -> Result<Vec<f64>, String> {
        decode!(value, Vec<f64>)
    }

    /// 🗂️ The rule table, the ranks, the least-challenge relation and the points of every committed score.
    pub fn rules(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let challenges = decode!(vectors["rules"]["challenges"], Vec<Challenge>)?;
        let name = |challenge: &Challenge| serde_json::to_value(challenge).ok().and_then(|value| value.as_str().map(str::to_string)).unwrap_or_default();
        let mut table = Map::new();
        let mut ranks = Map::new();
        let mut meets = Map::new();
        for challenge in &challenges {
            table.insert(name(challenge), serde_json::to_value(challenge_rules(*challenge)).map_err(|error| error.to_string())?);
            ranks.insert(name(challenge), json!(challenge_rank(*challenge)));
            meets.insert(name(challenge), Value::Object(challenges.iter().map(|least| (name(least), Value::Bool(challenge_meets(*challenge, *least)))).collect()));
        }
        let mut earned = Map::new();
        for vector in group(&vectors, "points")? {
            let score = vector["score"].as_f64().ok_or_else(|| format!("no score in {vector}"))?;
            earned.insert(id(vector), json!(points(score, decode!(vector["challenge"], Challenge)?)));
        }
        projected(json!({ "table": table, "ranks": ranks, "meets": meets, "points": earned }))
    }

    /// 📡️ The reach of every committed set of values, and whether every committed value misses its truth.
    pub fn reach_of(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let mut reaches = Map::new();
        for vector in group(&vectors, "reaches")? {
            reaches.insert(id(vector), bounded(reach(&numbers(&vector["values"])?, decode!(vector["scale"], Scale)?)));
        }
        let mut missed = Map::new();
        for vector in group(&vectors, "misses")? {
            let scale = decode!(vector["scale"], Scale)?;
            let found = reach(&numbers(&vector["values"])?, scale);
            let (value, truth) = (vector["value"].as_f64().ok_or_else(|| format!("no value in {vector}"))?, vector["truth"].as_f64().ok_or_else(|| format!("no truth in {vector}"))?);
            missed.insert(id(vector), json!({ "reach": bounded(found), "miss": misses(value, truth, scale, found) }));
        }
        projected(json!({ "reaches": reaches, "misses": missed }))
    }

    /// 🕰️ The seconds of every committed timed task and every committed instant raised to its floor.
    pub fn clock(ctx: &Context) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let mut seconds = Map::new();
        for vector in group(&vectors, "seconds")? {
            let count = |member: &str| vector[member].as_u64().and_then(|count| usize::try_from(count).ok()).ok_or_else(|| format!("no {member} in {vector}"));
            seconds.insert(id(vector), json!(task_seconds(decode!(vector["kind"], TaskKind)?, count("items")?, count("dimensions")?)));
        }
        let mut instants = Map::new();
        for vector in group(&vectors, "instants")? {
            let instant = |member: &str| vector[member].as_u64().ok_or_else(|| format!("no {member} in {vector}"));
            instants.insert(id(vector), json!(acted(instant("at")?, instant("floor")?, instant("now")?)));
        }
        projected(json!({ "seconds": seconds, "instants": instants }))
    }

    /// 📈️ The compare hints of sortings and matchings and every case without a hint.
    pub fn hints(ctx: &Context) -> Result<Outcome, String> {
        hinted(ctx, "hints")
    }

    /// 🧺️ The profile, group and category hints of classifications.
    pub fn classification_hints(ctx: &Context) -> Result<Outcome, String> {
        hinted(ctx, "classificationHints")
    }

    /// 📚️ The hints of the tasks of the authored quizzes on sheets they deal.
    pub fn quiz_hints(ctx: &Context) -> Result<Outcome, String> {
        hinted(ctx, "quizHints")
    }

    /// 💡️ The hints of every committed task, sheet task and answer of one vector group.
    fn hinted(ctx: &Context, name: &str) -> Result<Outcome, String> {
        let vectors = committed(ctx)?;
        let tasks = decode!(vectors["tasks"], Vec<Task>)?;
        let mut projection = Map::new();
        for vector in group(&vectors, name)? {
            let task = tasks.iter().find(|task| vector["task"] == task.id().as_str()).ok_or_else(|| format!("unknown task in {vector}"))?;
            let answer = vector.get("answer").map(|answer| decode!(answer, Answer)).transpose()?;
            projection.insert(id(vector), serde_json::to_value(hints_of(task, &decode!(vector["sheetTask"], SheetTask)?, answer.as_ref())).map_err(|error| error.to_string())?);
        }
        projected(Value::Object(projection))
    }
}

/// 🧭️ Subject role only — the oracle is the numpy-corroborated reference in `🐍️.py`.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("rules", subject::rules).subject("reach", subject::reach_of).subject("clock", subject::clock).subject("hints", subject::hints).subject("classification-hints", subject::classification_hints).subject("quiz-hints", subject::quiz_hints);
    built
}
