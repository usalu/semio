@capability-quiz-sorting-score
@oracle-quiz-scipy-stats
@comparison-quiz-score-v1
Feature: A sorting answer scores its magnitude-weighted pair concordance
  A sorting task is scored over the learner's order `o` (design §6): for every pair `i < j` the weight
  is `|s(v[o_i]) − s(v[o_j])|`, with `s` the identity on a linear quantity and `log10` on a logarithmic
  one; the pair is discordant when `v[o_i] > v[o_j]`, and the score is `1 − discordant/total`, or 1 when
  no pair carries weight. Swapping near neighbours costs little, swapping a tea light and a nuclear
  plant costs a lot; a perfect order scores exactly 1 and a reversed one exactly 0. The item results
  follow the learner's order with the item's value, its position and its rank in the true ascending
  order of the sheet items (ties by definition index), and carry the item's explanation when it has one.

  THE REFERENCE is `🐍️.py` beside this file. Its pair loop is written from the design text; before a
  score is projected, numpy recomputes it over the whole pair matrix in an unrelated evaluation and
  summation order, and scipy's Kendall τ must reproduce the unit-weight reading of the same engine —
  with `w = 1` for every pair of distinct values the score is `(1 + τ_a)/2` over those pairs, `τ_a`
  derived from `scipy.stats.kendalltau`'s `τ_b` and the tie count. A second vector group holds the rank
  reading of the weighted engine itself: with distinct, equally spaced values on a linear scale the
  magnitude weights are rank differences and the score of every order is `(1 + ρ)/2`, with ρ from
  `scipy.stats.spearmanr` — so design §6's "`s = rank`" remark is checked in the form that holds (Spearman,
  not Kendall, for rank-difference weights). Scores compare under `quiz-score-v1`
  (1e-12, because `log10` may differ by one ulp between runtimes); everything else compares exactly.
  The subjects are `scoreTask` of `@semio-tech/quiz` and `score_task` of the `quiz` crate.

  The vectors cover a logarithmic task spanning eight orders of magnitude (perfect, reversed, a tie
  swapped — still perfect —, a neighbour swap, a small neighbour swap, the extreme swap, the untouched
  sheet order, all eight items), a linear task (perfect, reversed, every single swap and a rotation)
  and a task whose values are all equal, where no pair carries weight and every order scores 1.

  Design §13 (revisions after audit) adds how both cores degrade on inputs that bypass validation: an
  answer that §5 holds invalid or incomplete, or a sheet task the task cannot resolve, scores none (and a
  submission would be refused `run-incomplete`), an empty mean is 0, and — for classification — a profile
  that misses an axis of the task is not profiled. The `degraded` group of the vectors carries its own
  deliberately invalid tasks, sheet tasks and answers and is scored under `@mode-error`.

  The vectors shared://📏️sorting-concordance/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-scores
  @level-fundamental
  @mode-differential
  Scenario: Every committed sorting answer scores the committed task result
    Given the committed vectors shared://📏️sorting-concordance/🔣️.json
    When every vector's answer is scored against its task and sheet task
    Then every implementation projects the same score within 1e-12 and the same item results per vector
    And a perfect order scores exactly 1 and a reversed order exactly 0

  @id-rank-weights
  @level-fundamental
  @mode-differential
  Scenario: Equally spaced values score (1 + ρ)/2 with Spearman's ρ
    Given the committed rank vectors of shared://📏️sorting-concordance/🔣️.json
    When every order of the equally spaced floors — ascending, descending and shuffled subsets of 2, 3, 5 and 8 — is scored
    Then scipy's Spearman ρ of every order implies its score, and every implementation projects the same task result within 1e-12

  @id-degraded
  @level-fundamental
  @mode-error
  Scenario: Inputs that bypass validation degrade identically — scored or none, never a throw or NaN
    Given the committed degraded vectors of shared://📏️sorting-concordance/🔣️.json
    When every degraded vector is scored against its own task and sheet task
    Then every implementation projects the same result, or none for an order that is not a permutation of the sheet items, a wrong kind or a foreign sheet task, and exactly 1 for an empty sheet
