@capability-s-space-1-mutate
@no-oracle-s-space-index-mutation-semantics
@comparison-ordered-json-v1
@mutations-s-space-1-any
Feature: Apply every typed s.space.space index mutation to its committed specification vector

  `s.space.space` is a semio-NATIVE artifact — no third party reads `.sspace.dsl.semio` — so no
  oracle is registered (recorded as the `s-space-index-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, which also records why generic table
  readers and content-addressed store crates were surveyed and DECLINED rather than merely absent).

  What distinguishes this subset is that it is an INDEX, not a document. Each row carries an
  artifact's metadata — `id`, `name`, `kind_id`, `schema`, a nested `dialect` block of
  `(artifactKind, standard, subset)`, and two clock pairs — and never that artifact's own bytes,
  which live in their own backbone document addressed by the same `id`. The DSL layout is therefore a
  `#[dsl(table)]` row grid with a BLOCK-typed column, which is why a flat record reader is not
  enough to round-trip it.

  Four verbs, and they are not four instances of one shape. `create-artifact` appends a whole row and
  inverts to a delete of the id it minted. `delete-artifact` removes one and inverts by re-inserting
  the captured row, so an inverse that rebuilt the row from the payload rather than from BASE loses
  every field the payload never carried. `rename-artifact` writes `name` alone and must leave
  `kind_id`, `schema` and the whole `dialect` block untouched. `touch-artifact` is the only verb that
  writes a CLOCK: it stamps `updated_at_ms` and `updated_by` together, so its inverse has to restore
  BOTH halves of the pair — restoring the timestamp and forgetting the editor is the exact failure
  its committed vector, `stamps-artifact-1-with-a-new-editor`, is built to expose.

  The identity round trip reads the artifact's own committed demo example, a space index whose
  `space-id` is `demo-space` and whose row table is declared with its full nine-column header and
  then left EMPTY — the one document shape where a codec that confuses "no rows" with "no table"
  produces something that still parses.

  Where the assertions live. This case records a no-oracle decision, so the runner dispatches NO
  oracle role at all: `oracleDecision` resolves an oracle implementation from an `@oracle-` tag, this
  feature has none, and the comparison profile therefore never receives two sides to compare. Every
  law below is asserted INSIDE the adapter's handler, through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio subsets use — `divergence` for
  a path-named first difference, `mutation_is_observable` for the forward law, `inverse_restores` for
  the inverse law, `round_trip_preserves` and `carrier_is_exact` for the identity law. A handler that
  applied the mutation and returned would report a pass having checked nothing, which is exactly the
  failure this platform exists to prevent.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied to that vector's before-snapshot through s_space_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id              |
      | create-artifact |
      | delete-artifact |
      | rename-artifact |
      | touch-artifact  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through s_space_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id              |
      | create-artifact |
      | delete-artifact |
      | rename-artifact |
      | touch-artifact  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed space index document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
