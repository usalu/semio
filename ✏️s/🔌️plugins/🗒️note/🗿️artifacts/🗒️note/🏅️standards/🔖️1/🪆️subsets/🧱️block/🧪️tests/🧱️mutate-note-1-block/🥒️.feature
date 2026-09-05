@capability-note-1-block-mutate
@oracle-note-python-independent
@comparison-ordered-json-v1
@mutations-note-1-block
Feature: Apply every typed note document block mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-note-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds have a subset-owned test. The reference is `🐍️.py` in this directory:
  a second implementation of the `s.note.note` document and this subset's typed mutations, written in
  Python from `../../../✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 1, 2, 3, 5 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the committed vectors. It imports nothing from this repository's Rust. It carries the FULL
  document shape — not only this subset's own members — because every scenario validates the whole
  document, not merely the fields this subset's own kinds write.

  This subset owns the block TREE itself: creation, deletion (singular and bulk), duplication (singular and bulk), re-parenting, dragging a subtree, renaming, visibility, locking, absolute move, resize and font size. Blocks are id-keyed but their ORDER is the z-order the canvas paints in, so every tree inverse here restores POSITION and not merely membership: `delete-blocks` puts its blocks back in ASCENDING index order so each lands where it was, and `move-block-to-container` inverts to a re-parent back to the original container AT the original index. `drag-blocks` translates a named block and its whole SUBTREE.

  📌️ A FINDING THE REFERENCE MADE WHILE IT WAS BEING WRITTEN. `duplicate-blocks` computes each copy's insertion index against the PRE-MUTATION list and does not re-base it as earlier copies land. Its committed vector duplicates `blk-ink` (root index 1) and `blk-table` (root index 2) in one mutation, and the committed after-snapshot orders the root `blk-text, blk-ink, blk-ink-copy, blk-table-COPY, blk-table, …` — the second copy lands BEFORE its own source, where the singular `duplicate-block` places its copy after. Both implementations reproduce the committed order; naming it here is what keeps it from passing as intended behaviour.

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
      | id                      | vector                                                                 |
      | create-block            | ➕️create-block/🧪️tests/📷️inserts-a-photo-block-at-root-index-2           |
      | delete-block            | ❌️delete-block/🧪️tests/➖️removes-the-math-block                          |
      | delete-blocks           | 🧹️delete-blocks/🧪️tests/🗑️removes-the-ink-and-image-blocks               |
      | duplicate-block         | 📋️duplicate-block/🧪️tests/📋️copies-the-math-block-right-after-its-source |
      | duplicate-blocks        | 👥️duplicate-blocks/🧪️tests/👥️copies-ink-and-table-with-shifting-indices  |
      | move-block-to-container | 🚚️move-block-to-container/🧪️tests/📥️reparents-ink-into-the-callout-group |
      | drag-blocks             | 🤏️drag-blocks/🧪️tests/🤏️nudges-ink-and-the-whole-group-subtree           |
      | rename-block            | 🔖️rename-block/🧪️tests/🏷️renames-the-table-block                         |
      | change-block-visible    | 👀️change-block-visible/🧪️tests/🙈️hides-the-image-block                   |
      | change-block-locked     | 🔒️change-block-locked/🧪️tests/🔒️locks-the-callout-group                  |
      | move-block              | 📍️move-block/🧪️tests/📍️repositions-the-math-block                        |
      | resize-block            | ↔️resize-block/🧪️tests/📐️enlarges-the-image-block                        |
      | change-block-font-size  | 🔤️change-block-font-size/🧪️tests/🔤️enlarges-the-intro-font               |

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
      | id                      | vector                                                                 |
      | create-block            | ➕️create-block/🧪️tests/📷️inserts-a-photo-block-at-root-index-2           |
      | delete-block            | ❌️delete-block/🧪️tests/➖️removes-the-math-block                          |
      | delete-blocks           | 🧹️delete-blocks/🧪️tests/🗑️removes-the-ink-and-image-blocks               |
      | duplicate-block         | 📋️duplicate-block/🧪️tests/📋️copies-the-math-block-right-after-its-source |
      | duplicate-blocks        | 👥️duplicate-blocks/🧪️tests/👥️copies-ink-and-table-with-shifting-indices  |
      | move-block-to-container | 🚚️move-block-to-container/🧪️tests/📥️reparents-ink-into-the-callout-group |
      | drag-blocks             | 🤏️drag-blocks/🧪️tests/🤏️nudges-ink-and-the-whole-group-subtree           |
      | rename-block            | 🔖️rename-block/🧪️tests/🏷️renames-the-table-block                         |
      | change-block-visible    | 👀️change-block-visible/🧪️tests/🙈️hides-the-image-block                   |
      | change-block-locked     | 🔒️change-block-locked/🧪️tests/🔒️locks-the-callout-group                  |
      | move-block              | 📍️move-block/🧪️tests/📍️repositions-the-math-block                        |
      | resize-block            | ↔️resize-block/🧪️tests/📐️enlarges-the-image-block                        |
      | change-block-font-size  | 🔤️change-block-font-size/🧪️tests/🔤️enlarges-the-intro-font               |
