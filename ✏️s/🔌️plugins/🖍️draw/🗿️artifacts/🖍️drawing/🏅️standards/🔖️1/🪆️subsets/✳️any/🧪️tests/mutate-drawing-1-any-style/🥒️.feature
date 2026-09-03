@capability-drawing-1-style-mutate
@no-oracle-drawing-mutation-semantics
@comparison-ordered-json-v1
@mutations-drawing-1-any-style
Feature: Apply every typed drawing-document style mutation to its committed specification vector
  🧩️ Duplicated from `../../../✳️style/🧪️tests/mutate-drawing-1-style/` (shard F4, this ticket) to close `unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` + `✳️any/🚪️io/🧬️mutations` owner — same mechanism E3 already proved on `sequence`. Reuses the ALREADY-manifested `drawing-1-style-mutate` capability, so no new v2 manifest entry or runtime-inventory coordinate is created.

  `s.draw.drawing` is a semio-NATIVE artifact: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so no reference LIBRARY is registered. That is recorded as the
  `drawing-mutation-semantics` no-oracle decision in `../../🧪️oracle/🔣️.json`, and its
  substitutes are the committed per-kind specification vectors plus the inverse law. This case
  re-exercises those SAME committed bytes end-to-end through
  `apply_drawing_mutation_json`/`undo_drawing_mutation_json`.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. What blocks a second
  implementation TODAY is stated in the decision: this case's vectors are not declared as `asset://`
  fixtures — the adapter reads the committed files through `include_str!` — so the plan pins none of
  their digests and a Python reference cannot read them at all.

  This subset owns how a layer PAINTS: `set-layer-opacity` and `set-layer-blend-mode` edit a scalar
  each, while `replace-layer-fill` and `replace-layer-stroke` carry vocabularies far larger than a
  scalar — swapping a solid for a linear gradient, or attaching a dashed stroke — which is why the
  adapter DECODES the committed payload rather than restating it as a Rust literal beside it.

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
      | id                   |
      | set-layer-opacity    |
      | set-layer-blend-mode |
      | replace-layer-fill   |
      | replace-layer-stroke |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_drawing_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id                   |
      | set-layer-opacity    |
      | set-layer-blend-mode |
      | replace-layer-fill   |
      | replace-layer-stroke |
