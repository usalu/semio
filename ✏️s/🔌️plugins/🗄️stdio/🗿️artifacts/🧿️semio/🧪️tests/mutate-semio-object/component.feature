@capability-semio-v1-object-mutate
@no-oracle-semio-object-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-object
Feature: Apply every typed semio OBJECT mutation to its committed specification fixtures
  `s.stdio.semio.object` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-object-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/
  ✳️object/🧪️oracle/🔣️component.json`). Every one of this subset's 9 kinds carries an independently
  handcrafted `(before, mutation, after, diff)` specification fixture under its own leaf's
  `🧪️tests/` directory, and this feature re-exercises those SAME committed bytes end-to-end through
  `apply_semio_object_mutation` rather than calling `Mutation::diff`/`inverse` directly the way the
  in-crate fixture tests do.

  What distinguishes this subset is that it was the first COMPOSITE one. Alongside the composite
  `transform` field — split into `move-object`/`rotate-object`/`scale-object` so that a translation
  edit provably leaves rotation and scale alone — it carries three optional owned CHILD slots:
  `brep`, `mesh` and `properties`. A child is a two-string handle (`childId`, plus an artifact id
  and its dialect), never embedded content, so `create-<slot>`/`delete-<slot>` attach and detach a
  reference rather than moving geometry. The fixtures are chosen so a slot-confusing implementation
  cannot pass: `delete-brep` runs against an object carrying BOTH a brep and a mesh child and must
  leave the mesh handle intact, `delete-properties` runs against one carrying both a properties and
  a mesh child, and every `create-<slot>` runs against an object where that slot is empty and must
  land the exact artifact id and dialect the payload names.

  Because this case records a no-oracle decision, the runner executes NO oracle role — every
  assertion below therefore lives inside the subject handler, which compares the applied snapshot
  against the committed after-snapshot and the undone snapshot against the committed
  before-snapshot, and fails with both JSON documents printed. A handler that merely ran the
  mutation and returned would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_object_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                 |
      | move-object        |
      | rotate-object      |
      | scale-object       |
      | create-brep        |
      | delete-brep        |
      | create-mesh        |
      | delete-mesh        |
      | create-properties  |
      | delete-properties  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_object_mutation
    And the mutation's own computed inverse is applied through apply_semio_object_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id                 |
      | move-object        |
      | rotate-object      |
      | scale-object       |
      | create-brep        |
      | delete-brep        |
      | create-mesh        |
      | delete-mesh        |
      | create-properties  |
      | delete-properties  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed crate object through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🖼️assets/🎒️example.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees on the same placed object, a non-identity translation with all three child slots occupied by real brep, mesh and value handles
