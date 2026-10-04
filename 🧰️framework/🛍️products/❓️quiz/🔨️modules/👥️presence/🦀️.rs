//! 👥️ Shared presence and cursors (design §15, §16): the presence scopes of a catalog and the admission rules
//! of the ephemeral states learners publish there. The roster scope `<catalog>` carries `PresenceState`; the
//! room scopes `<catalog>/introduction`, `<catalog>/home`, `<catalog>/leaderboard`, `<catalog>/badges` and
//! `<catalog>/quiz/<quiz>` (the quiz page, its runs and results) carry `CursorState`. The identity, learner and
//! preferences screens have no room.
//!
//! Problems use the validation vocabulary (JSON Pointer + code); structure is guaranteed by serde.
//!
//! @see ../../🧬️schema/🔣️.json — `Place`, `Anchor`, `Cursor`, `PresenceState`, `CursorState`
//! @see ../👥️presence/🟦️.ts — the TypeScript twin

use crate::schema::{ClassificationAnswer, Cursor, CursorState, Identity, Place, PresenceState, Screen, SortingAnswer, ThinkingAnswer, ThinkingMatchingAnswer, ThinkingState};
use crate::validation::{is_slug, normalize_handle, pointer, IssueCode, ValidationIssue};
use std::collections::{BTreeMap, BTreeSet};

/// 🏠️ The catalog-wide presence room: who is online and where.
pub fn roster_scope(catalog: &str) -> String {
    catalog.to_string()
}

/// 🚪️ The cursor room of a place; `None` for the identity screen (no tag exists yet), the personal learner and
/// preferences pages, and a quiz, run or results place without a quiz.
pub fn room_scope(catalog: &str, place: &Place) -> Option<String> {
    let room = match place.screen {
        Screen::Identity | Screen::Learner | Screen::Preferences => return None,
        Screen::Introduction => "introduction".to_string(),
        Screen::Home => "home".to_string(),
        Screen::Leaderboard => "leaderboard".to_string(),
        Screen::Badges => "badges".to_string(),
        Screen::Quiz | Screen::Run | Screen::Results => format!("quiz/{}", place.quiz.as_ref()?),
    };
    Some(format!("{catalog}/{room}"))
}

/// 🛃️ Every issue of a presence state, sorted by path, then code: `tag-invalid` at `/tag`, `handle-invalid`
/// at `/identity/handle` (not a normalized display handle), `slug-invalid` at `/place/quiz` or `/place/task`, and at
/// `/place/quiz` `required` for a quiz, run or results place without a quiz or `quiz-outside-run` for a quiz on
/// any other screen, `task-without-run` at `/place/task` for a task outside a run.
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
    if *screen != Screen::Run && task.is_some() {
        issues.push(issue("/place/task", IssueCode::TaskWithoutRun));
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

/// 🗯️ The live thinking room of a quiz: `<catalog>/quiz/<quiz>/thinking`.
pub fn thinking_scope(catalog: &str, quiz: &str) -> String {
    format!("{catalog}/quiz/{quiz}/thinking")
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

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
