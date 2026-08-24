@capability-bmp-3-mutate
@oracle-image-bmp-3-mutate
@comparison-semantic-raster-v1
@mutations-bmp-3-any
Feature: Apply every typed BMP v3 mutation to a real-world document
  The input is a real 2334x2560, 8-bit indexed architectural floor plan (rathaus-ahlen-grundriss),
  not a synthetic gradient. It is derived ONCE from
  🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png (PNG IHDR
  color type 3/PLTE indexed, 233-entry palette): the independent `png` 0.18 decoder recovers its
  genuine index buffer and palette table, and the `image` 0.25 reference encoder's palette-aware
  `BmpEncoder::encode_with_palette` writes them back as an 8-bit indexed BITMAPINFOHEADER BMP v3 —
  so the committed fixture genuinely exercises BMP's palette path rather than being downgraded to
  24-bit RGB. That derivation is a one-off (not a test step, `#[ignore]`d in the subset's own oracle
  module); the committed result, shared://🖼️rathaus-ahlen-grundriss.bmp, is what every scenario below
  reads. Each scenario copies it into the case work directory before touching it; the committed
  document is never written to.

  The derivation pads the colour table from the PNG's 233 real entries to 240. That is not
  decoration — it is what makes three of the seven kinds expressible at all. `BmpSnapshot.pixels`
  holds palette-RESOLVED RGBA and `palette` is an independent field, so this subset's semantics for
  a palette edit are "change the colour table, leave the picture alone", and `encode_bmp` re-indexes
  on the way out and reports an Err rather than narrowing when a pixel's colour no longer has an
  entry. All 233 real colours are referenced — index 0 alone covers 5,659,668 of the image's
  5,975,040 pixels — so an edit to any of them is unencodable; seven spare entries no pixel resolves
  to are what give set-palette-entry and remove-palette-entry a legal target. A full 256-entry table
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
  set-header-fields is a real mutation here and not an accepted no-op.

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

  ⚠️ KNOWN OPEN DIVERGENCE — `mutate-set-pixel-data` (parity 14/15, 2026-08-24). The row fills the
  whole raster with rgb(200,40,40), a colour the committed 240-entry table has no entry for. The
  oracle answers by switching the document to 24-bit direct colour (`storage: direct`,
  `paletteEntries: 0`); `encode_bmp` answers with an Err, and
  `unrepresentable_palette_edit_is_reported_not_narrowed`
  (`../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️component.rs`) pins that refusal as deliberate,
  because the same snapshot shape is also what a palette edit that orphans a real pixel produces —
  and there a silent 24-bit fallback WOULD hide the edit. The declared kind ("Replaces the whole
  decoded canonical-RGBA `pixels` buffer") says nothing about storage, so both sides are
  extrapolating from an under-specified verb. Resolving it means saying what `set-pixel-data` means
  for an indexed BMP — most likely that it moves the document to direct colour, which then also
  makes its inverse a full `set-snapshot` rather than a second `set-pixel-data` — and making both
  sides implement that. Do not weaken the profile, the row's parameters or the fixture to close it.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🖼️rathaus-ahlen-grundriss.bmp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                            |
      | no-mutation            | {}                                                                                 |
      | set-snapshot           | {"width":3,"height":2,"fill":[64,128,192,255]}                                    |
      | set-header-fields      | {"rowOrder":"top-down","xPixelsPerMeter":2835,"yPixelsPerMeter":2835}              |
      | insert-palette-entry   | {"index":240,"entry":{"b":10,"g":20,"r":30,"reserved":0}}                         |
      | remove-palette-entry   | {"index":239}                                                                     |
      | set-palette-entry      | {"index":239,"entry":{"b":1,"g":2,"r":3,"reserved":0}}                            |
      | set-pixel-data         | {"fill":[200,40,40,255]}                                                          |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🖼️rathaus-ahlen-grundriss.bmp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own algebraic inverse is applied next
    Then the oracle and the subject agree on the semantic projection
    And that projection matches the untouched original document
    Examples:
      | id                    | params                                                            |
      | no-mutation            | {}                                                                                 |
      | set-snapshot           | {"width":3,"height":2,"fill":[64,128,192,255]}                                    |
      | set-header-fields      | {"rowOrder":"top-down","xPixelsPerMeter":2835,"yPixelsPerMeter":2835}              |
      | insert-palette-entry   | {"index":240,"entry":{"b":10,"g":20,"r":30,"reserved":0}}                         |
      | remove-palette-entry   | {"index":239}                                                                     |
      | set-palette-entry      | {"index":239,"entry":{"b":1,"g":2,"r":3,"reserved":0}}                            |
      | set-pixel-data         | {"fill":[200,40,40,255]}                                                          |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document, reproducing it exactly
    Given the real input document shared://🖼️rathaus-ahlen-grundriss.bmp
    When the document is decoded, printed through the DSL text codec, reparsed and re-encoded
    Then the output reproduces the input byte for byte
    And the oracle and the subject agree on the semantic projection
