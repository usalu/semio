@capability-drawing-1-metadata-mutate
@no-oracle-drawing-mutation-semantics
@comparison-ordered-json-v1
@mutations-drawing-1-metadata
Feature: Apply every typed drawing-document metadata mutation to its committed specification vector
  `s.draw.drawing` is a semio-NATIVE artifact: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so no reference LIBRARY is registered. That is recorded as the
  `drawing-mutation-semantics` no-oracle decision in `../../../✳️any/🧪️oracle/🔣️.json`, and its
  substitutes are the committed per-kind specification vectors plus the inverse law. This case
  re-exercises those SAME committed bytes end-to-end through
  `apply_drawing_mutation_json`/`undo_drawing_mutation_json`.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. What blocks a second
  implementation TODAY is stated in the decision: this case's vectors are not declared as `asset://`
  fixtures — the adapter reads the committed files through `include_str!` — so the plan pins none of
  their digests and a Python reference cannot read them at all.

  This subset owns the three kinds that edit a single layer's shared `base` record or its
  `attributes` by NAME rather than by structural position: `rename-layer` renames without touching
  the layer's content-addressed id, `set-layer-visible` and `set-layer-locked` flip a boolean each.

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
      | id                 |
      | set-layer-visible  |
      | set-layer-locked   |
      | rename-layer       |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_drawing_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id                 |
      | set-layer-visible  |
      | set-layer-locked   |
      | rename-layer       |
