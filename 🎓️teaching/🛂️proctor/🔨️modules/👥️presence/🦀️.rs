//! 👥️ The presence rooms of the served course and what may be shared in them (design §15–§17).
//!
//! | Scope | Room | State |
//! |---|---|---|
//! | `<course>` | the roster | `PresenceState`: who is online, where, whether the tab is visible |
//! | `<course>/introduction`, `/home`, `/leaderboard`, `/badges`, `/learner`, `/preferences` | a page | `CursorState` without a drag |
//! | `<course>/quiz/<quiz>` | a quiz: its page, runs and results | `CursorState`, dragging only items of the quiz |
//! | `<course>/quiz/<quiz>/thinking` | what the learners in a run of the quiz currently think | `ThinkingState` |
//!
//! No other scope is a room: joining or watching one is closed by policy and a state published into
//! one is refused. The rules are the quiz core's (`presence_problem`, `cursor_problem`,
//! `thinking_problem`); the proctor adds only what the core cannot know — which quizzes, tasks,
//! items, categories and dimensions this course has, so peers aggregating by id never meet an id
//! nobody renders. A refusal reason is the issue code followed by its JSON pointer
//! (`tag-invalid /tag`), or [`STATE_INVALID`] for a state that is not the room's type, or
//! [`SCOPE_UNKNOWN`].
//!
//! @see ../../../../🧰️framework/🛍️products/📚️course/🔨️modules/👥️presence/🦀️.rs — the rules and scopes
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🛰️presence-stream/🦀️.rs — the presence socket

use std::collections::{HashMap, HashSet};

use course_presence::{cursor_problem, presence_problem, room_scope, roster_scope, thinking_problem, thinking_scope};
use course::{CursorState, Place, PresenceState, Quiz, Scale, Screen, Task, ThinkingAnswer, ThinkingState, ValidationIssue};
use serde::Deserialize;
use server::contract::OpaqueJson;

use crate::course::LoadedCourse;

/// 🚫️ The reason for a state published into a scope that is no room of this course.
pub const SCOPE_UNKNOWN: &str = "scope-unknown";
/// 🚫️ The reason for a state that is not the room's state type.
pub const STATE_INVALID: &str = "state-invalid";
/// 🚫️ The code of a place naming a quiz this course does not list.
pub const QUIZ_UNKNOWN: &str = "quiz-unknown";
/// 🚫️ The code of a place or draft naming a task its quiz does not have.
pub const TASK_UNKNOWN: &str = "task-unknown";
/// 🚫️ The code of a draft answer whose kind is not its task's kind.
pub const KIND_MISMATCH: &str = "kind-mismatch";
/// 🚫️ The code of a drag or draft naming an item, category or dimension its task does not have.
pub const ID_UNKNOWN: &str = "id-unknown";
/// 🚫️ The code of a guess or matching value no answer can carry: one that is not positive on a
/// logarithmic scale.
pub const VALUE_INVALID: &str = "value-invalid";

/// 🏠️ What one room carries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Room {
    Roster,
    Page,
    Quiz(String),
    Thinking(String),
}

/// 🧩️ The ids one task renders, by the kind of answer it takes, and the scale its numbers are
/// guessed on: the sorting's quantity, per matching dimension its own.
#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Classification { items: HashSet<String>, categories: HashSet<String> },
    Sorting { items: HashSet<String>, scale: Scale },
    Matching { items: HashSet<String>, dimensions: HashMap<String, Scale> },
}

/// 🗺️ Every presence room of one course, and the ids each quiz renders.
#[derive(Clone, Debug, PartialEq)]
pub struct Rooms {
    roster: String,
    rooms: HashMap<String, Room>,
    tasks: HashMap<String, HashMap<String, Shape>>,
    items: HashMap<String, HashSet<String>>,
}

impl Rooms {
    /// 🧭️ The rooms of `course`: its roster, its six shared pages, and a quiz room and a thinking
    /// room per quiz.
    pub fn of(course: &LoadedCourse) -> Self {
        let id = course.id();
        let pages = [Screen::Introduction, Screen::Home, Screen::Leaderboard, Screen::Badges, Screen::Learner, Screen::Preferences].into_iter().filter_map(|screen| room_scope(id, &Place { screen, quiz: None, task: None })).map(|scope| (scope, Room::Page));
        let quizzes = course.entries.iter().flat_map(|entry| {
            let quiz = entry.quiz.id.clone();
            room_scope(id, &Place { screen: Screen::Quiz, quiz: Some(quiz.clone()), task: None }).map(|scope| (scope, Room::Quiz(quiz.clone()))).into_iter().chain([(thinking_scope(id, &quiz), Room::Thinking(quiz))])
        });
        let tasks: HashMap<String, HashMap<String, Shape>> = course.entries.iter().map(|entry| (entry.quiz.id.clone(), shapes(&entry.quiz))).collect();
        let items = tasks.iter().map(|(quiz, shapes)| (quiz.clone(), shapes.values().flat_map(|shape| shape.items().iter().cloned()).collect())).collect();
        Self { roster: roster_scope(id), rooms: pages.chain(quizzes).collect(), tasks, items }
    }

    /// 🔎️ The room `scope` names, if it is one.
    pub fn room(&self, scope: &str) -> Option<Room> {
        if scope == self.roster {
            Some(Room::Roster)
        } else {
            self.rooms.get(scope).cloned()
        }
    }

    /// 🧾️ Every room scope: the roster first, then the others in order.
    pub fn scopes(&self) -> Vec<String> {
        let mut rooms: Vec<String> = self.rooms.keys().cloned().collect();
        rooms.sort();
        std::iter::once(self.roster.clone()).chain(rooms).collect()
    }

    /// 🛃️ Admit `state` into `scope`, or name why not.
    pub fn admit(&self, scope: &str, state: &OpaqueJson) -> Result<(), String> {
        match self.room(scope) {
            None => Err(SCOPE_UNKNOWN.to_string()),
            Some(Room::Roster) => {
                let state: PresenceState = shaped(state)?;
                presence_problem(&state).map_or_else(|| { self.place(&state.place)?; self.presentation(&state) }, |issue| Err(reason(&issue)))
            }
            Some(Room::Page) => {
                let state: CursorState = shaped(state)?;
                cursor_problem(&state).map_or_else(|| { self.cursor_place(scope, &state)?; state.drag.map_or(Ok(()), |_| Err(format!("{ID_UNKNOWN} /drag/item"))) }, |issue| Err(reason(&issue)))
            }
            Some(Room::Quiz(quiz)) => {
                let state: CursorState = shaped(state)?;
                cursor_problem(&state).map_or_else(|| { self.cursor_place(scope, &state)?; self.drag(&quiz, &state) }, |issue| Err(reason(&issue)))
            }
            Some(Room::Thinking(quiz)) => {
                let state: ThinkingState = shaped(state)?;
                thinking_problem(&state).map_or_else(|| self.drafts(&quiz, &state), |issue| Err(reason(&issue)))
            }
        }
    }

    fn place(&self, place: &Place) -> Result<(), String> {
        let Some(quiz) = &place.quiz else { return Ok(()) };
        let Some(tasks) = self.tasks.get(quiz) else { return Err(format!("{QUIZ_UNKNOWN} /place/quiz")) };
        match &place.task {
            Some(task) if !tasks.contains_key(task) => Err(format!("{TASK_UNKNOWN} /place/task")),
            _ => Ok(()),
        }
    }

    fn cursor_place(&self, scope: &str, state: &CursorState) -> Result<(), String> {
        let Some(place) = &state.place else { return Ok(()) };
        self.place(place)?;
        if room_scope(&self.roster, place).as_deref() != Some(scope) { return Err(format!("{SCOPE_UNKNOWN} /place")); }
        Ok(())
    }

    fn presentation(&self, state: &PresenceState) -> Result<(), String> {
        let Some(run) = state.view.as_ref().and_then(|view| view.run.as_ref()) else { return Ok(()) };
        if state.place.quiz.as_deref() != Some(&run.quiz) || run.sheet.quiz != run.quiz || !matches!(state.place.screen, Screen::Run | Screen::Results) {
            return Err(format!("{QUIZ_UNKNOWN} /view/run/quiz"));
        }
        let tasks = self.tasks.get(&run.quiz).ok_or_else(|| format!("{QUIZ_UNKNOWN} /view/run/quiz"))?;
        for (index, task) in run.sheet.tasks.iter().enumerate() {
            if !tasks.contains_key(task.id()) { return Err(format!("{TASK_UNKNOWN} /view/run/sheet/tasks/{index}/id")); }
        }
        for (task, answer) in &run.answers {
            let shown = run.sheet.tasks.iter().find(|shown| shown.id() == task).ok_or_else(|| format!("{TASK_UNKNOWN} /view/run/answers/{task}"))?;
            if course::answer_rejection(shown, answer).is_some() { return Err(format!("{VALUE_INVALID} /view/run/answers/{task}")); }
        }
        Ok(())
    }

    fn drag(&self, quiz: &str, state: &CursorState) -> Result<(), String> {
        match &state.drag {
            Some(drag) if !self.items.get(quiz).is_some_and(|items| items.contains(&drag.item)) => Err(format!("{ID_UNKNOWN} /drag/item")),
            _ => Ok(()),
        }
    }

    fn drafts(&self, quiz: &str, state: &ThinkingState) -> Result<(), String> {
        let tasks = self.tasks.get(quiz).ok_or_else(|| SCOPE_UNKNOWN.to_string())?;
        for (task, answer) in &state.answers {
            let at = format!("/answers/{task}");
            let Some(shape) = tasks.get(task) else { return Err(format!("{TASK_UNKNOWN} {at}")) };
            let unknown = |path: String| Err(format!("{ID_UNKNOWN} {path}"));
            match (shape, answer) {
                (Shape::Classification { items, categories }, ThinkingAnswer::Classification(answer)) => {
                    if let Some((item, _)) = answer.assignments.iter().find(|(item, category)| !items.contains(*item) || !categories.contains(*category)) {
                        return unknown(format!("{at}/assignments/{item}"));
                    }
                }
                (Shape::Sorting { items, scale }, ThinkingAnswer::Sorting(answer)) => {
                    if let Some(index) = answer.order.iter().position(|item| !items.contains(item)) {
                        return unknown(format!("{at}/order/{index}"));
                    }
                    let guesses = answer.guesses.iter().flatten();
                    if let Some((item, _)) = guesses.clone().find(|(item, _)| !items.contains(*item)) {
                        return unknown(format!("{at}/guesses/{item}"));
                    }
                    if let Some((item, _)) = guesses.clone().find(|(_, guess)| !carried(*scale, **guess)) {
                        return Err(format!("{VALUE_INVALID} {at}/guesses/{item}"));
                    }
                }
                (Shape::Matching { items, dimensions }, ThinkingAnswer::Matching(answer)) => {
                    for (dimension, assigned) in &answer.values {
                        let Some(scale) = dimensions.get(dimension) else { return unknown(format!("{at}/values/{dimension}")) };
                        if let Some(item) = assigned.keys().find(|item| !items.contains(*item)) {
                            return unknown(format!("{at}/values/{dimension}/{item}"));
                        }
                        if let Some((item, _)) = assigned.iter().find(|(_, value)| !carried(*scale, **value)) {
                            return Err(format!("{VALUE_INVALID} {at}/values/{dimension}/{item}"));
                        }
                    }
                }
                _ => return Err(format!("{KIND_MISMATCH} {at}")),
            }
        }
        Ok(())
    }
}

impl Shape {
    fn of(task: &Task) -> Self {
        match task {
            Task::Classification(task) => Self::Classification { items: ids(task.items.iter().map(|item| &item.id)), categories: ids(task.categories.iter().map(|category| &category.id)) },
            Task::Sorting(task) => Self::Sorting { items: ids(task.items.iter().map(|item| &item.id)), scale: task.quantity.scale },
            Task::Matching(task) => Self::Matching { items: ids(task.items.iter().map(|item| &item.id)), dimensions: task.dimensions.iter().map(|dimension| (dimension.id.clone(), dimension.quantity.scale)).collect() },
        }
    }

    fn items(&self) -> &HashSet<String> {
        match self {
            Self::Classification { items, .. } | Self::Sorting { items, .. } | Self::Matching { items, .. } => items,
        }
    }
}

/// 🔢️ Whether an answer can carry `value` on `scale`: a card value or a guess, finite by the core's
/// rules, and positive where the scale is logarithmic.
fn carried(scale: Scale, value: f64) -> bool {
    scale == Scale::Linear || value > 0.0
}

fn ids<'a>(ids: impl Iterator<Item = &'a String>) -> HashSet<String> {
    ids.cloned().collect()
}

fn shapes(quiz: &Quiz) -> HashMap<String, Shape> {
    quiz.tasks.iter().map(|task| (task.id().to_string(), Shape::of(task))).collect()
}

fn shaped<'a, T: Deserialize<'a>>(state: &'a OpaqueJson) -> Result<T, String> {
    T::deserialize(state).map_err(|_| STATE_INVALID.to_string())
}

fn reason(issue: &ValidationIssue) -> String {
    format!("{} {}", issue.code.as_str(), issue.path)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
