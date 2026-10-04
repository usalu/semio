@capability-quiz-schema-conformance
@oracle-quiz-jsonschema
@comparison-ordered-json-v1
Feature: Every quiz document the product meets conforms to the normative schema
  `🧬️schema/🔣️.json` (JSON Schema draft-07) is the normative contract; the cores validate quizzes and
  catalogs with an owned structural and semantic validator (`quizIssues`, `catalogIssues`) and link no
  schema library at runtime. This case holds that validator to the schema itself.

  THE REFERENCE is python-jsonschema's Draft 7 validator reading the normative file (`🐍️.py` beside
  this file). It judges three populations: every schema-typed document inside the committed vectors
  of this owner, listed in the table below (a row names a fixture, a JSON pointer whose `*` matches
  every array element or object member, a `$defs` entry and, for a catalog, the pointer of its quizzes);
  every `❓️quiz/🔣️.json` authored under `🎓️teaching`, judged by the definition its `schema` field names;
  and the committed rejected documents, each of which must break the schema at the one rule its vector
  names. Only quizzes and catalogs are projected — as accepted or rejected — because only they have an
  owned validator on the subject side: `quizIssues`/`catalogIssues` of `@semio-tech/quiz` and
  `quiz_issues`/`catalog_issues` of the `quiz` crate accept a document exactly when they report no issue.
  Every other typed document of the table (sheets, answers, results, commands, events, views) is still
  validated by the oracle and fails the oracle phase when it breaks the contract; the Rust subjects of
  the sibling cases additionally decode every input document (quizzes, tasks, sheet tasks, answers,
  results, badges, catalogs, commands, events) into the typed twins, which refuse unknown members, and
  project the twins' own serialization. The randomness vectors carry no schema-typed document, and the
  `degraded` groups of the scoring vectors deliberately break the contract (design §13) and are left out.

  The challenge levels (challenge design §2) add and change definitions that no fixture document covers
  from both sides, so the committed vectors also carry `instances`: per new or changed definition —
  `Challenge`, `Quantity` (with its required `additive` and its optional `short`), `ShortText`, `Verdict`,
  the items, categories and axes that carry a `short` label (and the sorting and matching items their
  `familiar` flag), `SheetItem`, the hints of challenge design §8 and §8.4a (`CompareHint` with its `verdict`,
  `ProfileHint` with its anchor `other` and `above`, `GroupHint`, `CategoryHint` and their union `Hint`),
  `Best`, `SheetAxis`, the sheet tasks and dimension, `Sheet`, the answers, the item results, `RunResult`,
  the commands `start-run`, `open-task` and `record-answer`, `Rejection`, the events `run-started` and
  `task-opened`, `RunView`, `RunSummary`, `LearnerView`, `LeaderboardRow` and `BadgeRule` — conforming
  instances and instances that break the schema at the one rule they name (a quantity without `additive`, a
  short label of 41 code points, a sheet item that carries `familiar`, a compare hint with both a factor and
  a difference or neither, a compare hint with the removed `under`, a profile hint with an anchor but no side,
  a removed `magnitude` or `misplaced` hint, an axis unit without its range, a best of negative points, a
  command without its challenge or instant, an empty hint list, a badge rule at an unknown challenge …).
  jsonschema judges every one of them in the oracle phase of the rejected documents; they have no owned
  validator, so they are not projected, and the Rust subject decodes every conforming one into its typed
  twin. Badge rules with a least challenge also reach the owned catalog validator: a catalog with a least
  challenge on every kind of rule that may carry one is accepted everywhere, and catalogs naming an unknown
  challenge, a challenge that is no text or a challenge on `completed-quizzes` are rejected everywhere. The
  rejected quiz with `"difficulty": "hard"` stays rejected: a quiz carries no challenge. A quiz whose quantity
  lacks `additive`, or whose matching dimension states it as text, is rejected everywhere too, and so is one
  whose item carries a short label beyond 40 code points or a `familiar` flag that is no boolean, or whose
  quantity has a short label without German; a quiz with short labels of exactly 40 code points and familiar
  items is accepted everywhere.

  The vectors shared://🧬️schema-conformance/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-fixture-documents
  @level-fundamental
  @mode-differential
  Scenario: Every schema-typed document of the committed vectors conforms, and the owned validator accepts every quiz and catalog
    Given the committed vectors and the schema documents they carry
      | id                          | fixture                                   | pointer                                     | definition  | quizzes  |
      | sheet-quizzes               | shared://🃏️sheet-assembly/🔣️.json       | /quizzes/*                                  | Quiz        | -        |
      | sheet-sheets                | shared://🃏️sheet-assembly/🔣️.json       | /sheets/*/sheet                             | Sheet       | -        |
      | sorting-tasks               | shared://📏️sorting-concordance/🔣️.json  | /tasks/*                                    | Task        | -        |
      | sorting-sheet-tasks         | shared://📏️sorting-concordance/🔣️.json  | /vectors/*/sheetTask                        | SheetTask   | -        |
      | sorting-answers             | shared://📏️sorting-concordance/🔣️.json  | /vectors/*/answer                           | Answer      | -        |
      | sorting-results             | shared://📏️sorting-concordance/🔣️.json  | /vectors/*/expected                         | TaskResult  | -        |
      | sorting-rank-sheet-tasks    | shared://📏️sorting-concordance/🔣️.json  | /rankVectors/*/sheetTask                    | SheetTask   | -        |
      | sorting-rank-answers        | shared://📏️sorting-concordance/🔣️.json  | /rankVectors/*/answer                       | Answer      | -        |
      | sorting-rank-results        | shared://📏️sorting-concordance/🔣️.json  | /rankVectors/*/expected                     | TaskResult  | -        |
      | sorting-guessed-sheet-tasks | shared://📏️sorting-concordance/🔣️.json  | /guessed/*/sheetTask                        | SheetTask   | -        |
      | sorting-guessed-answers     | shared://📏️sorting-concordance/🔣️.json  | /guessed/*/answer                           | Answer      | -        |
      | sorting-guessed-results     | shared://📏️sorting-concordance/🔣️.json  | /guessed/*/expected                         | TaskResult  | -        |
      | matching-tasks              | shared://🔀️matching-concordance/🔣️.json | /tasks/*                                    | Task        | -        |
      | matching-sheet-tasks        | shared://🔀️matching-concordance/🔣️.json | /vectors/*/sheetTask                        | SheetTask   | -        |
      | matching-answers            | shared://🔀️matching-concordance/🔣️.json | /vectors/*/answer                           | Answer      | -        |
      | matching-results            | shared://🔀️matching-concordance/🔣️.json | /vectors/*/expected                         | TaskResult  | -        |
      | matching-guessed-sheet-tasks | shared://🔀️matching-concordance/🔣️.json | /guessed/*/sheetTask                       | SheetTask   | -        |
      | matching-guessed-answers    | shared://🔀️matching-concordance/🔣️.json | /guessed/*/answer                           | Answer      | -        |
      | matching-guessed-results    | shared://🔀️matching-concordance/🔣️.json | /guessed/*/expected                         | TaskResult  | -        |
      | classification-tasks        | shared://🕸️profile-similarity/🔣️.json   | /tasks/*                                    | Task        | -        |
      | classification-sheet-tasks  | shared://🕸️profile-similarity/🔣️.json   | /vectors/*/sheetTask                        | SheetTask   | -        |
      | classification-answers      | shared://🕸️profile-similarity/🔣️.json   | /vectors/*/answer                           | Answer      | -        |
      | classification-results      | shared://🕸️profile-similarity/🔣️.json   | /vectors/*/expected                         | TaskResult  | -        |
      | classification-timed-sheet-tasks | shared://🕸️profile-similarity/🔣️.json | /timed/*/sheetTask                     | SheetTask   | -        |
      | classification-timed-answers | shared://🕸️profile-similarity/🔣️.json  | /timed/*/answer                             | Answer      | -        |
      | classification-timed-results | shared://🕸️profile-similarity/🔣️.json  | /timed/*/expected                           | TaskResult  | -        |
      | validation-sheet-tasks      | shared://✅️answer-validation/🔣️.json     | /sheetTasks/*                               | SheetTask   | -        |
      | validation-answers          | shared://✅️answer-validation/🔣️.json     | /vectors/*/answer                           | Answer      | -        |
      | badge-quizzes               | shared://🏅️badge-rules/🔣️.json           | /quizzes/*                                  | Quiz        | -        |
      | badge-badges                | shared://🏅️badge-rules/🔣️.json           | /badges/*                                   | Badge       | -        |
      | badge-results               | shared://🏅️badge-rules/🔣️.json           | /vectors/*/results/*                        | RunResult   | -        |
      | lifecycle-catalog           | shared://🧾️learner-lifecycle/🔣️.json     | /catalog                                    | Catalog     | /quizzes |
      | lifecycle-quizzes           | shared://🧾️learner-lifecycle/🔣️.json     | /quizzes/*                                  | Quiz        | -        |
      | lifecycle-limits            | shared://🧾️learner-lifecycle/🔣️.json     | /limits                                     | Limits      | -        |
      | lifecycle-quota-limits      | shared://🧾️learner-lifecycle/🔣️.json     | /quotas/*/limits                            | Limits      | -        |
      | lifecycle-claim-commands    | shared://🧾️learner-lifecycle/🔣️.json     | /registrations/*/steps/*/command            | Command     | -        |
      | lifecycle-claim-events      | shared://🧾️learner-lifecycle/🔣️.json     | /registrations/*/steps/*/expected/events/*  | Event       | -        |
      | lifecycle-claim-rejections  | shared://🧾️learner-lifecycle/🔣️.json     | /registrations/*/steps/*/expected/rejection | Rejection   | -        |
      | lifecycle-given-events      | shared://🧾️learner-lifecycle/🔣️.json     | /learners/*/given/*                         | Event       | -        |
      | lifecycle-commands          | shared://🧾️learner-lifecycle/🔣️.json     | /learners/*/steps/*/command                 | Command     | -        |
      | lifecycle-events            | shared://🧾️learner-lifecycle/🔣️.json     | /learners/*/steps/*/expected/events/*       | Event       | -        |
      | lifecycle-rejections        | shared://🧾️learner-lifecycle/🔣️.json     | /learners/*/steps/*/expected/rejection      | Rejection   | -        |
      | lifecycle-learner-views     | shared://🧾️learner-lifecycle/🔣️.json     | /learners/*/views/expected/learner          | LearnerView | -        |
      | lifecycle-run-views         | shared://🧾️learner-lifecycle/🔣️.json     | /learners/*/views/expected/runs/*           | RunView     | -        |
      | leaderboard-catalog         | shared://🏆️leaderboard/🔣️.json           | /catalog                                    | Catalog     | /quizzes |
      | leaderboard-quizzes         | shared://🏆️leaderboard/🔣️.json           | /quizzes/*                                  | Quiz        | -        |
      | leaderboard-catalog-quizzes | shared://🏆️leaderboard/🔣️.json           | /catalogs/*/quizzes/*                       | Quiz        | -        |
      | leaderboard-catalog-views   | shared://🏆️leaderboard/🔣️.json           | /catalogs/*/expected                        | CatalogView | -        |
      | leaderboard-events          | shared://🏆️leaderboard/🔣️.json           | /vectors/*/learners/*/events/*              | Event       | -        |
      | leaderboard-learner-views   | shared://🏆️leaderboard/🔣️.json           | /vectors/*/expected/learnerViews/*          | LearnerView | -        |
      | leaderboard-rankings        | shared://🏆️leaderboard/🔣️.json           | /vectors/*/expected/leaderboards/*/*        | Leaderboard | -        |
      | leaderboard-daily-windows   | shared://🏆️leaderboard/🔣️.json           | /windows/*/expected/daily                   | LeaderboardWindow | -  |
      | leaderboard-weekly-windows  | shared://🏆️leaderboard/🔣️.json           | /windows/*/expected/weekly                  | LeaderboardWindow | -  |
      | leaderboard-monthly-windows | shared://🏆️leaderboard/🔣️.json           | /windows/*/expected/monthly                 | LeaderboardWindow | -  |
      | leaderboard-periods         | shared://🏆️leaderboard/🔣️.json           | /vectors/*/boards/*/period                  | LeaderboardPeriod | -  |
      | identity-handles            | shared://🪪️identity-shapes/🔣️.json       | /handles/*/expected/display                 | Handle      | -        |
      | presence-places             | shared://👥️shared-presence/🔣️.json       | /scopes/*/place                             | Place       | -        |
      | crowd-quizzes               | shared://📊️crowd-view/🔣️.json            | /quizzes/*                                  | Quiz        | -        |
      | crowd-results               | shared://📊️crowd-view/🔣️.json            | /vectors/*/results/*                        | RunResult   | -        |
      | crowd-views                 | shared://📊️crowd-view/🔣️.json            | /vectors/*/expected                         | CrowdView   | -        |
      | challenge-names             | shared://⛰️challenge-rules/🔣️.json        | /rules/challenges/*                         | Challenge   | -        |
      | challenge-point-challenges  | shared://⛰️challenge-rules/🔣️.json        | /points/*/challenge                         | Challenge   | -        |
      | challenge-tasks             | shared://⛰️challenge-rules/🔣️.json        | /tasks/*                                    | Task        | -        |
      | challenge-hint-sheet-tasks  | shared://⛰️challenge-rules/🔣️.json        | /hints/*/sheetTask                          | SheetTask   | -        |
      | challenge-hint-answers      | shared://⛰️challenge-rules/🔣️.json        | /hints/*/answer                             | Answer      | -        |
      | challenge-hints             | shared://⛰️challenge-rules/🔣️.json        | /hints/*/expected/*                         | Hint        | -        |
      | challenge-classification-sheet-tasks | shared://⛰️challenge-rules/🔣️.json | /classificationHints/*/sheetTask     | SheetTask   | -        |
      | challenge-classification-answers | shared://⛰️challenge-rules/🔣️.json   | /classificationHints/*/answer               | Answer      | -        |
      | challenge-classification-hints | shared://⛰️challenge-rules/🔣️.json     | /classificationHints/*/expected/*           | Hint        | -        |
      | challenge-quiz-sheet-tasks  | shared://⛰️challenge-rules/🔣️.json        | /quizHints/*/sheetTask                      | SheetTask   | -        |
      | challenge-quiz-answers      | shared://⛰️challenge-rules/🔣️.json        | /quizHints/*/answer                         | Answer      | -        |
      | challenge-quiz-hints        | shared://⛰️challenge-rules/🔣️.json        | /quizHints/*/expected/*                     | Hint        | -        |
      | lifecycle-malformed-views   | shared://🧾️learner-lifecycle/🔣️.json     | /malformed/learners/*/views/expected/runs/* | RunView     | -        |
    When every document a row's pointer reaches is validated against the row's definition
    Then jsonschema accepts every one of them
    And every implementation accepts the same quizzes and catalogs

  @id-repository-quizzes
  @level-fundamental
  @mode-differential
  Scenario: Every quiz and catalog authored in the teaching area conforms
    Given every ❓️quiz/🔣️.json under 🎓️teaching, found by walking the area at run time
    When each document is judged by the definition its schema field names
    Then every implementation accepts or rejects the same documents, keyed by their repository path

  @id-rejected-quizzes
  @level-fundamental
  @mode-error
  Scenario: Documents that break one rule of the schema are rejected everywhere, their unbroken bases accepted
    Given the committed vectors shared://🧬️schema-conformance/🔣️.json
    When the unbroken base quiz and catalogs and every rejected quiz and catalog are validated, a catalog with the committed quizzes its paths name
    Then jsonschema accepts every base and reports the rule each rejected vector names
    And jsonschema accepts every conforming typed instance and reports the rule each breaking one names
    And every implementation accepts every base and rejects every rejected document
