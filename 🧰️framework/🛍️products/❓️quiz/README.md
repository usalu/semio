# ❓️ Quiz

The semio quiz product: a declarative, render-independent quiz model — catalogs, quizzes and their tasks,
seeded solution-free sheets, the four challenges of a run, partial-credit scoring, badges, the learner run
lifecycle and the leaderboard — together with a React renderer and proctor client. The model knows nothing about the DOM
or the network; the proctor (`🎓️teaching/🛂️proctor`) wraps its pure deciders in the framework server, and
the renderer is one target among possible others. Two cores implement the model bit for bit: TypeScript
(`@semio-tech/quiz`) and Rust (`semio-framework-quiz`), side by side in every module.

## Layout

| Path | What lives there |
|---|---|
| `🧬️schema/🔣️.json` | The normative contract (JSON Schema draft-07): every document, command, event, query and view. |
| `🧬️schema/🟦️.ts`, `🧬️schema/🦀️.rs` | Hand-written type twins, one type per `$defs` entry under the same name. |
| `🔨️modules/🎲️randomness/` | FNV-1a run seeds, MT19937, the rejection-sampled uniform index and the Fisher–Yates shuffle. |
| `🔨️modules/⛰️challenge/` | The rules of the four challenges: what a challenge shows, hints and times, its par points, reach and miss, the seconds of a task, the instant a learner acted, lowered to the clock lead and raised to its floor, and the hints of an answer. |
| `🔨️modules/🃏️sheet/` | `sheetOf(quiz, seed, challenge)`: the randomized, solution-free sheet of a run at its challenge. |
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
catalog view. A run keeps the first task first, presents the others in random order, is played at one challenge and is
scored only as a whole. Every learner-visible text carries English and German; there is no default
language.

### Task kinds

- **Classification** — assign every item to one category. Categories may carry a profile: a value on every axis
  of the task (at least three axes, rendered as a spider diagram). Items name their correct category. Where the
  challenge hides the keys, the categories show no description and the diagrams no numbers.
- **Sorting** — order the items ascending by their quantity. Items carry their true value. Where the challenge
  shows the keys, every place of the order shows its key — the true values of the presented items as an ascending
  ladder — and the learner moves the items onto them. Where it hides them, the learner types a numeric guess per
  item (`2 kW`, `1,5 MWh`): the field shows the unit and reads an optional SI prefix, the items order themselves
  by their guesses, and the guesses are the answer.
- **Matching** — for every dimension (a quantity), assign each item one of the offered value cards; the cards
  are the multiset of the drawn items' true values. Where the challenge hides the keys there are no cards: the
  learner types a guess per item and dimension.

What a challenge shows, hints and times is one table for every quiz: see [Challenge levels](#challenge-levels).

A quantity has a label, a unit, a scale — `linear`, or `logarithmic` for quantities spanning orders of
magnitude — whether display scales the unit with SI prefixes, and whether amounts of it add up (`additive`:
powers and energies do, U-values, loads per m² and air change rates do not), which hints speak of ("1,000 × “R”
together"). A task may `draw` a random subset of its items.

A quantity, an axis, a category and an item may carry a `short` label (at most 40 code points per language) that
hints use in place of the label; the sheet carries it wherever it carries the label. A sorting or matching item may
be `familiar` — an everyday thing a learner can picture, such as a tea light or a kettle — which a hint prefers
as its reference; the sheet never says which item is familiar.

A task, an item, a category and a matching dimension may each carry an `icon`: one emoji grapheme (1…16 code
points) that pictures it and one of the looping microanimations `bounce`, `pulse`, `spin`, `sway`, `float` or
`flip`. Icons are part of the sheet (the task's also of the catalog view). The task's icon replaces the icon
of its kind in its card's title chip and stands before its title in the run's task list; an item's and a
category's stand before their labels wherever they show — the draggable chips and bins of a classification,
the rows of a sorting and a matching, the result tables and the crowd figures; a dimension's stands before its
heading and on each of its draggable value cards. An icon must never give the answer away: it pictures the
thing, not its value or category. The glyph is hidden from assistive technology; neighbours in a list start
their loops apart so they never move in step. Whether the icons play is the learner's choice ("Animate icons"
in the preferences, kept on the device and marked as chosen); until they choose, the device decides — still on
one that asks for reduced motion, where the preferences say so, moving on every other.

### Sheet

The sheet is the randomized, solution-free presentation of a quiz for one run, a pure function of the quiz,
the run seed and the run's challenge — `sheetOf(quiz, seed, challenge)`, Rust `sheet_of` — bit-exact across both
cores:

1. `seed = FNV-1a-32(UTF-8 bytes of the run id)`; the generator is MT19937 seeded with `init_genrand(seed)`.
2. `uniform(n)`: `n = 1 → 0` without a draw; else draw until `x < 2³² − (2³² mod n)` and return `x mod n`.
3. `shuffle`: Fisher–Yates from the end, `j = uniform(i + 1)` for `i = len − 1 … 1`, on a copy.
4. The task order is shuffled first, and the first task of the quiz is then put back in front: it stays first.
   Then every task in definition order shuffles its items (and keeps the first `draw`), a classification
   shuffles its categories, a sorting already in ascending order (by value, ties by definition index) is rotated
   left by one, and a matching shuffles the cards of every dimension.
5. The generator is consumed alike at every challenge — the same draws in the same order, the card shuffle also
   where no card shows — so one seed deals the same items in the same order at every challenge.

Sheet items carry only `{ id, label }` and their icon. The sheet names its `challenge`, and after the draws the
challenge decides what else it carries:

| Sheet task | The challenge shows the keys | The challenge hides the keys |
|---|---|---|
| Sorting | `keys`: the true values of the presented items, ascending | no `keys` |
| Matching | `cards` per dimension | no `cards` |
| Classification | every category's `description`; axes with `unit`, `min` and `max`; profile values as authored | no `description`; axes as `{ id, label }`; every profile value as its share of the axis range, `(v − min) / (max − min)` |

On a timed challenge every sheet task also carries `seconds`, the time it allows once it is opened. Whatever it
carries, a sheet never says which item a number belongs to; where the keys are hidden it carries none of them: no
sorting `keys`, no `cards`, no descriptions, no axis numbers, and profiles only as shares of their axis range.

### Answer

- Classification: a category id per item id. Sorting: the item ids, smallest first (`order`). Matching: per
  dimension a card index per item id (`assignments`).
- **Guesses.** Where the sheet task hides the keys, the answer is the learner's guesses: a sorting carries
  `guesses`, a number per item id, beside its `order`; a matching carries `guesses`, per dimension a number per
  item id, in place of `assignments`. A guess is in the quantity's base unit, never SI-prefixed. An answer
  carries guesses only there (a partial answer may lack some or all of them): `guesses` on a sorting with `keys` or on a matching with `cards`, and
  `assignments` on a matching without `cards`, are invalid.
- An answer is valid (`answerRejection(sheetTask, answer)`, Rust `answer_rejection`) when its kind matches, it
  references only items, categories and dimensions of the sheet task, card indices are in range and used at
  most once per dimension, a sorting is a permutation of the sheet items, every guess is finite (positive on a
  logarithmic scale), and the guessed items of a sorting stand in non-decreasing guess order within the order.
  Partial classification and matching answers and a sorting with guesses for some of its items are valid while
  the run is open.
- Complete (`answerComplete(sheetTask, answer)`, Rust `answer_complete`): every item classified; every item
  matched, or guessed, in every dimension; a sorting with `keys` once it is recorded, a sorting without them
  when every item has a guess.

### Scoring

Every task scores in [0, 1]; a run scores the mean of its task scores in sheet order. `s(v)` is `v` on a linear
scale and `log₁₀ v` on a logarithmic one (values must then be positive). The score means accuracy at every
challenge; beside it a run earns **points**, `points(score, challenge) = score × par` of its challenge, and
`scoreRun(quiz, sheet, answers)` answers `RunResult { quiz, challenge, score, points, tasks }` with the challenge
of the sheet.

**Sorting — magnitude-weighted pair concordance.** Over the learner's order `o`, for every pair `i < j`:

```
w = |s(v[oᵢ]) − s(v[oⱼ])|        total += w        if v[oᵢ] > v[oⱼ]: discordant += w
score = total > 0 ? 1 − discordant / total : 1
```

Where the sheet task hides the keys, the guesses are judged with the order. With `r = reach(presented true
values, scale)`, an item **misses** when it has no guess or `misses(guess, value, scale, r)`, and a pair is
discordant also when either of its items misses:

```
if v[oᵢ] > v[oⱼ] or miss(oᵢ) or miss(oⱼ): discordant += w
score = total > 0 ? 1 − discordant / total : (any miss ? 0 : 1)
```

**Matching** — per dimension, items in sheet order, `aᵢ` the assigned card value and `tᵢ` the true value:
`w = |s(tᵢ) − s(tⱼ)|`; the pair is discordant (`w`) when `sign(tᵢ − tⱼ) · sign(aᵢ − aⱼ) < 0`, and half
discordant (`w / 2`) when `aᵢ = aⱼ` on distinct true values. The task score is the mean over the dimensions.
Where the sheet task hides the keys, `aᵢ` is the guess and `r` the reach of the presented true values of the
dimension: a pair with an item that misses is discordant (`w`) whatever its order, every other pair counts as
with cards, and a dimension without spread (`total = 0`) scores 0 when an item misses, else 1.

**Classification — profile similarity.** A correct item earns 1. A wrong item earns
`max(0, 1 − d(assigned, correct) / d_max)` when both categories carry profiles, else 0; `d` is the Euclidean
distance of the profiles normalised per axis to `(v − min) / (max − min)`, and `d_max` the largest such distance
over all profiled category pairs (`d_max = 0 → 0`).

**Partial answers.** On a timed sheet a task may have no answer or an incomplete one, and what is missing scores
as a miss: an item without a guess misses, a sorting without an answer takes its items in sheet order without
guesses, an unassigned classification item earns 0. On every other sheet a run is scored only when every answer
is complete.

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

**Why a miss costs every pair it touches.** Typing `1, 2, 3` in the right order must not earn a perfect score,
and exactness must not be required either. A guess beyond the reach of its set — three orders of magnitude off on
a wide ladder — does not know the item, wherever it happens to stand, so every pair it takes part in counts as
wrong. Perfect stays reachable: the right order with every guess within reach.

Scoring never throws and never yields NaN on inputs that bypass validation: `scoreTask`/`scoreRun` return
`undefined` (Rust: `None`) for an invalid answer, an incomplete one on a sheet that is not timed or an unresolvable
task, a mean over nothing is 0, a
category whose profile misses an axis counts as unprofiled, and a matching item missing a dimension value is
presented as a NaN card and leaves its task unscored; a complete run that cannot be scored is rejected
`run-incomplete`.

Results list classification items in sheet order (`assigned`, `correct`, `credit`), sorting items in the
learner's order (`value`, `position`, `rank` in the true ascending order), and matching items per dimension in
sheet order (`assigned` and `correct` value), each with the item's explanation. Where the sheet task hid the keys,
a sorting item carries the learner's `guess` (where there is one), a matching item's `assigned` is the guess, and
every item of both says whether it is a `miss`. An item the learner left unanswered has no `assigned`. Scores
agree across the cores within 1e-12 (`log₁₀` may differ by one ulp); a perfect score is exact.

### Badges

Badges are evaluated after every submission, in catalog order, over all submitted results of the learner
including the new one; badges already held are skipped, and a new award records the triggering run and time.

- `perfect-quiz { quiz, challenge? }` — some result of the quiz scored 1.
- `perfect-tasks { taskKind?, quiz?, challenge? }` — every catalog task matching the selector scored 1 in some
  result; a selector matching no task never awards.
- `completed-quizzes` — every catalog quiz has a submitted result.

`challenge` is the least challenge that counts: a rule that names one sees only the results of runs whose
challenge meets it (`challengeMeets`), a rule without one sees every result. So the hints of `easy` do not hand
out a badge that asks for more.

### Challenge levels

A learner chooses the **challenge** of a run when it starts: `easy`, `medium`, `hard` or `expert` (`CHALLENGES`).
The challenge is a fact of the run, fixed for it, and lives in no quiz and no catalog except as the least
challenge of a badge rule — one table serves every quiz, and the revision of a quiz does not depend on it. Each
challenge is the one before plus one step (`CHALLENGE_RULES`, `challengeRules(challenge)`):

| Challenge | Keys | Hints | Timed | Par points |
|---|---|---|---|---|
| `easy` | shown | yes | no | 100 |
| `medium` | shown | no | no | 200 |
| `hard` | hidden | no | no | 300 |
| `expert` | hidden | no | yes | 400 |

`challengeRank(challenge)` is the position in this order, 0…3, and `challengeMeets(challenge, least)` holds when
the rank of the first is not below the rank of the second.

- **Keys** — the numbers a task turns on. A sorting's keys are the true values of the presented items as an
  ascending ladder: place *i* of the order shows the *i*-th smallest. A matching's keys are its value cards. A
  classification's keys are the descriptions of its categories and the numbers on the axes of its spider
  diagrams. Shown, the learner only has to assign them. Hidden, a sorting and a matching ask for a typed numeric
  guess per item (and dimension), and the guesses are the answer; a classification has no number to guess, so its
  descriptions go and its diagrams show shape only. A classification whose categories have neither descriptions nor
  axes therefore has nothing to hide, and on hard it would be medium for more points; the core allows it, so every
  shipped catalog holds each of its classifications to at least one of the two (the site's catalog test does).
- **Reach and miss** — a value **misses** when it lies farther from the truth than the **reach** of its set.
  `reach(values, scale)` over the presented true values, `lo` the smallest and `hi` the largest, is a factor on a
  logarithmic scale, `hi > lo ? min(REACH_FACTOR, sqrt(hi / lo)) : REACH_FACTOR` with `REACH_FACTOR` = 1000, and a
  distance on a linear one, `hi > lo ? (hi − lo) / 2 : ∞` (where nothing then misses).
  `misses(value, truth, scale, reach)` is `max(value, truth) / min(value, truth) > reach × (1 + REACH_SLACK)` on a
  logarithmic scale and `|value − truth| > reach × (1 + REACH_SLACK)` on a linear one, with `REACH_SLACK` = 1e-9: a
  decimal typed at exactly the reach lies a hair beyond it in binary (`0.018` is just below 18 / 1000) and still
  counts as within, while anything a learner can tell apart from the reach is judged as it is. Only `/`, `*`, `+`,
  `sqrt`, `−`, `abs`, min, max and comparisons enter, and every language rounds them exactly, so an exact ×1000
  never misses and both cores agree bit for bit — at the widened reach itself too. Why a factor of 1000 or the
  square root of the set's ratio: being off by more than a factor of 1000 is far off on a ladder that spans many
  decades, but among values that span three decades or fewer no key can be that far off, and up to six decades a
  fixed factor would be too lenient; half of what the set spans in decades (the square root of the ratio) carries
  the same idea to every set. One concept serves the
  hints and the scoring of guesses.
- **Hints** — at a challenge that hints, while the run is open, `hintsOf(task, sheetTask, answer)` questions what
  is far off in the recorded answer of a task. Every hint questions one concrete relation the learner's own answer
  claims between two items (or an item and a category), never a bare direction or a count, and never states the
  truth; at most one per item (and dimension) and at most `HINTS_PER_TASK` = 3 per task. It is a pure function, so
  the proctor and the deputy say the same and no event records a hint; `RunView.hints` carries the hints per task,
  only for tasks that have one. A hint names things by their `short` label where they have one.
  - Sorting (in the learner's order) and matching (per dimension, then item, in sheet order): for an item `X`
    whose key — the ladder key at its place, or its assigned card — misses its value,
    `{ kind: "compare", item, other, dimension?, factor | difference, verdict }`. `other` is, among the other items
    holding a key, preferably those whose own key does not miss (the learner's anchors), the one whose claimed
    relation to `X` is the most wrong — the ratio of claimed to true ratio on a logarithmic scale, the distance of
    claimed and true difference on a linear one. Errors within a relative 1e-9 of the largest tie; among them an
    item no earlier hint of the task (of the same dimension) names wins, then a `familiar` item (an everyday thing
    a learner can picture), then the smallest claim, then the first in sheet order — so three hints rarely lean on
    one reference. References are chosen after the cap, in the order the hints are given. `factor` = key(X) / key(other) or `difference` = key(X) − key(other) is what the learner claims.
    `verdict` (`verdictOf`) compares it with the truth around 1 (around 0): `reversed` when the claim lies on one
    side and the truth at or beyond the other — the pair is the wrong way round, and the question asks for the
    order without a number ("Are you sure “X” is higher in heating demand than “R”?"); otherwise `under` when the
    truth lies beyond the claim ("only …") and `over` when it does not ("really …"). An additive quantity reads "Are
    you sure 1,000 × “R” together only add up to the power of 1 × “X”?". Every question names its quantity (German
    "in puncto Heizwärmebedarf"); a count from 10¹⁵ reads "1.1 × 10¹⁶".
  - Classification, per item in a category not its own, in sheet order: where both categories carry profiles,
    `{ kind: "profile", item, category, axis, other?, above? }` when on some axis the two profiles lie farther
    apart than the axis's reach (half the spread of the presented categories on it), naming the axis with the
    largest gap relative to its reach — a near miss gives none. Where an item placed in its own category has a
    value on that axis strictly between the assigned and the own category's, `other` names the one farthest from
    the own value (the first in sheet order on ties) and `above` the side the placement claims ("Are you sure “X”
    is higher in heating demand than “R”?"); without one the client shows the assigned category's value. Otherwise
    `{ kind: "group", item, other, together: true }` with an item the learner put beside it whose own category
    differs, else `together: false` with an item of its own category the learner put elsewhere, else
    `{ kind: "category", item, category }`. None says which of the two is wrong.
  - The cap keeps the compare and profile hints with the largest error first — a compare hint's largest error, a profile
    hint's gap relative to the reach, the earlier on ties — then group and category hints in order; the kept hints
    stay in the order above.
- **The clock** — on a timed challenge every task has its own clock. It allows
  `seconds = taskSeconds(kind, items, dimensions)`: `TASK_SECONDS.base` = 30 plus, per presented item, 8 in a
  classification and 12 in a sorting, and 12 per item and dimension in a matching. The clock starts when the
  learner opens the task (`open-task`, once: a repeat is `already-opened`), and the device keeps the time:
  `start-run`, `open-task` and `record-answer` carry `at`, the instant the learner acted. The run starts at the
  `at` of its `start-run` — a run started offline keeps the device's start however late it is delivered, so a
  task opened on the device gains no time after the sync — and a decider counts every `at` as
  `acted(at, floor, now) = max(min(at, now + CLOCK_LEAD), floor)`: lowered to `CLOCK_LEAD` = 300 000 ms (five
  minutes) past its own clock `now`, then raised to its floor, which is 0 for a start, the start of the run for an
  opening and the opening of the task for an answer (the start of the run where the run has no clock). A device
  decides at its own clock, so its claims never meet the lead; the proctor lowers a claim only when the device's
  clock runs more than five minutes ahead of its own. Within that lead the decider's clock never enters a time
  verdict, so the proctor is never stricter than the device, whatever the delay of delivery and however far the
  device's clock lies behind; an opening or a start dated an hour ahead is lowered and buys no time. An answer
  whose instant lies more than `seconds` after the opening is refused `time-up`: the task takes no more answers.
  Nothing happens at the deadline by itself — no decider is scheduled, the rule applies when a command arrives —
  and a timed run may be submitted with tasks unanswered or partly answered; what is missing scores as a miss.
  Why the device keeps the time: the deputy decides while the proctor is away and the proctor decides again at
  delivery, so by the proctor's clock a connection shortage would turn a timely answer into a late one. A device
  with a deputy holds every solution anyway; its clock is trusted no further than the device already is, and the
  lead bounds how far ahead it may claim to be.
- **Points** — `points(score, challenge) = score × par`. The score stays in [0, 1] and means accuracy at every
  challenge — badges, score bins and percentages rest on it — and the points are what a harder challenge earns
  more of: a perfect run earns 100 on `easy` and 400 on `expert`. The best run of a quiz is the submitted run
  with the most points, and a learner's total is the sum of the points of the best runs.
- **One open run** — a quiz has at most one open run per learner, whatever its challenge. `start-run` at the
  challenge of the open run is refused `run-open`, and the client resumes that run; at another challenge it voids
  the open run and starts the new one, so a learner leaves a challenge that proves too hard without submitting
  it first.

Rust names them `CHALLENGE_RULES`, `challenge_rules`, `challenge_rank`, `challenge_meets`, `points`, `REACH_FACTOR`,
`REACH_SLACK`, `reach`, `misses`, `TASK_SECONDS`, `task_seconds`, `CLOCK_LEAD`, `acted`, `HINTS_PER_TASK`, `verdict_of`
and `hints_of`.

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
| learner | `start-run` | open run of the quiz at the current revision and at the command's challenge | reject `run-open` |
| learner | `start-run` | `limits.runs` runs started in total, or `limits.runsPerQuiz` of the quiz — open, submitted and voided alike | reject `runs-exhausted` |
| learner | `start-run` | open run of the quiz at a stale revision or at another challenge | `run-voided { at = now }`, `run-started` |
| learner | `start-run` | otherwise | `run-started { challenge, revision, seed = FNV-1a(run), at = command.at }` |
| learner | `open-task` | run unknown / not open | reject `unknown-run` / `run-closed` |
| learner | `open-task` | stale revision | reject `quiz-revised` |
| learner | `open-task` | the challenge of the run is not timed | reject `run-untimed` |
| learner | `open-task` | task not in the sheet | reject `unknown-task` |
| learner | `open-task` | the task is already opened | reject `already-opened` (no event, so no receipt is stored) |
| learner | `open-task` | otherwise | `task-opened { at = acted(command.at, run start, now) }` |
| learner | `record-answer` | run unknown / not open | reject `unknown-run` / `run-closed` |
| learner | `record-answer` | stale revision | reject `quiz-revised` |
| learner | `record-answer` | the run already recorded `limits.answersPerRun` answers | reject `answers-exhausted` |
| learner | `record-answer` | task not in the sheet | reject `unknown-task` |
| learner | `record-answer` | timed run, the task is not opened | reject `task-unopened` |
| learner | `record-answer` | timed run, `t = acted(command.at, opening, now)` lies more than the task's `seconds` after the opening | reject `time-up` |
| learner | `record-answer` | answer invalid | reject `answer-invalid` |
| learner | `record-answer` | otherwise | `answer-recorded { at = t }`, on a run without a clock `t = acted(command.at, run start, now)` — the latest answer per task wins |
| learner | `submit-run` | run unknown / not open | reject `unknown-run` / `run-closed` |
| learner | `submit-run` | stale revision | `run-voided` |
| learner | `submit-run` | incomplete, on a run without a clock | reject `run-incomplete` |
| learner | `submit-run` | otherwise | `run-submitted { result }`, then `badge-awarded` per newly earned badge |

Deciders are pure: `decideHandle(state, command, now)` and `decideLearner(state, command, { now, catalog, quizzes,
limits })` return `{ events }` or `{ rejection }`; `evolveHandle(state, event)` and `evolveLearner(state, event)`
fold one event; `at` is the decision time, except on `run-started`, where it is the instant the learner started the
run as `acted(at, 0, now)` lowers it, and on `task-opened` and `answer-recorded`, where it is the instant the learner acted as `acted` raises it. A run's state holds its `challenge` and, per opened task, the instant of
the opening (`RunState.opened`, empty on a run without a clock); every sheet of a run is dealt at the run's
challenge. A timed run is submitted as it stands: tasks unanswered or partly answered do not hold it back.
Idempotency by command id belongs to the framework server (`CommandEnvelope.idempotencyKey`). Before a
registration is decided, `registrationRejection(learners, limits)` is `roster-full` once a proctor holds
`limits.learners` registrations.

A `Timestamp` is an integer from 0 to `MAX_TIMESTAMP` = 2^53 − 1, the largest integer every language reads exactly.
A `challenge` that is none of the four and an `at` that is no non-negative integer make a command malformed: the
Rust core cannot decode it (the proctor answers `command-malformed` before any decision), and in TypeScript the
types keep such a command from being built or restored. An `at` beyond `MAX_TIMESTAMP` decodes in Rust, and both
cores refuse it in `commandRejection` as they refuse a malformed id (`id-invalid`), so no view ever carries an
instant a TypeScript reader cannot hold; should any malformed command reach the TypeScript core, `commandRejection`
refuses it as `id-invalid` too, so the deputy never accepts what the proctor refuses. No rejection code of its own names
it.

**Caps** (`Limits`, `DEFAULT_LIMITS`) keep every state bounded: `learners` 100 000 registrations, `runs` 1 000
runs started per learner, `runsPerQuiz` 200 of them in one quiz — open, submitted and voided runs count alike, so
switching the challenge back and forth cannot grow a learner without bound —, `answersPerRun` 2 000 recorded answers per run;
a quiz has at most one open run per learner, whatever its challenge, and a task is opened at most once per run. A
proctor configures the two totals.

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
order (`TranscriptRun { quiz, challenge, score, points, at }`) and the badges, each with the quiz of the run that
earned it, beside the learner id.
A transcript changes only when a run is submitted, a badge is awarded or the identity changes — never when an
answer is recorded. The **standing** of a transcript in a scope (`standing(transcript, catalog, scope)`: the
unranked row beside the learner id) is made of the runs in scope only: `best` names per catalog quiz the best of
them (`Best { challenge, score, points }`: the run with the most points, the earliest of equals), the total is
the sum of the points of those bests — the points of the best run of the one quiz on a board of one quiz —,
`reachedAt` is the one of them that last raised a best, `runs` counts them, `lastActivity` is the last of
them and `badges` are those they earned; a learner with no run in scope has no standing there. There is no
leaderboard per challenge: runs at every challenge count toward the one total, and a row's `best` says at which
challenge each best was earned. The learner view carries the same `best` and `total` over all of a learner's
submitted runs, and each of its run summaries the `challenge` and, once submitted, the `points` of the run.
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
  publisher's own answer into its draft (card index → card value, a guess as the number it is; `undefined` for an
  answer that does not fit the sheet task). `thinkingProblem(state)` (all issues: `thinkingIssues`) admits a state whose drafts are structurally
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
  A quiz has one crowd, the runs of every challenge mixed, so its figures stay comparable. An assigned card counts
  under its value; a guessed matching value counts under the nearest value among all authored items of the task for
  that dimension (distance in `s`; a tie takes the smaller value), so guesses fall into the columns the cards make.
  A sorting counts its places from `position` whether its keys showed or not, and a sorting whose keys were hidden
  and in which no item was guessed (a timed run left it unanswered) adds its score bin only.
- **Crowd distributions** — the crowd view carries whole distributions, so every figure is drawn from data the proctor
  already aggregated:
  - `scores` are `CROWD_SCORE_BINS` = 10 counts, the bins `[0, 10)`, `[10, 20)`, … `[80, 90)`, `[90, 100]` of the
    whole percent a client prints: `scoreBin(score) = min(9, ⌊percent / 10⌋)` with `percent = ⌊score · 100 + ½⌋`.
    The bins take the score — accuracy — at every challenge, never the points.
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
  without a connection. A learner's own drafts are published either way. During a run whose challenge hides the keys
  the others are not offered at all — no figure and no button to ask, whatever the preference — because the crowd
  would show the hidden keys; the same holds on the quiz's page while such a run of the quiz is open; on the results,
  and on the quiz's page otherwise, they show as after any run.
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
  demonstrator do on phones. A card of the list is as wide as the screen leaves it, up to a width that reads.

### Adaptive layout

Every part of the client lays itself out for the room it actually has — the body of its card, the interaction of its
task, the list of its rows — never for the window, because the same card stands in a phone's column, a pane of the
overview, a dialog and a desktop page (`🎯️targets/⚛️react/🎨️.css`, container queries). Rooms are measured in rem of
the root text size, so a learner's larger text gets the layout of the narrower room it leaves. The tiers are shared
vectors (`🧫️fixtures/📐️adaptive-layout`), held against the stylesheet by `lightningcss` and walked in a real browser
at a phone's, a tablet's and two desktops' widths (`🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/📐️adaptive-layout`).

- **One line where it fits.** A classified item puts its category select right of its label, a sorted item its guess
  and move buttons, a matched item its value select and remove button; where the room ends, the control moves below
  its label. The selects of one list share one width, so they line up.
- **Panes on wide tasks.** From 84 rem the pool of a classification stands beside its bins, from 64 rem the value cards
  of a matching beside its rows — both stay in view while the rows scroll, so nothing is dragged across the page. Runs
  and results take up to 100 rem of a large screen; running text keeps a line length that reads.
- **Tables that fold.** Below the width its columns need (`Records`, `--quiz-fold` in rem) a table — the results of a
  task, the leaderboard, the learner's runs, a diagram's values — is a list of records: the name and its lead values
  on the first line, the facts side by side each under the heading of its column, the explanation below. The
  leaderboard's sort buttons become chips above the records. Every part keeps its table role. A figure of what
  everyone answered folds below the width its items and columns need: each item over its columns, which then name
  themselves; a matching's many value columns show only those somebody chose. Nothing scrolls sideways.
- **The run's head.** A list of the tasks, title left and state right, in a narrow card; chips in a row from 40 rem;
  chips left and the progress right from 64 rem.
- **Forms.** The preferences put their names in a column left of the controls from 28 rem (above them, as wide as the
  card, below); the ways to appear stand side by side from 44 rem.
- **Touch.** On a coarse pointer the controls of a task are at least 2.5 rem tall and their grips and icon buttons as
  wide, so a finger hits them.
- **Reading order.** The source order of every row is its reading and focus order in every layout; a dragged row's
  ghost is laid out in an empty copy of its list, so it keeps the shape of its source.

### The leaderboards on screen

The leaderboard page and the card at the centre of the overview show one of the leaderboards at a time
(`🎯️targets/⚛️react/🔨️modules/🏆️leaderboard`), and the learner chooses which:

- **Period.** A segmented choice — Today, This week, This month, All time — on the page and on the card. A leaderboard
  of a period says what it counts: its window, in the learner's own time zone.
- **Category.** A second segmented choice on the page: every quiz, or one quiz of the catalog. The leaderboard of one
  quiz shows its points in place of the total and the bests of every quiz; a best shows as its points with the
  challenge they were earned at. The card names the chosen category.
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

### The challenge on screen

The web client lets the learner choose the challenge of a run, plays every task as the challenge asks and says
wherever a run shows at which challenge it was played.

- **Choosing.** The page of a quiz shows a group of four radio buttons, "Challenge", each with one line that says
  what it does and the most points it earns. The choice is remembered on the device per quiz
  (`preferences.challenges[quiz]`, read by `challengeOf(preferences, quiz)` and set by `withChallenge`; `medium`
  until the learner chooses for that quiz; it is not part of the preferences page), so an expert run chosen on one
  quiz never makes another quiz's card start on expert. The quiz's card on the overview starts a run at the
  challenge remembered for its quiz and names it in its action (`QuizSession.startRun(quiz, challenge, signal)`). While a run of the quiz is open, the page and the card offer to continue it and name its challenge;
  choosing another challenge on the page asks before the open run is discarded.
- **Keys shown.** A sorting shows its key at every place; the items are moved by the buttons or by dragging, and
  there is no guess field. A matching offers its cards.
- **Keys hidden.** A sorting has a guess field per item and the items order themselves by their guesses; nothing
  is moved by hand. A matching has no cards and one guess field per item and dimension. A classification shows no
  category descriptions, and its diagrams no axis numbers.
- **Hints.** A hint shows beside its item as one question in words and a "?" symbol (never an arrow that gives
  the direction away), never by colour alone, and is announced politely once when it appears. With a deputy the run view, and with it
  the hints, comes from the deputy after every answer; without one the session reads the run again once an answer
  of a run that hints is delivered.
- **The clock.** Until it is opened, a timed task shows only its title, the time it allows and "Start the clock"
  (`QuizSession.openTask(run, task, signal)`). Then the remaining time shows as text (`m:ss`, tabular figures, not
  a live region) beside a separate polite status that speaks at the start, at 30 s, at 10 s and when time is up.
  When time is up the task turns read-only and says so; focus is not moved. The countdown runs from the device's
  own opening instant on the session's clock seam (`now`): the deadline is an instant, so a hidden tab changes
  nothing. Submitting a timed run names, in its confirmation, the tasks that are still open or unanswered.
- **Showing.** A run names its challenge. The results say the score ("87 %") and, below it, challenge and points
  ("Hard · Points: 261 of 300"), show "Your guess" with a mark on every miss where the keys were hidden, and "Not answered" where time
  ran out. Where the keys were hidden, each sorting and each matching dimension says above its table how far a guess
  may lie off and still count — "A guess counts within ×N of the true value" on a logarithmic scale, "within ±d" on
  a linear one, the core's `reach` over the presented true values cut down to two significant digits — and every
  guess says how far it lay ("×5.3 from the true value", or the difference), rounded up where it misses and down
  where it does not, so the two never contradict. Play never shows the tolerance on hard and expert, since it would
  tell the spread of the hidden values; easy questions the learner's own relations instead. The quiz's card and page show the best run as its points with its challenge, the learner's table of
  runs the challenge and the points of every run, and the leaderboard every best as points with its challenge.

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
lives in the renderer in two halves: what the client needs before any pet came — the choice, the switch, the topic
marks, the loading — in the first script (`🎯️targets/⚛️react/🔨️modules/🐾️pets`), and what matters only once pets
came — the scene, the names on stage, the layer as the quiz sets it up, "Play with the pets" — in a chunk of its own
(`🔨️modules/🐾️pets/🎪️stage`, `QuizPetLayer`, `PetsPlayground`) that imports no value of the pets product and so
shares no module with its chunks. Their strings stay in the quiz's bundles, which hold every key the client uses.

- **Choice** — the preference `pets` is `off`, `still`, `calm` (unless the learner chose otherwise) or `lively`; it is
  persisted local-only. The menagerie, the layer and the second half of the glue are lazy chunks fetched together
  and only when pets are wanted: `off` — or a site without pets — downloads and renders nothing. A device that forces its own colours gets no pets
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
  `onCast`; `petNames`, `usePetCast`). A run is a time of concentration (`quiet`): while one is on screen the pets
  start nothing by themselves — no walks, no encounters, no whims, the cast does not rotate — but they answer the
  learner's own clicks and can be picked up, as far as the learner allows it, and they play with the page only after
  the learner has been idle for a long while. The glue only says what the learner allows; the pets' stage decides what
  of it fits a run. Results are not quiet: a learner who reads a score is done concentrating, and the switch is one
  Tab away.
- **Play and mischief** — two more preferences, both on until the learner says no and stored with the others:
  `petsPlay` ("Pets react to clicks and can be picked up") and `petsMischief` ("Pets may play with the page"). Only an
  explicit `false` in storage turns one off; neither is a choice of liveliness, so neither sets `petsChosen`. The layer
  gets them as `play` and `mischief` while the pets are calm or lively and `false` for both while they are still —
  also when a device that asks for reduced motion holds them still by default; the two checkboxes are then off,
  disabled and described by "Only calm and lively pets play.", as they are while the pets are off.
- **The hand** — the layer takes a press on a pet only where nothing of the quiz acts on it: besides every link,
  button, form field, label and ARIA widget, the glue names the drag grips (`[data-quiz-grip]`, an `aria-hidden`
  span), the drop zones (`[data-quiz-drop]`) and the cards of the overview (`[data-layered-card]`, which open their
  page on a click anywhere but on a control) as `QUIZ_PET_CONTROLS`. A pet standing beside the title tab of an
  overview card is therefore not picked up there; the settings play with it instead.
- **Topics** — pets play with the page only on lifted copies of what the quiz marks with a topic key among the
  grounds of the menagerie, `data-pet-prop="<quiz>"`, `"<quiz>/<task>"` or `"<quiz>/<task>/<item>"` (`petProp`): the
  tasks of an opened quiz's page (`<quiz>/<task>`), the items of a run that are no table rows — classification chips,
  sorting rows, the rows of a matching's items, never its cards (`<quiz>/<task>/<item>`, from the run's quiz and the
  task around them, `PetTopic`, `usePetTopic`) — and the true order of a sorting's results (never a table row). The
  key says what an element is about, never its value or whether an answer is right. The pets survey these with
  `QUIZ_PET_PROPS` (`[data-pet-prop]` outside the copy a drag carries, which clones every attribute of its row). The
  cards of the overview carry their quiz as `data-pet-topic` (`QuizCard`'s `topic`): a card is clicked, hovered,
  lifted and measured and is never lifted by a pet, so it is matched for its topic only and never surveyed as a
  thing to play with.
- **Play with the pets** — under the pets' row the settings name every pet on stage, each in a group of its own with
  four buttons — Hello, Trick, Pet, Toss (Hallo, Kunststück, Streicheln, Hochwerfen) — that hand the layer a deed
  through its handle (`ref`, `PetLayerHandle.play`; `PetsPlay`, `PET_DEEDS`): the keyboard and single-pointer way to
  everything the hand does (WCAG 2.1.1, 2.5.1, 2.5.7). A polite status line says what the learner just asked for
  ("Sunny says hello.", `PET_DEED_SAID`), never what the pets do by themselves. The group is there only while the pets
  are calm or lively, play is allowed and somebody is on stage.
- **Tempo** — `data-pets-tempo` on the document root (`QUIZ_PETS_TEMPO`, read once per mounted layer by `petsTempo()`)
  makes the pets' time pass up to eight times as fast. It is a test seam — nothing in the client sets it — for the
  end-to-end proofs that wait for a walk or for two pets that meet, which takes a minute or two of real time.
- **Stage** — ephemeral local-only: every device simulates its own pets. They stand on what the cards of the screen
  show — the title tab, the edge of the body beside it — and on the footer line (`QUIZ_PET_SURFACES`), never in front
  of a card, keep clear of controls, text, drag items, drop zones and the navigation bar (`QUIZ_PET_KEEPOUTS`), turn
  see-through while the pointer rests on them, and glance at the cursors of other learners while those show. Where a
  screen leaves no room — a task that fills a phone — no pet shows. The layer is decoration: hidden from assistive
  technology, never in the way of the keyboard or of a press on anything of the quiz that acts on one; a menagerie that
  cannot be fetched or a layer that fails leaves the quiz as it is.

### State classes

| Class | Held where | Examples |
|---|---|---|
| Persisted shared | the proctor's event store and projections | learner, run (with its challenge), task opening, answer, submission and badge events; the crowd view |
| Persisted local-only | the browser | learner id, locale, theme, when the others' answers show, the challenge last chosen, the outbox of commands the proctor has not decided yet, cached catalog, learner and run views |
| Ephemeral shared | polled from the proctor / relayed by presence sockets | the leaderboards looked at; presence, cursors and drafts (thinking) |
| Ephemeral local-only | the renderer | drag state, focus, the current step, the leaderboard chosen and the order of its table, the ask to see the others before submitting |

Answers apply locally at once and travel through an outbox coalesced per run and task, retried with jittered
backoff and applied once by command id, so short connection shortages never freeze a run. The opening of a timed
task keeps its place in the outbox before the answers of its task, and every start, opening and answer carries the
instant it was given (a stored command without a valid instant is not restored), so the clock of a task does not
depend on when its commands arrive.

### While the proctor is away

A site may hand the web client its material — the catalog and its quizzes, solutions included
(`mountQuiz(root, { material })`). The client then has a **deputy** (`🎯️targets/⚛️react/🔨️modules/🫡️deputy`): the
core's own deciders on the device.

| | Without material | With material |
|---|---|---|
| Catalog | shows once the proctor answered (cached afterwards) | shows at once; the proctor's replaces it when it answers |
| The proctor answers | the proctor decides every command | the same: the proctor decides, nothing is provisional |
| The proctor does not answer | answers and task openings wait in the outbox; registering, starting and submitting wait for the proctor | the deputy decides: a learner registers, starts runs at any challenge, opens tasks, gets hints, submits runs and earns badges on the device |
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
  and submitted after them. The challenge of a run is the one its sheet names (or its summary in the learner view),
  and the tasks it opened are those of `RunView.opened`. A run known only from the learner view counts with its score
  and points; a sheet that is not the one the material deals for its seed and challenge marks a revised quiz, and its
  run is voided instead of scored.
- **The challenge and the time.** The deputy decides with the challenge as the proctor does: it deals the sheet of the
  run's challenge, computes the hints, voids the open run when a run starts at another challenge and scores guesses
  and points. Time is the device's at both deciders: starts, openings and answers carry the instant the learner
  acted, a run starts at the device's start at both, and the proctor, deciding them again at delivery, raises them
  to the same floors and consults its own clock only for the five-minute lead, which the claims of a device whose
  clock runs less than five minutes ahead never meet — an answer given in time on the device is in time at the
  proctor however late it arrives, and a task opened on the device gains no time once a run started offline is
  delivered. An opening the proctor already holds (`already-opened`) counts as done on the device. An answer is
  never the deputy's to decide: the session records it, refuses it with `time-up` on the device when the device's
  clock says the task is over, and the proctor decides it at delivery.
- **Provisional.** The proctor decides every delivered command again with the same deciders, and its verdict is the
  one that counts. While the device is ahead, the proctor's views are behind and are not adopted; once nothing the
  deputy decided waits any more, the learner view and the views of the runs the proctor heard of are read again and
  replace what the device made up (the proctor's clock, the proctor's score).
- **A handle the proctor already knew.** The deputy knows no holder of any pseudonym or name. When the proctor says
  the handle is claimed, the device continues as its holder — the recall rule: whoever enters a claimed handle
  continues as that learner. Every command still waiting and every view held moves to the holder's id before the next
  command leaves, and the learner is told.
- **A run the proctor will not start** (an open run of the quiz at the same challenge on another device, a cap)
  never existed for it: it is voided on the device with the usual notice, and its answers are not sent.
- **Waiting, never giving up.** A spent sign-up allowance and a full roster hold the registration in the outbox until
  the proctor takes it; the learner goes on playing meanwhile.
- **A proctor of another contract.** Client and proctor agree on the contract by its wire version
  (`$defs/WireVersion`, `WIRE_VERSION` in both cores): every quiz envelope carries it, the proctor declares it for every
  quiz kind at `GET /instance` and refuses an envelope of another version. Before its first call — and after any
  refusal, since a refusal may be the first sign of a proctor that was replaced — the client reads `GET /instance`; a
  proctor that does not declare every quiz kind the client sends at the client's version is *incompatible* and is sent
  nothing, asked again only after `AGREEMENT_RECHECK_MS` (30 s). It is as away as a silent proctor (reachability
  `incompatible`, a transient failure): whatever waits in the outbox keeps waiting, never refused and dropped, and the
  deputy decides meanwhile. An *older* or *foreign* proctor reads like an unreachable one; a *newer* one means the page
  is out of date, and the client says so with a reload. Views are held to their shape before they are adopted
  (a catalog, learner or run view of another contract is an answer that cannot be read, like a network's sign-in page
  in place of the proctor's JSON): a shortage, retried, never a crash. The wire version is raised with every change of
  the contract beyond its prose; the case `🧪️tests/🤝️wire-version` pins the fingerprint of the contract to it, so a
  change without a raise fails.
- **A screen that fails.** Every screen renders inside a boundary: a screen that throws shows what happened, with a
  way to show it again and a reload, and the rest of the client goes on — everything the learner did lives in the
  session, not in the screen.

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
| `value-invalid` | a `const` or enum mismatch (schema version, task kind, scale, badge rule kind, the challenge of a badge rule) |
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
