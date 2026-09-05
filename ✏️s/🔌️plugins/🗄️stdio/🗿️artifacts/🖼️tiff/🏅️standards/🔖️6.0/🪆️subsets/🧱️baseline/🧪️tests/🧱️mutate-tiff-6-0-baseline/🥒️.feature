@capability-tiff-6-0-baseline-mutate
@no-oracle-tiff-6-0-baseline-conformance-class-semantics
@comparison-ordered-json-v1
@mutations-tiff-6-0-baseline
Feature: Move a real scanned TIFF across every axis of the Adobe TIFF 6.0 Baseline class
  This is a CONFORMANCE-CLASS vocabulary, not a document one. The sibling `✳️any` subset owns the
  generic IFD chain — `set-byte-order`, `insert-ifd`, `remove-ifd`, `set-tag`, `remove-tag`,
  `set-pixels` — and its `set-tag` can write any of the 65 536 tag numbers with any field type,
  which is the right vocabulary for editing a TIFF and the wrong one for moving a document between
  conformance classes. A Baseline class is a property of five specific fields of IFD 0, and
  `../../🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🦀️component.rs`'s
  `check_tiff_baseline_conformance` reads exactly those: `Compression` (259) restricted to
  {1 none, 2 CCITT G3 1-D, 32773 PackBits}, `PhotometricInterpretation` (262) to 0..=3,
  `BitsPerSample` (258) to {1, 4, 8}, `TileWidth`/`TileLength` (322/323) absent because Baseline is
  strip-organized, and `StripOffsets` (273) present when the IFD is not tiled. One kind per axis,
  plus the two every vocabulary carries, plus the insert/remove pairings the two structural axes
  need to be reachable in both directions.

  ⚠️ WHAT THIS CASE DOES NOT CLAIM. It makes no byte-level claim for `set-compression`,
  `set-photometric-interpretation`, `set-bits-per-sample` or `set-strip-offsets`, and that is
  deliberate: `encode_tiff` REGENERATES every one of `CORE_STRIP_TAGS` from the raster it is about
  to write, so those four are normalized away on re-serialization — correctly, because each of them
  describes the strip the encoder emits. Only the two tile kinds survive a re-encode, because
  `TileWidth`/`TileLength` sit outside that set and travel verbatim. A byte-level exhaustive case
  built on this catalog would therefore report four of its nine rows green while the mutation never
  reached a byte, which is the shape of shallow green this ticket exists to remove. The vocabulary
  is measured where its axes live instead: on the DECODED SNAPSHOT, against the checker's own
  verdict. That is also why no oracle is registered — `image` 0.25, the reference the `✳️any` subset
  does register, decodes and re-encodes a raster under its own choice of these very tags and has no
  API to set an arbitrary compression, an out-of-range photometric or a tiled IFD (recorded as the
  `tiff-6-0-baseline-conformance-class-semantics` no-oracle decision).

  The input is the real scanned TIFF the `✳️any` case reads, shared by both subsets rather than
  copied. The `code` column names the diagnostic each kind must raise on it, and it is empty for
  three rows that move their axis in the direction that stays INSIDE the class: `remove-tile-tags`
  restores strip organization, `set-strip-offsets` rewrites a pointer list the IFD already carries,
  and `no-mutation` is the identity element. `remove-tile-tags` is the one row that cannot be
  exercised against the committed document as it stands — a strip-organized scan has no tile tags to
  remove — so its `setup` column names the mutation that makes the removal meaningful, and its
  observability is measured from THAT state rather than from the untouched file. Every other row's
  `setup` is empty.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the real scan and read the class verdict back
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the <id> mutation is applied to the decoded snapshot
      """
      {"kind": "<id>", "code": "<code>", "setup": <setup>, "params": <params>}
      """
    Then the conformance verdict gains exactly <code>, and the projection moves on this kind's own axis
    Examples:
      | id                             | code                                            | setup                                                                    | params                                          |
      | set-snapshot                   | stdio.tiff.baseline.unsupported-compression     | {}                                                                       | {"compression": 5, "photometric": 6, "bits": [16, 16, 16]} |
      | set-compression                | stdio.tiff.baseline.unsupported-compression     | {}                                                                       | {"compression": 5}                              |
      | set-photometric-interpretation | stdio.tiff.baseline.unsupported-photometric     | {}                                                                       | {"photometric": 6}                              |
      | set-bits-per-sample            | stdio.tiff.baseline.unsupported-bits-per-sample | {}                                                                       | {"bits": [16, 16, 16]}                          |
      | insert-tile-tags               | stdio.tiff.baseline.tiled-not-baseline          | {}                                                                       | {"tileWidth": 256, "tileLength": 256}           |
      | remove-tile-tags               |                                                 | {"kind": "insert-tile-tags", "params": {"tileWidth": 256, "tileLength": 256}} | {}                                          |
      | set-strip-offsets              |                                                 | {}                                                                       | {"offsets": [8, 65536]}                         |
      | remove-strip-offsets           | stdio.tiff.baseline.missing-strip-offsets       | {}                                                                       | {}                                              |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-conformance
  Scenario: Apply no-mutation to the real scan and read the class verdict back
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the no-mutation mutation is applied to the decoded snapshot
      """
      {"kind": "no-mutation", "code": "", "setup": {}, "params": {}}
      """
    Then the conformance verdict gains exactly , and the projection moves on this kind's own axis

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> puts the real scan back where it started
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When <id> is applied to the decoded snapshot and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "code": "<code>", "setup": <setup>, "params": <params>}
      """
    Then the conformance projection is the pre-mutation one again, tag for tag
    Examples:
      | id                             | code                                            | setup                                                                    | params                                          |
      | set-snapshot                   | stdio.tiff.baseline.unsupported-compression     | {}                                                                       | {"compression": 5, "photometric": 6, "bits": [16, 16, 16]} |
      | set-compression                | stdio.tiff.baseline.unsupported-compression     | {}                                                                       | {"compression": 5}                              |
      | set-photometric-interpretation | stdio.tiff.baseline.unsupported-photometric     | {}                                                                       | {"photometric": 6}                              |
      | set-bits-per-sample            | stdio.tiff.baseline.unsupported-bits-per-sample | {}                                                                       | {"bits": [16, 16, 16]}                          |
      | insert-tile-tags               | stdio.tiff.baseline.tiled-not-baseline          | {}                                                                       | {"tileWidth": 256, "tileLength": 256}           |
      | remove-tile-tags               |                                                 | {"kind": "insert-tile-tags", "params": {"tileWidth": 256, "tileLength": 256}} | {}                                          |
      | set-strip-offsets              |                                                 | {}                                                                       | {"offsets": [8, 65536]}                         |
      | remove-strip-offsets           | stdio.tiff.baseline.missing-strip-offsets       | {}                                                                       | {}                                              |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation puts the real scan back where it started
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When no-mutation is applied to the decoded snapshot and then its own computed inverse steps are applied
      """
      {"kind": "no-mutation", "code": "", "setup": {}, "params": {}}
      """
    Then the conformance projection is the pre-mutation one again, tag for tag

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real scan without passing bytes through
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the scan is decoded into a snapshot and re-serialized from that snapshot alone
    Then the re-encoded bytes reproduce the reference writer's own file exactly, flipping one byte of the decoded raster changes them, the document is still Baseline-conforming, and the INDEPENDENT IFD reader agrees on the geometry of both
