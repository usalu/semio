@capability-note-1-ink-mutate
@oracle-note-python-independent
@comparison-ordered-json-v1
@mutations-note-1-ink
Feature: Apply every typed note document ink mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-note-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds have a subset-owned test. The reference is `🐍️.py` in this directory:
  a second implementation of the `s.note.note` document and this subset's typed mutations, written in
  Python from `../../../✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 1, 2, 3, 5 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the committed vectors. It imports nothing from this repository's Rust. It carries the FULL
  document shape — not only this subset's own members — because every scenario validates the whole
  document, not merely the fields this subset's own kinds write.

  This subset owns everything about how ink is DRAWN: `change-pencil-width` and `change-eraser-radius` are document-level tool scalars, while `change-block-ink-width` and `edit-block-ink-stroke` reach INSIDE a stroke block's own typed content — the polyline and its width — rather than moving the block at all.

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
      | id                     | vector                                                      |
      | change-pencil-width    | ✏️change-pencil-width/🧪️tests/✏️thickens-pencil               |
      | change-eraser-radius   | 🧽️change-eraser-radius/🧪️tests/🧽️enlarges-eraser              |
      | change-block-ink-width | 🖊️change-block-ink-width/🧪️tests/🖊️thickens-the-sketch-stroke |
      | edit-block-ink-stroke  | 🎨️edit-block-ink-stroke/🧪️tests/🎨️redraws-the-sketch-polyline |

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
      | id                     | vector                                                      |
      | change-pencil-width    | ✏️change-pencil-width/🧪️tests/✏️thickens-pencil               |
      | change-eraser-radius   | 🧽️change-eraser-radius/🧪️tests/🧽️enlarges-eraser              |
      | change-block-ink-width | 🖊️change-block-ink-width/🧪️tests/🖊️thickens-the-sketch-stroke |
      | edit-block-ink-stroke  | 🎨️edit-block-ink-stroke/🧪️tests/🎨️redraws-the-sketch-polyline |
