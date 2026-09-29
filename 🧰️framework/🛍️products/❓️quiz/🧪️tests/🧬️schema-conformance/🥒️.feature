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
      | matching-tasks              | shared://🔀️matching-concordance/🔣️.json | /tasks/*                                    | Task        | -        |
      | matching-sheet-tasks        | shared://🔀️matching-concordance/🔣️.json | /vectors/*/sheetTask                        | SheetTask   | -        |
      | matching-answers            | shared://🔀️matching-concordance/🔣️.json | /vectors/*/answer                           | Answer      | -        |
      | matching-results            | shared://🔀️matching-concordance/🔣️.json | /vectors/*/expected                         | TaskResult  | -        |
      | classification-tasks        | shared://🕸️profile-similarity/🔣️.json   | /tasks/*                                    | Task        | -        |
      | classification-sheet-tasks  | shared://🕸️profile-similarity/🔣️.json   | /vectors/*/sheetTask                        | SheetTask   | -        |
      | classification-answers      | shared://🕸️profile-similarity/🔣️.json   | /vectors/*/answer                           | Answer      | -        |
      | classification-results      | shared://🕸️profile-similarity/🔣️.json   | /vectors/*/expected                         | TaskResult  | -        |
      | validation-sheet-tasks      | shared://✅️answer-validation/🔣️.json     | /sheetTasks/*                               | SheetTask   | -        |
      | validation-answers          | shared://✅️answer-validation/🔣️.json     | /vectors/*/answer                           | Answer      | -        |
      | badge-quizzes               | shared://🏅️badge-rules/🔣️.json           | /quizzes/*                                  | Quiz        | -        |
      | badge-badges                | shared://🏅️badge-rules/🔣️.json           | /badges/*                                   | Badge       | -        |
      | badge-results               | shared://🏅️badge-rules/🔣️.json           | /vectors/*/results/*                        | RunResult   | -        |
      | lifecycle-catalog           | shared://🧾️learner-lifecycle/🔣️.json     | /catalog                                    | Catalog     | /quizzes |
      | lifecycle-quizzes           | shared://🧾️learner-lifecycle/🔣️.json     | /quizzes/*                                  | Quiz        | -        |
      | lifecycle-roster-commands   | shared://🧾️learner-lifecycle/🔣️.json     | /roster/*/steps/*/command                   | Command     | -        |
      | lifecycle-roster-events     | shared://🧾️learner-lifecycle/🔣️.json     | /roster/*/steps/*/expected/events/*         | Event       | -        |
      | lifecycle-roster-rejections | shared://🧾️learner-lifecycle/🔣️.json     | /roster/*/steps/*/expected/rejection        | Rejection   | -        |
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
      | leaderboard-rankings        | shared://🏆️leaderboard/🔣️.json           | /vectors/*/expected/leaderboard             | Leaderboard | -        |
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
    When the unbroken base quiz and catalog and every rejected quiz and catalog are validated, a catalog with the committed quizzes its paths name
    Then jsonschema accepts both bases and reports the rule each rejected vector names
    And every implementation accepts both bases and rejects every rejected document
