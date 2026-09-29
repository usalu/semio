@capability-quiz-presence
@oracle-quiz-jsonschema
@comparison-ordered-json-v1
Feature: Shared presence, cursors and thinking are scoped per place and refused before they leak learner ids
  Every learner sees who else is online, where, and — in the room of the same place — their pointer,
  keyboard focus and what they drag, and in a quiz what the others currently think (design §15, with the
  screens of §16 and the sharing within quizzes of §17). The catalog-wide roster room
  (`rosterScope(catalog)` = the catalog id) carries a `PresenceState`: the public learner tag, the
  identity, the place and whether the learner is active. A shared place has a room of its own carrying
  `CursorState`s (`roomScope(catalog, place)`): `<catalog>/introduction`, `<catalog>/home`,
  `<catalog>/leaderboard` and `<catalog>/badges`, and `<catalog>/quiz/<quiz>` for the read-only page, the
  run and the results of one quiz. The identity screen has no room (a learner there has no tag yet), the
  personal pages `learner` and `preferences` have none, and neither has a quiz page, run or results place
  without its quiz. The thinking room `thinkingScope(catalog, quiz)` = `<catalog>/quiz/<quiz>/thinking`
  carries `ThinkingState`s: a learner's draft answers per task of the open run — classification and sorting
  drafts as answers, matching drafts as the values assigned per dimension and item (`ThinkingMatchingAnswer`),
  never card indices, which point into the publisher's own shuffled cards. Both cores refuse a state
  before the proctor relays it (`presenceProblem`, `cursorProblem`, `thinkingProblem`): a tag is 8
  lowercase hex digits and never a learner id; a cursor sits at `x, y ∈ [0, 1]` relative to an anchor
  every learner renders — a card, an item `item:<id>` or a category `category:<id>` — and a dragged item
  is named by its slug (§17 supersedes §15's "never share drags"); draft answers are structurally valid,
  a sorting draft repeats no item and a state stays within the bounds both cores share; no undeclared
  member may smuggle a learner id.

  THE REFERENCE is `🐍️.py` beside this file. python-jsonschema's Draft 7 validator judges every state
  against the normative `PresenceState`/`CursorState`/`ThinkingState` definitions, including the boundaries
  (0 and 1 are inside, −0 is 0, 1.0000001 and −0.001 are outside), the anchor grammar, `drag.item` and every
  length limit; the place rules the schema cannot state are written there from the design text — the page,
  the run and the results of a quiz name their quiz (`quiz-required`), no other screen names a quiz
  (`quiz-not-allowed`), only a run names the task on screen (`task-without-run`) — and a thinking state is
  refused when a sorting draft repeats an item (`duplicate-id`) or beyond `THINKING_LIMIT` = 64 (`too-many`):
  at most 64 tasks and 64 entries per order, assignment map, dimension map and dimension; the vectors sit
  on both sides of every bound. The limit is the value both cores share; the design text names no number.
  A non-finite matching value cannot be a JSON number, so it is covered as `null`, as `"120"` and as
  `"Infinity"`. A refused vector names the
  rule that refuses it and the oracle checks that exactly that rule fires. The subjects are
  `rosterScope`/`roomScope`/`thinkingScope`/`presenceProblem`/`cursorProblem`/`thinkingProblem` of
  `@semio-tech/quiz` and their snake_case twins in the `quiz` crate, projected as the scopes and as
  accepted-or-refused per state (the wording of a problem is each core's own), and the harness holds the
  two subjects to each other as well. JSON cannot carry NaN or an infinity, so a non-number coordinate is
  covered as a text and as null, and every projection is NaN-free.

  The vectors shared://👥️shared-presence/🔣️.json are generated, never hand-edited, from the Python
  reference in this directory, from the repository root:
  `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/generate_quiz_vectors.py`
  (`.venv/bin/python` outside Windows).

  @id-room-scopes
  @level-fundamental
  @mode-differential
  Scenario: Every place maps to the committed roster scope and room scope
    Given the committed scope vectors of shared://👥️shared-presence/🔣️.json
    When the roster scope of every catalog and the room scope of every place are computed
    Then every implementation projects the same roster scope and the same room for every screen, or none for identity, learner and preferences and for a quiz page or run without its quiz

  @id-presence-states
  @level-fundamental
  @mode-differential
  Scenario: Presence states are accepted or refused exactly where the schema and the place rules say
    Given the committed presence vectors of shared://👥️shared-presence/🔣️.json
    When every presence state is judged
    Then every implementation accepts the same states and refuses bad tags, smuggled ids and answers, bad identities, unknown screens and places that break a place rule

  @id-cursor-states
  @level-fundamental
  @mode-differential
  Scenario: Cursor states are accepted or refused at every boundary of the anchor, the coordinates, the drag and the tag
    Given the committed cursor vectors of shared://👥️shared-presence/🔣️.json
    When every cursor state is judged
    Then every implementation accepts card, item and category anchors and dragged items, and refuses coordinates outside 0…1, non-numbers, bad anchors and focus, bad drags, bad tags and smuggled members

  @id-thinking-scopes
  @level-fundamental
  @mode-differential
  Scenario: Every quiz has the committed thinking room
    Given the committed thinking scope vectors of shared://👥️shared-presence/🔣️.json
    When the thinking scope of every catalog and quiz is computed
    Then every implementation projects the same thinking room

  @id-thinking-states
  @level-fundamental
  @mode-differential
  Scenario: Thinking states are accepted or refused by their answers, their tag and their size
    Given the committed thinking vectors of shared://👥️shared-presence/🔣️.json
    When every thinking state is judged
    Then every implementation accepts draft answers of every kind up to 64 entries and refuses bad tags, malformed answers, card indices instead of values, non-numbers, task and dimension ids that are no slug, smuggled members, repeated sorting items and anything beyond 64
