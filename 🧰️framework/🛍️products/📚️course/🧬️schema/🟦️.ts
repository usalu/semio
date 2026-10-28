/** ❓️ Typed twin of `🧬️schema/🔣️.json`: quizzes, courses, badges, sheets, answers, results, the run lifecycle and its views.
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

/** ✒️ A handle as registered and shown: 1…64 code points of Latin letters, ASCII digits, `'` `.` `_` `-` and single spaces between words, with at least one letter or digit; always in NFC. */
export type Handle = string;

/** ⏱️ Milliseconds since the Unix epoch, a safe integer (at most {@link MAX_TIMESTAMP}). */
export type Timestamp = number;

/** 🌌️ The latest timestamp: 2^53 − 1, the largest integer every language reads exactly. */
export const MAX_TIMESTAMP: Timestamp = Number.MAX_SAFE_INTEGER;

/** 🤝️ The version of this contract on the wire: the `version` of every quiz envelope and of every quiz kind a proctor
 * declares; raised with every change of the contract beyond its prose. */
export type WireVersion = 10;

/** 📡️ The wire version of this contract (`$defs/WireVersion`). */
export const WIRE_VERSION: WireVersion = 10;

/** 💯️ A score or credit in [0, 1]. */
export type Score = number;

/** 🌍️ A learner-visible text in every supported language; there is no default language. */
export type Text = { readonly en: string; readonly de: string };

/** ✂️ A short form of a label in every supported language, at most {@link SHORT_LENGTH} code points each, that hints name the labelled thing by instead of its label. */
export type ShortText = Text;

/** 🤏️ How many code points a {@link ShortText} carries at most per language. */
export const SHORT_LENGTH = 40;

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

/** 🏔️ The challenges of a run, each the one before plus one step: easy shows the keys and hints, medium shows the keys, hard hides them, expert hides them and runs every task against a clock. */
export const CHALLENGES = ["easy", "medium", "hard", "expert"] as const;

/** 🧗️ One of {@link CHALLENGES}: how demanding a run is, chosen when it starts and fixed for it. */
export type Challenge = (typeof CHALLENGES)[number];

/** 🎞️ The looping microanimations of an icon, still for learners who prefer reduced motion. */
export const MOTIONS = ["bounce", "pulse", "spin", "sway", "float", "flip"] as const;

/** 🎬️ One of {@link MOTIONS}. */
export type Motion = (typeof MOTIONS)[number];
//#endregion 🔖️Scalars

//#region 🔖️Quiz
/** ⚖️ A measured quantity: label (and its short form for hints), unit symbol, distance scale, whether display scales the unit with SI prefixes and whether amounts of it add up (powers, energies; not U-values, loads per m² or air change rates). */
export type Quantity = { readonly label: Text; readonly short?: ShortText; readonly unit: string; readonly scale: Scale; readonly prefixed: boolean; readonly additive: boolean };

/** 🔎️ One quantity a matching task asks for per item. */
export type Dimension = { readonly id: Slug; readonly quantity: Quantity; readonly icon?: Icon };

/** 🕸️ One spoke of a profile (spider diagram); values normalise to (value − min) / (max − min). */
export type Axis = { readonly id: Slug; readonly label: Text; readonly short?: ShortText; readonly unit: string; readonly min: number; readonly max: number };

/** 🎯️ Values of a category on every axis of its task, keyed by axis id. */
export type Profile = Readonly<Record<Slug, number>>;

/** 🖼️ The icon of a task, an item, a category or a dimension (shown on its value cards): one emoji grapheme picturing it and the {@link Motion} it plays. */
export type Icon = { readonly emoji: string; readonly motion: Motion };

/** 🗂️ A category of a classification task, optionally carrying a profile. */
export type Category = { readonly id: Slug; readonly label: Text; readonly short?: ShortText; readonly icon?: Icon; readonly description?: Text; readonly profile?: Profile };

/** 🔖️ An item to classify together with its correct category. */
export type ClassificationItem = { readonly id: Slug; readonly label: Text; readonly short?: ShortText; readonly icon?: Icon; readonly mockup?: Mockup; readonly category: Slug; readonly explanation?: Text };

/** 🔢️ An item to sort together with its true value; `familiar` marks an everyday thing a compare hint prefers as its reference (never presented). */
export type SortingItem = { readonly id: Slug; readonly label: Text; readonly short?: ShortText; readonly icon?: Icon; readonly mockup?: Mockup; readonly value: number; readonly familiar?: boolean; readonly explanation?: Text };

/** 🧷️ An item to match together with its true value per dimension id; `familiar` marks an everyday thing a compare hint prefers as its reference (never presented). */
export type MatchingItem = { readonly id: Slug; readonly label: Text; readonly short?: ShortText; readonly icon?: Icon; readonly mockup?: Mockup; readonly values: Readonly<Record<Slug, number>>; readonly familiar?: boolean; readonly explanation?: Text };

/** 🧺️ Fixed semantic slots with rotating variants and a per-question time budget. */
export type QuestionPool = { readonly seconds: number; readonly slots: readonly (readonly Slug[])[] };

/** 🗃️ Assign every item to one category; a wrong category with a profile earns partial credit by profile similarity. */
export type ClassificationTask = {
  readonly kind: "classification";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly icon?: Icon;
  readonly axes?: readonly Axis[];
  readonly categories: readonly Category[];
  readonly items: readonly ClassificationItem[];
  readonly draw?: number;
  readonly pool?: QuestionPool;
};

/** 📶️ Order the items ascending by their quantity. */
export type SortingTask = {
  readonly kind: "sorting";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly icon?: Icon;
  readonly quantity: Quantity;
  readonly items: readonly SortingItem[];
  readonly draw?: number;
  readonly pool?: QuestionPool;
};

/** 🔗️ For every dimension, assign each item one of the offered value cards. */
export type MatchingTask = {
  readonly kind: "matching";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly icon?: Icon;
  readonly dimensions: readonly Dimension[];
  readonly items: readonly MatchingItem[];
  readonly draw?: number;
  readonly pool?: QuestionPool;
};

/** 🧩️ One task of a quiz. */
export type Task = ClassificationTask | SortingTask | MatchingTask;

/** 📝️ A quiz: an ordered set of tasks, with its first task presented first and the rest randomized; scored only as a whole. Its emoji identifies it on cards and headers. */
export type Quiz = {
  readonly $schema?: string;
  readonly schema: "semio.quiz/v1";
  readonly id: Slug;
  readonly emoji: string;
  readonly title: Text;
  readonly short?: ShortText;
  readonly description: Text;
  readonly tasks: readonly Task[];
};

/** 🧾️ Localized cell content, including meaningful empty cells. */
export type MockupText = Text;
/** 🔲️ Zero-based selected rectangle of the reference grid. */
export type MockupSelection = { readonly row: number; readonly column: number; readonly rowEnd?: number; readonly columnEnd?: number };
/** 🎯️ The highlighted Excel interface part. */
export type ExcelTarget = "cell" | "range" | "row" | "column" | "worksheet" | "workbook" | "formula" | "name-box";
/** 📗️ A workbook showing the given input of an exercise. */
export type ExcelMockup = { readonly kind: "excel"; readonly caption: Text; readonly workbook: Text; readonly sheets: readonly Text[]; readonly activeSheet: number; readonly columns: readonly string[]; readonly rows: readonly (readonly MockupText[])[]; readonly selection: MockupSelection; readonly formula: MockupText; readonly target: ExcelTarget };
/** 🔌️ A labelled node port. */
export type MockupPort = { readonly id: Slug; readonly label: Text };
/** 🧩️ A computation or parameter with identified ports. */
export type MockupNode = { readonly id: Slug; readonly label: Text; readonly inputs: readonly MockupPort[]; readonly outputs: readonly MockupPort[]; readonly widget?: MockupWidget };
/** 📍️ A port reference within a workspace. */
export type MockupEndpoint = { readonly node: Slug; readonly port: Slug };
/** 🧵️ A directed output-to-input connection. */
export type MockupWire = { readonly from: MockupEndpoint; readonly to: MockupEndpoint };
/** 🌿️ A tree path and its ordered reference values. */
export type MockupBranch = { readonly path: string; readonly values: readonly MockupText[] };
/** 🔦️ The highlighted Grasshopper interface part. */
export type GrasshopperTargetKind = "node" | "input" | "output" | "wire" | "tree" | "branch" | "item";
/** 🪧️ A referenced node/port or zero-based wire, branch or first-branch item. */
export type MockupTarget = { readonly kind: GrasshopperTargetKind; readonly index: number; readonly node?: Slug; readonly port?: Slug };
/** 🦗️ A bounded node graph and tree of given data. */
export type GrasshopperMockup = { readonly kind: "grasshopper"; readonly caption: Text; readonly nodes: readonly MockupNode[]; readonly wires: readonly MockupWire[]; readonly tree: readonly MockupBranch[]; readonly target: MockupTarget };
/** 🪟️ A solution-free workspace preview, independent of grading truth. */
export type Mockup = ExcelMockup | GrasshopperMockup;

//#endregion 🔖️Quiz

//#region 🔖️Course
/** 📜️ When a badge is earned: a perfect quiz, every selected task perfect once, or every course quiz submitted; `challenge` is the least challenge whose runs count. */
export type BadgeRule =
  | { readonly kind: "perfect-quiz"; readonly quiz: Slug; readonly challenge?: Challenge }
  | { readonly kind: "perfect-tasks"; readonly taskKind?: TaskKind; readonly quiz?: Slug; readonly challenge?: Challenge }
  | { readonly kind: "completed-quizzes" };

/** 🔰️ A badge of a course. */
export type Badge = { readonly id: Slug; readonly emoji: string; readonly label: Text; readonly description: Text; readonly rule: BadgeRule };

/** 👋️ The introduction a learner reads on the first visit. */
export type Introduction = { readonly title: Text; readonly paragraphs: readonly Text[] };

/** 🧭️ Page names reserved for the course's root navigation. */
export const COURSE_ROOT_IDS = ["home", "identity", "introduction", "learner", "intro", "board", "badges", "prefs", "quizzes", "run", "results"] as const;

/** 👥️ An ordered course group; an empty quiz list marks a placeholder. */
export type Topic = { readonly id: Slug; readonly emoji: string; readonly title: Text; readonly quizzes: readonly Slug[] };

/** 🗓️ Release times by topic, quiz or quiz/task; null awaits scheduling, absent keys add no restriction. */
export type Unlocks = Readonly<Record<string, Timestamp | null>>;

/** 🖨️ A course PDF resolved by its catalog id, filename and explicit build producer. */
export type AssignmentDocument = { readonly namespace: "course"; readonly id: Slug; readonly file: string; readonly producer: string };

/** 🌍️ A printable artifact for every course language. */
export type AssignmentDocuments = { readonly en: AssignmentDocument; readonly de: AssignmentDocument };

/** ✍️ An ungraded topic exercise with a printable PDF in every supported language. */
export type Assignment = { readonly id: Slug; readonly topic: Slug; readonly emoji: string; readonly title: Text; readonly description: Text; readonly documents: AssignmentDocuments };

/** 📚️ Authored course material, quizzes and badges. */
export type Course = {
  readonly assignments?: readonly Assignment[];
  readonly unlocks?: Unlocks;
  readonly $schema?: string;
  readonly schema: "semio.course/v1";
  readonly id: Slug;
  readonly title: Text;
  readonly short?: ShortText;
  readonly introduction: Introduction;
  readonly quizzes: readonly string[];
  readonly topics?: readonly Topic[];
  readonly badges: readonly Badge[];
};
//#endregion 🔖️Course

//#region 🔖️Sheet
/** 🪧️ A solution-free item of a sheet: its id, label, short label when it has one, and icon. */
export type SheetItem = { readonly id: Slug; readonly label: Text; readonly short?: ShortText; readonly icon?: Icon; readonly mockup?: Mockup };

/** 🛞️ One spoke of a profile as a sheet presents it: `unit`, `min` and `max` travel together and are present exactly when the challenge shows the keys; without them the profile values of the sheet are shares of the axis range. */
export type SheetAxis = { readonly id: Slug; readonly label: Text; readonly short?: ShortText; readonly unit?: string; readonly min?: number; readonly max?: number };

/** 🗄️ A classification task as presented: shuffled categories (without descriptions where the keys are hidden), drawn items in order, and its `seconds` on a timed sheet. */
export type SheetClassificationTask = {
  readonly kind: "classification";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly icon?: Icon;
  readonly axes?: readonly SheetAxis[];
  readonly categories: readonly Category[];
  readonly items: readonly SheetItem[];
  readonly seconds?: number;
};

/** 🪜️ A sorting task as presented: drawn items in a never-already-sorted order, their true values as an ascending ladder of `keys` where the challenge shows them, and its `seconds` on a timed sheet. */
export type SheetSortingTask = {
  readonly kind: "sorting";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly icon?: Icon;
  readonly quantity: Quantity;
  readonly keys?: readonly number[];
  readonly items: readonly SheetItem[];
  readonly seconds?: number;
};

/** 🎴️ One dimension of a presented matching task: its card values in presentation order, addressed by index, where the challenge shows the keys. */
export type SheetDimension = { readonly id: Slug; readonly quantity: Quantity; readonly icon?: Icon; readonly cards?: readonly number[] };

/** 🪢️ A matching task as presented: drawn items in order, shuffled cards per dimension where the challenge shows the keys, and its `seconds` on a timed sheet. */
export type SheetMatchingTask = {
  readonly kind: "matching";
  readonly id: Slug;
  readonly title: Text;
  readonly prompt: Text;
  readonly icon?: Icon;
  readonly dimensions: readonly SheetDimension[];
  readonly items: readonly SheetItem[];
  readonly seconds?: number;
};

/** 🧱️ One presented task of a sheet. */
export type SheetTask = SheetClassificationTask | SheetSortingTask | SheetMatchingTask;

/** 🃏️ The randomized presentation of a quiz for one run seed at one challenge, never saying which item a number belongs to; a pure function of (quiz, seed, challenge) that deals the same items in the same order at every challenge. */
export type Sheet = { readonly quiz: Slug; readonly seed: number; readonly challenge: Challenge; readonly title: Text; readonly description: Text; readonly tasks: readonly SheetTask[] };
//#endregion 🔖️Sheet

//#region 🔖️Answer
/** ☑️ Category id per item id. */
export type ClassificationAnswer = { readonly kind: "classification"; readonly assignments: Readonly<Record<Slug, Slug>> };

/** 🔃️ Item ids, smallest first; where the sheet task hides the keys, the learner's numeric guess per item id is the answer. */
export type SortingAnswer = { readonly kind: "sorting"; readonly order: readonly Slug[]; readonly guesses?: Readonly<Record<Slug, number>> };

/** 🔀️ Per dimension id: the card index per item id where the sheet task shows the keys (`assignments`), the guessed number per item id where it hides them (`guesses`). */
export type MatchingAnswer = {
  readonly kind: "matching";
  readonly assignments?: Readonly<Record<Slug, Readonly<Record<Slug, number>>>>;
  readonly guesses?: Readonly<Record<Slug, Readonly<Record<Slug, number>>>>;
};

/** ✍️ A learner's answer to one task. */
export type Answer = ClassificationAnswer | SortingAnswer | MatchingAnswer;
//#endregion 🔖️Answer

//#region 🔖️Result
/** 🎚️ The credit of one classified item; `assigned` is absent when the learner left it unanswered (timed runs only). */
export type ClassificationItemResult = { readonly item: Slug; readonly assigned?: Slug; readonly correct: Slug; readonly credit: Score; readonly explanation?: Text };

/** 📍️ One sorted item: its value, the learner's position and its true rank (both zero-based); where the keys were hidden also the learner's `guess` (when there is one) and whether the item is a `miss`. */
export type SortingItemResult = { readonly item: Slug; readonly value: number; readonly position: number; readonly rank: number; readonly guess?: number; readonly miss?: boolean; readonly explanation?: Text };

/** 🧮️ One matched item: the assigned card value or the guess (absent when unanswered), the correct value and, where the keys were hidden, whether the item is a `miss`. */
export type MatchingItemResult = { readonly item: Slug; readonly assigned?: number; readonly correct: number; readonly miss?: boolean; readonly explanation?: Text };

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

/** 🏁️ The scored run: its challenge, the mean of its task scores (accuracy at every challenge), its points = score × par of the challenge, tasks in sheet order. */
export type RunResult = { readonly quiz: Slug; readonly challenge: Challenge; readonly score: Score; readonly points: number; readonly tasks: readonly TaskResult[] };
//#endregion 🔖️Result

//#region 🔖️Lifecycle
/** 🎭️ How a learner appears: anonymous learners are always new; pseudonyms and names share one handle namespace; the handle is the normalized display. */
export type Identity = { readonly kind: "anonymous" } | { readonly kind: "pseudonym" | "name"; readonly handle: Handle };

/** 🖋️ An identity as a learner asks for it: the handle as typed (at most 256 code points), which the proctor normalizes or refuses. */
export type IdentityClaim = { readonly kind: "anonymous" } | { readonly kind: "pseudonym" | "name"; readonly handle: string };

/** 🧢️ The caps a proctor decides with, so that no learner and no client grows its state without bound. */
export type Limits = { readonly learners: number; readonly runsPerQuiz: number; readonly runs: number; readonly answersPerRun: number };

/** 🛟️ The caps of a proctor nobody configured: far beyond a real class (300 learners playing every quiz dozens of
 * times). The registrations are sized against what they cost a proctor to keep — a few kilobytes of disk each — and
 * against the pace one client address may register at, so that the cap is weeks away from any one of them. */
export const DEFAULT_LIMITS: Limits = { learners: 100_000, runsPerQuiz: 200, runs: 1_000, answersPerRun: 2_000 };

/** 🙋️ Register a new learner: an anonymous one in its own stream, a pseudonym or name in the stream of its handle key. */
export type IdentifyLearnerCommand = { readonly type: "identify-learner"; readonly id: Id; readonly learner: Id; readonly identity: IdentityClaim };

/** ▶️ Start a run of a quiz at a challenge at `at`, the instant the learner started it by the device's clock; at another challenge than the open run of the quiz it voids that run. */
export type StartRunCommand = { readonly type: "start-run"; readonly id: Id; readonly learner: Id; readonly origin?: Id; readonly run: Id; readonly quiz: Slug; readonly challenge: Challenge; readonly at: Timestamp };

/** ⏰️ Open one task of a timed run at `at`, the instant the learner acted by the device's clock: its clock starts. */
export type OpenTaskCommand = { readonly type: "open-task"; readonly id: Id; readonly learner: Id; readonly run: Id; readonly task: Slug; readonly at: Timestamp };

/** 🖊️ Record the latest answer to one task of an open run, given at `at` by the device's clock. */
export type RecordAnswerCommand = { readonly type: "record-answer"; readonly id: Id; readonly learner: Id; readonly run: Id; readonly task: Slug; readonly answer: Answer; readonly at: Timestamp };

/** 📨️ Submit an open run for scoring. */
export type SubmitRunCommand = { readonly type: "submit-run"; readonly id: Id; readonly learner: Id; readonly run: Id };

/** 📮️ Learner intent; the client-generated id makes a retry after a connection shortage apply exactly once. */
export type Command = IdentifyLearnerCommand | StartRunCommand | OpenTaskCommand | RecordAnswerCommand | SubmitRunCommand;

/** 🚫️ Every reason a command is rejected. */
export const REJECTIONS = [
  "unknown-learner",
  "unknown-quiz",
  "unknown-run",
  "unknown-task",
  "run-open",
  "run-closed",
  "run-incomplete",
  "answer-invalid",
  "quiz-revised",
  "handle-invalid",
  "handle-claimed",
  "id-invalid",
  "learner-exists",
  "roster-full",
  "runs-exhausted",
  "answers-exhausted",
  "run-untimed",
  "task-unopened",
  "time-up",
  "already-opened",
] as const;

/** ⛔️ One of {@link REJECTIONS}. */
export type Rejection = (typeof REJECTIONS)[number];

/** 🆕️ A learner was registered under the given identity. */
export type LearnerRegisteredEvent = { readonly type: "learner-registered"; readonly learner: Id; readonly identity: Identity; readonly at: Timestamp };

/** 🚀️ A run started at a challenge against the quiz revision (content hash) with the seed of its sheet. */
export type RunStartedEvent = { readonly type: "run-started"; readonly learner: Id; readonly run: Id; readonly quiz: Slug; readonly challenge: Challenge; readonly revision: string; readonly seed: number; readonly at: Timestamp };

/** 🗑️ An open run was voided because its quiz was revised or the learner started the quiz at another challenge. */
export type RunVoidedEvent = { readonly type: "run-voided"; readonly learner: Id; readonly run: Id; readonly at: Timestamp };

/** ⏳️ A task of a timed run was opened: its clock runs from `at`. */
export type TaskOpenedEvent = { readonly type: "task-opened"; readonly learner: Id; readonly run: Id; readonly task: Slug; readonly at: Timestamp };

/** 💾️ The latest answer to one task of an open run. */
export type AnswerRecordedEvent = { readonly type: "answer-recorded"; readonly learner: Id; readonly run: Id; readonly task: Slug; readonly answer: Answer; readonly at: Timestamp };

/** 📬️ A run was submitted and scored. */
export type RunSubmittedEvent = { readonly type: "run-submitted"; readonly learner: Id; readonly run: Id; readonly result: RunResult; readonly at: Timestamp };

/** 🎖️ A badge was earned by the submission of a run. */
export type BadgeAwardedEvent = { readonly type: "badge-awarded"; readonly learner: Id; readonly badge: Slug; readonly run: Id; readonly at: Timestamp };

/** ⚡️ A fact of the learner stream; a registration under a pseudonym or name is first the one fact of its handle stream. */
export type Event = LearnerRegisteredEvent | RunStartedEvent | RunVoidedEvent | TaskOpenedEvent | AnswerRecordedEvent | RunSubmittedEvent | BadgeAwardedEvent;
//#endregion 🔖️Lifecycle

//#region 🔖️Views
/** 🧾️ One task of a course quiz as the client lists it. */
export type CourseTaskView = { readonly id: Slug; readonly kind: TaskKind; readonly title: Text; readonly icon?: Icon };

/** 🗒️ One quiz of the course without its solutions. */
export type CourseQuizView = { readonly id: Slug; readonly emoji: string; readonly title: Text; readonly short?: ShortText; readonly description: Text; readonly tasks: readonly CourseTaskView[] };

/** 🏵️ One badge of the course without its rule. */
export type CourseBadgeView = { readonly id: Slug; readonly emoji: string; readonly label: Text; readonly description: Text };

/** 📖️ The solution-free course a client renders. */
export type CourseView = { readonly assignments?: readonly Assignment[]; readonly unlocks?: Unlocks; readonly id: Slug; readonly title: Text; readonly short?: ShortText; readonly introduction: Introduction; readonly quizzes: readonly CourseQuizView[]; readonly topics?: readonly Topic[]; readonly badges: readonly CourseBadgeView[] };

/** 🚥️ The states of a run. */
export const RUN_STATUSES = ["open", "submitted", "voided"] as const;

/** 🚦️ One of {@link RUN_STATUSES}. */
export type RunStatus = (typeof RUN_STATUSES)[number];

/** 🪞️ How a claimed relation between two items stands to the true one around its pivot (1 for a ratio, 0 for a difference): `reversed` when the claim lies on one side and the truth at the pivot or on the other, else `under` when the truth lies beyond the claim (a claim at the pivot: when the truth lies above it), else `over`. */
export const VERDICTS = ["under", "over", "reversed"] as const;

/** 🩻️ One of {@link VERDICTS}. */
export type Verdict = (typeof VERDICTS)[number];

/** 🤨️ Questions the relation the learner's keys claim between `item`, whose key misses, and `other` (in `dimension` for matching): `factor` = key(item) / key(other) on a logarithmic scale, `difference` = key(item) − key(other) on a linear one, exactly one of them; `verdict` says how the claim stands to the truth. */
export type CompareHint = { readonly kind: "compare"; readonly item: Slug; readonly other: Slug; readonly dimension?: Slug; readonly factor?: number; readonly difference?: number; readonly verdict: Verdict };

/** 🧐️ Questions whether `item` fits `category`, the assigned category, on `axis`, where the two categories' profiles lie farthest apart beyond the axis's reach; `other`, an item placed in its own category whose value on the axis lies strictly between the assigned and the own category's, and `above`, the side the placement claims for `item`, come together or not at all. */
export type ProfileHint = { readonly kind: "profile"; readonly item: Slug; readonly category: Slug; readonly axis: Slug; readonly other?: Slug; readonly above?: boolean };

/** 👯️ Questions whether `item` and `other` belong to the same category (`together`) or to different ones, as the learner's classification claims. */
export type GroupHint = { readonly kind: "group"; readonly item: Slug; readonly other: Slug; readonly together: boolean };

/** 📛️ Questions whether `item` belongs to `category`, the assigned category, where no pairing exists to question. */
export type CategoryHint = { readonly kind: "category"; readonly item: Slug; readonly category: Slug };

/** 💡️ What an open run at a challenge that hints asks the learner about the recorded answer of a task: one concrete relation the learner's answer claims, questioned, at most one per item (and dimension) and at most three per task. */
export type Hint = CompareHint | ProfileHint | GroupHint | CategoryHint;

/** 🥇️ The best submitted run of a quiz: the one with the most points (the earliest of equals), with its challenge and score. */
export type Best = { readonly challenge: Challenge; readonly score: Score; readonly points: number };

/** 🏃️ One run with its sheet, answers and, once submitted, its result; on a timed run also when each opened task was opened, and while it is open at a challenge that hints the hints per task. */
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
  readonly opened?: Readonly<Record<Slug, Timestamp>>;
  readonly hints?: Readonly<Record<Slug, readonly Hint[]>>;
};

/** 📇️ One run of a learner as listed, with its score and points once submitted. */
export type RunSummary = {
  readonly run: Id;
  readonly quiz: Slug;
  readonly challenge: Challenge;
  readonly status: RunStatus;
  readonly startedAt: Timestamp;
  readonly score?: Score;
  readonly points?: number;
  readonly submittedAt?: Timestamp;
};

/** 🎗️ A badge held by a learner with the run and time that earned it. */
export type BadgeAward = { readonly badge: Slug; readonly run: Id; readonly at: Timestamp };

/** 👤️ A learner's runs (newest first), badges, best run per quiz and total points. */
export type LearnerView = {
  readonly learner: Id;
  readonly identity: Identity;
  readonly runs: readonly RunSummary[];
  readonly badges: readonly BadgeAward[];
  readonly best: Readonly<Record<Slug, Best>>;
  readonly total: number;
};

/** 🎽️ One learner on a public leaderboard; it never carries the learner id, only its non-reversible tag (FNV-1a as 8 lowercase hex digits). `best` is the best run per quiz by points whatever the challenge, `total` their points, `runs` counts the submitted runs in the scope of the leaderboard, `lastActivity` is the last of them. */
export type LeaderboardRow = {
  readonly rank: number;
  readonly tag: string;
  readonly identity: Identity;
  readonly total: number;
  readonly reachedAt: Timestamp;
  readonly best: Readonly<Record<Slug, Best>>;
  readonly badges: readonly Slug[];
  readonly runs: number;
  readonly lastActivity: Timestamp;
};

/** 🔝️ How many rows a {@link Leaderboard} carries at most. */
export const LEADERBOARD_TOP = 100;

/** 🗓️ Which runs a leaderboard counts by when they were submitted, in the order they are offered: those of the current day, ISO week (from Monday) or month — calendar periods in UTC around the proctor's clock — or all of them. */
export const LEADERBOARD_PERIODS = ["daily", "weekly", "monthly", "all-time"] as const;

/** 📅️ One of {@link LEADERBOARD_PERIODS}. */
export type LeaderboardPeriod = (typeof LEADERBOARD_PERIODS)[number];

/** 🪟️ The time a leaderboard period spans: a run counts when it was submitted at or after `from` and before `until`. */
export type LeaderboardWindow = { readonly from: Timestamp; readonly until: Timestamp };

/** 🏆️ One leaderboard: the learners with a submitted run in its scope — the runs submitted inside `window` (every run when there is none), of `quiz` only when it names one: total ↓, badges ↓, reachedAt ↑, learner id ↑ (internally); rank is the 1-based position. Every row is made of the runs in scope only. `rows` are the top {@link LEADERBOARD_TOP} only, `learners` counts every ranked learner, `submissions` every run submitted in the course whatever the period and quiz, `own` is the caller's row when the query names a ranked learner — also when it is inside the top. */
export type Leaderboard = {
  readonly period: LeaderboardPeriod;
  readonly quiz?: Slug;
  readonly window?: LeaderboardWindow;
  readonly rows: readonly LeaderboardRow[];
  readonly learners: number;
  readonly submissions: number;
  readonly own?: LeaderboardRow;
};

/** 🔦️ Who holds a handle: the normalized display of the asked handle and, when claimed, the learner holding it with the identity it registered. Recalling a handle is this read. */
export type HandleView = { readonly display: Handle; readonly holder?: { readonly learner: Id; readonly identity: Identity } };

/** 🔍️ A query a proctor answers; `leaderboard` names its period, may name the one quiz it counts and the caller, `handle` carries the handle as typed. */
export type Query =
  | { readonly type: "course" }
  | { readonly type: "learner"; readonly learner: Id }
  | { readonly type: "run"; readonly run: Id }
  | { readonly type: "leaderboard"; readonly period: LeaderboardPeriod; readonly quiz?: Slug; readonly learner?: Id }
  | { readonly type: "crowd"; readonly quiz: Slug }
  | { readonly type: "handle"; readonly handle: string };
//#endregion 🔖️Views

//#region 🔖️Presence
/** 🖥️ Every page a learner can be on: `quiz` is the read-only page of one quiz, `learner` the own profile, `badges` every badge, `preferences` the settings; `run` and `results` belong to a run. */
export const SCREENS = ["introduction", "identity", "home", "quiz", "run", "results", "leaderboard", "learner", "badges", "preferences"] as const;

/** 📺️ One of {@link SCREENS}. */
export type Screen = (typeof SCREENS)[number];

/** 📌️ Where a learner is: the screen, for the quiz page, a run or its results the quiz, for a run the task on screen; learners at the same place share one presence room. */
export type Place = { readonly screen: Screen; readonly quiz?: Slug; readonly task?: Slug };

/** ⚓️ A landmark every learner at the same place renders, addressed by a stable key (`^[a-z0-9]+(?:[:-][a-z0-9]+)*$`, 1…64 chars, e.g. `task:power-ladder`). */
export type Anchor = string;

/** 🖱️ A pointer position relative (0…1) to the box of an anchor. */
export type Cursor = { readonly anchor: Anchor; readonly x: number; readonly y: number };

/** 🟢️ Ephemeral shared presence in the course-wide room: who is online and where, by public tag, never by learner id. */
export type PresenceState = { readonly tag: string; readonly identity: Identity; readonly place: Place; readonly active: boolean; readonly view?: PresencePresentation };

/** 💎️ A public run presentation carrying the exact shuffled sheet and answers without private learner or run ids. */
export type PresenceRun = Omit<RunView, "run" | "learner">;

/** 🎥️ A normalized scroll or camera position. */
export type PresencePoint = { readonly x: number; readonly y: number };

/** 🎨️ Public display choices affecting the followed presentation. */
export type PresencePreferences = {
  readonly device?: "desktop" | "tablet";
  readonly theme?: "system" | "light" | "dark";
  readonly textSize?: "normal" | "large" | "larger" | "largest";
  readonly showCursors?: boolean;
  readonly others?: "never" | "submitted" | "always";
  readonly animateIcons?: boolean;
  readonly iconsChosen?: boolean;
  readonly pets?: "off" | "still" | "calm" | "lively";
  readonly petsChosen?: boolean;
  readonly petsLiveliness?: "still" | "calm" | "lively";
  readonly petsPlay?: boolean;
  readonly petsMischief?: boolean;
  readonly challenges?: Readonly<Record<Slug, Challenge>>;
};

/** 🏅️ The visible public profile, omitting learner ids and private run links from history and awards. */
export type PresenceProfile = { readonly runs: readonly Omit<RunSummary, "run">[]; readonly badges: readonly Omit<BadgeAward, "run">[]; readonly best: Readonly<Record<Slug, Best>>; readonly total: number };

/** 🎤️ Ephemeral shared screen presentation, rendered by followers without modifying their own saved state. */
export type PresencePresentation = {
  readonly locale?: "en" | "de";
  readonly home?: string;
  readonly swipe?: string;
  readonly layout?: { readonly grid: "desktop" | "tablet"; readonly mode: "strip" | "swipe" };
  readonly reveal?: string;
  readonly revealedBy?: "focus" | "pointer";
  readonly taskPage?: number;
  readonly viewport?: PresencePoint;
  readonly camera?: PresencePoint;
  readonly preferences?: PresencePreferences;
  readonly run?: PresenceRun;
  readonly profile?: PresenceProfile;
  readonly asked?: readonly Slug[];
  readonly board?: { readonly period: LeaderboardPeriod; readonly quiz?: Slug };
  readonly sort?: { readonly key: string; readonly direction: "ascending" | "descending" };
  readonly submission?: { readonly phase: "confirming" | "saving" | "submitting" | "results"; readonly done?: number; readonly total?: number };
  readonly leaderboard?: Leaderboard;
  readonly leaderboardAt?: Timestamp;
  readonly overall?: Leaderboard;
  readonly overallAt?: Timestamp;
  readonly crowd?: CrowdView;
  readonly awards?: readonly Slug[];
};

/** 👆️ Ephemeral shared pointer, keyboard focus and the item being dragged in the room of one place; anchors may be cards, items (`item:<id>`) or categories (`category:<id>`). */
export type CursorState = { readonly tag: string; readonly place?: Place; readonly home?: string; readonly cursor?: Cursor; readonly focus?: Anchor; readonly drag?: { readonly item: Slug } };

/** 🧪️ A matching draft in semantic form: per dimension id the value assigned to each item id (card indices mean nothing to peers), or the guessed number where the keys are hidden. */
export type ThinkingMatchingAnswer = { readonly kind: "matching"; readonly values: Readonly<Record<Slug, Readonly<Record<Slug, number>>>> };

/** 🗨️ A draft answer as peers can read it: classification and sorting answers are already semantic, matching drafts carry values. */
export type ThinkingAnswer = ClassificationAnswer | SortingAnswer | ThinkingMatchingAnswer;

/** 💭️ Ephemeral shared draft answers of one learner's open run per task id, published in the thinking room of its quiz; carries the public tag only. */
export type ThinkingState = { readonly tag: string; readonly answers: Readonly<Record<Slug, ThinkingAnswer>> };
//#endregion 🔖️Presence

//#region 🔖️Crowd
/** 🎟️ How often one category (classification) or value (matching, rendered as a JSON number) was given. */
export type CrowdCount = { readonly key: string; readonly count: number };

/** 🙋‍♀️️ How often an item was answered and how: counts in ascending key order, or for sortings the mean normalized position (0 smallest … 1 largest) beside `places`, how often the learners put it at each place a sheet of the task presents (sums to `answers`). */
export type CrowdItem = { readonly item: Slug; readonly answers: number; readonly counts?: readonly CrowdCount[]; readonly meanPosition?: number; readonly places?: readonly number[] };

/** 🔟️ How many bins a {@link CrowdScores} has: `[0, 10)`, `[10, 20)`, … `[80, 90)`, `[90, 100]` in whole percent. */
export const CROWD_SCORE_BINS = 10;

/** 📉️ How many scores fell into each of the {@link CROWD_SCORE_BINS} bins, lowest bin first. */
export type CrowdScores = readonly number[];

/** 🐝️ The crowd of one task (one per dimension for matching): the scores of the results that count for it (the dimension's scores for a matching), items in definition order, unanswered items left out. */
export type CrowdTask = { readonly task: Slug; readonly kind: TaskKind; readonly dimension?: Slug; readonly scores: CrowdScores; readonly items: readonly CrowdItem[] };

/** 👪️ What the learners answered and scored in the submitted runs of one quiz — the run scores (summing to `runs`), then per task (and dimension) and item: a persisted shared projection of run-submitted events. */
export type CrowdView = { readonly quiz: Slug; readonly runs: number; readonly scores: CrowdScores; readonly tasks: readonly CrowdTask[] };
//#endregion 🔖️Crowd

/** 🎛️ Static software-native node appearance, independent of grading truth. */
export type MockupWidget = { readonly kind: "parameter" | "component" } | { readonly kind: "slider"; readonly min: number; readonly max: number; readonly value: number } | { readonly kind: "panel"; readonly lines: readonly MockupText[] } | { readonly kind: "plugin"; readonly label: Text };
