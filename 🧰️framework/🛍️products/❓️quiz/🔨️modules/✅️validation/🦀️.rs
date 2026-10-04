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
//! **Ids and handles.** [`is_id`] and [`is_slug`] are the shapes every id of a command or query must
//! have ([`command_rejection`], [`query_rejection`]). The handle policy ([`normalize_handle`]) owns its
//! tables: the `White_Space` set and the Latin letters of `$defs/Handle` are written out as code point
//! ranges, so both cores accept the same handles whatever Unicode version their runtime ships. `std`
//! has no normalizer and none is needed: every string over the handle alphabet is in Normalization
//! Form C (no combining mark, no character whose canonical decomposition recomposes differently), so
//! refusing everything outside the alphabet refuses every non-NFC spelling. The unit tests hold the
//! tables to the `unicode-normalization` crate and the conformance case to Python's `unicodedata`.
//!
//! @see <https://www.rfc-editor.org/rfc/rfc6901> — JSON Pointer
//! @see <https://www.unicode.org/reports/tr15/> — Unicode normalization forms
//! @see ../../🧬️schema/🔣️.json — the constraints mirrored here
//! @see ../✅️validation/🟦️.ts — the TypeScript twin

use crate::schema::{Answer, Axis, Badge, BadgeRule, Catalog, Category, ClassificationTask, Command, MatchingItem, MatchingTask, Quantity, Query, Quiz, Rejection, Scale, SheetItem, SheetMatchingTask, SheetTask, SortingTask, Task, Icon, Text, MAX_TIMESTAMP, SHORT_LENGTH};
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
    HandleInvalid,
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
            Self::HandleInvalid => "handle-invalid",
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
/// item, category and dimension exists and a sorting order is a permutation of the sheet items. Where
/// the sheet task shows the keys a sorting carries no guesses and a matching its card assignments
/// (indices in range and unique per dimension) and no guesses; where it hides them a sorting's guesses
/// name sheet items, are finite (positive on a logarithmic quantity) and stand in non-decreasing order
/// along `order` (ties allowed, unguessed items unconstrained), and a matching carries guesses per
/// sheet dimension and item that fit the dimension's scale and no assignments. Partial classification
/// and matching answers are valid.
pub fn answer_rejection(sheet_task: &SheetTask, answer: &Answer) -> Option<Rejection> {
    let valid = match (sheet_task, answer) {
        (SheetTask::Classification(task), Answer::Classification(answer)) => answer.assignments.iter().all(|(item, category)| presented(&task.items, item) && task.categories.iter().any(|candidate| &candidate.id == category)),
        (SheetTask::Sorting(task), Answer::Sorting(answer)) => {
            answer.order.len() == task.items.len()
                && answer.order.iter().collect::<BTreeSet<_>>().len() == answer.order.len()
                && answer.order.iter().all(|item| presented(&task.items, item))
                && match (&task.keys, &answer.guesses) {
                    (_, None) => true,
                    (Some(_), Some(_)) => false,
                    (None, Some(guesses)) => {
                        let guessed = answer.order.iter().filter_map(|item| guesses.get(item)).collect::<Vec<_>>();
                        guesses.iter().all(|(item, &guess)| presented(&task.items, item) && guess_fits(guess, task.quantity.scale)) && guessed.windows(2).all(|pair| pair[0] <= pair[1])
                    }
                }
        }
        (SheetTask::Matching(task), Answer::Matching(answer)) => match (cardless(task), &answer.assignments, &answer.guesses) {
            (true, None, None) => true,
            (true, None, Some(guesses)) => guesses.iter().all(|(dimension, guessed)| task.dimensions.iter().find(|candidate| &candidate.id == dimension).is_some_and(|dimension| guessed.iter().all(|(item, &guess)| presented(&task.items, item) && guess_fits(guess, dimension.quantity.scale)))),
            (false, Some(assignments), None) => assignments.iter().all(|(dimension, assignment)| {
                task.dimensions.iter().find(|candidate| &candidate.id == dimension).is_some_and(|dimension| {
                    let (count, mut used) = (dimension.cards.as_ref().map_or(0, Vec::len), BTreeSet::new());
                    assignment.iter().all(|(item, &card)| presented(&task.items, item) && card < count && used.insert(card))
                })
            }),
            _ => false,
        },
        _ => false,
    };
    (!valid).then_some(Rejection::AnswerInvalid)
}

/// ☑️ Whether an answer completes its sheet task: every item classified; every item matched in every
/// dimension, with a card where the keys show and a guess where they are hidden; a recorded sorting
/// where the keys show, a guess for every item where they are hidden.
pub fn answer_complete(sheet_task: &SheetTask, answer: Option<&Answer>) -> bool {
    match (sheet_task, answer) {
        (SheetTask::Classification(task), Some(Answer::Classification(answer))) => task.items.iter().all(|item| answer.assignments.contains_key(&item.id)),
        (SheetTask::Sorting(task), Some(Answer::Sorting(answer))) => task.keys.is_some() || answer.guesses.as_ref().is_some_and(|guesses| task.items.iter().all(|item| guesses.contains_key(&item.id))),
        (SheetTask::Matching(task), Some(Answer::Matching(answer))) => task.dimensions.iter().all(|dimension| match &dimension.cards {
            Some(_) => answer.assignments.as_ref().and_then(|assignments| assignments.get(&dimension.id)).is_some_and(|assignment| task.items.iter().all(|item| assignment.contains_key(&item.id))),
            None => answer.guesses.as_ref().and_then(|guesses| guesses.get(&dimension.id)).is_some_and(|guessed| task.items.iter().all(|item| guessed.contains_key(&item.id))),
        }),
        _ => false,
    }
}

fn guess_fits(guess: f64, scale: Scale) -> bool {
    guess.is_finite() && (scale != Scale::Logarithmic || guess > 0.0)
}

fn cardless(task: &SheetMatchingTask) -> bool {
    task.dimensions.iter().any(|dimension| dimension.cards.is_none())
}

/// 🐍️ Whether `value` is a slug: `^[a-z0-9]+(?:-[a-z0-9]+)*$` and at most 64 characters.
pub fn is_slug(value: &str) -> bool {
    value.len() <= 64 && value.split('-').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

/// 🪪️ Whether `value` is an id: exactly 32 lowercase hex characters.
pub fn is_id(value: &str) -> bool {
    value.len() == 32 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// 🛃️ `id-invalid` unless every id a command carries has its shape: the command, learner and run ids
/// are ids, the quiz and task ids slugs, and every `at` lies at most at [`MAX_TIMESTAMP`] (as in the
/// TypeScript twin, so no view ever carries an integer a TypeScript reader cannot hold exactly). A
/// challenge that is no challenge or an instant that is no `u64` is a malformed command, never a
/// rejection: serde refuses to decode it.
pub fn command_rejection(command: &Command) -> Option<Rejection> {
    let valid = match command {
        Command::IdentifyLearner { id, learner, .. } => is_id(id) && is_id(learner),
        Command::StartRun { id, learner, run, quiz, at, .. } => is_id(id) && is_id(learner) && is_id(run) && is_slug(quiz) && *at <= MAX_TIMESTAMP,
        Command::OpenTask { id, learner, run, task, at } | Command::RecordAnswer { id, learner, run, task, at, .. } => is_id(id) && is_id(learner) && is_id(run) && is_slug(task) && *at <= MAX_TIMESTAMP,
        Command::SubmitRun { id, learner, run } => is_id(id) && is_id(learner) && is_id(run),
    };
    (!valid).then_some(Rejection::IdInvalid)
}

/// 🧐️ `id-invalid` for a query whose learner or run is no id or whose quiz is no slug, `handle-invalid`
/// for a `handle` query outside the handle policy.
pub fn query_rejection(query: &Query) -> Option<Rejection> {
    match query {
        Query::Catalog => None,
        Query::Learner { learner } => (!is_id(learner)).then_some(Rejection::IdInvalid),
        Query::Run { run } => (!is_id(run)).then_some(Rejection::IdInvalid),
        Query::Leaderboard { quiz, learner, .. } => (learner.as_deref().is_some_and(|learner| !is_id(learner)) || quiz.as_deref().is_some_and(|quiz| !is_slug(quiz))).then_some(Rejection::IdInvalid),
        Query::Crowd { quiz } => (!is_slug(quiz)).then_some(Rejection::IdInvalid),
        Query::Handle { handle } => normalize_handle(handle).is_none().then_some(Rejection::HandleInvalid),
    }
}

/// 📏️ The most code points a handle holds.
pub const HANDLE_MAX: usize = 64;

/// 🧵️ The most code points a handle may be typed with before it is normalized.
pub const HANDLE_INPUT_MAX: usize = 256;

/// ⬜️ The Unicode `White_Space` code points as inclusive ranges: what collapses to one space between
/// the words of a handle.
pub const WHITE_SPACE: [(u32, u32); 10] = [(0x0009, 0x000D), (0x0020, 0x0020), (0x0085, 0x0085), (0x00A0, 0x00A0), (0x1680, 0x1680), (0x2000, 0x200A), (0x2028, 0x2029), (0x202F, 0x202F), (0x205F, 0x205F), (0x3000, 0x3000)];

/// 🔤️ The letters of a handle as inclusive code point ranges: the upper- and lowercase letters of
/// Basic Latin, Latin-1 Supplement, Latin Extended-A, Latin Extended-B and Latin Extended Additional
/// without a compatibility decomposition.
pub const HANDLE_LETTERS: [(u32, u32); 14] = [
    (0x0041, 0x005A),
    (0x0061, 0x007A),
    (0x00C0, 0x00D6),
    (0x00D8, 0x00F6),
    (0x00F8, 0x0131),
    (0x0134, 0x013E),
    (0x0141, 0x0148),
    (0x014A, 0x017E),
    (0x0180, 0x01BA),
    (0x01BC, 0x01BF),
    (0x01CD, 0x01F0),
    (0x01F4, 0x024F),
    (0x1E00, 0x1E99),
    (0x1E9C, 0x1EFF),
];

/// ❜️ The punctuation of a handle: apostrophe, hyphen-minus, full stop, underscore.
pub const HANDLE_PUNCTUATION: [char; 4] = ['\'', '-', '.', '_'];

/// 🪞️ A handle as displayed and as keyed (the display lowercased); pseudonyms and names share one
/// key space.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedHandle {
    pub display: String,
    pub key: String,
}

fn within(ranges: &[(u32, u32)], point: u32) -> bool {
    ranges.iter().any(|&(low, high)| (low..=high).contains(&point))
}

/// 🧽️ The display handle and its key, or `None` for a handle outside the policy of `$defs/Handle`.
///
/// `White_Space` runs collapse to one space and are trimmed, the typographic apostrophe U+2019 becomes
/// `'`; what remains must be 1…[`HANDLE_MAX`] code points of [`HANDLE_LETTERS`], ASCII digits,
/// [`HANDLE_PUNCTUATION`] and single spaces, with at least one letter or digit. Control and format
/// characters, combining marks (so every NFD spelling), other scripts and input over
/// [`HANDLE_INPUT_MAX`] code points are refused. The key is the lowercased display.
pub fn normalize_handle(handle: &str) -> Option<NormalizedHandle> {
    if handle.len() > 4 * HANDLE_INPUT_MAX || handle.chars().count() > HANDLE_INPUT_MAX {
        return None;
    }
    let mut display = String::new();
    let (mut length, mut gap, mut worded) = (0, false, false);
    for typed in handle.chars() {
        if within(&WHITE_SPACE, u32::from(typed)) {
            gap = length > 0;
            continue;
        }
        let character = if typed == '\u{2019}' { '\'' } else { typed };
        let word = within(&HANDLE_LETTERS, u32::from(character)) || character.is_ascii_digit();
        if !word && !HANDLE_PUNCTUATION.contains(&character) {
            return None;
        }
        if gap {
            display.push(' ');
            length += 1;
        }
        gap = false;
        worded |= word;
        display.push(character);
        length += 1;
    }
    (worded && length <= HANDLE_MAX).then(|| NormalizedHandle { key: display.to_lowercase(), display })
}

/// 🪝️ The id of the actor that holds a handle key: the lowercase hex of the key's UTF-8 bytes.
pub fn handle_actor_id(key: &str) -> String {
    key.bytes().map(|byte| format!("{byte:02x}")).collect()
}

/// 🔓️ The handle key an actor id names — the inverse of [`handle_actor_id`] — or `None` for anything
/// but lowercase hex of well-formed UTF-8.
pub fn handle_key_of(actor: &str) -> Option<String> {
    let digit = |byte: u8| match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    };
    if actor.is_empty() || !actor.len().is_multiple_of(2) {
        return None;
    }
    let bytes: Option<Vec<u8>> = actor.as_bytes().chunks(2).map(|pair| Some(digit(pair[0])? << 4 | digit(pair[1])?)).collect();
    String::from_utf8(bytes?).ok()
}

/// 🪡️ `base` extended by one JSON Pointer reference token (RFC 6901 escaping of `~` and `/`).
pub fn pointer(base: &str, key: &str) -> String {
    format!("{base}/{}", key.replace('~', "~0").replace('/', "~1"))
}

fn presented(items: &[SheetItem], id: &str) -> bool {
    items.iter().any(|item| item.id == id)
}

#[derive(Clone, Copy)]
struct Head<'a> {
    id: &'a str,
    label: &'a Text,
    short: Option<&'a Text>,
    icon: Option<&'a Icon>,
    explanation: Option<&'a Text>,
}

impl<'a> Head<'a> {
    fn of(id: &'a str, label: &'a Text, short: Option<&'a Text>, icon: Option<&'a Icon>, explanation: Option<&'a Text>) -> Self {
        Self { id, label, short, icon, explanation }
    }
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

    fn icon(&mut self, base: &str, icon: Option<&Icon>) {
        if let Some(icon) = icon {
            self.length(format!("{base}/icon/emoji"), &icon.emoji, 1, 16);
        }
    }

    fn optional_text(&mut self, base: &str, text: Option<&Text>) {
        if let Some(text) = text {
            self.text(base, text);
        }
    }

    fn short(&mut self, base: &str, short: Option<&Text>) {
        if let Some(short) = short {
            self.length(format!("{base}/short/en"), &short.en, 1, SHORT_LENGTH);
            self.length(format!("{base}/short/de"), &short.de, 1, SHORT_LENGTH);
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
        self.short(base, quantity.short.as_ref());
        self.length(format!("{base}/unit"), &quantity.unit, 1, 32);
    }

    fn head(&mut self, base: &str, head: Head<'_>) {
        self.slug(format!("{base}/id"), head.id);
        self.text(&format!("{base}/label"), head.label);
        self.short(base, head.short);
        self.icon(base, head.icon);
        self.optional_text(&format!("{base}/explanation"), head.explanation);
    }

    fn items_and_draw<'a>(&mut self, base: &str, draw: Option<usize>, heads: impl ExactSizeIterator<Item = Head<'a>> + Clone) {
        let count = heads.len();
        self.at_least(format!("{base}/items"), count, 2, IssueCode::ItemsTooFew);
        for (index, head) in heads.clone().enumerate() {
            self.head(&format!("{base}/items/{index}"), head);
        }
        self.unique(&format!("{base}/items"), heads.map(|head| head.id));
        match draw {
            Some(draw) if draw < 2 => self.push(format!("{base}/draw"), IssueCode::BelowMinimum),
            Some(draw) if draw > count => self.push(format!("{base}/draw"), IssueCode::DrawExceedsItems),
            _ => {}
        }
    }

    fn classification(&mut self, base: &str, task: &ClassificationTask) {
        self.text(&format!("{base}/title"), &task.title);
        self.text(&format!("{base}/prompt"), &task.prompt);
        self.icon(base, task.icon.as_ref());
        let axes: &[Axis] = task.axes.as_deref().unwrap_or_default();
        if task.axes.is_some() {
            self.at_least(format!("{base}/axes"), axes.len(), 3, IssueCode::ItemsTooFew);
        }
        let mut ranges: BTreeMap<&str, (f64, f64)> = BTreeMap::new();
        for (index, axis) in axes.iter().enumerate() {
            let path = format!("{base}/axes/{index}");
            self.slug(format!("{path}/id"), &axis.id);
            self.text(&format!("{path}/label"), &axis.label);
            self.short(&path, axis.short.as_ref());
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
        self.items_and_draw(base, task.draw, task.items.iter().map(|item| Head::of(&item.id, &item.label, item.short.as_ref(), item.icon.as_ref(), item.explanation.as_ref())));
    }

    fn category(&mut self, base: &str, category: &Category, has_axes: bool, axis_ids: &BTreeSet<&str>, ranges: &BTreeMap<&str, (f64, f64)>) {
        self.slug(format!("{base}/id"), &category.id);
        self.text(&format!("{base}/label"), &category.label);
        self.short(base, category.short.as_ref());
        self.icon(base, category.icon.as_ref());
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
        self.icon(base, task.icon.as_ref());
        self.quantity(&format!("{base}/quantity"), &task.quantity);
        if task.quantity.scale == Scale::Logarithmic {
            for (index, _) in task.items.iter().enumerate().filter(|(_, item)| item.value <= 0.0) {
                self.push(format!("{base}/items/{index}/value"), IssueCode::ValueNotPositive);
            }
        }
        self.items_and_draw(base, task.draw, task.items.iter().map(|item| Head::of(&item.id, &item.label, item.short.as_ref(), item.icon.as_ref(), item.explanation.as_ref())));
    }

    fn matching(&mut self, base: &str, task: &MatchingTask) {
        self.text(&format!("{base}/title"), &task.title);
        self.text(&format!("{base}/prompt"), &task.prompt);
        self.icon(base, task.icon.as_ref());
        self.at_least(format!("{base}/dimensions"), task.dimensions.len(), 1, IssueCode::ItemsTooFew);
        let mut scales: BTreeMap<&str, Scale> = BTreeMap::new();
        for (index, dimension) in task.dimensions.iter().enumerate() {
            self.quantity(&format!("{base}/dimensions/{index}/quantity"), &dimension.quantity);
            self.icon(&format!("{base}/dimensions/{index}"), dimension.icon.as_ref());
            if self.slug(format!("{base}/dimensions/{index}/id"), &dimension.id) {
                scales.entry(&dimension.id).or_insert(dimension.quantity.scale);
            }
        }
        self.unique(&format!("{base}/dimensions"), task.dimensions.iter().map(|dimension| dimension.id.as_str()));
        for (index, item) in task.items.iter().enumerate() {
            self.values(&format!("{base}/items/{index}/values"), item, &scales);
        }
        self.items_and_draw(base, task.draw, task.items.iter().map(|item| Head::of(&item.id, &item.label, item.short.as_ref(), item.icon.as_ref(), item.explanation.as_ref())));
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
            BadgeRule::PerfectQuiz { quiz, .. } => (Some(quiz), None),
            BadgeRule::PerfectTasks { task_kind, quiz, .. } => (quiz.as_ref(), *task_kind),
            BadgeRule::CompletedQuizzes {} => return,
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
