@capability-jpg-jfif-1-01-baseline-mutate
@oracle-libjpeg-jpg-jfif-1-01-baseline-marker-cli
@comparison-ordered-json-v1
@mutations-jpg-jfif-1-01-baseline
Feature: Move a real photographic JPEG across every axis of the T.81 baseline conformance class
  This is a CONFORMANCE-CLASS vocabulary, not a document one. The sibling `🧾️document` subset owns the
  JFIF header, the quantization and Huffman tables by id, the restart interval, the retained
  segments and the raster; not one of those kinds addresses whether the document IS baseline. That
  is a property of the frame header and the entropy-coding mode, and
  `../../🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/🦀️component.rs`'s
  `check_baseline_conformance` reads exactly five axes of it: the SOF marker (T.81 Table B.1 admits
  only SOF0 for Annex F), the sample precision (§4.2 mandates 8), the presence of a DAC
  arithmetic-conditioning segment (Annex F is Huffman-only), the per-class Huffman table count
  (§B.2.4.2's practical ceiling of two) and each component's sampling factors (§B.2.2's 1..=4
  range). One kind per axis, plus the two every vocabulary carries, plus the insert/remove pairings
  the two counting axes need to be reachable in both directions.

  ⚠️ WHAT THIS CASE DOES NOT CLAIM. It makes no byte-level claim about any `mutate-<kind>` row, and
  that is deliberate rather than a shortfall. `encode_jpg` writes a conforming baseline file and no
  other kind of JPEG at all — `FF C0` for the frame marker, precision 8, exactly four DHT segments,
  never a DAC — so four of this vocabulary's five axes are normalized away on re-serialization. The
  fifth, per-component sampling, is NOT normalized: T.81 §B.2.2 makes `H`/`V` frame parameters that
  belong to the document, so `encode_jpg` writes back the factors the frame carries and a 4:4:4 scan
  stays 4:4:4 across a round trip. That is asserted by `identity-round-trip` below rather than by a
  `mutate-<kind>` row, because these rows are still measured on the decoded snapshot. A byte-level
  exhaustive case built on this catalog would report the four normalized kinds green while the
  mutation never reached a byte, which is the exact shape of shallow green this ticket exists to
  remove. The vocabulary is therefore measured where its axes actually live:
  the subject applies each row to the DECODED SNAPSHOT and reads its own checker's verdict, and the
  reference — libjpeg-turbo's `djpeg -v -v` marker trace and `rdjpgcom -verbose`, run as separate
  processes — reads the same axes out of the real scan, applies the row as T.81 defines the field it
  names and reads the class off the specification's own tables. The two projections must agree field
  for field. The decode/re-encode law is its own case, `🔁️round-trip-jpg-jfif-1-01-baseline`.

  The input is the real 2275x2560 architectural scan the `🧾️document` case reads, shared by both subsets
  rather than copied: two DQT, SOF0 with three components, four DHT and SOS. The `code` column names
  the diagnostic each kind is expected to raise on that document, and it is empty for exactly three
  rows — `remove-huffman-table`, `insert-frame-component` and `remove-frame-component` move their
  axis in the direction that stays INSIDE the class (four tables down to three is still ≤2 per
  class, three components up to four is still ≤4), so they must move the projection and raise
  nothing. Reading them as failures would be reading the standard backwards. `no-mutation` is the
  identity element and is the one row exempt from the observability law.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real scan and read the class verdict back
    Given the real input document shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the <id> mutation is applied to the decoded snapshot
      """
      {"kind": "<id>", "code": "<code>", "params": <params>}
      """
    Then the conformance verdict gains exactly <code>, and the projection moves on this kind's own axis
    Examples:
      | id                      | code                                            | params                                                    |
      | set-snapshot            | stdio.jpg.baseline.sof-marker                   | {"sofMarker": 194, "precision": 12, "arithmetic": true}   |
      | set-sof-marker          | stdio.jpg.baseline.sof-marker                   | {"marker": 194}                                           |
      | set-sample-precision    | stdio.jpg.baseline.precision                    | {"precision": 12}                                         |
      | set-arithmetic          | stdio.jpg.baseline.arithmetic-conditioning-present | {"arithmetic": true}                                   |
      | insert-huffman-table    | stdio.jpg.baseline.huffman-table-count          | {"index": 4, "class": "dc", "id": 2}                      |
      | remove-huffman-table    |                                                 | {"class": "dc", "id": 0}                                  |
      | insert-frame-component  |                                                 | {"index": 3, "id": 4, "hSampling": 1, "vSampling": 1}     |
      | remove-frame-component  |                                                 | {"id": 3}                                                 |
      | set-component-sampling  | stdio.jpg.baseline.component-sampling           | {"id": 1, "hSampling": 5, "vSampling": 1}                 |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-conformance
  Scenario: Apply no-mutation to the real scan and read the class verdict back
    Given the real input document shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the no-mutation mutation is applied to the decoded snapshot
      """
      {"kind": "no-mutation", "code": "", "params": {}}
      """
    Then the conformance verdict gains exactly , and the projection moves on this kind's own axis

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> puts the real scan back inside the class
    Given the real input document shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When <id> is applied to the decoded snapshot and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "code": "<code>", "params": <params>}
      """
    Then the conformance projection is the original one again, axis for axis
    Examples:
      | id                      | code                                            | params                                                    |
      | set-snapshot            | stdio.jpg.baseline.sof-marker                   | {"sofMarker": 194, "precision": 12, "arithmetic": true}   |
      | set-sof-marker          | stdio.jpg.baseline.sof-marker                   | {"marker": 194}                                           |
      | set-sample-precision    | stdio.jpg.baseline.precision                    | {"precision": 12}                                         |
      | set-arithmetic          | stdio.jpg.baseline.arithmetic-conditioning-present | {"arithmetic": true}                                   |
      | insert-huffman-table    | stdio.jpg.baseline.huffman-table-count          | {"index": 4, "class": "dc", "id": 2}                      |
      | remove-huffman-table    |                                                 | {"class": "dc", "id": 0}                                  |
      | insert-frame-component  |                                                 | {"index": 3, "id": 4, "hSampling": 1, "vSampling": 1}     |
      | remove-frame-component  |                                                 | {"id": 3}                                                 |
      | set-component-sampling  | stdio.jpg.baseline.component-sampling           | {"id": 1, "hSampling": 5, "vSampling": 1}                 |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation puts the real scan back inside the class
    Given the real input document shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When no-mutation is applied to the decoded snapshot and then its own computed inverse steps are applied
      """
      {"kind": "no-mutation", "code": "", "params": {}}
      """
    Then the conformance projection is the original one again, axis for axis
