@capability-quiz-crowd
@oracle-quiz-python-reference
@comparison-quiz-score-v1
Feature: The submitted runs of a quiz aggregate into the same crowd view everywhere
  The quizzes are for fun: learners see what the others answered (design §17). `crowdView(quiz, results)`
  aggregates the submitted results of one quiz into a `CrowdView`, the projection the proctor keeps and
  serves as `{ type: "crowd", quiz }`. Everything is semantic, by item id, because every learner's sheet
  differs: only results of the viewed quiz count (`runs`); tasks follow the quiz's definition order, a
  matching gives one crowd task per dimension in definition order, items follow the task's definition
  order and an item nobody answered is left out. A classification item counts the assigned categories
  and a matching item the assigned values, as `{key, count}` with keys ascending; a sorting item carries
  the mean of its normalized position `position / (n − 1)` over the learners who ordered it.

  THE REFERENCE is `🐍️.py` beside this file, a second implementation written in Python from the design
  text and the schema: every count is recounted with `collections.Counter` and every mean recomputed
  with `numpy.mean` before it is projected. The subjects are `crowdView` of `@semio-tech/quiz` and
  `crowd_view` of the `quiz` crate; the harness holds the two subjects to each other as well. Mean
  positions compare under `quiz-score-v1` (1e-12), everything else exactly.

  Two readings are pinned here because the text leaves them open: keys ascend as strings by code point
  (the schema types a key as a string), and a value key is written in JSON number syntax as JavaScript
  prints it — so the energy density `120` sorts before `15`; a one-item order (possible only for a
  result that bypassed the sheet) puts its item at normalized position 0.

  The results are real: the lifecycle reference scored them from perfect, worst and mixed answers to
  real sheets of the energy quiz (all three kinds; heating oil and diesel equal in both matching
  dimensions, wood pellets and firewood sharing a CO₂ factor), plus a cooling result and a hand-built
  cooling result with a one-item sorting order. The vectors cover three runs, one run (so drawn-out
  items are unanswered), a foreign result that must be ignored, no runs at all, and the one-item order.

  The vectors shared://📊️crowd-view/🔣️.json are generated, never hand-edited, from the Python reference
  in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-crowds
  @level-fundamental
  @mode-differential
  Scenario: Every committed quiz and set of results aggregates into the committed crowd view
    Given the committed vectors shared://📊️crowd-view/🔣️.json
    When the crowd view of every vector's quiz is built from its results
    Then every implementation projects the same runs, tasks, dimensions, items, counts in ascending key order and mean positions within 1e-12
