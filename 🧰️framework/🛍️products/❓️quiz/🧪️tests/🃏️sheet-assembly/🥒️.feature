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

  The sheet is a function of (quiz, seed, challenge) (challenge design §3.2): the stream is consumed
  alike at every challenge, so one seed deals the same task order, items, category and card shuffles at
  every challenge, and the sheet names its `challenge`. Where the challenge shows the keys (easy, medium)
  a sorting task carries `keys`, the true values of its presented items ascending, and a matching its
  `cards`; where it hides them (hard, expert) a sorting has no keys, a matching dimension no cards (the
  card shuffle is still drawn), a category no description and an axis only its id, label and short label,
  every profile value becoming its share of the axis range, `(v − min) / (max − min)`; a timed challenge
  (expert) gives every task `seconds = 30 + per(kind) × presented items` (× dimensions for a matching).
  A presented item carries its id, label, `short` label where it has one (challenge design §8.4a) and icon,
  never its `familiar` flag; a short label of an axis, a category or a quantity travels with it at every
  challenge (`energy-basics` carries one on a classification item, an axis, a category, a sorting item and a
  quantity, and a familiar living room that its sheets never show as familiar).
  Every vector names its challenge: every quiz and seed at medium, and one seed of the energy, cooling
  and icon quizzes and the rotation that rotates both tasks at all four challenges (their ids end in
  `-easy`, `-hard`, `-expert`). Before a sheet is projected the reference holds it to one deal per seed at
  every challenge, to numpy's sort of the presented values as its ladder of keys, to the presented values
  as its cards, to `numpy.interp`'s shares as its hidden profiles, and to the seconds of a timed task.

  The vectors shared://🃏️sheet-assembly/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-sheets
  @level-fundamental
  @mode-differential
  Scenario: Every committed quiz and seed assembles the committed sheet
    Given the committed vectors shared://🃏️sheet-assembly/🔣️.json
    When the sheet of every vector's quiz is assembled for the vector's seed and challenge
    Then every implementation projects the same task order, drawn items, category order and card order per vector
    And the same keys, cards, descriptions, axes, profiles and seconds the vector's challenge shows
    And no sheet item carries a category, a value or true values
