@capability-note-1-canvas-mutate
@oracle-note-python-independent
@comparison-ordered-json-v1
@mutations-note-1-canvas
Feature: Apply every typed note document canvas mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-note-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds have a subset-owned test. The reference is `🐍️.py` in this directory:
  a second implementation of the `s.note.note` document and this subset's typed mutations, written in
  Python from `../../../✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 1, 2, 3, 5 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the committed vectors. It imports nothing from this repository's Rust. It carries the FULL
  document shape — not only this subset's own members — because every scenario validates the whole
  document, not merely the fields this subset's own kinds write.

  This subset owns the six canvas-presentation scalars: grid visibility, spacing, subdivisions and opacity, plus snap-to-grid enablement and its own spacing. All six are document-level fields with no block addressing at all — rule 1's document-level scalar setters.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector asset://🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_note_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                       | vector                                                       |
      | change-grid-visible      | 👁️change-grid-visible/🧪️tests/🙈️hides-the-grid                 |
      | change-grid-spacing      | 📏️change-grid-spacing/🧪️tests/📏️widens-grid-spacing            |
      | change-grid-subdivisions | 🔢️change-grid-subdivisions/🧪️tests/🔢️doubles-grid-subdivisions |
      | change-grid-opacity      | 🌫️change-grid-opacity/🧪️tests/🌫️raises-grid-opacity            |
      | change-snap-enabled      | 🧲️change-snap-enabled/🧪️tests/🧲️enables-snap                   |
      | change-snap-grid-spacing | 📐️change-snap-grid-spacing/🧪️tests/📐️halves-snap-grid-spacing  |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_note_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                       | vector                                                       |
      | change-grid-visible      | 👁️change-grid-visible/🧪️tests/🙈️hides-the-grid                 |
      | change-grid-spacing      | 📏️change-grid-spacing/🧪️tests/📏️widens-grid-spacing            |
      | change-grid-subdivisions | 🔢️change-grid-subdivisions/🧪️tests/🔢️doubles-grid-subdivisions |
      | change-grid-opacity      | 🌫️change-grid-opacity/🧪️tests/🌫️raises-grid-opacity            |
      | change-snap-enabled      | 🧲️change-snap-enabled/🧪️tests/🧲️enables-snap                   |
      | change-snap-grid-spacing | 📐️change-snap-grid-spacing/🧪️tests/📐️halves-snap-grid-spacing  |
