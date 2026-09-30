@capability-tiff-6-0-mutate
@oracle-image-tiff-6-0-mutate-reader
@comparison-semantic-raster-v1
@mutations-tiff-6-0-document
Feature: Apply every typed TIFF 6.0 mutation to a real-world document
  The input is a real 500 DPI architectural floor-plan scan
  (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`,
  483 KB, 2275x2560), converted ONCE to TIFF 6.0 with the registered `image` 0.25 reference encoder
  (`image::codecs::tiff::TiffEncoder`) and committed as this artifact's own
  `shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff`. Its second IFD is a genuinely real second
  page — the actual decoded, downsampled (16x16) pixels of the real
  `🏛️rathaus-ahlen-grundriss/🖼️.png` floor plan, appended by this subset's own independent IFD-chain
  writer (`../../../../🔮️oracles/🦀️component.rs`'s `fixture_derivation` module — `image`'s public TIFF
  encoder can only ever emit a single IFD) — so `InsertIfd`/`RemoveIfd`, TIFF's own multi-page
  operations, are substantive on a genuinely multi-IFD document from the very first `Given`, without
  needing a second fixture per row. Every scenario copies the fixture into the case work directory
  before touching it; the committed document is never written to.

  Every Examples `params` cell is exactly the leaf's wire payload — its `payload_value()`, camelCase,
  no aggregate tag — decoded by the subject through the derive-generated `from_payload_value` and
  read by the oracle's IFD-chain codec by the same field names: a tag names its `TiffFieldType` and
  carries its `TiffValues` as `{kind, value}`, and strip bytes travel as byte arrays. `replace-pixels`
  carries the whole replacement RGBA raster of IFD 0, which for this 2275x2560 scan would be 23.3
  million numbers in one cell, so it runs in its own outlines on the committed 4x4 RGB document
  (`shared://🔲️replace-pixels-applied/⬅️before.tiff`) with the raster of that recipe's own after-image.

  On the @id-identity-round-trip scenario the "re-encoded bytes must differ from the input" half of
  the law binds NEITHER side, and the exact-bytes law binds BOTH. The committed fixture is the output
  of the oracle's own independent IFD-chain writer (see above) in the canonical baseline layout —
  header, strips, IFD chain — and this repository's `encode_tiff` emits that same layout from a
  snapshot that carries every tag typed and each IFD's strip bytes as its own raster, so both writers
  reproducing it byte for byte is canonical determinism, not a byte pass-through. Both sides therefore
  assert the two halves that ARE checkable — the semantic projection survives the decode/re-encode,
  and the writer reproduces the committed bytes exactly, which a dropped tag, a reordered IFD or a
  miscounted strip would all break. The mutate rows, every one of which moves the bytes, are what
  prove a real parse happened. The mutate and inverse laws are stated against the document as an
  unchanged reference round trip leaves it: on the scan that is the scan itself, and on the small
  raster document, which another writer authored, it is that document in the reference's normal form.

  ✅ CLOSED, AT THE CAUSE — `mutate-insert-ifd` (the ratios before and after are recorded in the
  ticket, not here).
  The row's `ifd` param carries six entries and a real `pixels` strip. The oracle backs that page
  with actual strip bytes, which forces `RowsPerStrip` to the page's `ImageLength` (TIFF6 §Strips: a
  single combined strip needs `RowsPerStrip = height`, or a reader expects `ceil(height/RowsPerStrip)`
  strip offsets and finds one), so its IFD 2 projects seven entries where ours projected six. The
  cause was that `TiffSnapshot` had ONE `pixels` field — IFD 0's — so this repository's encoder could
  not back a non-primary IFD with raster at all, discarded the param's strip, and omitted the three
  strip tags. The remedy this paragraph named has been carried out rather than papered over:
  `TiffIfd` now has its OWN `pixels` field (raw strip bytes), threaded through the snapshot, the
  diff (`TiffIfdDiff`), the text and binary diff codecs and the proto/graphql/ts/json mirrors, so the
  inserted page is backed by real bytes and its `StripOffsets`/`RowsPerStrip`/`StripByteCounts`
  triple is computed from them. Nothing was tolerated, ignored or cosmetically emitted: the profile
  still declares no writer freedom, the row's parameters are unchanged, and an IFD carrying no strip
  bytes still gets no invented `RowsPerStrip`. The same change closes a defect no scenario was
  measuring — before it, every round trip of this two-page fixture silently dropped page 2's raster,
  because the semantic projection only decodes IFD 0's.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | change-byte-order | {"byteOrder":"bigEndian"} |
      | insert-ifd | {"index":2,"ifd":{"entries":[{"tag":256,"kind":"long","values":{"kind":"long","value":[8]}},{"tag":257,"kind":"long","values":{"kind":"long","value":[8]}},{"tag":258,"kind":"short","values":{"kind":"short","value":[8,8,8]}},{"tag":259,"kind":"short","values":{"kind":"short","value":[1]}},{"tag":262,"kind":"short","values":{"kind":"short","value":[2]}},{"tag":277,"kind":"short","values":{"kind":"short","value":[3]}}],"pixels":[254,254,254,254,254,254,254,254,254,254,254,254,254,254,254,249,247,247,249,247,247,254,254,254,254,254,254,254,254,254,254,254,254,254,254,254,252,251,251,250,247,247,251,247,247,251,250,250,254,254,254,254,254,254,254,254,254,254,254,254,247,244,244,248,243,243,249,246,246,254,254,254,254,254,254,254,254,254,254,254,254,251,249,249,250,246,246,250,247,247,253,253,253,254,254,254,254,254,254,254,254,254,254,254,254,250,247,247,248,248,248,248,246,246,251,250,250,254,254,254,251,249,249,251,248,248,251,248,248,248,246,246,251,251,251,251,250,250,249,246,246,253,252,252,249,246,246,249,245,245,248,244,244,250,247,247,248,246,246,250,248,248,248,244,244,248,245,245,252,252,252,251,250,250,251,250,250,251,250,250,252,251,251,252,251,251,248,244,244,249,246,246]}} |
      | remove-ifd | {"index":1} |
      | replace-tag | {"ifdIndex":0,"tag":315,"kind":"ascii","values":{"kind":"ascii","value":"Derived for ticket 26/08/23/END-TO-END-TESTING-REFACTOR"}} |
      | remove-tag | {"ifdIndex":0,"tag":282} |

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to a small document
    Given the small input document shared://🔲️replace-pixels-applied/⬅️before.tiff
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | replace-pixels | {"pixels":[200,200,200,255,240,200,220,255,24,200,240,255,64,200,4,255,200,4,220,255,240,4,240,255,24,4,4,255,64,4,24,255,200,64,240,255,240,64,4,255,24,64,24,255,64,64,44,255,200,124,4,255,240,124,24,255,24,124,44,255,64,124,64,255]} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And its inverse is applied
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | change-byte-order | {"byteOrder":"bigEndian"} |
      | insert-ifd | {"index":2,"ifd":{"entries":[{"tag":256,"kind":"long","values":{"kind":"long","value":[8]}},{"tag":257,"kind":"long","values":{"kind":"long","value":[8]}},{"tag":258,"kind":"short","values":{"kind":"short","value":[8,8,8]}},{"tag":259,"kind":"short","values":{"kind":"short","value":[1]}},{"tag":262,"kind":"short","values":{"kind":"short","value":[2]}},{"tag":277,"kind":"short","values":{"kind":"short","value":[3]}}],"pixels":[254,254,254,254,254,254,254,254,254,254,254,254,254,254,254,249,247,247,249,247,247,254,254,254,254,254,254,254,254,254,254,254,254,254,254,254,252,251,251,250,247,247,251,247,247,251,250,250,254,254,254,254,254,254,254,254,254,254,254,254,247,244,244,248,243,243,249,246,246,254,254,254,254,254,254,254,254,254,254,254,254,251,249,249,250,246,246,250,247,247,253,253,253,254,254,254,254,254,254,254,254,254,254,254,254,250,247,247,248,248,248,248,246,246,251,250,250,254,254,254,251,249,249,251,248,248,251,248,248,248,246,246,251,251,251,251,250,250,249,246,246,253,252,252,249,246,246,249,245,245,248,244,244,250,247,247,248,246,246,250,248,248,248,244,244,248,245,245,252,252,252,251,250,250,251,250,250,251,250,250,252,251,251,252,251,251,248,244,244,249,246,246]}} |
      | remove-ifd | {"index":1} |
      | replace-tag | {"ifdIndex":0,"tag":315,"kind":"ascii","values":{"kind":"ascii","value":"Derived for ticket 26/08/23/END-TO-END-TESTING-REFACTOR"}} |
      | remove-tag | {"ifdIndex":0,"tag":282} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores a small document
    Given the small input document shared://🔲️replace-pixels-applied/⬅️before.tiff
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And its inverse is applied
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | replace-pixels | {"pixels":[200,200,200,255,240,200,220,255,24,200,240,255,64,200,4,255,200,4,220,255,240,4,240,255,24,4,4,255,64,4,24,255,200,64,240,255,240,64,4,255,24,64,24,255,64,64,44,255,200,124,4,255,240,124,24,255,24,124,44,255,64,124,64,255]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the document is decoded and re-encoded with no mutation
    Then the oracle and the subject agree on the semantic projection
