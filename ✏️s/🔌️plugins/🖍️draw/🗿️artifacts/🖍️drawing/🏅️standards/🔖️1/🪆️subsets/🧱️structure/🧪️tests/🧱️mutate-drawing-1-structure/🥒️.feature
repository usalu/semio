@capability-drawing-1-structure-mutate
@no-oracle-drawing-mutation-semantics
@comparison-ordered-json-v1
@mutations-drawing-1-structure
Feature: Apply every typed drawing-document structure mutation to its committed specification vector
  `s.draw.drawing` is a semio-NATIVE artifact: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so no reference LIBRARY is registered. That is recorded as the
  `drawing-mutation-semantics` no-oracle decision in `../../../✳️any/🔮️oracle/🔣️.json`, and its
  substitutes are the committed per-kind specification vectors plus the inverse law. This case
  re-exercises those SAME committed bytes end-to-end through
  `apply_drawing_mutation_json`/`undo_drawing_mutation_json`.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. What blocks a second
  implementation TODAY is stated in the decision: this case's vectors are not declared as `asset://`
  fixtures — the adapter reads the committed files through `include_str!` — so the plan pins none of
  their digests and a Python reference cannot read them at all.

  This subset owns the tree SHAPE itself: `create-layer`, `duplicate-layer` and `reorder-layer`
  address a PARENT plus an index, which is why undoing them has to restore a position in the tree and
  not merely a membership, and `delete-layer` removes a node together with its whole subtree. Layer
  ids are content-addressed, so re-creating an existing node collides for real rather than producing
  a second copy.

  `duplicate-layer`'s only committed specification vector is a REFUSAL: it names a source layer the
  document does not contain and the leaf declares `status: "rejected"`, `code:
  "mutation.target-missing"`. This case therefore asserts that documented refusal for that kind
  instead of pretending an application happened, and says so here rather than letting a scenario pass
  by observing nothing. An accepting `duplicate-layer` vector does not exist yet and is a real gap.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion below lives in the subject handler, which compares against the committed after-document
  through the shared `⚖️law` module and fails with the first divergence named by JSON path.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> reaches its committed after-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_drawing_mutation_json
    Then the resulting document is the committed after-document, and the mutation moved it
    Examples:
      | id            |
      | create-layer  |
      | delete-layer  |
      | reorder-layer |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_drawing_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id               | code                    |
      | duplicate-layer  | mutation.target-missing |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_drawing_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id               |
      | create-layer     |
      | duplicate-layer  |
      | delete-layer     |
      | reorder-layer    |
