@capability-quiz-answer-validation
@oracle-quiz-python-reference
@comparison-ordered-json-v1
Feature: An answer is valid for its sheet task, and complete once it answers every sheet item
  A proctor records an answer only when it fits the sheet task (design §5): its kind matches, every
  item, category and dimension it names exists in the sheet task, every card index is in range and used
  at most once per dimension, and a sorting order is a permutation of the sheet items — otherwise
  `answer-invalid`. What else an answer carries follows the sheet task (challenge design §3.3). Where
  the keys show — a sorting task with `keys`, a matching whose dimensions carry `cards` — the learner
  assigns: a sorting answer is its order alone and carries no `guesses`, a matching answer carries its
  `assignments` — required, as they always were — and no `guesses`. Where the keys are hidden the guesses
  are the answer, and an answer without them is valid and incomplete: a sorting answer
  carries numeric `guesses` (item id to the learner's guess in the quantity's base unit), each naming a
  sheet item, a finite number (positive on a logarithmic quantity), the guessed items standing in
  `order` in non-decreasing guess order — ties allowed, unguessed items unconstrained, an empty `guesses`
  valid —; a matching answer carries `guesses` per dimension and item instead of `assignments`, each
  naming a sheet dimension and a sheet item, a finite number (positive on a logarithmic quantity, any on
  a linear one), and `assignments` beside or instead of them are `answer-invalid`. A member counts as
  present also when it is empty (`{}`). Guesses that are no object or hold a
  non-number are `answer-invalid` too (those answers violate the schema and are kept apart under
  `malformed` in the vectors, since the schema-conformance case holds every `vectors` answer to the
  schema; a typed core that refuses them while decoding judges them `answer-invalid`). The `seconds` of
  a timed sheet task change nothing here. Partial classification and matching answers stay valid while
  the run is open. An untimed run is submitted only when every sheet task has a complete answer: a
  classification assigns every sheet item, a matching assigns — or guesses, where its cards are hidden —
  every sheet item in every dimension, a sorting order is complete where the keys show and complete
  where they are hidden once every sheet item is guessed; a missing answer is never complete.

  THE REFERENCE is `🐍️.py` beside this file: a second implementation of §5 written in Python from the
  design text alone. The subjects are `answerRejection`/`answerComplete` of `@semio-tech/quiz` and
  `answer_rejection`/`answer_complete` of the `quiz` crate. Completeness is projected only for an
  answer that is valid or absent — the contract does not define the completeness of an answer that
  may never be recorded.

  The vectors shared://✅️answer-validation/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-verdicts
  @level-fundamental
  @mode-differential
  Scenario: Every committed answer is judged valid or invalid, and complete or not
    Given the committed vectors shared://✅️answer-validation/🔣️.json
    When every vector's answer, or its absence, is judged against its sheet task
    Then every implementation projects the same rejection per vector, and the same completeness for every valid or absent answer
