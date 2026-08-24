@capability-semio-v1-kit-mutate
@no-oracle-semio-kit-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-kit
Feature: Apply every typed semio KIT mutation to its committed specification fixtures
  `s.stdio.semio.kit` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-kit-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/
  🧪️oracle/🔣️component.json`). A kit composes two owned CHILD collections (`objects`/`models`), one
  optional owned CHILD slot (`properties`), one independent-lifecycle LINK pool
  (`representations`, joined to a catalog TYPE by `role == type id`), and two id-keyed value
  collections (`types`, `designs` — a design's `pieces`/`connections` are edited as one authored
  unit). Every one of this subset's 15 kinds already carries an independently handcrafted `(before,
  mutation, after, diff)` specification fixture under its own leaf's `🧪️tests/` directory, authored
  by hand and already unit-tested inside the production crate itself — this feature re-exercises
  those SAME committed fixtures end-to-end through `apply_semio_kit_mutation`, the entry point this
  ticket added, instead of calling `Mutation::diff`/`inverse` directly the way the in-crate tests
  do. The `oracle` role below reads the committed fixture JSON literally (no recomputation, no
  reimplementation); the `subject` role runs the real production entry point and the
  `ordered-json-v1` profile compares the two structurally.

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
