/** ❓️ Typed twin of `🧬️schema/🔣️.json`: quizzes, catalogs, badges, sheets, answers, results, the run lifecycle and its views.
 *
 * Schema-first — `🔣️.json` is the single source of truth; this module restates every `$defs` entry under the same
 * name with `readonly` fields for TypeScript consumers. No runtime dependency.
 *
 * @see 🧬️schema/🔣️.json — the normative contract
 * @see 🧬️schema/🦀️.rs — the Rust twin
 * @see README.md — the domain model
 */

//#region 🔖️Scalars
/** 🏷️ A kebab-case identifier, unique within its scope (`^[a-z0-9]+(?:-[a-z0-9]+)*$`, 1…64 chars). */
export type Slug = string;

/** 🪪️ A learner, run or command id: 32 lowercase hex chars (128 random bits). */
export type Id = string;

/** ⏱️ Milliseconds since the Unix epoch. */
export type Timestamp = number;

/** 💯️ A score or credit in [0, 1]. */
export type Score = number;

/** 🌍️ A learner-visible text in every supported language; there is no default language. */
export type Text = { readonly en: string; readonly de: string };

/** 🗣️ The languages every {@link Text} carries, English first. */
export const LANGUAGES = ["en", "de"] as const;

/** 📐️ The scales distances between values are measured on. */
export const SCALES = ["linear", "logarithmic"] as const;

/** 📏️ Logarithmic for quantities spanning orders of magnitude, linear otherwise. */
export type Scale = (typeof SCALES)[number];

/** 🧰️ The task kinds a quiz is built from. */
export const TASK_KINDS = ["classification", "sorting", "matching"] as const;

/** 🔤️ One of {@link TASK_KINDS}. */
export type TaskKind = (typeof TASK_KINDS)[number];
//#endregion 🔖️Scalars

//#region 🔖️Quiz
/** ⚖️ A measured quantity: label, unit symbol, distance scale and whether display scales the unit with SI prefixes. */
export type Quantity = { readonly label: Text; readonly unit: string; readonly scale: Scale; readonly prefixed: boolean };

/** 🧭️ One quantity a matching task asks for per item. */
export type Dimension = { readonly id: Slug; readonly quantity: Quantity };

/** 🕸️ One spoke of a profile (spider diagram); values normalise to (value − min) / (max − min). */
export type Axis = { readonly id: Slug; readonly label: Text; readonly unit: string; readonly min: number; readonly max: number };

/** 🎯️ Values of a category on every axis of its task, keyed by axis id. */
export type Profile = Readonly<Record<Slug, number>>;

/** 🗂️ A category of a classification task, optionally carrying a profile. */
export type Category = { readonly id: Slug; readonly label: Text; readonly description?: Text; readonly profile?: Profile };

/** 🔖️ An item to classify together with its correct category. */
export type ClassificationItem = { readonly id: Slug; readonly label: Text; readonly category: Slug; readonly explanation?: Text };

/** 🔢️ An item to sort together with its true value. */
export type SortingItem = { readonly id: Slug; readonly label: Text; readonly value: number; readonly explanation?: Text };

/** 🧷️ An item to match together with its true value per dimension id. */
export type MatchingItem = { readonly id: Slug; readonly label: Text; readonly values: Readonly<Record<Slug, number>>; readonly explanation?: Text };

/** 🗃️ Assign every item to one category; a wrong category with a profile earns partial credit by profile similarity. */
export type ClassificationTask = {
  readonly kind: "classification";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly axes?: readonly Axis[];
  readonly categories: readonly Category[];
  readonly items: readonly ClassificationItem[];
  readonly draw?: number;
};

/** 📶️ Order the items ascending by their quantity. */
export type SortingTask = {
  readonly kind: "sorting";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly quantity: Quantity;
  readonly items: readonly SortingItem[];
  readonly draw?: number;
};

/** 🔗️ For every dimension, assign each item one of the offered value cards. */
export type MatchingTask = {
  readonly kind: "matching";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly dimensions: readonly Dimension[];
  readonly items: readonly MatchingItem[];
  readonly draw?: number;
};

/** 🧩️ One task of a quiz. */
export type Task = ClassificationTask | SortingTask | MatchingTask;

/** 📝️ A quiz: an ordered set of tasks, presented randomized and scored only as a whole. */
export type Quiz = {
  readonly $schema?: string;
  readonly schema: "semio.quiz/v1";
  readonly id: Slug;
  readonly title: Text;
  readonly description: Text;
  readonly tasks: readonly Task[];
};
//#endregion 🔖️Quiz

//#region 🔖️Catalog
/** 📜️ When a badge is earned: a perfect quiz, every selected task perfect once, or every catalog quiz submitted. */
export type BadgeRule = { readonly kind: "perfect-quiz"; readonly quiz: Slug } | { readonly kind: "perfect-tasks"; readonly taskKind?: TaskKind; readonly quiz?: Slug } | { readonly kind: "completed-quizzes" };

/** 🏅️ A badge of a catalog. */
export type Badge = { readonly id: Slug; readonly emoji: string; readonly label: Text; readonly description: Text; readonly rule: BadgeRule };

/** 👋️ The introduction a learner reads on the first visit. */
export type Introduction = { readonly title: Text; readonly paragraphs: readonly Text[] };

/** 📚️ The quizzes one site offers, its introduction and the badges spanning its quizzes; quiz paths are relative to the catalog file. */
export type Catalog = {
  readonly $schema?: string;
  readonly schema: "semio.quiz.catalog/v1";
  readonly id: Slug;
  readonly title: Text;
  readonly introduction: Introduction;
  readonly quizzes: readonly string[];
  readonly badges: readonly Badge[];
};
//#endregion 🔖️Catalog

//#region 🔖️Sheet
/** 🪧️ A solution-free item of a sheet. */
export type SheetItem = { readonly id: Slug; readonly label: Text };

/** 🗄️ A classification task as presented: shuffled categories, drawn items in order. */
export type SheetClassificationTask = {
  readonly kind: "classification";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly axes?: readonly Axis[];
  readonly categories: readonly Category[];
  readonly items: readonly SheetItem[];
};

/** 🪜️ A sorting task as presented: drawn items in a never-already-sorted order. */
export type SheetSortingTask = {
  readonly kind: "sorting";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly quantity: Quantity;
  readonly items: readonly SheetItem[];
};

/** 🎴️ One dimension of a presented matching task: its card values in presentation order, addressed by index. */
export type SheetDimension = { readonly id: Slug; readonly quantity: Quantity; readonly cards: readonly number[] };

/** 🪢️ A matching task as presented: drawn items in order and shuffled cards per dimension. */
export type SheetMatchingTask = {
  readonly kind: "matching";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly dimensions: readonly SheetDimension[];
  readonly items: readonly SheetItem[];
};

/** 🧱️ One presented task of a sheet. */
export type SheetTask = SheetClassificationTask | SheetSortingTask | SheetMatchingTask;

/** 🃏️ The randomized, solution-free presentation of a quiz for one run seed; a pure function of (quiz, seed). */
export type Sheet = { readonly quiz: Slug; readonly seed: number; readonly title: Text; readonly description: Text; readonly tasks: readonly SheetTask[] };
//#endregion 🔖️Sheet

//#region 🔖️Answer
/** ☑️ Category id per item id. */
export type ClassificationAnswer = { readonly kind: "classification"; readonly assignments: Readonly<Record<Slug, Slug>> };

/** 🔃️ Item ids, smallest first. */
export type SortingAnswer = { readonly kind: "sorting"; readonly order: readonly Slug[] };

/** 🔀️ Per dimension id: the card index per item id. */
export type MatchingAnswer = { readonly kind: "matching"; readonly assignments: Readonly<Record<Slug, Readonly<Record<Slug, number>>>> };

/** ✍️ A learner's answer to one task. */
export type Answer = ClassificationAnswer | SortingAnswer | MatchingAnswer;
//#endregion 🔖️Answer

//#region 🔖️Result
/** 🎚️ The credit of one classified item. */
export type ClassificationItemResult = { readonly item: Slug; readonly assigned: Slug; readonly correct: Slug; readonly credit: Score; readonly explanation?: Text };

/** 📍️ One sorted item: its value, the learner's position and its true rank (both zero-based). */
export type SortingItemResult = { readonly item: Slug; readonly value: number; readonly position: number; readonly rank: number; readonly explanation?: Text };

/** 🧮️ One matched item: the assigned card value and the correct value. */
export type MatchingItemResult = { readonly item: Slug; readonly assigned: number; readonly correct: number; readonly explanation?: Text };

/** 📊️ The score of one matching dimension and its items in sheet order. */
export type DimensionResult = { readonly dimension: Slug; readonly score: Score; readonly items: readonly MatchingItemResult[] };

/** 🗳️ A scored classification task, items in sheet order. */
export type ClassificationTaskResult = { readonly kind: "classification"; readonly task: Slug; readonly score: Score; readonly items: readonly ClassificationItemResult[] };

/** 📈️ A scored sorting task, items in the learner's order. */
export type SortingTaskResult = { readonly kind: "sorting"; readonly task: Slug; readonly score: Score; readonly items: readonly SortingItemResult[] };

/** 🪄️ A scored matching task, dimensions in definition order. */
export type MatchingTaskResult = { readonly kind: "matching"; readonly task: Slug; readonly score: Score; readonly dimensions: readonly DimensionResult[] };

/** 📑️ One scored task. */
export type TaskResult = ClassificationTaskResult | SortingTaskResult | MatchingTaskResult;

/** 🏁️ The scored run: the mean of its task scores, tasks in sheet order. */
export type RunResult = { readonly quiz: Slug; readonly score: Score; readonly tasks: readonly TaskResult[] };
//#endregion 🔖️Result

//#region 🔖️Lifecycle
/** 🎭️ How a learner appears: anonymous learners are always new; pseudonyms and names share one handle namespace. */
export type Identity = { readonly kind: "anonymous" } | { readonly kind: "pseudonym" | "name"; readonly handle: string };

/** 🙋️ Register a new learner or recall the one holding the handle. */
export type IdentifyLearnerCommand = { readonly type: "identify-learner"; readonly id: Id; readonly learner: Id; readonly identity: Identity };

/** ▶️ Start a run of a quiz. */
export type StartRunCommand = { readonly type: "start-run"; readonly id: Id; readonly learner: Id; readonly run: Id; readonly quiz: Slug };

/** 🖊️ Record the latest answer to one task of an open run. */
export type RecordAnswerCommand = { readonly type: "record-answer"; readonly id: Id; readonly learner: Id; readonly run: Id; readonly task: Slug; readonly answer: Answer };

/** 📨️ Submit an open run for scoring. */
export type SubmitRunCommand = { readonly type: "submit-run"; readonly id: Id; readonly learner: Id; readonly run: Id };

/** 📮️ Learner intent; the client-generated id makes a retry after a connection shortage apply exactly once. */
export type Command = IdentifyLearnerCommand | StartRunCommand | RecordAnswerCommand | SubmitRunCommand;

/** 🚫️ Every reason a command is rejected. */
export const REJECTIONS = ["unknown-learner", "unknown-quiz", "unknown-run", "unknown-task", "run-open", "run-closed", "run-incomplete", "answer-invalid", "quiz-revised", "handle-invalid"] as const;

/** ⛔️ One of {@link REJECTIONS}. */
export type Rejection = (typeof REJECTIONS)[number];

/** 🆕️ A learner was registered under the given identity. */
export type LearnerRegisteredEvent = { readonly type: "learner-registered"; readonly learner: Id; readonly identity: Identity; readonly at: Timestamp };

/** 🔁️ A handle was recalled to the learner holding it. */
export type LearnerRecalledEvent = { readonly type: "learner-recalled"; readonly learner: Id; readonly at: Timestamp };

/** 🚀️ A run started against the quiz revision (content hash) with the seed of its sheet. */
export type RunStartedEvent = { readonly type: "run-started"; readonly learner: Id; readonly run: Id; readonly quiz: Slug; readonly revision: string; readonly seed: number; readonly at: Timestamp };

/** 🗑️ An open run was voided because its quiz was revised. */
export type RunVoidedEvent = { readonly type: "run-voided"; readonly learner: Id; readonly run: Id; readonly at: Timestamp };

/** 💾️ The latest answer to one task of an open run. */
export type AnswerRecordedEvent = { readonly type: "answer-recorded"; readonly learner: Id; readonly run: Id; readonly task: Slug; readonly answer: Answer; readonly at: Timestamp };

/** 📬️ A run was submitted and scored. */
export type RunSubmittedEvent = { readonly type: "run-submitted"; readonly learner: Id; readonly run: Id; readonly result: RunResult; readonly at: Timestamp };

/** 🎖️ A badge was earned by the submission of a run. */
export type BadgeAwardedEvent = { readonly type: "badge-awarded"; readonly learner: Id; readonly badge: Slug; readonly run: Id; readonly at: Timestamp };

/** ⚡️ A fact of the learner stream. */
export type Event = LearnerRegisteredEvent | LearnerRecalledEvent | RunStartedEvent | RunVoidedEvent | AnswerRecordedEvent | RunSubmittedEvent | BadgeAwardedEvent;
//#endregion 🔖️Lifecycle

//#region 🔖️Views
/** 🧾️ One task of a catalog quiz as the client lists it. */
export type CatalogTaskView = { readonly id: Slug; readonly kind: TaskKind; readonly title: Text };

/** 🗒️ One quiz of the catalog without its solutions. */
export type CatalogQuizView = { readonly id: Slug; readonly title: Text; readonly description: Text; readonly tasks: readonly CatalogTaskView[] };

/** 🏵️ One badge of the catalog without its rule. */
export type CatalogBadgeView = { readonly id: Slug; readonly emoji: string; readonly label: Text; readonly description: Text };

/** 📖️ The solution-free catalog a client renders. */
export type CatalogView = { readonly id: Slug; readonly title: Text; readonly introduction: Introduction; readonly quizzes: readonly CatalogQuizView[]; readonly badges: readonly CatalogBadgeView[] };

/** 🚥️ The states of a run. */
export const RUN_STATUSES = ["open", "submitted", "voided"] as const;

/** 🚦️ One of {@link RUN_STATUSES}. */
export type RunStatus = (typeof RUN_STATUSES)[number];

/** 🏃️ One run with its sheet, answers and, once submitted, its result. */
export type RunView = {
  readonly run: Id;
  readonly learner: Id;
  readonly quiz: Slug;
  readonly status: RunStatus;
  readonly sheet: Sheet;
  readonly answers: Readonly<Record<Slug, Answer>>;
  readonly result?: RunResult;
  readonly startedAt: Timestamp;
  readonly submittedAt?: Timestamp;
};

/** 📇️ One run of a learner as listed. */
export type RunSummary = { readonly run: Id; readonly quiz: Slug; readonly status: RunStatus; readonly score?: Score; readonly startedAt: Timestamp; readonly submittedAt?: Timestamp };

/** 🎗️ A badge held by a learner with the run and time that earned it. */
export type BadgeAward = { readonly badge: Slug; readonly run: Id; readonly at: Timestamp };

/** 👤️ A learner's runs (newest first), badges, best score per quiz and total points. */
export type LearnerView = {
  readonly learner: Id;
  readonly identity: Identity;
  readonly runs: readonly RunSummary[];
  readonly badges: readonly BadgeAward[];
  readonly best: Readonly<Record<Slug, Score>>;
  readonly total: number;
};

/** 🥇️ One learner on the public leaderboard; it never carries the learner id, only its non-reversible tag (FNV-1a as 8 lowercase hex digits). */
export type LeaderboardRow = {
  readonly rank: number;
  readonly tag: string;
  readonly identity: Identity;
  readonly total: number;
  readonly reachedAt: Timestamp;
  readonly best: Readonly<Record<Slug, Score>>;
  readonly badges: readonly Slug[];
  readonly runs: number;
  readonly lastActivity: Timestamp;
};

/** 🏆️ Every learner with a submitted run: total ↓, badges ↓, reachedAt ↑, learner id ↑ (internally); rank is the 1-based position. */
export type Leaderboard = { readonly rows: readonly LeaderboardRow[] };

/** 🔍️ A query a proctor answers. */
export type Query = { readonly type: "catalog" } | { readonly type: "learner"; readonly learner: Id } | { readonly type: "run"; readonly run: Id } | { readonly type: "leaderboard" };
//#endregion 🔖️Views
