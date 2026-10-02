# ❓️ Quiz

The semio quiz product: a declarative, render-independent quiz model — catalogs, quizzes and their tasks,
seeded solution-free sheets, partial-credit scoring, badges, the learner run lifecycle and the
leaderboard — together with a React renderer and proctor client. The model knows nothing about the DOM
or the network; the proctor (`🎓️teaching/🛂️proctor`) wraps its pure deciders in the framework server, and
the renderer is one target among possible others. Two cores implement the model bit for bit: TypeScript
(`@semio-tech/quiz`) and Rust (`semio-framework-quiz`), side by side in every module.

## Layout

| Path | What lives there |
|---|---|
| `🧬️schema/🔣️.json` | The normative contract (JSON Schema draft-07): every document, command, event, query and view. |
| `🧬️schema/🟦️.ts`, `🧬️schema/🦀️.rs` | Hand-written type twins, one type per `$defs` entry under the same name. |
| `🔨️modules/🎲️randomness/` | FNV-1a run seeds, MT19937, the rejection-sampled uniform index and the Fisher–Yates shuffle. |
| `🔨️modules/🃏️sheet/` | `sheetOf(quiz, seed)`: the randomized, solution-free sheet of a run. |
| `🔨️modules/✅️validation/` | Structural and semantic validation of quizzes, catalogs and answers without any schema library; the handle policy (`normalizeHandle` and its owned Unicode tables), id and slug shapes, and the shape check of every command and query. |
| `🔨️modules/📏️scoring/` | Partial-credit scoring of tasks and runs. |
| `🔨️modules/🏅️badges/` | Badge rules evaluated after every submission. |
| `🔨️modules/🧾️lifecycle/` | The handle and learner deciders and the caps: pure `decide`/`evolve`. |
| `🔨️modules/👁️views/` | Catalog view, learner view, run view, transcripts, the windows of the periods and the leaderboards over them. |
| `🔨️modules/👥️presence/` | Presence rooms, the admission problems of presence and cursor states, and the client roster. |
| `📦️packages/🟦️typescript/` | `@semio-tech/quiz`: package glue only, a barrel over `🧬️schema` and `🔨️modules`. Zero runtime imports. |
| `📦️packages/🦀️rust/` | Crate `semio-framework-quiz` (lib `quiz`), nx `@semio-tech/quiz-rs`: `#[path]` glue over the Rust twins. |
| `🎯️targets/⚛️react/` | `@semio-tech/quiz-react`: the web renderer and proctor client. |
| `🧪️tests/🎚️config/🟦️.ts` | Vitest configuration of the TypeScript core. |
| `🧪️tests/<case>/` | Unit suites of the TypeScript core (`🟦️.ts` only) and the language-agnostic Protocol v2 cases (`🥒️.feature` with one adapter per language). |
| `🧫️fixtures/<case>/🔣️.json` | Shared vectors — inputs with expected outputs — read by both cores and by the Protocol v2 cases. |
| `🔮️oracles/🔣️.json` | The owner oracle registry: the third-party references the cases compare against. |

## Commands

Everything runs through each package's `📜️script.ts`; `📋️project.json` registers the same entry points as nx
targets.

```bash
cd "🧰️framework/🛍️products/❓️quiz/📦️packages/🟦️typescript"
bun ./📜️script.ts test quick              # the TypeScript core

bun nx run @semio-tech/quiz:test           # the same through nx
bun nx run @semio-tech/quiz-rs:test        # the Rust core
bun nx run @semio-tech/quiz-react:test     # the renderer
```

## Domain model

### Catalog

A catalog is what one site offers: an id, a title, the introduction a learner reads on the first visit, the
quizzes (file paths relative to the catalog file) and the badges spanning them. The proctor loads the catalog
and every quiz it names; the revision of a quiz is the lowercase hex SHA-256 of its file bytes.

### Quiz

A quiz is an ordered set of tasks with an id, an emoji, a title and a description. The emoji — one emoji
grapheme of 1…16 code points, required — identifies the quiz on cards and headers and is carried into the
catalog view. A run presents the tasks randomized and is scored only as a whole. Every learner-visible text carries English and German; there is no default
language.

### Task kinds

- **Classification** — assign every item to one category. Categories may carry a profile: a value on every axis
  of the task (at least three axes, rendered as a spider diagram). Items name their correct category.
- **Sorting** — order the items ascending by their quantity. Items carry their true value. A learner may also
  type a numeric guess per item (`2 kW`, `1,5 MWh`): the field shows the unit, reads an optional SI prefix and
  reorders the guessed items among the places they occupy by their guesses; moving a guessed item by hand removes
  its guess. Guesses never change the score — they are a way to arrive at an order.
- **Matching** — for every dimension (a quantity), assign each item one of the offered value cards; the cards
  are the multiset of the drawn items' true values.

A quantity has a label, a unit, a scale — `linear`, or `logarithmic` for quantities spanning orders of
magnitude — and whether display scales the unit with SI prefixes. A task may `draw` a random subset of its
items.

A task may carry an `icon`: one emoji grapheme (1…16 code points) that pictures what the task asks about and
one of the looping microanimations `bounce`, `pulse`, `spin`, `sway`, `float` or `flip`. The icon is part of
the sheet and the catalog view and replaces the icon of the task's kind in its card's title chip. The glyph is
hidden from assistive technology; it plays only where the learner neither prefers reduced motion nor switched
"Animate task icons" off in the preferences, and stands still everywhere else.

### Sheet

The sheet is the randomized, solution-free presentation of a quiz for one run, a pure function of the quiz
and the run seed, bit-exact across both cores:

1. `seed = FNV-1a-32(UTF-8 bytes of the run id)`; the generator is MT19937 seeded with `init_genrand(seed)`.
2. `uniform(n)`: `n = 1 → 0` without a draw; else draw until `x < 2³² − (2³² mod n)` and return `x mod n`.
3. `shuffle`: Fisher–Yates from the end, `j = uniform(i + 1)` for `i = len − 1 … 1`, on a copy.
4. The task order is shuffled first; then every task in definition order shuffles its items (and keeps the
   first `draw`), a classification shuffles its categories, a sorting already in ascending order (by value,
   ties by definition index) is rotated left by one, and a matching shuffles the cards of every dimension.

Sheet items carry only `{ id, label }`.

### Answer

- Classification: a category id per item id. Sorting: the item ids, smallest first, and optionally a numeric
  `guesses` value per item id in the quantity's base unit. Matching: per dimension a card index per item id.
- An answer is valid when its kind matches, it references only items, categories and dimensions of the sheet
  task, card indices are in range and used at most once per dimension, and a sorting is a permutation of the
  sheet items whose `guesses` name only sheet items with finite values (positive on a logarithmic scale) and
  keep the guessed items in non-decreasing guess order within the order. Partial classification and matching
  answers are valid while the run is open.
- Complete: every item classified; every item matched in every dimension; a recorded sorting always.

### Scoring

Every task scores in [0, 1]; a run scores the mean of its task scores in sheet order. `s(v)` is `v` on a linear
scale and `log₁₀ v` on a logarithmic one (values must then be positive).

**Sorting — magnitude-weighted pair concordance.** Over the learner's order `o`, for every pair `i < j`:

```
w = |s(v[oᵢ]) − s(v[oⱼ])|        total += w        if v[oᵢ] > v[oⱼ]: discordant += w
score = total > 0 ? 1 − discordant / total : 1
```

**Matching** — per dimension, items in sheet order, `aᵢ` the assigned card value and `tᵢ` the true value:
`w = |s(tᵢ) − s(tⱼ)|`; the pair is discordant (`w`) when `sign(tᵢ − tⱼ) · sign(aᵢ − aⱼ) < 0`, and half
discordant (`w / 2`) when `aᵢ = aⱼ` on distinct true values. The task score is the mean over the dimensions.

**Classification — profile similarity.** A correct item earns 1. A wrong item earns
`max(0, 1 − d(assigned, correct) / d_max)` when both categories carry profiles, else 0; `d` is the Euclidean
distance of the profiles normalised per axis to `(v − min) / (max − min)`, and `d_max` the largest such distance
over all profiled category pairs (`d_max = 0 → 0`).

**Why large misorders cost more than small ones.** Counting inverted pairs treats every mistake alike:
swapping a tea light (30 W) with a bulb (60 W) would cost as much as swapping it with a nuclear plant
(1.4 GW). Weighting each pair by the distance of its true values on the quantity's scale makes the penalty
proportional to how wrong the learner's mental picture is. On a logarithmic scale the distance counts orders of
magnitude, so confusing two neighbours of the same decade costs almost nothing while inverting the extremes of
a ladder spanning eight decades costs almost everything. The order of the learner is perfect (1) exactly when
nothing is discordant and worst (0) exactly when it is reversed; repairing any adjacent inversion strictly
raises the score. Two classical statistics fall out as special cases and serve as third-party checks: with unit
weights the score is `(1 + τₐ) / 2` (Kendall), and with equally spaced ranks as values on a linear scale it is
`(1 + ρ) / 2` (Spearman). The same reasoning drives profile similarity: confusing a low-energy house with a
passive house earns more than confusing it with an unrenovated one.

Scoring never throws and never yields NaN on inputs that bypass validation: `scoreTask`/`scoreRun` return
`undefined` (Rust: `None`) for an invalid or incomplete answer or an unresolvable task, a mean over nothing is 0, a
category whose profile misses an axis counts as unprofiled, and a matching item missing a dimension value is
presented as a NaN card and leaves its task unscored; a complete run that cannot be scored is rejected
`run-incomplete`.

Results list classification items in sheet order (`assigned`, `correct`, `credit`), sorting items in the
learner's order (`value`, `position`, `rank` in the true ascending order), and matching items per dimension in
sheet order (`assigned` and `correct` value), each with the item's explanation. Scores agree across the cores
within 1e-12 (`log₁₀` may differ by one ulp); a perfect score is exact.

### Badges

Badges are evaluated after every submission, in catalog order, over all submitted results of the learner
including the new one; badges already held are skipped, and a new award records the triggering run and time.

- `perfect-quiz { quiz }` — some result of the quiz scored 1.
- `perfect-tasks { taskKind?, quiz? }` — every catalog task matching the selector scored 1 in some result; a
  selector matching no task never awards.
- `completed-quizzes` — every catalog quiz has a submitted result.

### Run lifecycle

Two deciders share the lifecycle. The **handle** decider owns one handle key; the **learner** decider owns one
learner. Each row is checked in order; the first that applies decides.

| Decider | Command | Condition | Outcome |
|---|---|---|---|
| both | any | an id or slug of the command has not its shape | reject `id-invalid` |
| handle | `identify-learner` | anonymous identity, a handle outside the policy, or a handle of another key | reject `handle-invalid` |
| handle | `identify-learner` | the handle is held | reject `handle-claimed` |
| handle | `identify-learner` | otherwise | `learner-registered { identity }` with the normalized handle |
| learner | any | the command names another learner | reject `unknown-learner` |
| learner | `identify-learner` | pseudonym or name (those register at their handle) | reject `handle-invalid` |
| learner | `identify-learner` | anonymous, already registered | reject `learner-exists` |
| learner | `identify-learner` | anonymous, otherwise | `learner-registered { identity: anonymous }` |
| learner | `start-run` | learner not registered | reject `unknown-learner` |
| learner | `start-run` | quiz not loaded | reject `unknown-quiz` |
| learner | `start-run` | the run id is taken | reject `run-open` (open) or `run-closed` |
| learner | `start-run` | open run of the quiz at the current revision | reject `run-open` |
| learner | `start-run` | `limits.runs` submitted runs in total, or `limits.runsPerQuiz` of the quiz | reject `runs-exhausted` |
| learner | `start-run` | open run of the quiz at a stale revision | `run-voided`, `run-started` |
| learner | `start-run` | otherwise | `run-started { revision, seed = FNV-1a(run) }` |
| learner | `record-answer` | run unknown / not open | reject `unknown-run` / `run-closed` |
| learner | `record-answer` | stale revision | reject `quiz-revised` |
| learner | `record-answer` | the run already recorded `limits.answersPerRun` answers | reject `answers-exhausted` |
| learner | `record-answer` | task not in the sheet / answer invalid | reject `unknown-task` / `answer-invalid` |
| learner | `record-answer` | otherwise | `answer-recorded` — the latest answer per task wins |
| learner | `submit-run` | run unknown / not open | reject `unknown-run` / `run-closed` |
| learner | `submit-run` | stale revision | `run-voided` |
| learner | `submit-run` | incomplete | reject `run-incomplete` |
| learner | `submit-run` | otherwise | `run-submitted { result }`, then `badge-awarded` per newly earned badge |

Deciders are pure: `decideHandle(state, command, now)` and `decideLearner(state, command, { now, catalog, quizzes,
limits })` return `{ events }` or `{ rejection }`; `evolveHandle(state, event)` and `evolveLearner(state, event)`
fold one event; `at` is the decision time. Idempotency by command id belongs to the framework server
(`CommandEnvelope.idempotencyKey`). Before a registration is decided, `registrationRejection(learners, limits)` is
`roster-full` once a proctor holds `limits.learners` registrations.

**Caps** (`Limits`, `DEFAULT_LIMITS`) keep every state bounded: `learners` 100 000 registrations, `runs` 1 000
submitted runs per learner, `runsPerQuiz` 200 of them in one quiz, `answersPerRun` 2 000 recorded answers per run;
a quiz has at most one open run per learner. A proctor configures the two totals.

### Identity

A learner is its id — 32 lowercase hex characters, 128 random bits, the only secret there is — and is anonymous or
shows a pseudonym or a name, its **handle**. Pseudonyms and names share one key space.

- **Ids and slugs.** `isId` is exactly `^[0-9a-f]{32}$`, `isSlug` is `^[a-z0-9]+(?:-[a-z0-9]+)*$` with at most 64
  characters. `commandRejection(command)` and `queryRejection(query)` hold every id of a command or query to its shape
  (`id-invalid`) and a queried handle to the policy (`handle-invalid`); the deciders call the first themselves.
- **Handle policy.** `normalizeHandle(text)` answers `{ display, key }` or nothing (`handle-invalid`), bit for bit the
  same in both cores:
  1. more than `HANDLE_INPUT_MAX` (256) code points: refused;
  2. every run of Unicode `White_Space` is one space, leading and trailing ones are dropped, `’` (U+2019) is `'`;
  3. every remaining code point is a digit, a letter of `HANDLE_LETTERS` (basic Latin, Latin-1 Supplement, Latin
     Extended-A and -B and Latin Extended Additional letters — umlauts, `ß`, accents; 681 letters), one of `' . _ -`,
     or a single space between words; anything else — control, format, zero-width and bidirectional characters,
     combining marks, other scripts, emoji — is refused;
  4. at least one letter or digit, at most `HANDLE_MAX` (64) code points;
  5. `display` is the result, `key` its lowercase form: two handles are the same handle when their keys are equal.
- **No normalization library.** The alphabet contains no combining mark and no letter that changes under NFC, so
  every admitted handle is in NFC and canonically equivalent handles are equal code points; a decomposed spelling is
  refused, not composed. The tables are owned by the cores and held to the Unicode Character Database by the tests
  (`unicodedata`, ICU, `unicode-normalization`), and the schema's `Handle` pattern is the same alphabet.
- **Registering.** A pseudonym or name is registered by its handle's own decider, exactly once
  (`handleActorId(key)`, the lowercase hex of the key's UTF-8 bytes, names it; `handleKeyOf` is its inverse). An
  anonymous learner registers at its own decider.
- **Recalling** a claimed handle is a read (`Query` `handle` → `HandleView { display, holder? }`), never a command:
  it writes no event.

### Leaderboards

There are four leaderboards — **daily, weekly, monthly and all-time** (`LEADERBOARD_PERIODS`) — and each of them
for every quiz of the catalog or for one. A leaderboard has a **scope** (`boardScope(period, quiz, at)`): the
window of its period around the instant it is asked at, and the one quiz it counts when it names one.
`periodWindow(period, at)` is that window: the day, the ISO week (from Monday) or the month that contains `at`, in
UTC, half-open (`from ≤ submitted < until`); all-time has none. The periods are calendar periods in UTC so that a
board is the same for every viewer and a pure function of the events and the instant; the answer carries the
window, and a client shows its bounds in the viewer's own time zone.

Every learner with a submitted run has a **transcript** (`transcript(state)`): the submitted runs in submission
order (quiz, score, instant) and the badges, each with the quiz of the run that earned it, beside the learner id.
A transcript changes only when a run is submitted, a badge is awarded or the identity changes — never when an
answer is recorded. The **standing** of a transcript in a scope (`standing(transcript, catalog, scope)`: the
unranked row beside the learner id) is made of the runs in scope only: the total is the sum of their best scores
over the catalog quizzes in points (score × 100) — the best score of the one quiz on a board of one quiz —,
`reachedAt` is the one of them that last raised a best score, `runs` counts them, `lastActivity` is the last of
them and `badges` are those they earned; a learner with no run in scope has no standing there.
`compareStandings` orders the standings by total descending, badge count descending, `reachedAt` ascending and
learner id ascending; the rank is the 1-based position.

`leaderboard(transcripts, catalog, { period, quiz? }, at, caller?)` is the view:
`{ period, quiz?, window?, rows, learners, submissions, own? }` — the top `LEADERBOARD_TOP` (100) rows, the number
of ranked learners, the number of runs submitted in all (whatever the scope: it grows with every submission, so a
client knows when what submissions change is worth asking for again), and the caller's own row with its true rank
when the caller is ranked, inside the top or far below it. A proctor serving many learners keeps the transcripts
and, per board, what orders the learners on it in rank order, and answers the same view without sorting.

The leaderboard is public, so its rows never carry the learner id: without passwords that id is the only thing
standing between an anonymous learner and anyone acting as them. A row carries the learner's `tag` instead —
`learnerTag(learner)`, FNV-1a of the id as 8 lowercase hex digits — and a client highlights its own row by
computing the tag from its own id. The id still breaks ties internally.

### Presence and cursors

Every learner sees who else is online and where, and — in the room of the same place — their pointer and keyboard
focus. Presence is ephemeral shared state relayed by the framework server's presence sockets (latest state per
session, coalesced per tick); the quiz core owns the rooms, the admission rules and the roster.

- **Place** — the screen (`introduction`, `identity`, `home`, `quiz` — the read-only page of one quiz, `run`,
  `results`, `leaderboard`, `learner` — the own profile, `badges`, `preferences`), for the quiz page, a run or its
  results the quiz, for a run the task on screen. Learners at the same shared place meet in one room.
- **Rooms** — `rosterScope(catalog)` = `<catalog>` carries every learner's `PresenceState` (tag, identity, place,
  active). `roomScope(catalog, place)` names the room carrying `CursorState`s: `<catalog>/introduction`,
  `<catalog>/home`, `<catalog>/leaderboard`, `<catalog>/badges`, and `<catalog>/quiz/<quiz>` for the page, the runs
  and the results of one quiz. It is `undefined` for the identity screen (no tag exists before identification), for
  the personal `learner` and `preferences` pages, and for a quiz page, run or results without a quiz.
- **Anchors and cursors** — a cursor is `{ anchor, x, y }` with `x, y ∈ [0, 1]` relative to the box of an anchor every
  learner at that place renders (`home`, `leaderboard`, `card:quiz:<quiz>`, `task:<task>`, `item:<id>`,
  `category:<id>`, …), so positions survive different viewports and randomized item orders; `focus` is the anchor a
  keyboard user is on and `drag { item }` the item being dragged. Sharing what the others think is intended (see
  below); learner ids are never shared.
- **Admission** — `presenceProblem(state)` / `cursorProblem(state)` return the first issue (smallest path, then code)
  or `undefined`; the proctor refuses a state with a problem. The rules: the schema (`presenceIssues` /
  `cursorIssues` list them all) with `tag-invalid` (not 8 lowercase hex digits — a learner id is refused),
  `anchor-invalid`, `out-of-range` (a coordinate that is not finite in 0…1), `handle-invalid` (an identity whose
  handle is not a normalized handle of the handle policy), plus the place rules the schema cannot state: the quiz page, a run and its results name their quiz
  (`required` at `/place/quiz`), only they name a quiz (`quiz-outside-run`), only a run names a task
  (`task-without-run`).
- **Roster** — `presenceRoster(entries, display)` folds the socket's entries (`{ session, state }`) into the client's
  roster: one learner per tag (represented by an active session, then the smallest session id; all sessions
  listed), `online` and `active` learner counts, learners per quiz now (distinct learners on the page, a run or the
  results of the quiz) and the learners sorted by their display label (lowercase, then exact, then tag; code point order). The
  label comes from the client (`display(state)`), so the core stays language-neutral.

### What the others think

The quizzes are for fun: learners see what the others answered — by default once they have submitted, before that
only when they ask for it — and everything about the others is a figure. Sheets differ per learner, so everything is
aggregated semantically by item id, never by position.

- **Drafts (ephemeral shared)** — the thinking room `thinkingScope(catalog, quiz)` = `<catalog>/quiz/<quiz>/thinking`
  carries every open run's `ThinkingState { tag, answers }`: the learner's current, possibly partial draft answers per
  task as `ThinkingAnswer`s — classification and sorting answers as they are (already semantic: item and category
  ids), matching drafts as `{ kind: "matching", values: dimension → item → value }`, because card indices point into
  the publisher's own shuffled cards and mean nothing to peers. `thinkingAnswer(sheetTask, answer)` turns the
  publisher's own answer into its draft (card index → card value; `undefined` for an answer that does not fit the
  sheet task). `thinkingProblem(state)` (all issues: `thinkingIssues`) admits a state whose drafts are structurally
  valid per the schema, with finite values and guesses (`type-invalid` otherwise), and bounded by `THINKING_LIMIT` = 64: at most 64
  tasks and 64 entries per assignment map, order, guesses map, values map or dimension (`too-many`), no repeated item in an order
  (`duplicate-id`).
- **Live crowd** — `thinkingCrowd(states, sheetTask)` folds the peers' drafts (one per tag, the last given winning) for
  the viewer's sheet task, items in the viewer's sheet order, unanswered items left out: classification votes per
  category, matching votes per value key (`valueKey`) per dimension — keys and tags in code point order — and sorting
  positions `index / (len − 1)` in each learner's own order (0 for a single item).
- **Cursors on items** — the quiz room's `CursorState` may anchor to items (`item:<id>`) and categories
  (`category:<id>`) and carries `drag { item }` (a slug), so peers see what someone drags and where they point.
- **Crowd view (persisted shared)** — `crowdView(quiz, results)` aggregates the submitted results of one quiz (results
  of other quizzes ignored, `runs` counts the rest; per result the first task result with the task's id and kind):
  classification counts per assigned category, sorting the mean of the normalized position `position / (n − 1)` (0
  when `n < 2`) summed in result order, matching one crowd task per dimension with counts per assigned value. A value's
  key is `valueKey(value)`: ECMAScript `Number::toString`, the shortest round-trip JSON number (`0.12`, `250`,
  `1e+21`, `1e-7`, `-0` → `0`). Tasks, dimensions and items follow the definition order, unanswered items are left
  out, and counts are ordered by key in code point order — category ids and value keys alike, so `120` precedes `15`.
  The proctor keeps it as a projection and answers the query `{ type: "crowd", quiz }`.
- **Crowd distributions** — the crowd view carries whole distributions, so every figure is drawn from data the proctor
  already aggregated:
  - `scores` are `CROWD_SCORE_BINS` = 10 counts, the bins `[0, 10)`, `[10, 20)`, … `[80, 90)`, `[90, 100]` of the
    whole percent a client prints: `scoreBin(score) = min(9, ⌊points / 10⌋)` with `points = ⌊score · 100 + ½⌋`.
    `CrowdView.scores` bins the run scores of the counted results and sums to `runs`; `CrowdTask.scores` bins the
    scores of the results that count for the task — the task score of a classification or sorting, and for a
    matching's crowd task the score of that dimension in the result's first task result of the task (a result without
    the dimension does not count).
  - `CrowdItem.places` (sortings only, beside `meanPosition`) has one count per place a sheet of the task presents,
    `presented(task) = min(draw, items)` (every item without `draw`). A position `i` in a learner's order of `n` items
    counts for `placeBin(i, n, m) = n < 2 ? 0 : ⌊(2 · i · (m − 1) + (n − 1)) / (2 · (n − 1))⌋` — `i · (m − 1) / (n − 1)`
    rounded half up in integer arithmetic, `i` itself when `n = m`, never beyond the last place — so the places sum to
    `answers` whatever the length of each order (a `draw` changed after runs were submitted re-bins them).
  - A quiz nobody submitted has ten zeros in every `scores` and no items.
  Rust names them `CROWD_SCORE_BINS`, `score_bin`, `place_bin`, `presented` and `crowd_view`.
- **When the others show (web client)** — the persisted local-only preference `others` is `never`, `submitted` (the
  default) or `always`. `crowdGate(others, place, { asked, submitted })` answers for a run, the page of a quiz and the
  results of a run: `off` (never), `open` (always; the results; a quiz's page once the learner has a submitted run of
  the quiz), `asked` (the learner asked to see the others now) or `locked` (not yet). A locked gate shows one button,
  "Show it now", and why the others do not show yet; asked, the same button hides them again. The ask is ephemeral
  local-only (`QuizState.asked`, events `crowd-asked` / `crowd-unasked`, `session.askCrowd(quiz)` /
  `unaskCrowd(quiz)`) and ends with the run — submitted, voided or closed elsewhere — and with the learner, so the next
  run starts with the others out of sight again. The crowd is loaded when a run opens, so an ask shows at once, also
  without a connection. A learner's own drafts are published either way.
- **Figures (web client)** — nothing about the others is a sentence to be parsed; it is drawn, and every drawing is a
  table or list that says its numbers in words, never a live region, and keeps its size whatever the crowd puts into
  it. `answerFigure` gives a task (a matching: each dimension) a row per item of the learner's sheet and a column per
  category in sheet order, per place, or per value anyone gave, ascending; a cell is a column as tall as the share of
  the item's submitted answers under that key with the percentage on its cap, a dot in the presence colour of every
  learner thinking it right now, ● on the learner's own answer — whose column wears the active colour — and, once
  there is a result, ✓ on the correct one; a sorting row ends with the average place. `scoreFigure` draws the ten
  score bins over a percent axis with the learner's own bin in the active colour: the run scores beside the score on
  the results and on a quiz's page, the task's (a matching: the dimension's) scores beside every answer figure on the
  results. During a run the figures lie below the task's interaction, so nothing above them moves.

### Overview

Home is the layered overview of semio-tech play and the mit-bestand demonstrator, the same element used the same way
(`🎯️targets/⚛️react/🔨️modules/🏠️home` on the design system's `LayeredOverview`). It has two layers.

- **In front: the cards**, fixed on a grid — the learner, each quiz, how it works, the leaderboard at the larger centre,
  the badges, the preferences.
- **Behind one glass: the pages** those cards open, each as large as the screen, side by side on one strip in the
  cells of the same grid, all live.
- **Between the cards** the mouse pans the strip under the glass: the top-left corner shows the first page, the
  bottom-right the last.
- **On a card** (hover or keyboard focus) the strip glides to its page and the glass clears over it; leaving the card
  restores the glass. The card's heading, its action or the page's hash opens the page.
- **Whatever the device says about motion**: the pan and the glide are how the overview is read, and every browser in
  a Remote Desktop session reports reduced motion without its learner having asked for it, so they run there too, as
  they do on play and the demonstrator. Touch has no pointer to follow.
- **Narrow or short viewports** list the pages one below the other, each under its own glass and card, as play and the
  demonstrator do on phones.

### The leaderboards on screen

The leaderboard page and the card at the centre of the overview show one of the leaderboards at a time
(`🎯️targets/⚛️react/🔨️modules/🏆️leaderboard`), and the learner chooses which:

- **Period.** A segmented choice — Today, This week, This month, All time — on the page and on the card. A leaderboard
  of a period says what it counts: its window, in the learner's own time zone.
- **Category.** A second segmented choice on the page: every quiz, or one quiz of the catalog. The leaderboard of one
  quiz shows its points in place of the total and the best scores of every quiz; the card names the chosen category.
- **Sorting.** Every column heading of the page's table and of the card's excerpt is a button: a click puts the table
  in that column's order, a second click turns it around (`aria-sort` says which). The order is the reader's own and
  holds over every leaderboard; where the chosen column is not shown, the table is by rank.
- **One choice, held answers.** The choice is `QuizState.board` (ephemeral local-only, all-time of every quiz at first)
  and holds for the page and the card alike; `QuizSession.chooseBoard` changes it and asks for that leaderboard. The
  client keeps the last answer of every leaderboard looked at (`QuizState.leaderboards`), so one chosen again shows at
  once while it is asked for anew, and only the one looked at is polled. While it is not the overall one, the overall
  one — the learner's rank on the profile — is asked for again whenever an answer says that runs were submitted since.
- **What submissions change.** `QuizState.submissions` is the count of submitted runs the last answer carried; the
  overview asks for the crowd of every quiz again when it moves.

### Finding the way

The navbar leads through the client (`🎯️targets/⚛️react/🔨️modules/🚏️navigation`): the ways on its left, what the
quizzes are about — the catalog's title beside the site's logo — in its middle, the connection, who is online and the
language on its right.

- **Four ways**, in this order: the overview, back, forward and up. Each is a button named with the place it leads to
  ("Back: Leaderboard"); one that leads nowhere keeps its place and its tab stop and says that it is unavailable. They
  appear once a learner is identified. Escape still closes an opened page of the overview, and the overview's button
  names it as its key.
- **The trail** (`QuizState.trail`, ephemeral local-only like the step) holds the steps behind the one in front and
  those ahead of it. Opening a step leaves the one in front behind and forgets the steps ahead; back and forward lead
  to the nearest step that still stands — the overview and its pages always, a run while it is open, results once
  their run is submitted — so a submitted or voided run leaves the trail by itself. Up opens the place above like any
  opening: the overview above its pages, the page of its quiz above a run and its results. A new learner starts a new
  trail; it keeps at most `TRAIL_LIMIT` = 50 steps behind.
- **The address** names the page of the overview that shows (`#board`, `#<quiz>`) and nothing anywhere else. On
  arrival a hash that names a page opens it instead of the overview (nothing lies behind it); a hash changed later
  opens its page like any opening. The client replaces the address in place and writes no entry into the browser's
  history: the trail is the client's own, and the browser's back button leaves the site as before.
- **Narrow navbars** keep every control whole: the overview's word and the language names give way to the icon and
  the language codes below tablet width, and the title in the middle ends in an ellipsis, then leaves, then the logo
  leaves — nothing is cut in half and nothing lies over a control.

Shared vectors: `🧫️fixtures/🚏️navigation/🔣️.json`; the trails that close no run are replayed on jsdom's session
history (`🧪️tests/🚏️navigation`).

### Pets

A site may give the client pets: small rigged, animated companions of the pets product (`🧰️framework/🛍️products/🐾️pets`,
`@semio-tech/pets`, drawn by `@semio-tech/pets-react`) that stand on the cards of the screen, blink, follow the pointer
with their eyes and meet each other. The quiz knows no species and no pet travels over the wire or enters a quiz
document: the site hands `mountQuiz` where its menagerie comes from (`pets: () => Promise<Menagerie>`), and the glue
lives in the renderer (`🎯️targets/⚛️react/🔨️modules/🐾️pets`).

- **Choice** — the preference `pets` is `off`, `still`, `calm` (unless the learner chose otherwise) or `lively`; it is
  persisted local-only. The menagerie and the layer are one lazy chunk that is fetched only when pets are wanted:
  `off` — or a site without pets — downloads and renders nothing. A device that forces its own colours gets no pets
  whatever was chosen, and the preferences say so under the choice (`usePetsForced`).
- **Reduced motion** — `prefers-reduced-motion: reduce` decides the default only. A learner who has not chosen gets
  `still` on such a device (`calm` on any other); the preferences press what is in effect, `data-pets` on `.quiz-app`
  carries it, and the note under the choice says "Your device asks for less motion, so the pets stay still until you
  choose." (`usePetsReduced`). A choice the learner made — any of the four in the preferences, or the switch — holds
  as chosen on every device, and the note is gone (`effectivePetMode(choice, chosen, reducedMotion)`). That the learner
  chose is a fact of its own in the stored preferences, `petsChosen`: the preferences are written as a whole on every
  change, so the presence of `pets` in storage proves nothing. Only `withPets` sets it — the pets' row and the switch
  call it, no other preference does — and stored preferences without it count as not chosen. Why: some devices report
  reduced motion without their learner having asked for it (every browser in a Remote Desktop session does), and a
  learner who explicitly asks for moving pets should get them. The client shortens every transition on such a device
  (`.quiz-app * { transition-duration: 0.01ms !important }`); the pet layer forbids transitions inside itself, so a
  frame of moving pets shows as written instead of starting hundreds of transitions a second.
- **Switch** — every screen carries a "Show pets" checkbox on its footer line (`PetsSwitch`; WCAG 2.2.2, pause, stop,
  hide): it turns the pets off and back on. `petsLiveliness` remembers the last choice other than `off`, so the switch
  brings the pets back as lively as they were (`switchedPets`, `withPets`) — `calm` when there was none yet. Both ways
  count as choosing. A site without pets has no switch.
- **Scene** — `petScene(step, runs, catalog)` is the id of the quiz whose page is opened, that is being played or whose
  results show, and `home` everywhere else; the menagerie's cast of that scene is on stage, so the pets fit the topic.
  The preferences name the pets that are on stage right now in the learner's language (the layer reports them through
  `onCast`; `petNames`, `usePetCast`). A run is a time of concentration: while one is on screen the pets rest — their
  eyes still follow the pointer, nothing else does, the cast does not rotate and a click on a pet does nothing.
  Results are not: a learner who reads a score is done concentrating, and the switch is one Tab away.
- **Tempo** — `data-pets-tempo` on the document root (`QUIZ_PETS_TEMPO`, read once per mounted layer by `petsTempo()`)
  makes the pets' time pass up to eight times as fast. It is a test seam — nothing in the client sets it — for the
  end-to-end proofs that wait for a walk or for two pets that meet, which takes a minute or two of real time.
- **Stage** — ephemeral local-only: every device simulates its own pets. They stand on what the cards of the screen
  show — the title tab, the edge of the body beside it — and on the footer line (`QUIZ_PET_SURFACES`), never in front
  of a card, keep clear of controls, text, drag items, drop zones and the navigation bar (`QUIZ_PET_KEEPOUTS`), turn
  see-through while the pointer rests on them, and glance at the cursors of other learners while those show. Where a
  screen leaves no room — a task that fills a phone — no pet shows. The layer is decoration: hidden from assistive technology, never in the
  way of the pointer or the keyboard; a menagerie that cannot be fetched or a layer that fails leaves the quiz as it is.

### State classes

| Class | Held where | Examples |
|---|---|---|
| Persisted shared | the proctor's event store and projections | learner, run, answer, submission and badge events; the crowd view |
| Persisted local-only | the browser | learner id, locale, theme, when the others' answers show, the outbox of commands the proctor has not decided yet, cached catalog, learner and run views |
| Ephemeral shared | polled from the proctor / relayed by presence sockets | the leaderboards looked at; presence, cursors and drafts (thinking) |
| Ephemeral local-only | the renderer | drag state, focus, the current step, the leaderboard chosen and the order of its table, the ask to see the others before submitting |

Answers apply locally at once and travel through an outbox coalesced per run and task, retried with jittered
backoff and applied once by command id, so short connection shortages never freeze a run.

### While the proctor is away

A site may hand the web client its material — the catalog and its quizzes, solutions included
(`mountQuiz(root, { material })`). The client then has a **deputy** (`🎯️targets/⚛️react/🔨️modules/🫡️deputy`): the
core's own deciders on the device.

| | Without material | With material |
|---|---|---|
| Catalog | shows once the proctor answered (cached afterwards) | shows at once; the proctor's replaces it when it answers |
| The proctor answers | the proctor decides every command | the same: the proctor decides, nothing is provisional |
| The proctor does not answer | answers wait in the outbox; registering, starting and submitting wait for the proctor | the deputy decides: a learner registers, starts runs, submits them and earns badges on the device |
| Leaderboard without the proctor | the last standings held, else none | the last standings held, else the device's own standing, marked as such |

- **Who decides.** A command goes straight to the proctor unless the proctor is known not to answer (it did not, it asked
  to slow down, the browser is offline) or the device is *ahead* — a command the deputy decided still waits in the
  outbox, and nothing may overtake it. A direct attempt that meets a connection shortage falls to the deputy; the same
  command id makes a command the proctor did receive the same command when it arrives again. A learner waits
  `DEPUTY_PATIENCE_MS` (3 s) for the proctor's own decision — a silent host takes far longer to give up — then the
  deputy decides and the proctor counts as unreachable until it answers anything again. A submission whose answers
  have not all arrived is always the deputy's, so it queues behind them.
- **What the deputy knows.** Nothing of its own: the learner it decides for is derived from the views the client holds
  (the learner view and the run views), its events fold back into those views at once, and the command joins the
  outbox, which holds every kind of command in the order it was decided — a run is started before its answers arrive
  and submitted after them. A run known only from the learner view counts with its score; a sheet that is not the one
  the material deals for its seed marks a revised quiz, and its run is voided instead of scored.
- **Provisional.** The proctor decides every delivered command again with the same deciders, and its verdict is the
  one that counts. While the device is ahead, the proctor's views are behind and are not adopted; once nothing the
  deputy decided waits any more, the learner view and the views of the runs the proctor heard of are read again and
  replace what the device made up (the proctor's clock, the proctor's score).
- **A handle the proctor already knew.** The deputy knows no holder of any pseudonym or name. When the proctor says
  the handle is claimed, the device continues as its holder — the recall rule: whoever enters a claimed handle
  continues as that learner. Every command still waiting and every view held moves to the holder's id before the next
  command leaves, and the learner is told.
- **A run the proctor will not start** (an open run of the quiz on another device, a cap) never existed for it: it is
  voided on the device with the usual notice, and its answers are not sent.
- **Waiting, never giving up.** A spent sign-up allowance and a full roster hold the registration in the outbox until
  the proctor takes it; the learner goes on playing meanwhile.

The solutions travel in the site's scripts with this. The proctor still scores every run itself, so a leaderboard
never trusts what a device computed.

## Validation issues

`quizIssues(quiz)`, `catalogIssues(catalog, quizzes)`, `presenceIssues(state)` and `cursorIssues(state)` return
`{ path, code }[]`: a JSON pointer into the document and a kebab-case code, deduplicated and sorted by path, then
code, in code point order.

| Code | Meaning |
|---|---|
| `type-invalid` | wrong JSON type, or a non-finite number |
| `required` | a required member is missing (the pointer names it) |
| `property-unknown` | a member the schema does not declare |
| `value-invalid` | a `const` or enum mismatch (schema version, task kind, scale, badge rule kind) |
| `slug-invalid` | an id or key that is not a slug of 1…64 characters |
| `length-invalid` | a string outside its length bounds in code points (texts ≥ 1, units 1…32, quiz, badge and task icon emojis 1…16) |
| `integer-invalid` / `below-minimum` | `draw` not an integer ≥ 2 |
| `items-too-few` / `properties-too-few` | an array or map below its minimum size |
| `duplicate-path` | a quiz path listed twice in a catalog |
| `duplicate-id` | a repeated quiz, task, item, category, axis, dimension or badge id, or a repeated item in a draft sorting order |
| `category-unknown` | an item names a category its task does not declare |
| `axes-missing` | a profile on a task without axes |
| `profile-incomplete` / `axis-unknown` | a profile misses an axis / names an unknown one |
| `axis-range-invalid` | an axis whose `max` is not above its `min` |
| `profile-out-of-range` | a profile value outside its axis range |
| `value-missing` / `dimension-unknown` | a matching item misses a dimension / names an unknown one |
| `value-not-positive` | a value ≤ 0 on a logarithmic scale |
| `draw-exceeds-items` | `draw` larger than the item count |
| `quiz-count-mismatch` | the loaded quizzes do not match the catalog paths one to one |
| `quiz-unknown` | a badge rule names a quiz the catalog does not load |
| `badge-unreachable` | a `perfect-tasks` selector matches no task, so the badge is never awarded |
| `tag-invalid` | a public learner tag that is not 8 lowercase hex digits |
| `handle-invalid` | a shared identity whose handle is not exactly what `normalizeHandle` displays |
| `anchor-invalid` | an anchor outside `^[a-z0-9]+(?:[:-][a-z0-9]+)*$` or longer than 64 characters |
| `out-of-range` | a cursor coordinate that is not a finite number in 0…1 |
| `quiz-outside-run` | a place names a quiz on a screen other than the quiz page, a run or its results |
| `task-without-run` | a place names a task on a screen other than a run |
| `too-many` | a draft with more than `THINKING_LIMIT` (64) tasks, or an assignment map, order, guesses map, values map or dimension with more than 64 entries |
