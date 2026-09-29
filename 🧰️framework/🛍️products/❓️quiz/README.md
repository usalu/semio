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
| `🔨️modules/✅️validation/` | Structural and semantic validation of quizzes, catalogs and answers without any schema library. |
| `🔨️modules/📏️scoring/` | Partial-credit scoring of tasks and runs. |
| `🔨️modules/🏅️badges/` | Badge rules evaluated after every submission. |
| `🔨️modules/🧾️lifecycle/` | Handles, the roster and learner deciders: pure `decide`/`evolve`. |
| `🔨️modules/👁️views/` | Catalog view, learner view, run view and leaderboard. |
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

A quiz is an ordered set of tasks with an id, a title and a description. A run presents the tasks randomized
and is scored only as a whole. Every learner-visible text carries English and German; there is no default
language.

### Task kinds

- **Classification** — assign every item to one category. Categories may carry a profile: a value on every axis
  of the task (at least three axes, rendered as a spider diagram). Items name their correct category.
- **Sorting** — order the items ascending by their quantity. Items carry their true value.
- **Matching** — for every dimension (a quantity), assign each item one of the offered value cards; the cards
  are the multiset of the drawn items' true values.

A quantity has a label, a unit, a scale — `linear`, or `logarithmic` for quantities spanning orders of
magnitude — and whether display scales the unit with SI prefixes. A task may `draw` a random subset of its
items.

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

- Classification: a category id per item id. Sorting: the item ids, smallest first. Matching: per dimension a
  card index per item id.
- An answer is valid when its kind matches, it references only items, categories and dimensions of the sheet
  task, card indices are in range and used at most once per dimension, and a sorting is a permutation of the
  sheet items. Partial classification and matching answers are valid while the run is open.
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

| Command | Condition | Outcome |
|---|---|---|
| `start-run` | learner not registered | reject `unknown-learner` |
| `start-run` | quiz not loaded | reject `unknown-quiz` |
| `start-run` | the run id is taken | reject `run-open` (open) or `run-closed` |
| `start-run` | open run of the quiz at the current revision | reject `run-open` |
| `start-run` | open run of the quiz at a stale revision | `run-voided`, `run-started` |
| `start-run` | otherwise | `run-started { revision, seed = FNV-1a(run) }` |
| `record-answer` | run unknown / not open | reject `unknown-run` / `run-closed` |
| `record-answer` | stale revision | reject `quiz-revised` |
| `record-answer` | task not in the sheet / answer invalid | reject `unknown-task` / `answer-invalid` |
| `record-answer` | otherwise | `answer-recorded` — the latest answer per task wins |
| `submit-run` | run unknown / not open | reject `unknown-run` / `run-closed` |
| `submit-run` | stale revision | `run-voided` |
| `submit-run` | incomplete | reject `run-incomplete` |
| `submit-run` | otherwise | `run-submitted { result }`, then `badge-awarded` per newly earned badge |

Deciders are pure: `decideLearner(state, command, { now, catalog, quizzes })` returns `{ events }` or
`{ rejection }` and `evolveLearner(state, event)` folds one event; `at` is the decision time. Idempotency by
command id belongs to the framework server (`CommandEnvelope.idempotencyKey`).

### Identity

A learner is anonymous (always new), or identified by a pseudonym or name. Pseudonyms and names share one
handle key space and are recalled without a password: the roster decider registers an unclaimed handle and
recalls the learner holding a claimed one. The display handle is the trimmed handle with inner runs of Unicode
`White_Space` collapsed to one space; the key is its lowercase form; 1…64 code points, else `handle-invalid`.
Clients send handles in NFC. Learner, run and command ids are 32 lowercase hex characters (128 random bits).

### Leaderboard

Every learner with a submitted run, ordered by total descending, badge count descending, `reachedAt`
ascending and learner id ascending; the rank is the 1-based position. The total is the sum of the best scores
over the catalog quizzes in points (score × 100); `reachedAt` is the submission that last raised a best score;
`lastActivity` is the latest event of the learner.

The leaderboard is public, so its rows never carry the learner id: without passwords that id is the only thing
standing between an anonymous learner and anyone acting as them. A row carries the learner's `tag` instead —
`learnerTag(learner)`, FNV-1a of the id as 8 lowercase hex digits — and a client highlights its own row by
computing the tag from its own id. The id still breaks ties internally.

### State classes

| Class | Held where | Examples |
|---|---|---|
| Persisted shared | the proctor's event store and projections | learner, run, answer, submission and badge events |
| Persisted local-only | the browser | learner id, locale, theme, outbox, cached catalog and run views |
| Ephemeral shared | polled from the proctor | the leaderboard |
| Ephemeral local-only | the renderer | drag state, focus, the current step |

Answers apply locally at once and travel through an outbox coalesced per run and task, retried with jittered
backoff and applied once by command id, so short connection shortages never freeze a run.

## Validation issues

`quizIssues(quiz)` and `catalogIssues(catalog, quizzes)` return `{ path, code }[]`: a JSON pointer into the
document and a kebab-case code, deduplicated and sorted by path, then code, in code point order.

| Code | Meaning |
|---|---|
| `type-invalid` | wrong JSON type, or a non-finite number |
| `required` | a required member is missing (the pointer names it) |
| `property-unknown` | a member the schema does not declare |
| `value-invalid` | a `const` or enum mismatch (schema version, task kind, scale, badge rule kind) |
| `slug-invalid` | an id or key that is not a slug of 1…64 characters |
| `length-invalid` | a string outside its length bounds (code points) |
| `integer-invalid` / `below-minimum` | `draw` not an integer ≥ 2 |
| `items-too-few` / `properties-too-few` | an array or map below its minimum size |
| `duplicate-path` | a quiz path listed twice in a catalog |
| `duplicate-id` | a repeated quiz, task, item, category, axis, dimension or badge id |
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
