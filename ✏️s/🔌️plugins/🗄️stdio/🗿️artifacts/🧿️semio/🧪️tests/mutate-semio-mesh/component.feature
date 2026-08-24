@capability-semio-v1-mesh-mutate
@no-oracle-semio-mesh-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-mesh
Feature: Apply every typed semio MESH mutation to its committed specification fixtures
  `s.stdio.semio.mesh` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-mesh-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/
  🧪️oracle/🔣️component.json`). Every one of this subset's 17 kinds carries an independently
  handcrafted `(before, mutation, after, diff)` specification fixture under its own leaf's
  `🧪️tests/` directory, and this feature re-exercises those SAME committed bytes end-to-end through
  `apply_semio_mesh_mutation` rather than calling `Mutation::diff`/`inverse` directly the way the
  in-crate fixture tests do.

  What distinguishes this subset is that it is three independent pools joined by reference —
  `meshes` (each holding positional `primitives` with parallel `positions`/`normals`/`uvs`/`colors`
  arrays and an `indices` list), `materials` and `textures`. A primitive names its material by id,
  so `set-primitive-material` and `delete-material` reach ACROSS pools, while `move-vertex` reaches
  into one primitive's position array by index and must leave the parallel attribute arrays it is
  not addressing untouched. The fixtures are chosen against that: `replace-primitive-geometry` swaps
  a triangle for a textured quad so every parallel array has to change length together;
  `change-texture-mime` retags a texture WITHOUT touching its bytes and `replace-texture-bytes`
  swaps the payload WITHOUT retagging, so an implementation that conflated the two fails; and the
  delete kinds each remove the LEADING member of their pool and keep the trailing one, so a deletion
  that removed by position-from-the-end passes nothing.

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
    When <id> is applied through apply_semio_mesh_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                          |
      | create-mesh                 |
      | delete-mesh                 |
      | create-primitive            |
      | delete-primitive            |
      | set-primitive-topology      |
      | replace-primitive-geometry  |
      | set-primitive-material      |
      | create-material             |
      | delete-material             |
      | change-material-base-color  |
      | change-material-metallic    |
      | change-material-roughness   |
      | create-texture              |
      | delete-texture              |
      | change-texture-mime         |
      | replace-texture-bytes       |
      | move-vertex                 |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_mesh_mutation
    And the mutation's own computed inverse is applied through apply_semio_mesh_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id                          |
      | create-mesh                 |
      | delete-mesh                 |
      | create-primitive            |
      | delete-primitive            |
      | set-primitive-topology      |
      | replace-primitive-geometry  |
      | set-primitive-material      |
      | create-material             |
      | delete-material             |
      | change-material-base-color  |
      | change-material-metallic    |
      | change-material-roughness   |
      | create-texture              |
      | delete-texture              |
      | change-texture-mime         |
      | replace-texture-bytes       |
      | move-vertex                 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed cube mesh through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️mesh/📚️examples/🧊️cube/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️mesh/📚️examples/🧊️cube/🖼️assets/🎒️example.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees on the same one-mesh document, a single triangle primitive with parallel position, normal, uv and colour arrays bound to the one PBR material, plus one image/png texture
