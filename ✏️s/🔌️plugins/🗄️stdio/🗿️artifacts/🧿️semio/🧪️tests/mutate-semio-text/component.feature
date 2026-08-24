@capability-semio-v1-text-mutate
@no-oracle-semio-text-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-text
Feature: Apply every typed semio TEXT mutation to its committed specification fixtures
  `s.stdio.semio.text` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-text-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️text/
  🧪️oracle/🔣️component.json`). Every one of this subset's 7 kinds already carries an independently
  handcrafted `(before, mutation, after, diff)` specification fixture under its own leaf's
  `🧪️tests/` directory, authored by hand and already unit-tested inside the production crate itself
  — this feature re-exercises those SAME committed fixtures end-to-end through
  `apply_semio_text_mutation`, the entry point this ticket added, instead of calling
  `Mutation::diff`/`inverse` directly the way the in-crate tests do. The `oracle` role below reads
  the committed fixture JSON literally (no recomputation, no reimplementation); the `subject` role
  runs the real production entry point and the `ordered-json-v1` profile compares the two
  structurally.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_text_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                  |
      | insert-run          |
      | remove-run          |
      | edit-run            |
      | change-run-language |
      | reorder-runs        |
      | add-mark            |
      | remove-mark         |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_text_mutation
    And the mutation's own computed inverse is applied through apply_semio_text_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id                  |
      | insert-run          |
      | remove-run          |
      | edit-run            |
      | change-run-language |
      | reorder-runs        |
      | add-mark            |
      | remove-mark         |
