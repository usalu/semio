@capability-quiz-leaderboard
@oracle-quiz-python-reference
@comparison-quiz-score-v1
Feature: Catalogs and learner streams fold into the same read views and the same leaderboard
  The read side answers with schema views (`CatalogView`, `LearnerView`, `Leaderboard`). The catalog view
  is the catalog without quiz paths and badge rules, its quizzes reduced to their emoji, title,
  description and task ids, kinds and titles, in catalog order — no solution leaves the proctor. A learner's events fold into a learner
  view — runs newest first, badges in award order, the best submitted score per quiz and the total in
  points (the sum of the best scores × 100) — and the learners with at least one submitted run form the
  leaderboard: total descending, then badge count descending, then `reachedAt` ascending, then learner
  id ascending, ranked from 1. A row never carries the learner id — without passwords that id is an
  anonymous learner's only credential — but its `tag`, the FNV-1a 32-bit hash of the id as 8 lowercase
  hex digits, which a client recomputes from its own id to find its row. `reachedAt` is the submission
  that last raised a best score, a quiz's first submission raising it; `runs` counts submitted runs;
  `lastActivity` is the latest `at` of the stream.

  The read views are this product's policy, so no third party can judge them. THE REFERENCE is `🐍️.py`
  beside this file, a second implementation written in Python from the contract; the subjects answer
  with `catalogView`, and fold the same events with `evolveLearner` from `emptyLearnerState` before
  answering with `learnerView` and `leaderboard` (`@semio-tech/quiz`, and the snake_case twins of the
  `quiz` crate); the harness holds the two subjects to each other as well. Totals compare under
  `quiz-score-v1`; every committed score is dyadic, so totals and their ties are exact. The run view is
  held by the sibling case `🧾️learner-lifecycle`, whose runs carry real sheets and answers.

  The streams are real: the lifecycle reference decided every command that produced them. The first
  vector exercises the whole tie-break chain — two learners equal in total, badges and `reachedAt`
  split by learner id, one more with the same total and badges but a later `reachedAt`, one with the
  same total and fewer badges — plus an improving learner whose later, worse run does not move
  `reachedAt`, a learner scoring 0, and a learner who never submitted and is left out. The catalog views
  cover the leaderboard catalog and the lifecycle catalog, whose quizzes carry profiles, values and
  explanations that must all be stripped.

  The vectors shared://🏆️leaderboard/🔣️.json are generated, never hand-edited, from the Python reference
  in `../🧾️learner-lifecycle` and this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-catalog-views
  @level-fundamental
  @mode-differential
  Scenario: Every committed catalog and its quizzes answer with the committed solution-free catalog view
    Given the committed catalogs of shared://🏆️leaderboard/🔣️.json
    When every catalog is viewed with its quizzes in catalog order
    Then every implementation projects the same catalog view, carrying no quiz path, badge rule, category, value or explanation

  @id-learner-views
  @level-fundamental
  @mode-differential
  Scenario: Every committed learner stream folds into the committed learner view
    Given the committed vectors shared://🏆️leaderboard/🔣️.json
    When every learner's events are folded and viewed against the catalog
    Then every implementation projects the same runs, badges, best scores and total per learner

  @id-rankings
  @level-fundamental
  @mode-differential
  Scenario: Every committed set of learner streams ranks into the committed leaderboard
    Given the committed vectors shared://🏆️leaderboard/🔣️.json
    When every vector's learners are folded and ranked against the catalog
    Then every implementation projects the same rows in the same order with the same ranks, tags, totals, reachedAt, badges, run counts and last activity
    And no row carries a learner id
