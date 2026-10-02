@capability-quiz-sheet-assembly
@oracle-quiz-python-reference
@comparison-ordered-json-v1
Feature: A quiz and a seed assemble the same solution-free sheet in every language
  A run is presented as a sheet: the first task remains first and the remaining task order, the drawn items in their order, the category order
  and the card order, and nothing that reveals a solution (design §4). The sheet is a pure function of
  (quiz, seed) and consumes the MT19937 stream in a fixed order — the task order is shuffled first while keeping its first task in place, then every task
  in definition order: its items (then the draw), a classification task's categories, a matching
  task's cards per dimension in definition order. A sorting task whose drawn order happens to equal its
  true ascending order (by value, ties by definition index) is rotated left by one without a draw.

  THE REFERENCE is `🐍️.py` beside this file: a second implementation of §4 written in Python from the
  design text alone, drawing from numpy's MT19937. The subjects are `sheetOf` of `@semio-tech/quiz`
  and `sheet_of` of the `quiz` crate; the harness additionally holds the two subjects to each other.

  The three quizzes cover every branch: `energy-basics` has all three task kinds, a classification
  task with three axes, four profiled categories and one without a profile, a logarithmic sorting task
  with a tie, a linear sorting task without a draw, and a two-dimensional matching task whose drawn
  items repeat card values (heating oil and diesel are identical in both dimensions, wood pellets and
  firewood share a CO₂ factor); `cooling-basics` has a profile-free classification task with a draw;
  `rotation-demo` has two sorting tasks small enough to be shuffled into their solution, with seeds
  chosen so that one sheet rotates both, one draws a tie against definition order (value-ascending
  but not rotated, because the rule compares to the stable ascending order) and one rotates neither.

  The vectors shared://🃏️sheet-assembly/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-sheets
  @level-fundamental
  @mode-differential
  Scenario: Every committed quiz and seed assembles the committed sheet
    Given the committed vectors shared://🃏️sheet-assembly/🔣️.json
    When the sheet of every vector's quiz is assembled for the vector's seed
    Then every implementation projects the same task order, drawn items, category order and card order per vector
    And no sheet item carries a category, a value or true values
