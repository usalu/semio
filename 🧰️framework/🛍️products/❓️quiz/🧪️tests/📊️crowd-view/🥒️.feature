@capability-quiz-crowd
@oracle-quiz-python-reference
@comparison-quiz-score-v1
Feature: The submitted runs of a quiz aggregate into the same crowd view everywhere
  The quizzes are for fun: learners see what the others answered (design §17) and every figure is drawn
  from distributions the proctor already aggregated (design §20). `crowdView(quiz, results)` aggregates
  the submitted results of one quiz into a `CrowdView`, the projection the proctor keeps and serves as
  `{ type: "crowd", quiz }`. Everything is semantic, by item id, because every learner's sheet differs:
  only results of the viewed quiz count (`runs`); tasks follow the quiz's definition order, a matching
  gives one crowd task per dimension in definition order, items follow the task's definition order and
  an item nobody answered is left out. A classification item counts the assigned categories and a
  matching item the assigned values, as `{key, count}` with keys ascending; a sorting item carries the
  mean of its normalized position `position / (n − 1)` over the learners who ordered it and `places`,
  one count per place a sheet of the task presents (`m = presented(task) = min(draw, items)`): position
  `i` in an order of `n` items counts for `placeBin(i, n, m)`, `i · (m − 1) / (n − 1)` rounded half up
  (`i` itself when `n = m`, place 0 when `n < 2`), so the places sum to the item's answers.

  `scores` are ten bins of whole percents `⌊score · 100 + ½⌋` — `[0, 10)`, `[10, 20)`, … `[80, 90)`,
  `[90, 100]`, `scoreBin(score)` — of the run scores on the view (summing to `runs`), of the task scores
  on a classification or sorting, and of the dimension's scores on a matching's crowd task. A result
  counts for a task with its first task result of the task's id and kind, for a dimension with the first
  dimension of that id in it, and for an item with the first entry of that id.

  THE REFERENCE is `🐍️.py` beside this file, a second implementation written in Python from the design
  text and the schema: every count and place is recounted with `collections.Counter`, every mean
  recomputed with `numpy.mean` and every score distribution with `numpy.histogram` over the whole
  percents (edges 0, 10, … 90, 101) before it is projected; the reference rounds a place as an exact
  fraction, the recount in integer arithmetic. The subjects are `crowdView` of `@semio-tech/quiz` and
  `crowd_view` of the `quiz` crate; the harness holds the two subjects to each other as well. Mean
  positions compare under `quiz-score-v1` (1e-12), everything else exactly.

  Two readings are pinned here because the text leaves them open: keys ascend as strings by code point
  (the schema types a key as a string), and a value key is written in JSON number syntax as JavaScript
  prints it — so the energy density `120` sorts before `15`; a one-item order (possible only for a
  result that bypassed the sheet) puts its item at normalized position 0 and place 0.

  One crowd serves every challenge (challenge design §3.6): the bins take the score — the accuracy — of
  runs at every challenge. Where a sheet task hid its cards a matching item's `assigned` value is a guess
  (its result carries `miss`): it counts under the nearest value among all authored items of the task for
  that dimension, nearest on the quantity's scale (`log10` on a logarithmic one), the smaller value of two
  equally near. An item a timed run left unanswered — a classification or matching item result without
  `assigned` — counts nowhere, and a sorting task result without any guess where the keys were hidden is a
  task without an answer: it adds its score to the bins and nothing to the places.

  The results are real: the lifecycle reference scored them from perfect, worst and mixed answers to
  real sheets of the energy quiz (all three kinds, three tasks drawing fewer items than they define;
  heating oil and diesel equal in both matching dimensions, wood pellets and firewood sharing a CO₂
  factor), plus a cooling result and a hand-built cooling result with a one-item sorting order. The
  vectors cover three runs, one run (so drawn-out items are unanswered), a foreign result that must be
  ignored, no runs at all (ten zeros in every `scores`, no items), the one-item order, hand-built results
  whose run, task and dimension scores sit on and beside the bin edges (0, 0.0949, 0.095, 0.1, 0.5,
  0.8949, 0.895, 0.9, 0.995, 1; some without the second dimension, none with the room temperatures),
  orders the reference scored that are shorter and longer than the six power ratings and three room
  temperatures a sheet presents (1, 2, 3, 5, 6, 7 and 8 items, with halves that round up), and a result
  that repeats a task, a dimension and items and carries a task result of another kind. Further vectors
  mix runs at all four challenges, tally guessed appliance powers and lifespans of the icon quiz (equally
  near values, values beyond every authored one) and guessed energy densities nearer to another value on
  the logarithmic scale than on a linear one, and leave tasks and items unanswered on expert runs.

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
    Then every implementation projects the same runs, score bins, tasks, dimensions, items, counts in ascending key order, places and mean positions within 1e-12
