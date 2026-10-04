@capability-quiz-badge-rules
@oracle-quiz-python-reference
@comparison-ordered-json-v1
Feature: Badges are earned by the catalog's rules over every submitted result
  After every submission the catalog's badges are evaluated in catalog order over all submitted
  results of the learner, the new one included; badges already held are skipped (design §7). A
  `perfect-quiz` badge needs one result of its quiz scored exactly 1; a `perfect-tasks` badge needs every
  catalog task its selector matches (by task kind, by quiz, both or neither) to have scored exactly 1
  in some result — perfection may be spread over several runs — and a selector matching no task never
  awards; `completed-quizzes` needs a submitted result for every catalog quiz. A `perfect-quiz` or
  `perfect-tasks` rule may name a least `challenge` (challenge design §3.6): only results played at that
  challenge or a more demanding one count (easy < medium < hard < expert), so a perfect easy run never
  earns a badge that asks for medium. The catalog's `hard-cooling` asks for a perfect cooling run at hard
  and `expert-sorter` for every sorting task perfect at expert; the vectors play the cooling quiz
  perfectly at each of the four challenges, spread perfect sorting over expert runs (with and without
  one of them at hard), and play everything perfectly at expert, with no badge and with every badge held.
  Badges are this product's policy, so no third party can judge them. THE REFERENCE is `🐍️.py` beside
  this file, a second implementation of §7 written in Python from the design text; the subjects are
  `earnedBadges` of `@semio-tech/quiz` and `earned_badges` of the `quiz` crate, and the harness holds
  the two subjects to each other as well. The committed results are real run results, scored by the
  lifecycle reference from perfect and worst answers to real sheets.

  The vectors shared://🏅️badge-rules/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-awards
  @level-fundamental
  @mode-differential
  Scenario: Every committed set of results and held badges earns the committed badges in catalog order
    Given the committed vectors shared://🏅️badge-rules/🔣️.json
    When the catalog's badges are evaluated over every vector's results and held badges
    Then every implementation projects the same newly earned badge ids, in catalog order, per vector
    And the badge whose selector matches no task is never awarded
