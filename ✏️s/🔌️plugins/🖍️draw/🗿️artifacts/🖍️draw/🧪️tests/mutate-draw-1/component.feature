@capability-draw-1-mutate
@no-oracle-draw-mutation-semantics
@comparison-ordered-json-v1
@mutations-draw-1-any
Feature: Apply every typed draw-document mutation to its committed specification vector
  `s.draw.draw` is a semio-NATIVE artifact: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so no reference LIBRARY is registered. That is recorded as the
  `draw-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and its substitutes are the
  committed per-kind specification vectors plus the inverse law. This case re-exercises those SAME
  committed bytes end-to-end through `apply_draw_mutation_json`/`undo_draw_mutation_json`.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-note-1` and `mutate-program-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`, the rules of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` and the
  committed vectors this feature already replays. What blocks it TODAY is stated in the decision and
  is one edit: this case's vectors are not declared as `asset://` fixtures — the `Examples` table
  carries the payloads inline and the adapter reads the committed files through `include_str!` — so
  the plan pins none of their digests and a Python reference cannot read them at all. Separately, `identity-round-trip` would still be refused: this subset's committed
  snapshot text grammar is the generic `family-scene` canvas grammar, and the committed artifact
  carries no `layers` block at all.
  Until that is done, every assertion below still lives in the SUBJECT role, and the ceiling is the
  one this decision has always had: no second producer runs beside it, so a mistake shared by the
  handcrafted vector and the production code passes unseen.

  What distinguishes this subset is that a draw document is a RECURSIVE tree of differently shaped
  layer nodes over one shared `base` record. Ten kinds address a single node by id and edit the shared
  base or its `attributes`; `update-layer-trace-params` reaches a field only the trace node kind has,
  so it is the one kind that cannot be applied to an arbitrary layer; and `create-layer`,
  `duplicate-layer` and `reorder-layer` address a PARENT plus an index, which is why undoing them has
  to restore a position in the tree and not merely a membership. Layer ids are content-addressed, so
  re-creating an existing node collides for real rather than producing a second copy.

  Two of the fourteen kinds carry vocabularies far larger than a scalar — `replace-layer-fill` swaps a
  solid for a linear gradient and `replace-layer-stroke` attaches a dashed stroke — which is why the
  adapter DECODES the committed payload rather than restating it as a Rust literal beside it.

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
    When <id> is applied through apply_draw_mutation_json
    Then the resulting document is the committed after-document, and the mutation moved it
    Examples:
      | id                          |
      | set-layer-visible           |
      | set-layer-locked            |
      | set-layer-opacity           |
      | set-layer-blend-mode        |
      | rename-layer                |
      | update-layer-transform      |
      | replace-layer-fill          |
      | replace-layer-stroke        |
      | set-layer-boolean-operation |
      | update-layer-trace-params   |
      | create-layer                |
      | delete-layer                |
      | reorder-layer               |

  @id-mutate
  @level-exhaustive
  @mode-error
  Scenario Outline: Applying <id> is refused exactly as its vector declares
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> is applied through apply_draw_mutation_json
    Then the document is left untouched and the declared <code> refusal was raised
    Examples:
      | id                          | code                    |
      | duplicate-layer             | mutation.target-missing |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores its committed before-document
    Given the committed before-document and mutation payload of the <id> specification vector
    When <id> and then every step of its own computed inverse are applied through undo_draw_mutation_json
    Then the document is the committed before-document again, member positions included
    Examples:
      | id                          |
      | set-layer-visible           |
      | set-layer-locked            |
      | set-layer-opacity           |
      | set-layer-blend-mode        |
      | rename-layer                |
      | update-layer-transform      |
      | replace-layer-fill          |
      | replace-layer-stroke        |
      | set-layer-boolean-operation |
      | update-layer-trace-params   |
      | create-layer                |
      | duplicate-layer             |
      | delete-layer                |
      | reorder-layer               |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse and reprint the real committed example without passing bytes through
    Given the real committed example asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When it is parsed, printed back to DSL and parsed again through round_trip_draw_dsl
    Then both parses agree on one document, and the reprinted text reproduces the committed example byte for byte
