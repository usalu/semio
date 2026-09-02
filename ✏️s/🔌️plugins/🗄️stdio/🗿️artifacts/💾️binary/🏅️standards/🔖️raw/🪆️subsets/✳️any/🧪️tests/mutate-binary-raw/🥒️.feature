@capability-binary-raw-mutate
@no-oracle-raw-buffer-no-format
@comparison-exact-bytes-v1
@mutations-binary-raw-any
Feature: Apply every typed raw-binary mutation to a real-world byte buffer
  A raw byte buffer has no format — it is bytes, full stop. Unlike every sibling subset in this
  wave there is no grammar for a third-party crate to parse and no independent reader to project
  both sides through, so this feature records the no-oracle decision `raw-buffer-no-format` instead
  of hunting for a weak or irrelevant one: the specification IS `BinaryMutation`'s own five-variant
  vocabulary and its documented offset/remove_len contract (`🏅️standards/🔖️raw/🪆️subsets/✳️any/
  🧬️schema/🧬️mutations/🦀️component.rs`), and the evidence is the specification vectors below plus
  the inverse law as a metamorphic property — both discharged by this subset's own independently
  written oracle (`🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`), which never touches
  the subject's `BinaryDiff`/`apply_binary_mutation`. No scenario here is typed `@mode-differential`.

  The input is a real 483,496-byte JFIF/XMP photograph — a floor-plan scan already committed at
  `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.jpg`,
  copied once into this artifact's own `🧫️fixtures/` and referenced here as `shared://`. Real bytes
  matter here precisely because this subset does NOT parse structure: a splice at offset 6 below
  lands inside the file's genuine `JFIF\0` identifier, not in synthetic padding, so the mutation
  exercises a real byte boundary the same way a corrupt or hand-edited real file would. Every
  scenario copies the fixture into the case work directory before touching it; the committed asset
  is never written to.

  The identity round trip below is deliberately weak, and says so rather than contriving a
  difference: for a raw buffer `decode`/`encode` really is the identity (`store::ArtifactPack`'s
  `encode_pack_with`/`decode_pack_with` for `BinarySnapshot` are the identity function on `bytes`,
  proved by `carrier_native_is_raw` in `🚪️io/🦀️component.rs`), so the no-byte-pass-through tripwire
  every other subset in this wave enforces CANNOT apply here. Byte-for-byte equality is the correct
  answer for this one subset, not a smuggled input.

  The specification-vector scenarios below state plainly which byte-splice edge cases this subset
  defines as VALID (a zero-length splice, an offset of exactly 0, an offset of exactly the buffer's
  length, a splice spanning the whole buffer, a truncate to 0, a truncate past the current length —
  the vocabulary's own documented no-op) versus which it defines as an ERROR (an offset beyond the
  buffer, or a `removeLen` that reaches past it) — and the error cases below prove the rejection is
  clean, never a silent corruption.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real buffer
    Given the real input buffer shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the exact output bytes
    Examples:
      | id           | params                                                            |
      | set-snapshot | {"snapshot":{"bytes":[82,69,80,76,65,67,69,68]}}                 |
      | splice       | {"offset":6,"removeLen":5,"insert":[65,66,67]}                   |
      | append-bytes | {"data":[84,82,65,73,76,69,82]}                                  |
      | truncate-at  | {"offset":200000}                                                |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-conformance
  Scenario: Apply no-mutation to the real buffer
    Given the real input buffer shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the exact output bytes

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real buffer
    Given the real input buffer shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the buffer matches its pre-mutation exact bytes
    Examples:
      | id           | params                                                            |
      | set-snapshot | {"snapshot":{"bytes":[82,69,80,76,65,67,69,68]}}                 |
      | splice       | {"offset":6,"removeLen":5,"insert":[65,66,67]}                   |
      | append-bytes | {"data":[84,82,65,73,76,69,82]}                                  |
      | truncate-at  | {"offset":200000}                                                |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real buffer
    Given the real input buffer shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    And the mutation's own inverse is applied to the result
    Then the buffer matches its pre-mutation exact bytes

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real buffer, where byte identity IS the correct answer
    Given the real input buffer shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the buffer is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is bit-identical to the input, honestly, because this subset's decode/encode is the identity
    And the oracle and the subject agree on the exact output bytes

  @id-vector
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Specification vector — <id>
    Given the real input buffer shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the <kind> mutation is applied with its parameters
      """
      {"kind": "<kind>", "params": <params>}
      """
    Then the oracle and the subject agree on the exact output bytes, per the specification's own definition of this case
    Examples:
      | id                        | kind        | params                                              |
      | zero-length-splice        | splice      | {"offset":100000,"removeLen":0,"insert":[]}         |
      | splice-at-offset-zero     | splice      | {"offset":0,"removeLen":2,"insert":[255,217]}       |
      | splice-at-exact-end       | splice      | {"offset":483496,"removeLen":0,"insert":[90,90,90]} |
      | splice-spans-whole-buffer | splice      | {"offset":0,"removeLen":483496,"insert":[88,89]}    |
      | truncate-to-zero          | truncate-at | {"offset":0}                                        |
      | truncate-beyond-length    | truncate-at | {"offset":999999999}                                |

  @id-append-to-empty-buffer
  @level-exhaustive
  @mode-conformance
  Scenario: Appending to an empty buffer produces exactly the appended bytes
    Given the empty byte buffer
    When the append-bytes mutation is applied with its parameters
      """
      {"kind": "append-bytes", "params": {"data": [72,73]}}
      """
    Then the oracle and the subject agree on the exact output bytes

  @id-invalid-splice
  @level-exhaustive
  @mode-error
  Scenario Outline: An invalid splice fails cleanly — <id>
    Given the real input buffer shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the splice mutation is attempted with its parameters
      """
      {"kind": "splice", "params": <params>}
      """
    Then it is rejected and the buffer is left exactly as it was, never silently corrupted
    Examples:
      | id                        | params                                             |
      | offset-beyond-buffer      | {"offset":483497,"removeLen":0,"insert":[]}        |
      | remove-len-exceeds-buffer | {"offset":483490,"removeLen":100,"insert":[]}      |
