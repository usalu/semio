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
  read by the oracle's IFD-chain codec by the same field names: a tag carries its `TiffValues` as
  `{kind, value}` (ASCII as its NUL-terminated octets), and an IFD's raster travels as its `storage`
  — strip chunks of raw sample bytes whose offsets and byte counts each writer lays out itself.
  `paint-region` paints only uncompressed TILED pages and is guarded by the revision of the subject's
  canonical snapshot; neither committed document is tiled, so it is witnessed on the wire only.

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
  unchanged reference round trip leaves it, which on the scan is the scan itself.

  `insert-ifd` inserts an 8x8 RGB page whose `storage` is one strip of real sample bytes and whose
  `RowsPerStrip` states that one strip covers the page (TIFF6 §Strips), so both writers lay the page
  out identically and the projection compares its seven entries and its raster.

  The snapshot-editing kinds production dispatch offers are measured here too: `set-snapshot`
  installs a 2x2 RGB document whose IFD 0 states its geometry, one strip and a Software tag, and
  `patch-snapshot` flips the header byte order through the editor's path-addressed patch. The
  IFD-chain oracle reads both through its own model.

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
      | insert-ifd | {"index":2,"ifd":{"entries":[{"tag":256,"values":{"kind":"long","value":[8]}},{"tag":257,"values":{"kind":"long","value":[8]}},{"tag":258,"values":{"kind":"short","value":[8,8,8]}},{"tag":259,"values":{"kind":"short","value":[1]}},{"tag":262,"values":{"kind":"short","value":[2]}},{"tag":277,"values":{"kind":"short","value":[3]}},{"tag":278,"values":{"kind":"long","value":[8]}}],"storage":{"kind":"strips","offsetsKind":"long","byteCountsKind":"long","chunks":[[254,254,254,254,254,254,254,254,254,254,254,254,254,254,254,249,247,247,249,247,247,254,254,254,254,254,254,254,254,254,254,254,254,254,254,254,252,251,251,250,247,247,251,247,247,251,250,250,254,254,254,254,254,254,254,254,254,254,254,254,247,244,244,248,243,243,249,246,246,254,254,254,254,254,254,254,254,254,254,254,254,251,249,249,250,246,246,250,247,247,253,253,253,254,254,254,254,254,254,254,254,254,254,254,254,250,247,247,248,248,248,248,246,246,251,250,250,254,254,254,251,249,249,251,248,248,251,248,248,248,246,246,251,251,251,251,250,250,249,246,246,253,252,252,249,246,246,249,245,245,248,244,244,250,247,247,248,246,246,250,248,248,248,244,244,248,245,245,252,252,252,251,250,250,251,250,250,251,250,250,252,251,251,252,251,251,248,244,244,249,246,246]]}}} |
      | remove-ifd | {"index":1} |
      | replace-tag | {"ifdIndex":0,"tag":315,"values":{"kind":"ascii","value":[68,101,114,105,118,101,100,32,102,111,114,32,116,105,99,107,101,116,32,50,54,47,48,56,47,50,51,47,69,78,68,45,84,79,45,69,78,68,45,84,69,83,84,73,78,71,45,82,69,70,65,67,84,79,82,0]}} |
      | remove-tag | {"ifdIndex":0,"tag":282} |
      | set-snapshot | {"snapshot":{"schema":"stdio.tiff","byteOrder":"littleEndian","ifds":[{"entries":[{"tag":256,"values":{"kind":"short","value":[2]}},{"tag":257,"values":{"kind":"short","value":[2]}},{"tag":258,"values":{"kind":"short","value":[8,8,8]}},{"tag":259,"values":{"kind":"short","value":[1]}},{"tag":262,"values":{"kind":"short","value":[2]}},{"tag":277,"values":{"kind":"short","value":[3]}},{"tag":278,"values":{"kind":"long","value":[2]}},{"tag":305,"values":{"kind":"ascii","value":[115,101,109,105,111,0]}}],"storage":{"kind":"strips","offsetsKind":"long","byteCountsKind":"long","chunks":[[255,0,0,0,255,0,0,0,255,255,255,255]]}}]}} |
      | patch-snapshot | {"patch":{"operation":"set","path":"/byteOrder","value":"bigEndian"}} |

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
      | insert-ifd | {"index":2,"ifd":{"entries":[{"tag":256,"values":{"kind":"long","value":[8]}},{"tag":257,"values":{"kind":"long","value":[8]}},{"tag":258,"values":{"kind":"short","value":[8,8,8]}},{"tag":259,"values":{"kind":"short","value":[1]}},{"tag":262,"values":{"kind":"short","value":[2]}},{"tag":277,"values":{"kind":"short","value":[3]}},{"tag":278,"values":{"kind":"long","value":[8]}}],"storage":{"kind":"strips","offsetsKind":"long","byteCountsKind":"long","chunks":[[254,254,254,254,254,254,254,254,254,254,254,254,254,254,254,249,247,247,249,247,247,254,254,254,254,254,254,254,254,254,254,254,254,254,254,254,252,251,251,250,247,247,251,247,247,251,250,250,254,254,254,254,254,254,254,254,254,254,254,254,247,244,244,248,243,243,249,246,246,254,254,254,254,254,254,254,254,254,254,254,254,251,249,249,250,246,246,250,247,247,253,253,253,254,254,254,254,254,254,254,254,254,254,254,254,250,247,247,248,248,248,248,246,246,251,250,250,254,254,254,251,249,249,251,248,248,251,248,248,248,246,246,251,251,251,251,250,250,249,246,246,253,252,252,249,246,246,249,245,245,248,244,244,250,247,247,248,246,246,250,248,248,248,244,244,248,245,245,252,252,252,251,250,250,251,250,250,251,250,250,252,251,251,252,251,251,248,244,244,249,246,246]]}}} |
      | remove-ifd | {"index":1} |
      | replace-tag | {"ifdIndex":0,"tag":315,"values":{"kind":"ascii","value":[68,101,114,105,118,101,100,32,102,111,114,32,116,105,99,107,101,116,32,50,54,47,48,56,47,50,51,47,69,78,68,45,84,79,45,69,78,68,45,84,69,83,84,73,78,71,45,82,69,70,65,67,84,79,82,0]}} |
      | remove-tag | {"ifdIndex":0,"tag":282} |
      | set-snapshot | {"snapshot":{"schema":"stdio.tiff","byteOrder":"littleEndian","ifds":[{"entries":[{"tag":256,"values":{"kind":"short","value":[2]}},{"tag":257,"values":{"kind":"short","value":[2]}},{"tag":258,"values":{"kind":"short","value":[8,8,8]}},{"tag":259,"values":{"kind":"short","value":[1]}},{"tag":262,"values":{"kind":"short","value":[2]}},{"tag":277,"values":{"kind":"short","value":[3]}},{"tag":278,"values":{"kind":"long","value":[2]}},{"tag":305,"values":{"kind":"ascii","value":[115,101,109,105,111,0]}}],"storage":{"kind":"strips","offsetsKind":"long","byteCountsKind":"long","chunks":[[255,0,0,0,255,0,0,0,255,255,255,255]]}}]}} |
      | patch-snapshot | {"patch":{"operation":"set","path":"/byteOrder","value":"bigEndian"}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff
    When the document is decoded and re-encoded with no mutation
    Then the oracle and the subject agree on the semantic projection
