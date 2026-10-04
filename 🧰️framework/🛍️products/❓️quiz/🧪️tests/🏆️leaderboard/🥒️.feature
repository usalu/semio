@capability-quiz-leaderboard
@oracle-quiz-python-reference
@comparison-quiz-score-v1
Feature: Catalogs and learner streams fold into the same read views and the same leaderboards
  The read side answers with schema views (`CatalogView`, `LearnerView`, `Leaderboard`). The catalog view
  is the catalog without quiz paths and badge rules, its quizzes reduced to their emoji, title,
  description and task ids, kinds and titles, in catalog order — no solution leaves the proctor. A learner's events fold into a learner
  view — runs newest first, badges in award order, the best submitted score per quiz and the total in
  points (the sum of the best scores × 100) — and into a transcript: the submitted runs in submission
  order and the badges, each with the quiz of the run that earned it.

  There are four leaderboards — daily, weekly, monthly and all-time — and each of them for every quiz
  or for one. A leaderboard counts the runs in its scope: those submitted inside the window of its
  period around the asked instant — the day, the ISO week from Monday or the month, in UTC, half-open
  and never starting before the epoch; every run for all-time — and of its quiz only when it names
  one. The learners with at least one run in scope are ranked by their standing over those runs:
  total descending, then badge count descending, then `reachedAt` ascending, then learner id
  ascending, ranked from 1. A row never carries the learner id — without passwords that id is an
  anonymous learner's only credential — but its `tag`, the FNV-1a 32-bit hash of the id as 8 lowercase
  hex digits, which a client recomputes from its own id to find its row. `reachedAt` is the submission
  in scope that last raised a best score, a quiz's first submission raising it; `runs` counts the runs
  in scope; `lastActivity` is the last of them; `badges` are those these runs earned. A leaderboard
  answers its period, its quiz and its window, at most the top 100 rows, `learners` — how many learners
  are ranked —, `submissions` — how many runs were submitted in all, whatever the scope — and, when the
  caller named in the query is ranked, the caller's own row as `own`, inside the top or beyond it. It
  is built from one transcript per ranked learner, which never leaves the proctor, so a proctor keeps
  every board incrementally: only a submission, a badge or a registration changes a transcript.

  Points (challenge design §3.6) replace the score × 100 of earlier: a run earns its score times the
  par of its challenge (100, 200, 300, 400), and every result carries its challenge and its points. A
  transcript run carries the quiz, the challenge, the score, the points and the instant. The best run of
  a quiz is the one with the most points — a later run replaces it only with strictly more, so of equal
  points the earliest stays —, `best` names it per quiz as its challenge, score and points (in the learner
  view and in every row), `total` sums those points and `reachedAt` is the submission that last raised a
  best. No board is split by challenge. The vector `points-across-challenges` climbs from easy to
  a better hard run and keeps it against an equal later one, keeps a medium best against an expert run of
  equal points, and ties two totals of 500 made of different challenges.

  The read views are this product's policy, so no third party can judge them. THE REFERENCE is `🐍️.py`
  beside this file, a second implementation written in Python from the contract, which reads every
  window off Python's `datetime` calendar — the third party the calendar arithmetic of the subjects is
  held to; the subjects answer with `catalogView` and `periodWindow`, and fold the same events with
  `evolveLearner` from `emptyLearnerState` before answering with `learnerView`, `transcript` and
  `leaderboard` (`@semio-tech/quiz`, and the snake_case twins of the `quiz` crate); the harness holds
  the two subjects to each other as well. Totals compare under `quiz-score-v1`; every committed score
  is dyadic, so totals and their ties are exact. The run view is held by the sibling case
  `🧾️learner-lifecycle`, whose runs carry real sheets and answers.

  The streams are real: the lifecycle reference decided every command that produced them. The first
  vector exercises the whole tie-break chain — two learners equal in total, badges and `reachedAt`
  split by learner id, one more with the same total and badges but a later `reachedAt`, one with the
  same total and fewer badges — plus an improving learner whose later, worse run does not move
  `reachedAt`, a learner scoring 0, and a learner who never submitted and is left out; it is asked for
  the all-time board, both boards of one quiz, the month, an empty later month, both weeks its runs
  fall into, a day three learners tie on and a day nobody submitted on. The calendar edges vector
  submits runs on the last millisecond of a Sunday and of a month and on the first of the Monday and
  of the month after, and asks the day, the week and the month on both sides of each. Every board is
  asked by several callers: nobody, ranked learners, a registered learner without a submission and an
  id nobody holds. The windows cover the epoch and its truncated first week, year ends, leap days —
  also those that end a 400-year era — and the February of a common century. The cuts take the first
  100, 101, 130 and 0 transcripts of a committed crowd whose totals tie in pairs and project the
  outline of each answer (`rank:tag` per row, the learner count and the caller's own `rank:tag`), so
  the cut at 100 rows, the count beyond it and an own row at rank 101 and 130 are pinned without
  committing every row again. The catalog views cover the leaderboard catalog and the lifecycle
  catalog, whose quizzes carry profiles, values and explanations that must all be stripped.

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

  @id-windows
  @level-fundamental
  @mode-differential
  Scenario: Every period spans the committed window around every committed instant
    Given the committed instants of shared://🏆️leaderboard/🔣️.json
    When the window of every period is taken around every instant
    Then every implementation projects the same day, ISO week and month in UTC, each containing its instant, and no window for all-time

  @id-rankings
  @level-fundamental
  @mode-differential
  Scenario: Every committed set of learner streams ranks into every committed leaderboard for every caller
    Given the committed vectors shared://🏆️leaderboard/🔣️.json
    When every vector's learners are folded into transcripts and every committed board is ranked for every committed caller
    Then every implementation projects the same period, quiz and window and the same rows in the same order with the same ranks, tags, totals, reachedAt, badges, run counts and last activity
    And the same learner count, the same count of submissions and the same own row for a ranked caller, none for anybody else
    And no row carries a learner id

  @id-cuts
  @level-fundamental
  @mode-differential
  Scenario: A leaderboard answers the top 100 rows, counts every ranked learner and finds the caller beyond the top
    Given the committed crowd of transcripts of shared://🏆️leaderboard/🔣️.json
    When the first transcripts of every committed cut are ranked for every committed caller
    Then every implementation projects the same outline: at most 100 rows in rank order, the learner count and the caller's own rank and tag
