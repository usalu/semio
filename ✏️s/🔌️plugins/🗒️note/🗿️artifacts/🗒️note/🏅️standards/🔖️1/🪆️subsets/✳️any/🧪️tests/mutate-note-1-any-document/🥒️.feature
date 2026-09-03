@capability-note-1-document-mutate
@oracle-note-python-independent
@comparison-ordered-json-v1
@mutations-note-1-any-document
Feature: Apply the typed note document identity mutation twice — once in Rust, once in Python — and require the same answer
  🧩️ Duplicated from `../../../✳️document/🧪️tests/mutate-note-1-document/` (shard G2, this ticket) to
  close `unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` +
  `✳️any/🚪️io/🧬️mutations` owner — same mechanism E3 proved on `sequence` and F4 proved on
  `drawing`/`equation`/`fem2d`/`fem3d`. Reuses the ALREADY-manifested `note-1-document-mutate`
  capability, so no new v2 manifest entry or runtime-inventory coordinate is created. The committed
  vector this scenario replays is COPIED (not referenced) into this case's own `🧫️fixtures/`, read
  through `local://` rather than `asset://`, because a `✳️any`-owned case's escape guard cannot reach
  sideways into `✳️document`'s own physical leaves the way `asset://` requires.

  This case is a CROSS-LANGUAGE DIFFERENTIAL, relocated out of the artifact-level `mutate-note-1`
  case in ticket `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`
  so this subset's own kinds have a subset-owned test. The reference is `🐍️.py` in this directory:
  a second implementation of the `s.note.note` document and this subset's typed mutations, written in
  Python from `../../🧬️schema/📸️snapshot/🔣️.json`, from rules 1, 2, 3, 5 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the committed vectors. It imports nothing from this repository's Rust. It carries the FULL
  document shape — not only this subset's own members — because every scenario validates the whole
  document, not merely the fields this subset's own kinds write.

  This subset owns the one document-level identity field: `rename-note` sets `title` directly, the simplest of the nine document-level scalar setters rule 1 of the derivation rules describes.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot local://<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload local://<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot local://<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector local://<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_note_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id          | vector                                      |
      | rename-note | 🏷️rename-note/🧪️tests/retitles-the-document |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot local://<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload local://<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_note_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id          | vector                                      |
      | rename-note | 🏷️rename-note/🧪️tests/retitles-the-document |
