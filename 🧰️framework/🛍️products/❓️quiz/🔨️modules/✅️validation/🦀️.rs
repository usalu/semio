//! ✅️ Validation without any schema library: the constraints serde cannot express (slug patterns,
//! lengths, cardinalities, consts, draws) and the semantic rules of quizzes and catalogs (unique ids,
//! references, positive values on logarithmic scales, complete in-range profiles, a value per
//! dimension, reachable badges), plus the validity and completeness of answers against a sheet task
//! (design §5).
//!
//! Issues carry a JSON Pointer (RFC 6901) into the validated document and the kebab-case code of the
//! TypeScript twin; they are deduplicated and sorted by path, then code, in code point order (UTF-8
//! byte order). Structural codes the TypeScript twin reports for untyped input (`type-invalid`,
//! `required`, `property-unknown`, `integer-invalid`, `value-invalid` of enums) surface here as serde
//! deserialization errors instead.
//!
//! @see <https://www.rfc-editor.org/rfc/rfc6901> — JSON Pointer
//! @see ../../🧬️schema/🔣️.json — the constraints mirrored here
//! @see ../✅️validation/🟦️.ts — the TypeScript twin

use crate::schema::{Answer, Axis, Badge, BadgeRule, Catalog, Category, ClassificationTask, MatchingItem, MatchingTask, Quantity, Quiz, Rejection, Scale, SheetItem, SheetTask, SortingTask, Task, Text};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// 🔏️ The `schema` const of a quiz document.
pub const QUIZ_SCHEMA: &str = "semio.quiz/v1";

/// 📒️ The `schema` const of a catalog document.
pub const CATALOG_SCHEMA: &str = "semio.quiz.catalog/v1";

/// 🩺️ What is wrong at a path — the shared vocabulary of both cores.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IssueCode {
    TypeInvalid,
    Required,
    PropertyUnknown,
    LengthInvalid,
    SlugInvalid,
    IntegerInvalid,
    BelowMinimum,
    ValueInvalid,
    ItemsTooFew,
    PropertiesTooFew,
    DuplicateId,
    DuplicatePath,
    AxisRangeInvalid,
    AxesMissing,
    ProfileIncomplete,
    AxisUnknown,
    ProfileOutOfRange,
    CategoryUnknown,
    DrawExceedsItems,
    ValueNotPositive,
    ValueMissing,
    DimensionUnknown,
    QuizCountMismatch,
    QuizUnknown,
    BadgeUnreachable,
    TagInvalid,
    AnchorInvalid,
    OutOfRange,
    QuizOutsideRun,
    TaskWithoutRun,
    TooMany,
}

impl IssueCode {
    /// 💬️ The wire string, e.g. `duplicate-id`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TypeInvalid => "type-invalid",
            Self::Required => "required",
            Self::PropertyUnknown => "property-unknown",
            Self::LengthInvalid => "length-invalid",
            Self::SlugInvalid => "slug-invalid",
            Self::IntegerInvalid => "integer-invalid",
            Self::BelowMinimum => "below-minimum",
            Self::ValueInvalid => "value-invalid",
            Self::ItemsTooFew => "items-too-few",
            Self::PropertiesTooFew => "properties-too-few",
            Self::DuplicateId => "duplicate-id",
            Self::DuplicatePath => "duplicate-path",
            Self::AxisRangeInvalid => "axis-range-invalid",
            Self::AxesMissing => "axes-missing",
            Self::ProfileIncomplete => "profile-incomplete",
            Self::AxisUnknown => "axis-unknown",
            Self::ProfileOutOfRange => "profile-out-of-range",
            Self::CategoryUnknown => "category-unknown",
            Self::DrawExceedsItems => "draw-exceeds-items",
            Self::ValueNotPositive => "value-not-positive",
            Self::ValueMissing => "value-missing",
            Self::DimensionUnknown => "dimension-unknown",
            Self::QuizCountMismatch => "quiz-count-mismatch",
            Self::QuizUnknown => "quiz-unknown",
            Self::BadgeUnreachable => "badge-unreachable",
            Self::TagInvalid => "tag-invalid",
            Self::AnchorInvalid => "anchor-invalid",
            Self::OutOfRange => "out-of-range",
            Self::QuizOutsideRun => "quiz-outside-run",
            Self::TaskWithoutRun => "task-without-run",
            Self::TooMany => "too-many",
        }
    }
}

/// 🩹️ One finding: where (JSON Pointer) and what (code).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub path: String,
    pub code: IssueCode,
}

/// 🔎️ Every issue of one quiz document.
pub fn quiz_issues(quiz: &Quiz) -> Vec<ValidationIssue> {
    let mut issues = Issues::default();
    if quiz.schema != QUIZ_SCHEMA {
        issues.push("/schema".to_string(), IssueCode::ValueInvalid);
    }
    issues.slug("/id".to_string(), &quiz.id);
    issues.length("/emoji".to_string(), &quiz.emoji, 1, 16);
    issues.text("/title", &quiz.title);
    issues.text("/description", &quiz.description);
    issues.at_least("/tasks".to_string(), quiz.tasks.len(), 1, IssueCode::ItemsTooFew);
    for (index, task) in quiz.tasks.iter().enumerate() {
        let base = format!("/tasks/{index}");
        issues.slug(format!("{base}/id"), task.id());
        match task {
            Task::Classification(task) => issues.classification(&base, task),
            Task::Sorting(task) => issues.sorting(&base, task),
            Task::Matching(task) => issues.matching(&base, task),
        }
    }
    issues.unique("/tasks", quiz.tasks.iter().map(|task| task.id().as_str()));
    issues.finish()
}

/// 🗄️ Every issue of a catalog given its loaded quizzes in catalog order: structure, unique paths,
/// quiz ids and badge ids, and badge rules that reference loaded quizzes and select at least one task.
/// The quizzes' own issues are [`quiz_issues`].
pub fn catalog_issues(catalog: &Catalog, quizzes: &[Quiz]) -> Vec<ValidationIssue> {
    let mut issues = Issues::default();
    if catalog.schema != CATALOG_SCHEMA {
        issues.push("/schema".to_string(), IssueCode::ValueInvalid);
    }
    issues.slug("/id".to_string(), &catalog.id);
    issues.text("/title", &catalog.title);
    issues.text("/introduction/title", &catalog.introduction.title);
    issues.at_least("/introduction/paragraphs".to_string(), catalog.introduction.paragraphs.len(), 1, IssueCode::ItemsTooFew);
    for (index, paragraph) in catalog.introduction.paragraphs.iter().enumerate() {
        issues.text(&format!("/introduction/paragraphs/{index}"), paragraph);
    }
    issues.at_least("/quizzes".to_string(), catalog.quizzes.len(), 1, IssueCode::ItemsTooFew);
    let mut paths = BTreeSet::new();
    for (index, path) in catalog.quizzes.iter().enumerate() {
        if issues.length(format!("/quizzes/{index}"), path, 1, usize::MAX) && !paths.insert(path.as_str()) {
            issues.push(format!("/quizzes/{index}"), IssueCode::DuplicatePath);
        }
    }
    if catalog.quizzes.len() != quizzes.len() {
        issues.push("/quizzes".to_string(), IssueCode::QuizCountMismatch);
    }
    let mut quiz_ids = BTreeSet::new();
    for (index, quiz) in quizzes.iter().enumerate() {
        if !quiz_ids.insert(quiz.id.as_str()) {
            issues.push(format!("/quizzes/{index}"), IssueCode::DuplicateId);
        }
    }
    for (index, badge) in catalog.badges.iter().enumerate() {
        issues.badge(&format!("/badges/{index}"), badge, quizzes, &quiz_ids);
    }
    issues.unique("/badges", catalog.badges.iter().map(|badge| badge.id.as_str()));
    issues.finish()
}

/// 🚧️ `answer-invalid` unless the answer fits the sheet task: the kind matches, every referenced
/// item, category and dimension exists, card indices are in range and unique per dimension, and a
/// sorting order is a permutation of the sheet items. Partial classification and matching answers
/// are valid.
pub fn answer_rejection(sheet_task: &SheetTask, answer: &Answer) -> Option<Rejection> {
    let valid = match (sheet_task, answer) {
        (SheetTask::Classification(task), Answer::Classification(answer)) => answer.assignments.iter().all(|(item, category)| presented(&task.items, item) && task.categories.iter().any(|candidate| &candidate.id == category)),
        (SheetTask::Sorting(task), Answer::Sorting(answer)) => answer.order.len() == task.items.len() && answer.order.iter().collect::<BTreeSet<_>>().len() == answer.order.len() && answer.order.iter().all(|item| presented(&task.items, item)),
        (SheetTask::Matching(task), Answer::Matching(answer)) => answer.assignments.iter().all(|(dimension, assignment)| {
            task.dimensions.iter().find(|candidate| &candidate.id == dimension).is_some_and(|dimension| {
                let mut used = BTreeSet::new();
                assignment.iter().all(|(item, &card)| presented(&task.items, item) && card < dimension.cards.len() && used.insert(card))
            })
        }),
        _ => false,
    };
    (!valid).then_some(Rejection::AnswerInvalid)
}

/// ☑️ Whether an answer completes its sheet task: every item classified, every item matched in every
/// dimension; a recorded sorting of the same kind is always complete.
pub fn answer_complete(sheet_task: &SheetTask, answer: Option<&Answer>) -> bool {
    match (sheet_task, answer) {
        (SheetTask::Classification(task), Some(Answer::Classification(answer))) => task.items.iter().all(|item| answer.assignments.contains_key(&item.id)),
        (SheetTask::Sorting(_), Some(Answer::Sorting(_))) => true,
        (SheetTask::Matching(task), Some(Answer::Matching(answer))) => task.dimensions.iter().all(|dimension| answer.assignments.get(&dimension.id).is_some_and(|assignment| task.items.iter().all(|item| assignment.contains_key(&item.id)))),
        _ => false,
    }
}

/// 🐍️ Whether `value` is a slug: `^[a-z0-9]+(?:-[a-z0-9]+)*$` and at most 64 characters.
pub fn is_slug(value: &str) -> bool {
    value.len() <= 64 && value.split('-').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

/// 🪡️ `base` extended by one JSON Pointer reference token (RFC 6901 escaping of `~` and `/`).
pub fn pointer(base: &str, key: &str) -> String {
    format!("{base}/{}", key.replace('~', "~0").replace('/', "~1"))
}

fn presented(items: &[SheetItem], id: &str) -> bool {
    items.iter().any(|item| item.id == id)
}

#[derive(Default)]
struct Issues(Vec<ValidationIssue>);

impl Issues {
    fn finish(mut self) -> Vec<ValidationIssue> {
        self.0.sort_by(|left, right| left.path.cmp(&right.path).then_with(|| left.code.as_str().cmp(right.code.as_str())));
        self.0.dedup();
        self.0
    }

    fn push(&mut self, path: String, code: IssueCode) {
        self.0.push(ValidationIssue { path, code });
    }

    fn slug(&mut self, path: String, value: &str) -> bool {
        let valid = is_slug(value);
        if !valid {
            self.push(path, IssueCode::SlugInvalid);
        }
        valid
    }

    fn length(&mut self, path: String, value: &str, minimum: usize, maximum: usize) -> bool {
        let valid = (minimum..=maximum).contains(&value.chars().count());
        if !valid {
            self.push(path, IssueCode::LengthInvalid);
        }
        valid
    }

    fn text(&mut self, base: &str, text: &Text) {
        self.length(format!("{base}/en"), &text.en, 1, usize::MAX);
        self.length(format!("{base}/de"), &text.de, 1, usize::MAX);
    }

    fn optional_text(&mut self, base: &str, text: Option<&Text>) {
        if let Some(text) = text {
            self.text(base, text);
        }
    }

    fn at_least(&mut self, path: String, count: usize, minimum: usize, code: IssueCode) {
        if count < minimum {
            self.push(path, code);
        }
    }

    fn unique<'a>(&mut self, base: &str, ids: impl Iterator<Item = &'a str>) -> BTreeSet<&'a str> {
        let mut seen = BTreeSet::new();
        for (index, id) in ids.enumerate() {
            if !seen.insert(id) {
                self.push(format!("{base}/{index}/id"), IssueCode::DuplicateId);
            }
        }
        seen
    }

    fn quantity(&mut self, base: &str, quantity: &Quantity) {
        self.text(&format!("{base}/label"), &quantity.label);
        self.length(format!("{base}/unit"), &quantity.unit, 1, 32);
    }

    fn head(&mut self, base: &str, id: &str, label: &Text, explanation: Option<&Text>) {
        self.slug(format!("{base}/id"), id);
        self.text(&format!("{base}/label"), label);
        self.optional_text(&format!("{base}/explanation"), explanation);
    }

    fn items_and_draw<'a>(&mut self, base: &str, draw: Option<usize>, heads: impl ExactSizeIterator<Item = (&'a str, &'a Text, Option<&'a Text>)> + Clone) {
        let count = heads.len();
        self.at_least(format!("{base}/items"), count, 2, IssueCode::ItemsTooFew);
        for (index, (id, label, explanation)) in heads.clone().enumerate() {
            self.head(&format!("{base}/items/{index}"), id, label, explanation);
        }
        self.unique(&format!("{base}/items"), heads.map(|(id, _, _)| id));
        match draw {
            Some(draw) if draw < 2 => self.push(format!("{base}/draw"), IssueCode::BelowMinimum),
            Some(draw) if draw > count => self.push(format!("{base}/draw"), IssueCode::DrawExceedsItems),
            _ => {}
        }
    }

    fn classification(&mut self, base: &str, task: &ClassificationTask) {
        self.text(&format!("{base}/title"), &task.title);
        self.text(&format!("{base}/prompt"), &task.prompt);
        let axes: &[Axis] = task.axes.as_deref().unwrap_or_default();
        if task.axes.is_some() {
            self.at_least(format!("{base}/axes"), axes.len(), 3, IssueCode::ItemsTooFew);
        }
        let mut ranges: BTreeMap<&str, (f64, f64)> = BTreeMap::new();
        for (index, axis) in axes.iter().enumerate() {
            let path = format!("{base}/axes/{index}");
            self.slug(format!("{path}/id"), &axis.id);
            self.text(&format!("{path}/label"), &axis.label);
            self.length(format!("{path}/unit"), &axis.unit, 1, 32);
            if axis.max <= axis.min {
                self.push(format!("{path}/max"), IssueCode::AxisRangeInvalid);
            } else {
                ranges.insert(&axis.id, (axis.min, axis.max));
            }
        }
        let axis_ids = self.unique(&format!("{base}/axes"), axes.iter().map(|axis| axis.id.as_str()));
        self.at_least(format!("{base}/categories"), task.categories.len(), 2, IssueCode::ItemsTooFew);
        for (index, category) in task.categories.iter().enumerate() {
            self.category(&format!("{base}/categories/{index}"), category, task.axes.is_some(), &axis_ids, &ranges);
        }
        let category_ids = self.unique(&format!("{base}/categories"), task.categories.iter().map(|category| category.id.as_str()));
        for (index, item) in task.items.iter().enumerate() {
            let path = format!("{base}/items/{index}/category");
            if self.slug(path.clone(), &item.category) && !category_ids.contains(item.category.as_str()) {
                self.push(path, IssueCode::CategoryUnknown);
            }
        }
        self.items_and_draw(base, task.draw, task.items.iter().map(|item| (item.id.as_str(), &item.label, item.explanation.as_ref())));
    }

    fn category(&mut self, base: &str, category: &Category, has_axes: bool, axis_ids: &BTreeSet<&str>, ranges: &BTreeMap<&str, (f64, f64)>) {
        self.slug(format!("{base}/id"), &category.id);
        self.text(&format!("{base}/label"), &category.label);
        self.optional_text(&format!("{base}/description"), category.description.as_ref());
        let Some(profile) = &category.profile else { return };
        let path = format!("{base}/profile");
        self.at_least(path.clone(), profile.len(), 1, IssueCode::PropertiesTooFew);
        for key in profile.keys() {
            self.slug(pointer(&path, key), key);
        }
        if !has_axes {
            return self.push(path, IssueCode::AxesMissing);
        }
        for axis in axis_ids.iter().filter(|axis| !profile.contains_key(**axis)) {
            self.push(pointer(&path, axis), IssueCode::ProfileIncomplete);
        }
        for (key, &value) in profile {
            if !axis_ids.contains(key.as_str()) {
                self.push(pointer(&path, key), IssueCode::AxisUnknown);
            }
            if ranges.get(key.as_str()).is_some_and(|&(min, max)| value < min || value > max) {
                self.push(pointer(&path, key), IssueCode::ProfileOutOfRange);
            }
        }
    }

    fn sorting(&mut self, base: &str, task: &SortingTask) {
        self.text(&format!("{base}/title"), &task.title);
        self.text(&format!("{base}/prompt"), &task.prompt);
        self.quantity(&format!("{base}/quantity"), &task.quantity);
        if task.quantity.scale == Scale::Logarithmic {
            for (index, _) in task.items.iter().enumerate().filter(|(_, item)| item.value <= 0.0) {
                self.push(format!("{base}/items/{index}/value"), IssueCode::ValueNotPositive);
            }
        }
        self.items_and_draw(base, task.draw, task.items.iter().map(|item| (item.id.as_str(), &item.label, item.explanation.as_ref())));
    }

    fn matching(&mut self, base: &str, task: &MatchingTask) {
        self.text(&format!("{base}/title"), &task.title);
        self.text(&format!("{base}/prompt"), &task.prompt);
        self.at_least(format!("{base}/dimensions"), task.dimensions.len(), 1, IssueCode::ItemsTooFew);
        let mut scales: BTreeMap<&str, Scale> = BTreeMap::new();
        for (index, dimension) in task.dimensions.iter().enumerate() {
            self.quantity(&format!("{base}/dimensions/{index}/quantity"), &dimension.quantity);
            if self.slug(format!("{base}/dimensions/{index}/id"), &dimension.id) {
                scales.entry(&dimension.id).or_insert(dimension.quantity.scale);
            }
        }
        self.unique(&format!("{base}/dimensions"), task.dimensions.iter().map(|dimension| dimension.id.as_str()));
        for (index, item) in task.items.iter().enumerate() {
            self.values(&format!("{base}/items/{index}/values"), item, &scales);
        }
        self.items_and_draw(base, task.draw, task.items.iter().map(|item| (item.id.as_str(), &item.label, item.explanation.as_ref())));
    }

    fn values(&mut self, path: &str, item: &MatchingItem, scales: &BTreeMap<&str, Scale>) {
        self.at_least(path.to_string(), item.values.len(), 1, IssueCode::PropertiesTooFew);
        for key in item.values.keys() {
            self.slug(pointer(path, key), key);
        }
        for dimension in scales.keys().filter(|dimension| !item.values.contains_key(**dimension)) {
            self.push(pointer(path, dimension), IssueCode::ValueMissing);
        }
        for (key, &value) in &item.values {
            match scales.get(key.as_str()) {
                None => self.push(pointer(path, key), IssueCode::DimensionUnknown),
                Some(Scale::Logarithmic) if value <= 0.0 => self.push(pointer(path, key), IssueCode::ValueNotPositive),
                Some(_) => {}
            }
        }
    }

    fn badge(&mut self, base: &str, badge: &Badge, quizzes: &[Quiz], quiz_ids: &BTreeSet<&str>) {
        self.slug(format!("{base}/id"), &badge.id);
        self.length(format!("{base}/emoji"), &badge.emoji, 1, 16);
        self.text(&format!("{base}/label"), &badge.label);
        self.text(&format!("{base}/description"), &badge.description);
        let (quiz, task_kind) = match &badge.rule {
            BadgeRule::PerfectQuiz { quiz } => (Some(quiz), None),
            BadgeRule::PerfectTasks { task_kind, quiz } => (quiz.as_ref(), *task_kind),
            BadgeRule::CompletedQuizzes => return,
        };
        if quiz.is_some_and(|quiz| !self.slug(format!("{base}/rule/quiz"), quiz)) {
            return;
        }
        if quiz.is_some_and(|quiz| !quiz_ids.contains(quiz.as_str())) {
            return self.push(format!("{base}/rule/quiz"), IssueCode::QuizUnknown);
        }
        let selects = |candidate: &Quiz| quiz.is_none_or(|quiz| &candidate.id == quiz) && candidate.tasks.iter().any(|task| task_kind.is_none_or(|kind| task.kind() == kind));
        if matches!(badge.rule, BadgeRule::PerfectTasks { .. }) && !quizzes.iter().any(selects) {
            self.push(format!("{base}/rule"), IssueCode::BadgeUnreachable);
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
