@capability-tiff-6-0-mutate
@oracle-image-tiff-6-0-mutate
@comparison-semantic-raster-v1
@mutations-tiff-6-0-any
Feature: Apply every typed TIFF 6.0 mutation to a real-world document
  The input is a real 500 DPI architectural floor-plan scan
  (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.jpg`,
  483 KB, 2275x2560), converted ONCE to TIFF 6.0 with the registered `image` 0.25 reference encoder
  (`image::codecs::tiff::TiffEncoder`) and committed as this artifact's own
  `shared://🖼️abbau-aufbau-masterarbeit-grundriss.tiff`. Its second IFD is a genuinely real second
  page — the actual decoded, downsampled (16x16) pixels of the real
  `🖼️rathaus-ahlen-grundriss.png` floor plan, appended by this subset's own independent IFD-chain
  writer (`../../../../🧪️oracle/🦀️component.rs`'s `fixture_derivation` module — `image`'s public TIFF
  encoder can only ever emit a single IFD) — so `InsertIfd`/`RemoveIfd`, TIFF's own multi-page
  operations, are substantive on a genuinely multi-IFD document from the very first `Given`, without
  needing a second fixture per row. Every scenario copies the fixture into the case work directory
  before touching it; the committed document is never written to.

  On the @id-identity-round-trip scenario the "re-encoded bytes must differ from the input" half of
  the law binds the SUBJECT only, and deliberately does not bind the oracle: the committed fixture
  is itself the output of the oracle's own independent IFD-chain writer (see above), so that writer
  reproducing it byte-for-byte is canonical determinism, not a byte pass-through. The oracle side
  therefore asserts the two halves that ARE checkable of it — the semantic projection survives the
  decode/re-encode, and the writer reproduces its own committed output exactly, which any
  reader/writer asymmetry would break.

  ⚠️ KNOWN OPEN DIVERGENCE — `mutate-insert-ifd` (parity 16/17, 2026-08-24). The row's `ifd` param
  carries six entries and a real `pixels` strip. The oracle backs that page with actual strip bytes,
  which forces `RowsPerStrip` to the page's `ImageLength` (TIFF6 §Strips: a single combined strip
  needs `RowsPerStrip = height`, or a reader expects `ceil(height/RowsPerStrip)` strip offsets and
  finds one), so its IFD 2 projects seven entries. `TiffSnapshot` has ONE `pixels` field — IFD 0's —
  so this repository's encoder cannot back a non-primary IFD with raster at all and writes the six
  declared entries verbatim; see `MultiIfdEncodeScopeNote` in
  `../../🏅️standards/🔖️6.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`. The two sides genuinely produce
  different documents: the oracle's inserted page has pixels, ours is metadata only. Closing it means
  giving `TiffIfd` its own strip bytes — a schema-first change across the snapshot, diff, mutation,
  proto/graphql/ts mirrors and the binary protocol — not a tolerance, an `ignoreKeys` entry or a
  cosmetic `RowsPerStrip` our encoder would have nothing to back.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🖼️abbau-aufbau-masterarbeit-grundriss.tiff
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"byteOrder": "little-endian", "ifds": [{"entries": [{"tag":256,"type":4,"values":[8]},{"tag":257,"type":4,"values":[8]},{"tag":258,"type":3,"values":[8,8,8]},{"tag":259,"type":3,"values":[1]},{"tag":262,"type":3,"values":[2]},{"tag":277,"type":3,"values":[3]}], "pixels": "fefefefefefefefefefefefefefefef9f7f7f9f7f7fefefefefefefefefefefefefefefefcfbfbfaf7f7fbf7f7fbfafafefefefefefefefefefefefef7f4f4f8f3f3f9f6f6fefefefefefefefefefefefefbf9f9faf6f6faf7f7fdfdfdfefefefefefefefefefefefefaf7f7f8f8f8f8f6f6fbfafafefefefbf9f9fbf8f8fbf8f8f8f6f6fbfbfbfbfafaf9f6f6fdfcfcf9f6f6f9f5f5f8f4f4faf7f7f8f6f6faf8f8f8f4f4f8f5f5fcfcfcfbfafafbfafafbfafafcfbfbfcfbfbf8f4f4f9f6f6"}]} |
      | set-byte-order | {"byteOrder": "big-endian"} |
      | insert-ifd | {"index": 2, "ifd": {"entries": [{"tag":256,"type":4,"values":[8]},{"tag":257,"type":4,"values":[8]},{"tag":258,"type":3,"values":[8,8,8]},{"tag":259,"type":3,"values":[1]},{"tag":262,"type":3,"values":[2]},{"tag":277,"type":3,"values":[3]}], "pixels": "fefefefefefefefefefefefefefefef9f7f7f9f7f7fefefefefefefefefefefefefefefefcfbfbfaf7f7fbf7f7fbfafafefefefefefefefefefefefef7f4f4f8f3f3f9f6f6fefefefefefefefefefefefefbf9f9faf6f6faf7f7fdfdfdfefefefefefefefefefefefefaf7f7f8f8f8f8f6f6fbfafafefefefbf9f9fbf8f8fbf8f8f8f6f6fbfbfbfbfafaf9f6f6fdfcfcf9f6f6f9f5f5f8f4f4faf7f7f8f6f6faf8f8f8f4f4f8f5f5fcfcfcfbfafafbfafafbfafafcfbfbfcfbfbf8f4f4f9f6f6"}} |
      | remove-ifd | {"index": 1} |
      | set-tag | {"ifdIndex": 0, "tag": 315, "type": 2, "values": ["Derived for ticket 26/08/23/END-TO-END-TESTING-REFACTOR"]} |
      | remove-tag | {"ifdIndex": 0, "tag": 282} |
      | set-pixels | {"pixelsFixture": "local://🔄️flipped-scan.rgba"} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🖼️abbau-aufbau-masterarbeit-grundriss.tiff
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And its inverse is applied
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"byteOrder": "little-endian", "ifds": [{"entries": [{"tag":256,"type":4,"values":[8]},{"tag":257,"type":4,"values":[8]},{"tag":258,"type":3,"values":[8,8,8]},{"tag":259,"type":3,"values":[1]},{"tag":262,"type":3,"values":[2]},{"tag":277,"type":3,"values":[3]}], "pixels": "fefefefefefefefefefefefefefefef9f7f7f9f7f7fefefefefefefefefefefefefefefefcfbfbfaf7f7fbf7f7fbfafafefefefefefefefefefefefef7f4f4f8f3f3f9f6f6fefefefefefefefefefefefefbf9f9faf6f6faf7f7fdfdfdfefefefefefefefefefefefefaf7f7f8f8f8f8f6f6fbfafafefefefbf9f9fbf8f8fbf8f8f8f6f6fbfbfbfbfafaf9f6f6fdfcfcf9f6f6f9f5f5f8f4f4faf7f7f8f6f6faf8f8f8f4f4f8f5f5fcfcfcfbfafafbfafafbfafafcfbfbfcfbfbf8f4f4f9f6f6"}]} |
      | set-byte-order | {"byteOrder": "big-endian"} |
      | insert-ifd | {"index": 2, "ifd": {"entries": [{"tag":256,"type":4,"values":[8]},{"tag":257,"type":4,"values":[8]},{"tag":258,"type":3,"values":[8,8,8]},{"tag":259,"type":3,"values":[1]},{"tag":262,"type":3,"values":[2]},{"tag":277,"type":3,"values":[3]}], "pixels": "fefefefefefefefefefefefefefefef9f7f7f9f7f7fefefefefefefefefefefefefefefefcfbfbfaf7f7fbf7f7fbfafafefefefefefefefefefefefef7f4f4f8f3f3f9f6f6fefefefefefefefefefefefefbf9f9faf6f6faf7f7fdfdfdfefefefefefefefefefefefefaf7f7f8f8f8f8f6f6fbfafafefefefbf9f9fbf8f8fbf8f8f8f6f6fbfbfbfbfafaf9f6f6fdfcfcf9f6f6f9f5f5f8f4f4faf7f7f8f6f6faf8f8f8f4f4f8f5f5fcfcfcfbfafafbfafafbfafafcfbfbfcfbfbf8f4f4f9f6f6"}} |
      | remove-ifd | {"index": 1} |
      | set-tag | {"ifdIndex": 0, "tag": 315, "type": 2, "values": ["Derived for ticket 26/08/23/END-TO-END-TESTING-REFACTOR"]} |
      | remove-tag | {"ifdIndex": 0, "tag": 282} |
      | set-pixels | {"pixelsFixture": "local://🔄️flipped-scan.rgba"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🖼️abbau-aufbau-masterarbeit-grundriss.tiff
    When the document is decoded and re-encoded with no mutation
    Then the oracle and the subject agree on the semantic projection
