@capability-quiz-classification-credit
@oracle-quiz-scipy-stats
@comparison-quiz-score-v1
Feature: A classification answer earns full credit for hits and profile similarity for near misses
  A classification item earns credit 1 when the assigned category is the correct one (design §6).
  Otherwise, when both categories carry profiles on the task's axes (spider diagrams), it earns
  `max(0, 1 − d(assigned, correct)/d_max)`: `d` is the Euclidean distance over the axes in definition
  order of the values normalised to `(v − min)/(max − min)`, and `d_max` the largest distance between
  any two profiled categories of the task (`d_max = 0` earns 0). Any other miss earns 0. The task scores
  the mean credit over the sheet items, in sheet order, each result carrying the assigned and the
  correct category and the item's explanation when it has one.

  THE REFERENCE is `🐍️.py` beside this file. Its distance and credit loops are written from the design
  text; every distance they use is recomputed with `scipy.spatial.distance.euclidean`, and every
  `d_max` with `pdist`, and must agree within 1e-12 before a result is projected. Scores compare under
  `quiz-score-v1` (1e-12). The subjects are `scoreTask` of `@semio-tech/quiz` and `score_task` of the
  `quiz` crate.

  The vectors use the heating-systems task (three axes, four profiled categories and one unprofiled):
  all correct, a near profile miss, the farthest miss (exactly 0), a mid-range miss, an unprofiled
  category assigned, an unprofiled category missed, and everything wrong; a profile-free task where
  every miss earns 0; and a task whose two profiles are identical, so `d_max = 0` and a miss earns 0.
  The credit is always the task's: four of the heating vectors are repeated on the sheet task a challenge
  that hides the keys presents (challenge design §3.2: axes without numbers, categories without
  descriptions, profiles as shares of the axis ranges) and earn exactly what they earn where the keys show.

  A sheet task that carries `seconds` is timed (challenge design §3.4): its answer may leave items
  unassigned or be absent, and such an item earns 0 and carries no `assigned` category, the score staying
  the mean over every sheet item. The `timed` group covers a complete answer, one with a near miss, two
  items left out, an item left out beside a near miss, nothing assigned, no answer, and a profile-free
  task with one item left out and with no answer; every score is recomputed with `numpy.mean`.

  Design §13 (revisions after audit) adds how both cores degrade on inputs that bypass validation: an
  answer that §5 holds invalid or incomplete, or a sheet task the task cannot resolve, scores none (and a
  submission would be refused `run-incomplete`), an empty mean is 0, and — for classification — a profile
  that misses an axis of the task is not profiled. The `degraded` group of the vectors carries its own
  deliberately invalid tasks, sheet tasks and answers and is scored under `@mode-error`.

  The vectors shared://🕸️profile-similarity/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-credits
  @level-fundamental
  @mode-differential
  Scenario: Every committed classification answer earns the committed credits
    Given the committed vectors shared://🕸️profile-similarity/🔣️.json
    When every vector's answer is scored against its task and sheet task
    Then every implementation projects the same credits and the same mean score per vector within 1e-12
    And the farthest profile miss and every unprofiled miss earn exactly 0

  @id-timed
  @level-fundamental
  @mode-differential
  Scenario: Items left unassigned on a timed sheet task earn nothing
    Given the committed timed vectors of shared://🕸️profile-similarity/🔣️.json
    When every complete, partial or absent answer is scored against its task and timed sheet task
    Then every implementation projects the same credits, the same unassigned items and the same mean score per vector within 1e-12

  @id-degraded
  @level-fundamental
  @mode-error
  Scenario: Inputs that bypass validation degrade identically — scored or none, never a throw or NaN
    Given the committed degraded vectors of shared://🕸️profile-similarity/🔣️.json
    When every degraded vector is scored against its own task and sheet task
    Then every implementation projects the same result, or none for an unknown category or item (timed or not), an incomplete or absent untimed answer or a wrong kind, exactly 0 for a sheet without items, and partial credit computed over the complete profiles only
