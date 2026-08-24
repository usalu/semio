@capability-semio-v1-mesh-mutate
@no-oracle-semio-mesh-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-mesh
Feature: Apply every typed semio MESH mutation to its committed specification fixtures
  `s.stdio.semio.mesh` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-mesh-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/
  🧪️oracle/🔣️component.json`). Every one of this subset's 17 kinds — mesh lifecycle, primitive
  lifecycle plus topology/geometry/material, material lifecycle plus base-color/metallic/roughness,
  texture lifecycle plus mime/bytes, and the one scalar reposition `move-vertex` — already carries an
  independently handcrafted `(before, mutation, after, diff)` specification fixture under its own
  leaf's `🧪️tests/` directory, authored by hand and already unit-tested inside the production crate
  itself. This feature re-exercises those SAME committed fixtures end-to-end through
  `apply_semio_mesh_mutation`, the entry point this ticket added, instead of calling
  `Mutation::diff`/`inverse` directly the way the in-crate tests do. The `oracle` role below reads
  the committed fixture JSON literally (no recomputation, no reimplementation); the `subject` role
  runs the real production entry point and the `ordered-json-v1` profile compares the two
  structurally.

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
