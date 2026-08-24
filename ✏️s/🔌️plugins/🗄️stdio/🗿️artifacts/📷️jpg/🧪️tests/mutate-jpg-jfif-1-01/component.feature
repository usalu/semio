@capability-jpg-jfif-1-01-mutate
@oracle-image-jpeg-jfif-1-01-mutate
@comparison-semantic-jpg-mutate-v1
@mutations-jpg-jfif-1-01-any
Feature: Apply every typed JFIF 1.01 mutation to a real-world scanned document
  The input is a real 483 KB, 2275x2560, 500 DPI JFIF 1.01 scan of a floor plan
  (abbau-aufbau-masterarbeit-grundriss.jpg) — not a synthetic fixture — sourced from
  🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/ and already copied into this
  artifact's own 🧫️fixtures/. Every scenario copies it into the case work directory before touching
  it; the committed fixture is never written to.

  The @id-inverse and @id-identity-round-trip scenarios are laws the reference asserts on its own,
  before any oracle/subject comparison: the reference applies the row's kind, applies its own
  computed inverse on top of that real forward result, and requires the projection back within
  semantic-jpg-mutate-v1's OWN declared per-number slack; the round trip additionally requires the
  re-encoded bytes not to be bit-identical to the input. The slack is the profile's, not the
  handler's, and it exists because JPEG is lossy: measured on this fixture (2275x2560 = 5 824 000
  pixels), one reference decode/re-encode at quality 90 moves 413 pixels out of the darkest luma
  bucket, the inverse round trip's second re-encode raises that to 805, and set-re-encode-quality's
  pass through quality 50 moves 8841 out of the brightest — all far inside the slack, while the
  set-pixels and set-snapshot rows displace ~5.6 million pixels, three orders of magnitude past it.
  Exact per-bucket equality is a law JPEG does not have and is deliberately not asserted.

  This subset's own codec (../../🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/🦀️component.rs) is a
  complete from-scratch baseline JPEG codec, not a wrapper over the `image` reference crate, and it
  deliberately regenerates fresh Annex K DQT/DHT tables scaled by `re_encode_quality` on every encode
  rather than preserving whatever tables a mutation set on the decoded snapshot, and it never emits a
  DRI/restart marker at all. Consequence: set-quant-table, remove-quant-table, set-huffman-table,
  remove-huffman-table and set-restart-interval mutate only the in-memory typed snapshot — none of
  them is observable in the re-serialized bytes, by design, not by test gap. set-jfif-header and
  insert-other-segment/remove-other-segment ARE written to real bytes (a real JFIF APP0, verbatim
  segment echo) but remain metadata a conforming decoder does not need to reproduce for the decoded
  raster to be correct. JPEG is also lossy, so the semantic projection every scenario below is
  compared through (semantic-jpg-mutate-v1) reports geometry plus an 8-bucket luma histogram rather
  than raw samples: this platform's comparison tolerance is per-number and absolute with no aggregate
  mode, so an unbounded raw sample array could never be compared honestly.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real scanned document
    Given the real input document shared://🖼️abbau-aufbau-masterarbeit-grundriss.jpg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                             |
      | no-mutation           | {}                                                                  |
      | set-snapshot          | {"width": 3, "height": 2, "fill": [64, 128, 192, 255]}             |
      | set-jfif-header       | {"version": [1, 2], "densityUnits": "pixels-per-cm", "xDensity": 300, "yDensity": 300} |
      | set-quant-table       | {"id": 0, "fill": 12}                                              |
      | remove-quant-table    | {"id": 1}                                                          |
      | set-huffman-table     | {"class": "dc", "id": 0, "fill": 9}                                |
      | remove-huffman-table  | {"class": "ac", "id": 0}                                           |
      | set-restart-interval  | {"restartInterval": 16}                                            |
      | insert-other-segment  | {"index": 0, "marker": 226, "data": "0708"}                        |
      | remove-other-segment  | {"index": 0}                                                       |
      | set-pixels            | {"fill": [9, 9, 9, 255]}                                           |
      | set-re-encode-quality | {"quality": 50}                                                    |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🖼️abbau-aufbau-masterarbeit-grundriss.jpg
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                             |
      | no-mutation           | {}                                                                  |
      | set-snapshot          | {"width": 3, "height": 2, "fill": [64, 128, 192, 255]}             |
      | set-jfif-header       | {"version": [1, 2], "densityUnits": "pixels-per-cm", "xDensity": 300, "yDensity": 300} |
      | set-quant-table       | {"id": 0, "fill": 12}                                              |
      | remove-quant-table    | {"id": 1}                                                          |
      | set-huffman-table     | {"class": "dc", "id": 0, "fill": 9}                                |
      | remove-huffman-table  | {"class": "ac", "id": 0}                                           |
      | set-restart-interval  | {"restartInterval": 16}                                            |
      | insert-other-segment  | {"index": 0, "marker": 226, "data": "0708"}                        |
      | remove-other-segment  | {"index": 0}                                                       |
      | set-pixels            | {"fill": [9, 9, 9, 255]}                                           |
      | set-re-encode-quality | {"quality": 50}                                                    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🖼️abbau-aufbau-masterarbeit-grundriss.jpg
    When it is fully decoded to the typed snapshot and re-encoded from that snapshot alone
    Then the oracle and the subject agree on the semantic projection
