@capability-bmp-3-mutate
@oracle-image-bmp-3-mutate-reader
@comparison-semantic-raster-v1
@mutations-bmp-3-any
Feature: Apply every typed BMP v3 mutation to a real-world document
  The input is a real 2334x2560, 8-bit indexed architectural floor plan (rathaus-ahlen-grundriss),
  not a synthetic gradient. It is derived ONCE from
  🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png (PNG IHDR
  color type 3/PLTE indexed, 233-entry palette): the independent `png` 0.18 decoder recovers its
  genuine index buffer and palette table, and the `image` 0.25 reference encoder's palette-aware
  `BmpEncoder::encode_with_palette` writes them back as an 8-bit indexed BITMAPINFOHEADER BMP v3 —
  so the committed fixture genuinely exercises BMP's palette path rather than being downgraded to
  24-bit RGB. That derivation is a one-off (not a test step, `#[ignore]`d in the subset's own oracle
  module); the committed result, shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp, is what every scenario below
  reads. Each scenario copies it into the case work directory before touching it; the committed
  document is never written to.

  The derivation pads the colour table from the PNG's 233 real entries to 240. That is not
  decoration — it is what makes three of the seven kinds expressible at all. `BmpSnapshot.pixels`
  holds palette-RESOLVED RGBA and `palette` is an independent field, so this subset's semantics for
  a palette edit are "change the colour table, leave the picture alone", and `encode_bmp` re-indexes
  on the way out and reports an Err rather than narrowing when a pixel's colour no longer has an
  entry. All 233 real colours are referenced — index 0 alone covers 5,659,668 of the image's
  5,975,040 pixels — so an edit to any of them is unencodable; seven spare entries no pixel resolves
  to are what give replace-palette-entry and remove-palette-entry a legal target. A full 256-entry table
  (what most real 8-bpp BMP writers emit) would give that slack and then make insert-palette-entry
  unrepresentable, because 257 entries exceed what an 8-bit index can address. 240 leaves room for
  both. The rows below address entry 239 and append at 240, inside that spare range.

  ⚠️ The previous revision of this case claimed index 0 was "a palette entry no pixel actually
  resolves to" and targeted all three palette rows at it. Index 0 is the single most referenced
  entry in the image; the oracle returned the document unchanged for all three kinds, so nothing
  ever noticed.

  The oracle applies each mutation independently against the registered `image` reference crate,
  keeping the INDEXED layer intact — `BmpDecoder::set_indexed_color` for the index buffer,
  `get_palette` for the table, `encode_with_palette` to write both back — rather than resolving to
  RGBA and losing the half three kinds operate on. `image` neither reads nor writes the row order or
  the two pixels-per-metre fields (its encoder hard-codes both to 0 and always stores bottom-up), so
  the oracle patches those onto its output at their fixed BMP v3 offsets, which is also why
  change-header-fields is a real mutation here and not an accepted no-op.

  The subject fully parses the artifact into the typed `BmpSnapshot` and re-serializes from it. Both
  results are read back by the INDEPENDENT `image` decoder before the `semantic-raster-v1` profile
  compares geometry, row order, both pixels-per-metre fields, the colour table's length and digest,
  the raw index buffer's digest and the resolved samples' digest. BMP is lossless, so every one of
  those is an exact claim; the digests exist only so the comparison engine is not diffing ~24
  million JSON numbers per scenario.

  The identity round trip asserts EXACT bytes rather than "the bytes moved", and this is the one
  carrier in the fleet where that is the correct law rather than the suspicious one. An uncompressed
  BMP v3 leaves a writer nothing to choose: a 14-byte BITMAPFILEHEADER and a 40-byte
  BITMAPINFOHEADER whose every field is determined by the image, a colour table that is the palette
  verbatim, and a pixel array that is the index buffer padded to a 4-byte row stride. No filters, no
  compression level, no chunk order. The committed fixture was additionally authored by the
  reference encoder itself, so a byte that moves is a defect in a codec rather than freedom being
  exercised. What rules out a read/write shortcut is structural instead of assertional: on the
  subject side the ONLY channel from input to output is decode_bmp → the DSL text codec → parse_dsl
  → encode_bmp, so a byte that survives did so by being modelled.

  Every Examples `params` cell is exactly the leaf's wire payload — its `payload_value()`, camelCase,
  no aggregate tag — decoded by the subject through the derive-generated `from_payload_value` and
  read by the `image` oracle by the same field names. `replace-pixel-data`'s payload is the whole
  replacement RGBA raster, which for this 2334x2560 plan would be 23.9 million numbers in one cell,
  so that kind runs in its own outlines on the committed 4x4, 7-entry indexed document
  (`shared://🎨️replace-palette-entry-applied/⬅️before.bmp`) — the same palette storage the real
  plan has, so the row still lands on an indexed document.

  ✅ DECIDED — what `replace-pixel-data` does to an indexed BMP. The BITMAPINFOHEADER rules decide it: a
  `BI_RGB` bitmap of 8 bpp or less "has a color table immediately following the BITMAPINFOHEADER", and
  each of its pixels is an index into that table, while a 24-bit `BI_RGB` bitmap has no colour table and
  stores every pixel's RGB triplet itself
  (https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader, "Color
  Tables" and `biBitCount`). So the replacement raster stays indexed exactly when every one of its
  colours has a table entry (each pixel re-indexed to the first entry of its colour), and otherwise the
  document becomes 24-bit `BI_RGB` with no colour table, `biClrUsed`/`biClrImportant` 0 and
  `biSizeImage` the 24-bit stride times the height — the smallest storage change that holds the raster
  losslessly, never a narrowing onto the table. Its inverse is then a full `set-snapshot` of the
  indexed original. The subject's `ReplacePixelDataMutation` and the `image` oracle implement that one
  rule; the row below fills the 7-entry document with rgb(200,40,40), which the table lacks, so both
  sides land on `storage: direct`. A raster of the wrong length is refused (`mutation.target-mismatch`).

  `set-snapshot` installs this subset's own committed 2x2 indexed swatch (its `set-snapshot` wire
  witness); both sides keep it indexed, because every one of its pixel colours is an entry of its
  two-colour table.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | change-header-fields | {"rowOrder":"topDown","xPixelsPerMeter":2835,"yPixelsPerMeter":2835} |
      | insert-palette-entry | {"index":240,"entry":{"b":10,"g":20,"r":30,"reserved":0}} |
      | remove-palette-entry | {"index":239} |
      | replace-palette-entry | {"index":239,"entry":{"b":1,"g":2,"r":3,"reserved":0}} |
      | set-snapshot | {"snapshot":{"bitsPerPixel":8,"colorsImportant":2,"colorsUsed":2,"compression":0,"headerSize":40,"height":2,"imageSize":16,"palette":[{"b":0,"g":0,"r":255,"reserved":0},{"b":0,"g":255,"r":0,"reserved":0}],"pixels":[255,0,0,255,0,255,0,255,0,255,0,255,255,0,0,255],"planes":1,"rowOrder":"bottomUp","schema":"stdio.bmp","width":2,"xPixelsPerMeter":2835,"yPixelsPerMeter":2835}} |

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to a small indexed document
    Given the small indexed input document shared://🎨️replace-palette-entry-applied/⬅️before.bmp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | replace-pixel-data | {"pixels":[200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own algebraic inverse is applied next
    Then the oracle and the subject agree on the semantic projection
    And that projection matches the untouched original document
    Examples:
      | id | params |
      | change-header-fields | {"rowOrder":"topDown","xPixelsPerMeter":2835,"yPixelsPerMeter":2835} |
      | insert-palette-entry | {"index":240,"entry":{"b":10,"g":20,"r":30,"reserved":0}} |
      | remove-palette-entry | {"index":239} |
      | replace-palette-entry | {"index":239,"entry":{"b":1,"g":2,"r":3,"reserved":0}} |
      | set-snapshot | {"snapshot":{"bitsPerPixel":8,"colorsImportant":2,"colorsUsed":2,"compression":0,"headerSize":40,"height":2,"imageSize":16,"palette":[{"b":0,"g":0,"r":255,"reserved":0},{"b":0,"g":255,"r":0,"reserved":0}],"pixels":[255,0,0,255,0,255,0,255,0,255,0,255,255,0,0,255],"planes":1,"rowOrder":"bottomUp","schema":"stdio.bmp","width":2,"xPixelsPerMeter":2835,"yPixelsPerMeter":2835}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores a small indexed document
    Given the small indexed input document shared://🎨️replace-palette-entry-applied/⬅️before.bmp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own algebraic inverse is applied next
    Then the oracle and the subject agree on the semantic projection
    And that projection matches the untouched original document
    Examples:
      | id | params |
      | replace-pixel-data | {"pixels":[200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document, reproducing it exactly
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp
    When the document is decoded, printed through the DSL text codec, reparsed and re-encoded
    Then the output reproduces the input byte for byte
    And the oracle and the subject agree on the semantic projection
