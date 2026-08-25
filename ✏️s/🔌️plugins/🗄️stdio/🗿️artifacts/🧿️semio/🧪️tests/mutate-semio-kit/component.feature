@capability-semio-v1-kit-mutate
@no-oracle-semio-kit-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-kit
Feature: Apply every typed semio KIT mutation to its committed specification fixtures
  `s.stdio.semio.kit` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-kit-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/
  🧪️oracle/🔣️component.json`). Every one of this subset's 15 kinds carries an independently
  handcrafted `(before, mutation, after, diff)` specification fixture under its own leaf's
  `🧪️tests/` directory, and this feature re-exercises those SAME committed bytes end-to-end through
  `apply_semio_kit_mutation` rather than calling `Mutation::diff`/`inverse` directly the way the
  in-crate fixture tests do.

  What distinguishes this subset is that it is the only one carrying all four composition shapes at
  once: two owned CHILD collections (`objects`, `models`), one optional owned CHILD slot
  (`properties`), one independent-lifecycle LINK pool (`representations`, joined to a catalog type
  by `role == type id` and pinned to a revision), and two authored value collections (`types`,
  `designs`, where a design's `pieces` and `connections` are edited as one unit by `edit-design`).
  A child is owned and dies with its parent; a link is not, which is why `unbind-representation`
  detaches a pin without deleting anything and `change-representation-pin` moves a pin from head to
  a checkpoint. The fixtures are chosen against that: `delete-object` runs against a kit that also
  carries a model child and must leave it alone, `unbind-representation` removes the LEADING
  representation and keeps the trailing one, `remove-design` removes a design together with its
  pieces, and `rename-type` renames without recategorising so an implementation that rewrote the
  whole type record fails.

  Because this case records a no-oracle decision, the runner executes NO oracle role — every
  assertion below therefore lives inside the subject handler, which compares the applied snapshot
  against the committed after-snapshot and the undone snapshot against the committed
  before-snapshot, and fails with both JSON documents printed. A handler that merely ran the
  mutation and returned would report a pass having checked nothing.

  The `identity-round-trip` scenario carries the BYTE half of the identity law as well as the
  semantic half. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin,
  and both committed example files were produced by these very codecs — so re-printing the parsed
  snapshot and re-encoding it must reproduce those files BYTE FOR BYTE, and the scenario asserts
  exactly that through the shared `law::carrier_is_exact`. The must-differ tripwire the wave applies
  to third-party carriers would be backwards here: a re-emission that DIFFERED would be the defect,
  not the evidence. The two encodings also cross-check each other — the binary twin has to decode to
  the same document the text does, which no single codec can arrange on its own.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_kit_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                         |
      | create-object              |
      | delete-object              |
      | create-model               |
      | delete-model               |
      | create-properties          |
      | delete-properties          |
      | bind-representation        |
      | unbind-representation      |
      | change-representation-pin  |
      | add-type                   |
      | remove-type                |
      | rename-type                |
      | add-design                 |
      | remove-design              |
      | edit-design                |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_kit_mutation
    And the mutation's own computed inverse is applied through apply_semio_kit_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id                         |
      | create-object              |
      | delete-object              |
      | create-model               |
      | delete-model               |
      | create-properties          |
      | delete-properties          |
      | bind-representation        |
      | unbind-representation      |
      | change-representation-pin  |
      | add-type                   |
      | remove-type                |
      | rename-type                |
      | add-design                 |
      | remove-design              |
      | edit-design                |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed furniture kit through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🖼️assets/🎒️example.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees on the same kit, one chair type bound to one representation link, one living-room design of two pieces and a connection, and the object, model and properties children beside them
