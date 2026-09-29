@capability-quiz-learner-lifecycle
@oracle-quiz-python-reference
@comparison-quiz-score-v1
Feature: Identification, runs, answers and submissions decide the same events and rejections everywhere
  The proctor wraps two pure deciders that both cores share (design §8). The roster decides
  identify-learner: an anonymous learner is always new; a pseudonym or name is normalised — trimmed,
  inner whitespace runs collapsed to one space, 1…64 characters, else `handle-invalid` — and its
  lowercased key recalls the learner that claimed it (`learner-recalled`) or registers a new one with
  the display handle; pseudonyms and names share one key space. The learner decides start-run,
  record-answer and submit-run against the current quizzes and their revisions: unknown learners and
  quizzes are rejected, an open run of the current revision blocks a new one while a stale one is
  voided first, answers are checked against the run's sheet (`unknown-task`, `answer-invalid`, the
  latest answer per task wins) and refused on a revised quiz (`quiz-revised`), and a submission is
  voided on a revised quiz, refused while incomplete (`run-incomplete`), and otherwise emits
  `run-submitted` with the scored result followed by one `badge-awarded` per newly earned badge.

  THE REFERENCE is `🐍️.py` beside this file: a second implementation of §3–§8 written in Python from
  the design text alone (sheet over numpy's MT19937, answer rules, scoring, badges, deciders, folding,
  views). Only decisions and schema views are projected — never a core's private state — so each
  subject replays the committed `given` events and steps through its own `decide`/`evolve`
  (`@semio-tech/quiz`: `normalizeHandle`, `decideRoster`/`evolveRoster`, `emptyLearnerState`,
  `decideLearner`/`evolveLearner`, `learnerView`, `runView`; the snake_case twins of the `quiz` crate).
  Every step carries its decision time `now`; a step may revise a quiz from then on, the way a
  redeployed quiz file changes its revision. Scores inside `run-submitted` compare under `quiz-score-v1`.

  Command idempotency by command id is a property of the proctor's framework deciders, not of these
  pure functions, so every committed command id is distinct.

  The vectors shared://🧾️learner-lifecycle/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-handles
  @level-fundamental
  @mode-differential
  Scenario: Handles normalise to a display form and a lowercase key, or are invalid
    Given the committed vectors shared://🧾️learner-lifecycle/🔣️.json
    When every committed handle is normalised
    Then every implementation projects the same display and key per handle, or none for an empty or over-long handle

  @id-roster
  @level-fundamental
  @mode-differential
  Scenario: identify-learner registers, recalls or rejects in one handle key space
    Given the committed vectors shared://🧾️learner-lifecycle/🔣️.json
    When every committed roster sequence is decided and folded from an empty roster
    Then every implementation projects the same decision per step

  @id-learner-decisions
  @level-fundamental
  @mode-differential
  Scenario: start-run, record-answer and submit-run decide the committed events and rejections
    Given the committed vectors shared://🧾️learner-lifecycle/🔣️.json
    When every committed learner sequence folds its given events and then decides and folds every step
    Then every implementation projects the same decision per step, scores within 1e-12

  @id-learner-views
  @level-quick
  @mode-differential
  Scenario: The folded learner answers with the committed learner view and run views
    Given the committed vectors shared://🧾️learner-lifecycle/🔣️.json
    When every committed sequence of a registered learner is replayed and viewed against the final quizzes
    Then every implementation projects the same learner view and the same view of every committed run
