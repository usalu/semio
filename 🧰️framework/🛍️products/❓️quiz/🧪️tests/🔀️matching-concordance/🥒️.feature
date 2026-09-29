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

  @id-degraded
  @level-fundamental
  @mode-error
  Scenario: Inputs that bypass validation degrade identically — scored or none, never a throw or NaN
    Given the committed degraded vectors of shared://🔀️matching-concordance/🔣️.json
    When every degraded vector is scored against its own task and sheet task
    Then every implementation projects the same result, or none for a card out of range or reused, a dimension left incomplete, missing or unknown, or a wrong kind, and exactly 0 for a task without dimensions
