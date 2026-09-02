@capability-jpg-jfif-1-01-mutate
@oracle-image-jpeg-jfif-1-01-mutate-reader
@comparison-semantic-jpg-mutate-v1
@mutations-jpg-jfif-1-01-document
Feature: Apply every typed JFIF 1.01 mutation to a real-world scanned document
  The input is a real 483 KB, 2275x2560, 500 DPI JFIF 1.01 scan of a floor plan
  (abbau-aufbau-masterarbeit-grundriss.jpg) — not a synthetic fixture — sourced from
  🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/ and already copied into this
  artifact's own 🧫️fixtures/. Every scenario copies it into the case work directory before touching
  it; the committed fixture is never written to.

  The @id-mutate, @id-inverse and @id-identity-round-trip scenarios are laws the reference asserts on
  its own, before any oracle/subject comparison: it applies the row's kind and requires the result to
  be distinguishable from the unmutated document; it applies its own computed inverse on top of that
  real forward result and requires the projection back; and the round trip additionally requires the
  re-encoded bytes not to be bit-identical to the input. The slack every numeric comparison runs
  under is the profile's, not the handler's.

  The observability and inverse laws are stated against the document as no-mutation leaves it —
  ONE decode and re-encode by the reference — and not against the committed bytes. JPEG is lossy and
  both codecs regenerate their quantization tables from re_encode_quality rather than preserving the
  scanner's, so a single decode/re-encode already moves the raster and replaces the DQT. Measuring
  against the untouched scan would fold that unavoidable normalization into every scenario, making
  every kind look observable and every inverse look broken, both for the same reason and neither
  about the mutation.

  The slack exists because JPEG is lossy. Measured on this fixture (2275x2560 = 5 824 000 pixels),
  one decode/re-encode at quality 90 moves at most 2018 pixels between luma buckets, a pass through
  quality 50 at most 10 014 and through quality 5 at most 55 570 — all far inside it — while the
  replace-pixels and set-snapshot rows displace ~5.6 million pixels, an order of magnitude past it.
  Exact per-bucket equality is a law JPEG does not have and is deliberately not asserted.

  Walking the fixture's own marker chain gives APP0 (JFIF, version 1.1, density unit 1 = dots per
  inch, 500x500), APP1 carrying a real 31,385-byte Adobe XMP packet, two DQT, SOF0, four DHT and
  SOS. That real APP1 is what remove-other-segment removes and what insert-other-segment is inserted
  in front of; neither row addresses something the file does not have.

  This subset's own codec (../../🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🚪️io/🦀️component.rs) is a
  complete from-scratch baseline JPEG codec, not a wrapper over the `image` reference crate, and it
  deliberately regenerates fresh Annex K DQT/DHT tables scaled by `re_encode_quality` on every encode
  rather than preserving whatever tables a mutation set on the decoded snapshot, and it never emits a
  DRI/restart marker at all.

  ⚠️ Consequence: replace-quant-table, remove-quant-table, replace-huffman-table, remove-huffman-table and
  change-restart-interval mutate only the in-memory typed snapshot — none of the five is observable in
  the re-serialized bytes, by design, not by test gap. They are the ONLY five kinds named in the
  adapter's observability exemption list, and every other kind is required to move the compared
  projection or its scenario fails.

  change-jfif-header, insert-other-segment and remove-other-segment ARE written to real bytes — a real
  JFIF APP0 built from the snapshot's own fields, and the retained segments echoed verbatim right
  after it — so they are compared as real mutations here rather than reduced to "the file still
  decodes". The oracle reaches the density unit and both density values through `image`'s own
  `set_pixel_density`; the two JFIF version bytes (hard-coded to 1.2 in that crate's
  `build_jfif_header`) and the APPn/COM segments (no API at all) are written back at their fixed
  T.871 positions afterwards, which is stated in the oracle module against the crate's source.

  The projection therefore has two halves. The LOSSY half is an 8-bucket luma histogram, never raw
  samples: this platform's comparison tolerance is per-number and absolute with no aggregate mode, so
  an unbounded sample array could never be compared honestly. It is the only numeric member, and the
  only one the slack is for.

  ⚠️ Every EXACT member — the dimensions, the JFIF version, the density unit and both densities, each
  retained segment's marker/length/payload digest, and each DQT payload digest — is spelled as a
  STRING on purpose. The comparison engine applies the profile's tolerance to every NUMBER in the
  projection and compares strings by equality, so a numeric member cannot carry an exact claim at
  all: reported as numbers under a 400 000 slack, this real 2275x2560 scan and the 3x2 stub the
  set-snapshot row produces compared EQUAL, and so did every JFIF header field.

  quantTables is the DQT payload each side actually wrote. It is a shared, encoder-independent
  witness of the re-encode quality — `image`'s new_with_quality and this subset's own scale_quality
  implement the same IJG mapping over the same Annex K.1 base tables and emit them through the same
  zigzag — and it is what makes change-re-encode-quality observable at all, since a quality change is
  otherwise entirely inside the histogram's slack. It is the one member the identity round trip
  excludes from its own comparison, because BOTH codecs regenerate the DQT rather than carrying the
  source's forward, so the committed scan's own tables are gone by construction.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real scanned document
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                             |
      | change-jfif-header       | {"version": [1, 2], "densityUnits": "pixels-per-cm", "xDensity": 300, "yDensity": 300} |
      | replace-quant-table       | {"id": 0, "fill": 12}                                              |
      | remove-quant-table    | {"id": 1}                                                          |
      | replace-huffman-table     | {"class": "dc", "id": 0, "fill": 9}                                |
      | remove-huffman-table  | {"class": "ac", "id": 0}                                           |
      | change-restart-interval  | {"restartInterval": 16}                                            |
      | insert-other-segment  | {"index": 0, "marker": 226, "data": "0708"}                        |
      | remove-other-segment  | {"index": 0}                                                       |
      | replace-pixels            | {"fill": [9, 9, 9, 255]}                                           |
      | change-re-encode-quality | {"quality": 50}                                                    |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                             |
      | change-jfif-header       | {"version": [1, 2], "densityUnits": "pixels-per-cm", "xDensity": 300, "yDensity": 300} |
      | replace-quant-table       | {"id": 0, "fill": 12}                                              |
      | remove-quant-table    | {"id": 1}                                                          |
      | replace-huffman-table     | {"class": "dc", "id": 0, "fill": 9}                                |
      | remove-huffman-table  | {"class": "ac", "id": 0}                                           |
      | change-restart-interval  | {"restartInterval": 16}                                            |
      | insert-other-segment  | {"index": 0, "marker": 226, "data": "0708"}                        |
      | remove-other-segment  | {"index": 0}                                                       |
      | replace-pixels            | {"fill": [9, 9, 9, 255]}                                           |
      | change-re-encode-quality | {"quality": 50}                                                    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg
    When it is fully decoded to the typed snapshot and re-encoded from that snapshot alone
    Then the oracle and the subject agree on the semantic projection
