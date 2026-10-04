@capability-quiz-challenge-rules
@oracle-quiz-python-reference
@comparison-quiz-score-v1
Feature: The four challenges, their points, the reach of a set of values, the clock and the hints are the same everywhere
  A run is played at one of four challenges (challenge design §1, §3.1), each the one before plus one
  step: `easy` shows the keys and gives hints, `medium` shows the keys, `hard` hides them and the learner
  guesses, `expert` hides them and runs every task against a clock. `CHALLENGE_RULES` holds the table
  (`keys`, `hints`, `timed`, `par` 100, 200, 300, 400); `challengeRank` orders the challenges 0…3 and
  `challengeMeets(challenge, least)` holds when the challenge is at least as demanding; a run earns
  `points(score, challenge) = score × par`.

  A value misses when it lies farther from the truth than the reach of its set. On a logarithmic scale
  `reach(values, scale)` is a factor, `hi > lo ? min(1000, sqrt(hi / lo)) : 1000`, and
  `misses(value, truth, scale, reach)` holds when `max(value, truth) / min(value, truth) > reach × (1 +
  REACH_SLACK)`; on a linear scale the reach is a distance, `hi > lo ? (hi − lo) / 2 : unbounded`, and a
  value misses when `|value − truth| > reach × (1 + REACH_SLACK)`. `REACH_SLACK = 1e-9` lets a decimal
  typed at exactly the reach count as within it. Only `/`, `*`, `+`, `sqrt`, `−`, `abs`, min, max and
  comparisons enter, all exactly rounded: a ratio or distance of exactly the widened reach does not miss,
  and the next double beyond it does. An unbounded reach is projected as `null`. A timed task allows `taskSeconds(kind, items,
  dimensions) = 30 + per(kind) × items` seconds (8 per classification item, 12 per sorting item, 12 per
  matching item and dimension); `acted(at, floor, now) = max(min(at, now + CLOCK_LEAD), floor)` lowers the
  instant a learner acted to `CLOCK_LEAD` = 300000 ms past the decider's clock and raises it to its floor, so
  a claim up to five minutes ahead is kept and one further ahead buys no time.

  `hintsOf(task, sheetTask, answer)` tells what an easy run asks about a recorded answer (challenge design
  §8): every hint questions one concrete relation the learner's own answer claims, never a bare direction or
  a count, and never states the truth. A sorting item whose key on the ladder misses its value, or a matching
  item whose card misses it (reach over the presented true values of the dimension), is compared with a
  reference among the other items holding a key (in that dimension): the anchors — whose own keys do not
  miss — where there are any, else all of them, and none at all for a lone item. With the keys `k` and the
  values `v`, on a logarithmic scale the claim is `ρ = k_X / k_R`, the truth `τ = v_X / v_R` and the error
  `max(ρ, τ) / min(ρ, τ)`; on a linear one `δ = k_X − k_R`, `Δ = v_X − v_R` and `|δ − Δ|`. The reference is
  the pool member with the largest error; members whose error times `1 + REACH_SLACK` reaches the largest are
  tied (exact anchors give the same error up to rounding), and among them (challenge design §8.4a, §8.4b) an
  item no earlier kept compare hint of the task names as its reference — in the same dimension for a matching —
  wins first, then an item the task flags `familiar`, then the smallest oriented claim — `max(ρ, 1/ρ)` or `|δ|`
  —, then the first in sheet order; neither wins outside the tie window, so a reference is named again when
  the window holds no other. References are chosen after the cap, in the order the kept hints are given, so a
  hint the cap drops uses none up.
  `{kind: "compare", item, other, dimension?, factor | difference, verdict}` carries the claim unrounded
  (`factor` may be below 1). With the pivot `p` = 1 (logarithmic) or 0 (linear), the `verdict` is `reversed`
  when the claim lies off the pivot and the truth at it or on its other side (`ρ > 1 and τ ≤ 1 or ρ < 1 and
  τ ≥ 1`, likewise around 0), so a claim of exactly 1 or 0 — equal keys — is never reversed and a truth of
  exactly 1 or 0 — equal values — against a claim off it always is; else `under` when the truth lies beyond the
  claim, further from the pivot (`ρ ≥ 1 and τ > ρ or ρ < 1 and τ < ρ`, likewise with `δ ≥ 0`), else `over`. A
  misplaced classification item whose assigned category `P` and own category `Q` both carry profiles gets
  `{kind: "profile", item, category: P, axis, other?, above?}` when some axis gap `|P_a − Q_a|` exceeds that
  axis's reach — half the spread of the presented categories' values on it, widened by `REACH_SLACK` — naming
  the axis with the largest gap relative to its reach (first in axis order on ties; axes without spread never
  count), and nothing for a near miss. On that axis `other` names an item placed in its own category whose
  value `r` lies strictly between `P_a` and `Q_a` — the one farthest from `Q_a`, the first in sheet order on
  ties — and `above` = `P_a > r`, the side the placement claims; without such an item both are absent and the
  client shows the value. Any other misplaced item gets `{kind: "group", item, other, together: true}` with
  the first item in sheet order the learner put in the same category though its own differs, else
  `together: false` with the first item of its own category the learner put elsewhere, else `{kind:
  "category", item, category}` naming the assigned category. Order: sorting in the learner's order, matching
  per dimension then item in sheet order, classification in sheet order. A task gives at most
  `HINTS_PER_TASK` = 3 hints: those with the largest weight — the largest error of a compare hint's pool, the gap
  relative to the reach of a profile hint's axis — are kept first, then group and category hints, the earlier
  on ties, and the kept hints stay in that order. No hint without an answer or where the sheet task hides the
  numbers of a sorting or matching. Whether a quantity is `additive`, and the `short` labels, change only the
  client's wording.

  THE REFERENCE is `🐍️.py` beside this file: a second implementation of the challenge design §3.1 and §8.3
  written in Python from the design text alone. numpy recomputes every number by an unrelated route before it is
  projected — the extremes with `numpy.ptp` and `numpy.amin`, the reach with `numpy.sqrt` and
  `numpy.minimum`, the misses by vectorised ratios or distances, the lowered and raised instants with `numpy.minimum` and `numpy.maximum`, the seconds with `numpy.prod`, the ladder of keys
  with `numpy.sort`, the hints as arrays over the sheet order (every candidate's claimed and true ratio or
  difference at once, the verdict by `numpy.sign` around the pivot, the pool, the tie window, the references
  named so far (`numpy.isin`) and the familiar items as boolean masks, the reference by `numpy.argmin` of the
  oriented claims, whose first minimum is the
  first in sheet order, the profile reaches by `numpy.ptp` per axis, the profile anchor by `numpy.argmax`
  over masked distances, the partners by `numpy.flatnonzero`, the cap by the stable `numpy.lexsort`) —
  every logarithmic reach below the cap is held to the exactly rounded square root by
  `fractions.Fraction`, and the points to the exact rational product, rounded once. The subjects are
  `CHALLENGE_RULES`, `challengeRank`, `challengeMeets`, `points`, `reach`, `misses`, `taskSeconds`, `acted`
  and `hintsOf` of `@semio-tech/quiz` and their snake_case twins of the `quiz` crate; reaches compare under
  `quiz-score-v1`, everything else exactly. The boundary vectors sit on exact non-power-of-ten factors
  (`18000` for `18`), one double beyond them (within the slack), square-root reaches met exactly, typed
  decimals such as `0.018` for `18`, whose double lies one rounding beyond the factor 1000 and still counts
  as within, values clearly beyond, and the slack boundary itself: exactly `reach × (1 + 1e-9)` and the
  next double above it, on both scales and from both sides. Two vectors hold the Sun's power `3.828e+26` W of the
  physics quiz as the truth, with values one double beside the widened cap: they hold only when every implementation
  reads its documents exactly rounded (a reader that takes the digits times a table power of ten reads the Sun one
  double low and calls the miss within reach).

  The vectors shared://⛰️challenge-rules/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-rules
  @level-fundamental
  @mode-differential
  Scenario: The rule table, the order of the challenges and the points of every score at every challenge
    Given the committed vectors shared://⛰️challenge-rules/🔣️.json
    When the rules, the rank and the least-challenge relation of every challenge and the points of every committed score are asked
    Then every implementation projects the same table, ranks and relation, and the same points per score and challenge

  @id-reach
  @level-fundamental
  @mode-differential
  Scenario: The reach of every committed set of values, and whether a value misses at and beside every bound
    Given the committed vectors shared://⛰️challenge-rules/🔣️.json
    When the reach of every committed set is computed on its scale, and every committed value is held to its truth within it
    Then every implementation projects the same reach — none where it is unbounded — within 1e-12, and the same miss per value
    And a value exactly at the reach never misses

  @id-clock
  @level-fundamental
  @mode-differential
  Scenario: The seconds of every timed task, and every instant lowered to the lead and raised to its floor
    Given the committed vectors shared://⛰️challenge-rules/🔣️.json
    When the seconds of every committed kind and size and the instant of every committed claim, floor and decider's clock are computed
    Then every implementation projects the same seconds and the same instants, an instant after its floor and within the lead kept as it is

  @id-hints
  @level-fundamental
  @mode-differential
  Scenario: The compare hints of sortings and matchings on every branch
    Given the committed vectors shared://⛰️challenge-rules/🔣️.json
    When the hints of every committed sorting and matching answer in the group hints are computed
    Then every implementation projects the same hints in the same order per vector, with the same factor or difference bit for bit
    And an anchor is preferred to a missing item with a larger error, and with no anchor every other keyed item is a candidate
    And exact anchors and errors a millionth apart in 4000 tie to the smallest claim, equal claims to the first in sheet order, while errors ten millionths apart do not tie
    And inside the tie window a familiar anchor wins over a smaller claim and over an equal claim earlier in sheet order, the smallest claim among familiar ones, and a familiar anchor outside the window never
    And inside the tie window a reference no earlier hint of the task names wins over a smaller claim, an equal claim and a familiar one, per dimension of a matching, a reference is named again where the window holds no other, and a hint the cap drops names none
    And the truth beyond the claim on the same side of 1 or 0 is under, short of it over, and on the other side or at the pivot reversed, on both scales and on both sides
    And equal keys claim a factor of exactly 1 or a difference of exactly 0, which is never reversed, and equal values against unequal keys are always reversed
    And no task gives more than three hints: the largest errors are kept, ties to the earlier, across the dimensions of a matching, in the order they are given
    And a sheet task that hides its numbers, an absent answer, an answer without a miss and a lone assigned item hint nothing

  @id-classification-hints
  @level-fundamental
  @mode-differential
  Scenario: The profile, group and category hints of classifications
    Given the committed vectors shared://⛰️challenge-rules/🔣️.json
    When the hints of every committed classification answer in the group classificationHints are computed
    Then every implementation projects the same hints in the same order per vector
    And a profile hint names the axis of the largest gap relative to its reach, the first in axis order on ties and never an axis without spread
    And on that axis it names the item placed in its own category that lies between the assigned and the true value farthest from the true one, the first in sheet order on ties, with the side the placement claims, and no item where none lies between or none is placed in its own category
    And a gap exactly at the reach, a near miss and profiles without spread hint nothing
    And no task gives more than three hints: profile hints of the largest relative gap are kept before group and category hints, in sheet order
    And without a profile the partner together comes first, then the partner apart past unassigned and alike-placed items, then the assigned category

  @id-quiz-hints
  @level-fundamental
  @mode-differential
  Scenario: The hints of the authored quizzes on sheets they deal
    Given the committed vectors shared://⛰️challenge-rules/🔣️.json
    When the hints of every committed answer in the group quizHints to a task of the physics, heating, demand and cooling quizzes are computed
    Then every implementation projects the same hints in the same order per vector
    And the Sun on the tea light's key is compared with the tea light one rung up by a factor of 0.35, reversed, and the resting person on the top key with the familiar kettle
    And the passive house under profile B is questioned on its heating demand against the WSchVO 1995 house, which the placement claims it lies above
    And the sheets carry the short labels of the authored quizzes, and no task gives more than three hints
