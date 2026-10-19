//! 👥️ Shared presence and cursors (design §15, §16): the presence scopes of a course and the admission rules
//! of the ephemeral states learners publish there. The roster scope `<course>` carries `PresenceState`; the
//! room scopes `<course>/introduction`, `<course>/home`, `<course>/leaderboard`, `<course>/badges`, `<course>/learner`, `<course>/preferences` and
//! `<course>/quiz/<quiz>` (the quiz page, its runs and results) carry `CursorState`. Only the identity screen has no room.
//!
//! Problems use the validation vocabulary (JSON Pointer + code); structure is guaranteed by serde.
//!
//! @see ../../🧬️schema/🔣️.json — `Place`, `Anchor`, `Cursor`, `PresenceState`, `CursorState`
//! @see ../👥️presence/🟦️.ts — the TypeScript twin

use course::schema::{ClassificationAnswer, Cursor, CursorState, Identity, LearnerView, Place, PresenceBadgeAward, PresenceProfile, PresenceRun, PresenceRunSummary, PresenceState, RunView, Screen, SortingAnswer, ThinkingAnswer, ThinkingMatchingAnswer, ThinkingState};
use course::validation::{is_id, is_slug, normalize_handle, pointer, IssueCode, ValidationIssue};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_pack_json::{self as json, Number, Object, Value};
use serde::{Serialize, Serializer, ser::{self, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple, SerializeTupleStruct}};
use std::sync::OnceLock;

/// 🏠️ The course-wide presence room: who is online and where.
pub fn roster_scope(course: &str) -> String {
    course.to_string()
}

/// 🚪️ The cursor room of a place; `None` for the identity screen (no tag exists yet), a quiz, run or results place without a quiz.
pub fn room_scope(course: &str, place: &Place) -> Option<String> {
    let room = match place.screen {
        Screen::Identity => return None,
        Screen::Learner => "learner".to_string(),
        Screen::Preferences => "preferences".to_string(),
        Screen::Introduction => "introduction".to_string(),
        Screen::Home => "home".to_string(),
        Screen::Leaderboard => "leaderboard".to_string(),
        Screen::Badges => "badges".to_string(),
        Screen::Quiz | Screen::Run | Screen::Results => format!("quiz/{}", place.quiz.as_ref()?),
    };
    Some(format!("{course}/{room}"))
}

/// 🛃️ Every issue of a presence state, sorted by path, then code: `tag-invalid` at `/tag`, `handle-invalid`
/// at `/identity/handle` (not a normalized display handle), `slug-invalid` at `/place/quiz` or `/place/task`, and at
/// `/place/quiz` `required` for a quiz, run or results place without a quiz or `quiz-outside-run` for a quiz on
/// any other screen, `task-without-run` at `/place/task` for a task outside a run or its results.
pub fn presence_issues(state: &PresenceState) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    tag(&mut issues, &state.tag);
    if let Identity::Pseudonym { handle } | Identity::Name { handle } = &state.identity {
        if normalize_handle(handle).is_none_or(|normalized| normalized.display != *handle) {
            issues.push(issue("/identity/handle", IssueCode::HandleInvalid));
        }
    }
    let Place { screen, quiz, task } = &state.place;
    for (path, slug) in [("/place/quiz", quiz), ("/place/task", task)] {
        if slug.as_deref().is_some_and(|slug| !is_slug(slug)) {
            issues.push(issue(path, IssueCode::SlugInvalid));
        }
    }
    match (matches!(screen, Screen::Quiz | Screen::Run | Screen::Results), quiz.is_some()) {
        (true, false) => issues.push(issue("/place/quiz", IssueCode::Required)),
        (false, true) => issues.push(issue("/place/quiz", IssueCode::QuizOutsideRun)),
        _ => {}
    }
    if !matches!(screen, Screen::Run | Screen::Results) && task.is_some() {
        issues.push(issue("/place/task", IssueCode::TaskWithoutRun));
    }
    if let Some(view) = &state.view {
        let value = view.serialize(PresentationSerializer);
        let contract = presentation_contract();
        if value.as_ref().map_or(true, |value| !contract_matches(&contract["$defs"]["PresencePresentation"], value, contract)) {
            issues.push(issue("/view", IssueCode::ValueInvalid));
        }
        if value.as_ref().is_ok_and(|value| json::to_string(value).len() > PRESENCE_PRESENTATION_BYTES) {
            issues.push(issue("/view", IssueCode::TooMany));
        }
    }
    sorted(issues)
}

/// 🖲️ Every issue of a cursor state, sorted by path, then code: `tag-invalid` at `/tag`, `anchor-invalid` at
/// `/cursor/anchor` or `/focus` for a key that is no anchor (cards, `item:<id>` and `category:<id>` all follow
/// the one Anchor pattern), `out-of-range` at `/cursor/x` or `/cursor/y` for a coordinate that is not a finite
/// number in 0…1, `slug-invalid` at `/drag/item`.
pub fn cursor_issues(state: &CursorState) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    tag(&mut issues, &state.tag);
    if state.home.as_ref().is_some_and(|home| home.chars().count() > 64) { issues.push(issue("/home", IssueCode::LengthInvalid)); }
    if let Some(place) = &state.place {
        let presence = PresenceState { tag: state.tag.clone(), identity: Identity::Anonymous, place: place.clone(), active: true, view: None };
        issues.extend(presence_issues(&presence).into_iter().filter(|issue| issue.path.starts_with("/place/")));
    }
    if let Some(Cursor { anchor, x, y }) = &state.cursor {
        if !is_anchor(anchor) {
            issues.push(issue("/cursor/anchor", IssueCode::AnchorInvalid));
        }
        for (path, value) in [("/cursor/x", *x), ("/cursor/y", *y)] {
            if !(value.is_finite() && (0.0..=1.0).contains(&value)) {
                issues.push(issue(path, IssueCode::OutOfRange));
            }
        }
    }
    if state.focus.as_deref().is_some_and(|focus| !is_anchor(focus)) {
        issues.push(issue("/focus", IssueCode::AnchorInvalid));
    }
    if state.drag.as_ref().is_some_and(|drag| !is_slug(&drag.item)) {
        issues.push(issue("/drag/item", IssueCode::SlugInvalid));
    }
    sorted(issues)
}

/// 🔒️ Selects the public run fields explicitly so private learner and run ids cannot be published.
pub fn presence_run(view: &RunView) -> PresenceRun {
    PresenceRun { quiz: view.quiz.clone(), status: view.status, sheet: view.sheet.clone(), answers: view.answers.clone(), started_at: view.started_at, result: view.result.clone(), submitted_at: view.submitted_at, opened: view.opened.clone(), hints: view.hints.clone() }
}

/// 🏅️ Selects public profile fields, stripping private learner and run ids from history and badge awards.
pub fn presence_profile(view: &LearnerView) -> PresenceProfile {
    PresenceProfile { runs: view.runs.iter().map(|run| PresenceRunSummary { quiz: run.quiz.clone(), challenge: run.challenge, status: run.status, started_at: run.started_at, score: run.score, points: run.points, submitted_at: run.submitted_at }).collect(), badges: view.badges.iter().map(|award| PresenceBadgeAward { badge: award.badge.clone(), at: award.at }).collect(), best: view.best.clone(), total: view.total }
}

/// 🧳️ The presentation budget within the proctor's 128 KiB state allowance.
pub const PRESENCE_PRESENTATION_BYTES: usize = 120 * 1024;

fn presentation_contract() -> &'static Value {
    static CONTRACT: OnceLock<Value> = OnceLock::new();
    CONTRACT.get_or_init(|| json::parse(include_str!("../../🧬️schema/🔣️.json"), json::JsonMemberPolicy::Reject).expect("the normative quiz schema"))
}

fn contract_matches(schema: &Value, value: &Value, root: &Value) -> bool {
    if let Some(reference) = schema["$ref"].as_str() {
        return root.pointer(reference.trim_start_matches('#')).is_some_and(|schema| contract_matches(schema, value, root));
    }
    if let Some(variants) = schema["oneOf"].as_array() {
        if variants.iter().filter(|schema| contract_matches(schema, value, root)).count() != 1 { return false; }
    }
    if let Some(object) = value.as_object() { if schema["required"].as_array().is_some_and(|required| required.iter().any(|key| !key.as_str().is_some_and(|key| object.contains_key(key)))) { return false; } }
    if schema.get("const").is_some_and(|constant| constant != value) || schema["enum"].as_array().is_some_and(|allowed| !allowed.contains(value)) {
        return false;
    }
    match schema["type"].as_str() {
        Some("object") => {
            let Some(object) = value.as_object() else { return false };
            if schema["required"].as_array().is_some_and(|required| required.iter().any(|key| !key.as_str().is_some_and(|key| object.contains_key(key)))) { return false; }
            if !contract_count(schema, object.len(), "minProperties", "maxProperties") { return false; }
            for (key, value) in object {
                if let Some(names) = schema.get("propertyNames") { if !contract_matches(names, &Value::String(key.to_string()), root) { return false; } }
                if let Some(property) = schema["properties"].get(key) { if !contract_matches(property, value, root) { return false; } }
                else if schema["additionalProperties"] == Value::Bool(false) { return false; }
                else if schema["additionalProperties"].as_object().is_some() && !contract_matches(&schema["additionalProperties"], value, root) { return false; }
            }
            if let Some(dependencies) = schema["dependencies"].as_object() {
                for (key, required) in dependencies { if object.contains_key(key) && required.as_array().is_some_and(|keys| keys.iter().any(|key| !key.as_str().is_some_and(|key| object.contains_key(key)))) { return false; } }
            }
            true
        }
        Some("array") => value.as_array().is_some_and(|entries| contract_count(schema, entries.len(), "minItems", "maxItems") && entries.iter().all(|value| schema.get("items").is_none_or(|items| contract_matches(items, value, root)))),
        Some("string") => value.as_str().is_some_and(|text| contract_count(schema, text.chars().count(), "minLength", "maxLength") && schema["pattern"].as_str().is_none_or(|pattern| match pattern { "^[a-z0-9]+(?:-[a-z0-9]+)*$" => is_slug(text), "^[0-9a-f]{32}$" => is_id(text), "^[0-9a-f]{8}$" => is_tag(text), pattern if root["$defs"]["Handle"]["pattern"].as_str() == Some(pattern) => normalize_handle(text).is_some_and(|handle| handle.display == text), _ => false })),
        Some(kind @ ("number" | "integer")) => value.as_f64().is_some_and(|number| number.is_finite() && (kind != "integer" || number.fract() == 0.0) && schema["minimum"].as_f64().is_none_or(|min| number >= min) && schema["maximum"].as_f64().is_none_or(|max| number <= max) && schema["exclusiveMinimum"].as_f64().is_none_or(|min| number > min)),
        Some("boolean") => matches!(value, Value::Bool(_)),
        None => true,
        _ => false,
    }
}

fn contract_count(schema: &Value, count: usize, minimum: &str, maximum: &str) -> bool {
    schema[minimum].as_u64().is_none_or(|min| count as u64 >= min) && schema[maximum].as_u64().is_none_or(|max| count as u64 <= max)
}

/// 🗯️ The live thinking room of a quiz: `<course>/quiz/<quiz>/thinking`.
pub fn thinking_scope(course: &str, quiz: &str) -> String {
    format!("{course}/quiz/{quiz}/thinking")
}

/// 🪣️ The most tasks one thinking state names, and the most entries one answer (per dimension) holds.
pub const THINKING_LIMIT: usize = 64;

/// 🧠️ Every issue of a thinking state, sorted by path, then code: `tag-invalid` at `/tag`; `too-many` at
/// `/answers`, `/answers/<task>/assignments`, `/answers/<task>/order`, `/answers/<task>/guesses`, `/answers/<task>/values` or
/// `/answers/<task>/values/<dimension>` beyond [`THINKING_LIMIT`] entries; `slug-invalid` at `/answers/<task>`,
/// `/answers/<task>/assignments/<item>` (item or category), `/answers/<task>/order/<index>`,
/// `/answers/<task>/guesses/<item>`, `/answers/<task>/values/<dimension>` and `/answers/<task>/values/<dimension>/<item>`;
/// `duplicate-id` at `/answers/<task>/order/<index>` for a repeated item; `type-invalid` at
/// `/answers/<task>/guesses/<item>` and `/answers/<task>/values/<dimension>/<item>` for a guess or value that is not
/// a finite number. Guess order is not checked on drafts.
pub fn thinking_issues(state: &ThinkingState) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    tag(&mut issues, &state.tag);
    limit(&mut issues, "/answers".to_string(), state.answers.len());
    for (task, answer) in &state.answers {
        let base = pointer("/answers", task);
        if !is_slug(task) {
            issues.push(ValidationIssue { path: base.clone(), code: IssueCode::SlugInvalid });
        }
        match answer {
            ThinkingAnswer::Classification(ClassificationAnswer { assignments }) => {
                let path = format!("{base}/assignments");
                limit(&mut issues, path.clone(), assignments.len());
                for (item, _) in assignments.iter().filter(|(item, category)| !is_slug(item) || !is_slug(category)) {
                    issues.push(ValidationIssue { path: pointer(&path, item), code: IssueCode::SlugInvalid });
                }
            }
            ThinkingAnswer::Sorting(SortingAnswer { order, guesses }) => {
                let path = format!("{base}/order");
                limit(&mut issues, path.clone(), order.len());
                let mut seen = BTreeSet::new();
                for (index, item) in order.iter().enumerate() {
                    if !is_slug(item) {
                        issues.push(ValidationIssue { path: format!("{path}/{index}"), code: IssueCode::SlugInvalid });
                    }
                    if !seen.insert(item) {
                        issues.push(ValidationIssue { path: format!("{path}/{index}"), code: IssueCode::DuplicateId });
                    }
                }
                let path = format!("{base}/guesses");
                limit(&mut issues, path.clone(), guesses.as_ref().map_or(0, BTreeMap::len));
                for (item, guess) in guesses.iter().flatten() {
                    if !is_slug(item) {
                        issues.push(ValidationIssue { path: pointer(&path, item), code: IssueCode::SlugInvalid });
                    }
                    if !guess.is_finite() {
                        issues.push(ValidationIssue { path: pointer(&path, item), code: IssueCode::TypeInvalid });
                    }
                }
            }
            ThinkingAnswer::Matching(ThinkingMatchingAnswer { values }) => {
                let path = format!("{base}/values");
                limit(&mut issues, path.clone(), values.len());
                for (dimension, items) in values {
                    let path = pointer(&path, dimension);
                    if !is_slug(dimension) {
                        issues.push(ValidationIssue { path: path.clone(), code: IssueCode::SlugInvalid });
                    }
                    limit(&mut issues, path.clone(), items.len());
                    for (item, value) in items {
                        if !is_slug(item) {
                            issues.push(ValidationIssue { path: pointer(&path, item), code: IssueCode::SlugInvalid });
                        }
                        if !value.is_finite() {
                            issues.push(ValidationIssue { path: pointer(&path, item), code: IssueCode::TypeInvalid });
                        }
                    }
                }
            }
        }
    }
    sorted(issues)
}

/// 🤔️ The first [`thinking_issues`] entry, or `None` when a proctor may admit the state.
pub fn thinking_problem(state: &ThinkingState) -> Option<ValidationIssue> {
    thinking_issues(state).into_iter().next()
}

/// 🚨️ The first [`presence_issues`] entry, or `None` when a proctor may admit the state.
pub fn presence_problem(state: &PresenceState) -> Option<ValidationIssue> {
    presence_issues(state).into_iter().next()
}

/// 🕹️ The first [`cursor_issues`] entry, or `None` when a proctor may admit the state.
pub fn cursor_problem(state: &CursorState) -> Option<ValidationIssue> {
    cursor_issues(state).into_iter().next()
}

/// 🔰️ Whether `value` is a public learner tag: 8 lowercase hex digits.
pub fn is_tag(value: &str) -> bool {
    value.len() == 8 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// ⛵️ Whether `value` is an anchor: `^[a-z0-9]+(?:[:-][a-z0-9]+)*$` and at most 64 characters.
pub fn is_anchor(value: &str) -> bool {
    value.len() <= 64 && value.split([':', '-']).all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

fn limit(issues: &mut Vec<ValidationIssue>, path: String, count: usize) {
    if count > THINKING_LIMIT {
        issues.push(ValidationIssue { path, code: IssueCode::TooMany });
    }
}

fn tag(problems: &mut Vec<ValidationIssue>, value: &str) {
    if !is_tag(value) {
        problems.push(issue("/tag", IssueCode::TagInvalid));
    }
}

fn issue(path: &str, code: IssueCode) -> ValidationIssue {
    ValidationIssue { path: path.to_string(), code }
}

fn sorted(mut issues: Vec<ValidationIssue>) -> Vec<ValidationIssue> {
    issues.sort_by(|left, right| left.path.cmp(&right.path).then_with(|| left.code.as_str().cmp(right.code.as_str())));
    issues.dedup();
    issues
}


#[derive(Debug)]
struct PresentationError;
impl std::fmt::Display for PresentationError { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("presentation-invalid") } }
impl std::error::Error for PresentationError {}
impl ser::Error for PresentationError { fn custom<T: std::fmt::Display>(_: T) -> Self { Self } }

struct PresentationSerializer;
struct PresentationSeq(Vec<Value>);
struct PresentationMap { values: Object, key: Option<String> }

impl Serializer for PresentationSerializer {
    type Ok = Value;
    type Error = PresentationError;
    type SerializeSeq = PresentationSeq;
    type SerializeTuple = PresentationSeq;
    type SerializeTupleStruct = PresentationSeq;
    type SerializeTupleVariant = ser::Impossible<Value, PresentationError>;
    type SerializeMap = PresentationMap;
    type SerializeStruct = PresentationMap;
    type SerializeStructVariant = ser::Impossible<Value, PresentationError>;
    fn serialize_bool(self, value: bool) -> Result<Value, PresentationError> { Ok(Value::Bool(value)) }
    fn serialize_i8(self, value: i8) -> Result<Value, PresentationError> { self.serialize_i64(value.into()) }
    fn serialize_i16(self, value: i16) -> Result<Value, PresentationError> { self.serialize_i64(value.into()) }
    fn serialize_i32(self, value: i32) -> Result<Value, PresentationError> { self.serialize_i64(value.into()) }
    fn serialize_i64(self, value: i64) -> Result<Value, PresentationError> { Ok(Value::Number(Number::Int(value))) }
    fn serialize_u8(self, value: u8) -> Result<Value, PresentationError> { self.serialize_u64(value.into()) }
    fn serialize_u16(self, value: u16) -> Result<Value, PresentationError> { self.serialize_u64(value.into()) }
    fn serialize_u32(self, value: u32) -> Result<Value, PresentationError> { self.serialize_u64(value.into()) }
    fn serialize_u64(self, value: u64) -> Result<Value, PresentationError> { Ok(Value::Number(Number::UInt(value))) }
    fn serialize_f32(self, value: f32) -> Result<Value, PresentationError> { self.serialize_f64(value.into()) }
    fn serialize_f64(self, value: f64) -> Result<Value, PresentationError> { if value.is_finite() { Ok(Value::Number(Number::Float(value))) } else { Err(PresentationError) } }
    fn serialize_char(self, value: char) -> Result<Value, PresentationError> { self.serialize_str(&value.to_string()) }
    fn serialize_str(self, value: &str) -> Result<Value, PresentationError> { Ok(Value::String(value.to_string())) }
    fn serialize_bytes(self, value: &[u8]) -> Result<Value, PresentationError> { Ok(Value::Array(value.iter().map(|byte| Value::Number(Number::UInt((*byte).into()))).collect())) }
    fn serialize_none(self) -> Result<Value, PresentationError> { Ok(Value::Null) }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Value, PresentationError> { value.serialize(self) }
    fn serialize_unit(self) -> Result<Value, PresentationError> { Ok(Value::Null) }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Value, PresentationError> { self.serialize_unit() }
    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<Value, PresentationError> { self.serialize_str(variant) }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _: &'static str, value: &T) -> Result<Value, PresentationError> { value.serialize(self) }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Value, PresentationError> { Err(PresentationError) }
    fn serialize_seq(self, _: Option<usize>) -> Result<PresentationSeq, PresentationError> { Ok(PresentationSeq(Vec::new())) }
    fn serialize_tuple(self, _: usize) -> Result<PresentationSeq, PresentationError> { self.serialize_seq(None) }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<PresentationSeq, PresentationError> { self.serialize_seq(None) }
    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, PresentationError> { Err(PresentationError) }
    fn serialize_map(self, _: Option<usize>) -> Result<PresentationMap, PresentationError> { Ok(PresentationMap { values: Object::new(), key: None }) }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<PresentationMap, PresentationError> { self.serialize_map(None) }
    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, PresentationError> { Err(PresentationError) }
}

impl SerializeSeq for PresentationSeq {
    type Ok = Value;
    type Error = PresentationError;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PresentationError> { self.0.push(value.serialize(PresentationSerializer)?); Ok(()) }
    fn end(self) -> Result<Value, PresentationError> { Ok(Value::Array(self.0)) }
}
impl SerializeTuple for PresentationSeq {
    type Ok = Value;
    type Error = PresentationError;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PresentationError> { SerializeSeq::serialize_element(self, value) }
    fn end(self) -> Result<Value, PresentationError> { SerializeSeq::end(self) }
}
impl SerializeTupleStruct for PresentationSeq {
    type Ok = Value;
    type Error = PresentationError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PresentationError> { SerializeSeq::serialize_element(self, value) }
    fn end(self) -> Result<Value, PresentationError> { SerializeSeq::end(self) }
}
impl SerializeMap for PresentationMap {
    type Ok = Value;
    type Error = PresentationError;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PresentationError> { let Value::String(key) = value.serialize(PresentationSerializer)? else { return Err(PresentationError) }; self.key = Some(key); Ok(()) }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PresentationError> { self.values.insert(self.key.take().ok_or(PresentationError)?, value.serialize(PresentationSerializer)?); Ok(()) }
    fn end(self) -> Result<Value, PresentationError> { Ok(Value::Object(self.values)) }
}
impl SerializeStruct for PresentationMap {
    type Ok = Value;
    type Error = PresentationError;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), PresentationError> { self.values.insert(key, value.serialize(PresentationSerializer)?); Ok(()) }
    fn end(self) -> Result<Value, PresentationError> { Ok(Value::Object(self.values)) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
