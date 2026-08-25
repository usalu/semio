@capability-semio-v1-value-mutate
@no-oracle-semio-value-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-value
Feature: Apply every typed semio VALUE mutation to its committed specification fixtures
  `s.stdio.semio.value` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so no oracle is registered and the `semio-value-mutation-semantics` no-oracle
  decision is recorded instead (see the subset's own `🧪️oracle/🔣️component.json`, which also records
  the one candidate that was considered and why it is not reachable this wave — `json` 0.12 could
  serve as a second producer for the five tree-shaped kinds, but wiring it needs a `📦️lib.rs` edit
  outside an executor's authority, and it has no analogue for `Bytes`, `Ref`, lexeme-preserving
  numbers or the top-level id-keyed node graph).

  `set-snapshot` already carries an independently handcrafted `(before, mutation, after, diff)`
  specification vector under its own leaf's `🧪️tests/` directory, unit-tested inside the production
  crate itself, and is read here as an `asset://` reference into that committed leaf — never copied.
  The other eight kinds own no leaf, so their vectors are committed beside this case and read as
  `local://` references: one shared before-document carrying a map, a nested three-element list, a
  `Ref` and a two-node graph, so root addressing, key descent, list descent and the path-free node
  graph are each genuinely exercised. Every fixture is read at run time through the host's
  `Context::fixture_json`, so the `oracle` role (which reads the committed answer literally, with no
  recomputation) and the `subject` role (which decodes the same bytes into real
  `SemioValueSnapshot`/`SemioValueMutation` values and runs the real entry point) read the exact
  same committed bytes rather than a hand-transcribed copy that could drift from them.

  `remove-map-entry` is the sharpest of the eight: it drops the MIDDLE member of the root map, and
  `SetMapEntry` on an absent key always appends, so a single-step undo would restore the value while
  losing the position. Its inverse is therefore a multi-step sequence, and the property scenario
  compares against the committed before-document in order.

  A bytes-level decode/re-encode round trip is NOT expressible from an owner-root test case for this
  subset. `.dsl.semio`/`.pack.semio` are produced by `store::ArtifactDsl`/`store::ArtifactPack`,
  traits reached only through a private `extern crate … as store;` alias that nothing re-exports, so
  an adapter compiled as an external crate cannot name them — the same structural gap wave 7 recorded
  for `kit`, `object`, `text` and `table`. What IS reachable, and what `identity-round-trip` asserts,
  is the equivalent completeness law one level up: starting from `SemioValueSnapshot::default()`, the
  subset's own full-replace `set-snapshot` diff must reconstruct the committed before-document node
  for node, so no slot of the recursive typed model is silently dropped on the way through.

  ⚖️ Because this case records a no-oracle decision, the runner executes NO oracle role: it resolves
  an oracle implementation from an `@oracle-` tag this feature deliberately does not carry, so the
  comparison profile never receives two sides to compare and the `oracle` handlers below are the
  written statement of the reference answer rather than a second running party. Every law this
  feature claims is therefore asserted INSIDE the subject handler, which fails with both documents
  printed. A handler that merely ran the mutation and returned would report a pass having checked
  nothing. Here that means the applied document is checked against the committed
  after-snapshot with list and map-entry POSITION significant, which is what makes
  `remove-list-item`'s and `remove-map-entry`'s position-restoring multi-step undo checkable rather
  than merely runnable.

  This case makes NO byte claim, and that is deliberate rather than overlooked. Every sibling
  `mutate-semio-*` case's `identity-round-trip` reads its subset's own committed
  `📚️examples/…/🗣️example.dsl.semio` and `🎒️example.pack.semio` and asserts `law::carrier_is_exact` on
  both. `s.stdio.semio.value` is the only one of the eighteen subsets that commits no example
  artifact in either encoding, so there are no committed bytes here to reproduce and no input bytes a
  codec could have copied. What `identity-round-trip` asserts instead is the typed completeness law —
  a real assertion, but not a byte one.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the committed before-document
    Given the committed before-document local://⬅️before.json
    And the committed mutation fixture local://<mutation> for the <id> kind
    And the committed after-snapshot fixture local://<after> for the <id> kind
    When <id> is applied through apply_semio_value_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id               | mutation                       | after                       |
      | no-mutation      | no-mutation.mutation.json      | ⬅️before.json               |
      | set-value        | set-value.mutation.json        | set-value.after.json        |
      | set-map-entry    | set-map-entry.mutation.json    | set-map-entry.after.json    |
      | remove-map-entry | remove-map-entry.mutation.json | remove-map-entry.after.json |
      | insert-list-item | insert-list-item.mutation.json | insert-list-item.after.json |
      | remove-list-item | remove-list-item.mutation.json | remove-list-item.after.json |
      | set-node         | set-node.mutation.json         | set-node.after.json         |
      | remove-node      | remove-node.mutation.json      | remove-node.after.json      |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-document
    Given the committed before-document local://⬅️before.json
    And the committed mutation fixture local://<mutation> for the <id> kind
    When <id> is applied through apply_semio_value_mutation
    And the mutation's own computed inverse is applied through apply_semio_value_mutation
    Then the snapshot matches the committed before-document again
    Examples:
      | id               | mutation                       |
      | no-mutation      | no-mutation.mutation.json      |
      | set-value        | set-value.mutation.json        |
      | set-map-entry    | set-map-entry.mutation.json    |
      | remove-map-entry | remove-map-entry.mutation.json |
      | insert-list-item | insert-list-item.mutation.json |
      | remove-list-item | remove-list-item.mutation.json |
      | set-node         | set-node.mutation.json         |
      | remove-node      | remove-node.mutation.json      |

  @id-mutate-set-snapshot
  @level-exhaustive
  @mode-conformance
  Scenario: set-snapshot retypes a map member and repoints a graph node
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node/📸️snapshot/⬅️before/🔣️component.json for the set-snapshot kind
    And the committed mutation fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node/🦠️mutation/🔣️component.json for the set-snapshot kind
    And the committed after-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node/📸️snapshot/➡️after/🔣️component.json for the set-snapshot kind
    When set-snapshot is applied through apply_semio_value_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for set-snapshot

  @id-inverse-set-snapshot
  @level-exhaustive
  @mode-property
  Scenario: Undoing set-snapshot restores the committed leaf before-snapshot
    Given the committed before-snapshot fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node/📸️snapshot/⬅️before/🔣️component.json for the set-snapshot kind
    And the committed mutation fixture asset://🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/retypes-a-map-member-and-repoints-a-graph-node/🦠️mutation/🔣️component.json for the set-snapshot kind
    When set-snapshot is applied through apply_semio_value_mutation
    And the mutation's own computed inverse is applied through apply_semio_value_mutation
    Then the snapshot matches the committed leaf before-snapshot again

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Rebuilding the committed before-document from an empty snapshot carries every node
    Given the committed before-document local://⬅️before.json
    When the empty snapshot is replaced with it through apply_semio_value_mutation
    Then the rebuilt snapshot equals the committed before-document, node for node
