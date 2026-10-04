@capability-quiz-matching-score
@oracle-quiz-scipy-stats
@comparison-quiz-score-v1
Feature: A matching answer scores its weighted concordance per dimension
  A matching task offers, per dimension, the shuffled multiset of the drawn items' true values as
  cards; the learner assigns every item one card per dimension (design §6). Per dimension, over the
  items in sheet order with `a_i` the assigned card value and `t_i` the true value, every pair `i < j`
  weighs `|s(t_i) − s(t_j)|`; it is discordant when `sign(t_i − t_j) · sign(a_i − a_j) < 0` and costs
  half its weight when the learner gave two differently valued items equal cards. A dimension scores
  `1 − discordant/total` (1 without weight); the task scores the mean over its dimensions in definition
  order. Exchanging two cards of equal value is never a mistake.

  THE REFERENCE is `🐍️.py` beside this file. Its pair loop is written from the design text; before a
  result is projected numpy recomputes every dimension over the full pair matrix (sign products for
  discordance, equality masks for learner-made ties), and scipy's Kendall τ must reproduce the
  unit-weight reading: with `w = 1` for every pair of distinct true values the dimension score is
  `(C + T/2)/(C + D + T)` — `(1 + τ_a)/2` over the pairs the truth orders — with `τ_a` derived from
  `scipy.stats.kendalltau`'s `τ_b` and both tie counts. Scores compare under `quiz-score-v1` (1e-12).
  The subjects are `scoreTask` of `@semio-tech/quiz` and `score_task` of the `quiz` crate.

  The vectors use the two-dimensional energy-carrier task (a logarithmic energy density and a linear
  CO₂ factor, with two items identical in both dimensions and two sharing a CO₂ factor) and a
  one-dimensional appliance task with two equal cards: perfect, equal cards exchanged, reversed card
  indices, a neighbour swap, the extreme swap, a learner-made tie, and an underrated item.

  Where the sheet task hides its cards (challenge design §3.4: no `cards` on its dimensions) the answer
  is the learner's `guesses` per dimension and item, and `a_i` is the guess. An item misses when it has no
  guess or its guess lies farther from its true value than the reach of the presented true values of the
  dimension — on a logarithmic scale the factor `min(1000, sqrt(hi / lo))`, met when the larger of guess
  and value over the smaller exceeds it widened by `1e-9` of itself; on a linear scale the distance
  `(hi − lo) / 2`, widened alike —; a
  pair costs its whole weight when either of its items misses, else as before; a dimension without weight
  scores 0 with a miss and 1 without. Every item result carries `miss`, and `assigned` is the guess, absent
  where the learner guessed none. A sheet task that carries `seconds` is timed: its answer may leave items
  out or be absent, and an item left out misses even where the cards show (its result then has no
  `assigned` and no `miss`). The `guessed` group covers exact and rough guesses, an item missing in both
  dimensions, equal guesses for different values, exchanged neighbours within the reach, guesses that all
  miss, a negative guess on a linear scale, guesses at the reach and just beyond it, an exact ×1000 and
  ÷1000 where the values do not spread, one double beyond it (within the slack) and clearly beyond it, a
  dimension without
  weight (in reach and missing), and timed answers: exact, partly guessed, without guesses, absent, with an
  unguessed item of equal value, and with cards one item unassigned or no answer. numpy recomputes the
  reach (`ptp`, `sqrt`, `minimum`), every miss and the score over the pair matrix before it is projected, and
  guesses without a miss must score what cards of the same values score — the reading scipy's Kendall τ
  judges.

  Design §13 (revisions after audit) adds how both cores degrade on inputs that bypass validation: an
  answer that §5 holds invalid or incomplete, or a sheet task the task cannot resolve, scores none (and a
  submission would be refused `run-incomplete`), an empty mean is 0, and — for classification — a profile
  that misses an axis of the task is not profiled. The `degraded` group of the vectors carries its own
  deliberately invalid tasks, sheet tasks and answers and is scored under `@mode-error`.

  The vectors shared://🔀️matching-concordance/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-scores
  @level-fundamental
  @mode-differential
  Scenario: Every committed matching answer scores the committed task result
    Given the committed vectors shared://🔀️matching-concordance/🔣️.json
    When every vector's answer is scored against its task and sheet task
    Then every implementation projects the same task score, dimension scores and item results per vector within 1e-12
    And exchanging equal cards keeps a perfect answer at exactly 1

  @id-guessed
  @level-fundamental
  @mode-differential
  Scenario: Guesses where the cards are hidden, and items left out on a timed sheet task, score the committed task result
    Given the committed guessed vectors of shared://🔀️matching-concordance/🔣️.json
    When every guessed or incomplete answer, or its absence on a timed sheet task, is scored against its task and sheet task
    Then every implementation projects the same task score, dimension scores, guesses and misses per vector within 1e-12
    And guesses at the true values or within the reach in the true order score exactly 1, and an absent answer exactly 0

  @id-degraded
  @level-fundamental
  @mode-error
  Scenario: Inputs that bypass validation degrade identically — scored or none, never a throw or NaN
    Given the committed degraded vectors of shared://🔀️matching-concordance/🔣️.json
    When every degraded vector is scored against its own task and sheet task
    Then every implementation projects the same result, or none for a card out of range or reused, a dimension left incomplete, missing or unknown, a wrong kind, guesses where the cards show or assignments where they are hidden, a guess of an unknown item or of zero on a logarithmic scale, and an untimed answer that leaves an item unguessed or is absent, and exactly 0 for a task without dimensions
