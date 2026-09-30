@capability-png-1-2-mutate
@oracle-png-png-1-2-mutate-reader
@comparison-semantic-raster-v1
@mutations-png-1-2-any
Feature: Apply every typed PNG 1.2 mutation to a real-world document
  The input is a real 250 KB, 2334x2560, 8-bit COLORMAP architectural floor plan
  (rathaus-ahlen-grundriss.png), not a synthetic fixture — it exercises the PLTE/palette decode
  path, not just RGBA. Walking its chunk chain gives exactly IHDR (colour type 3, bit depth 8,
  non-interlaced), PLTE (233 entries), eight IDAT chunks and IEND: no tRNS, no gAMA/cHRM/sRGB/pHYs,
  no tIME, no bKGD, no text chunk, no private chunk. Every scenario copies it into the case work
  directory before touching it; the committed document is never written to.

  Three kinds address an EXISTING text or unknown chunk, and the real document carries neither, so
  remove-text-chunk, replace-text-chunk and remove-unknown-chunk are exercised on the real document
  after the reference implementation has inserted their target first — the same arrange step the
  OOXML conformance cases use for their own removal kinds. Anything else would be a row whose
  parameters address nothing, which passes without testing anything.

  The oracle applies each mutation independently against the registered `png` reference crate's own
  Encoder/Decoder API; the subject fully parses the artifact into the typed `PngSnapshot` and
  re-serializes from it. Both results are read back by the INDEPENDENT `png` decoder. The compared
  projection is the WHOLE document, not just its raster: geometry and a digest of the decoded RGBA
  samples (PNG is lossless, so a digest is an exact claim), plus the palette, the five typed
  ancillary chunks, the timestamp, the background colour, the text chunks by keyword and value, and
  the private chunks by type and payload digest. tIME and private chunks come from a fixed-grammar
  walk over §5.3's chunk chain, because `png::Info` models neither.

  Every Examples `params` cell is exactly the leaf's wire payload — its `payload_value()`, camelCase,
  `Option`s as `null`, no aggregate tag — decoded by the subject through the derive-generated
  `from_payload_value` and read by the `png` oracle by the same field names; the oracle refuses a
  member it cannot write (a zTXt/iTXt or compressed text chunk, a non-RGB bKGD) rather than
  approximating it. `replace-pixels` carries the whole replacement RGBA raster, which for this
  2334x2560 plan would be 23.9 million numbers in one cell, so it runs in its own outlines on the
  committed 4x2 COLORMAP document (`shared://🎨️replace-palette-applied/⬅️before.png`) — the same
  PLTE decode path as the real plan.

  ⚠️ Two of the seventeen kinds genuinely cannot reach the bytes, and the case says so rather than
  letting them pass as though they had:
    – change-header — IHDR must describe the IDAT that follows it, and both encoders always write
      colour type 6 / bit depth 8 / interlace 0 because `PngSnapshot.pixels` is a canonical RGBA
      buffer (`encode_png`'s own 🚫️EncodeScopeNote). `SetHeader` also does not resize `pixels`, so
      changing width or height would only make the snapshot unencodable. Every field of this kind
      is model-only.
    – change-transparency — §11.3.3 forbids tRNS alongside colour types 4 and 6, so at the colour type
      both encoders write, the chunk can never appear. `encode_png` used to emit it anyway from the
      snapshot, producing a file the reference decoder rejects outright (`ColorWithBadTrns`); it now
      omits it, with the source's alpha already resolved into `pixels`.
  Both are named in the adapter's observability exemption list. Every other kind must move the
  projection, and the oracle fails the scenario if it does not.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.png
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | change-header | {"width":2334,"height":2560,"bitDepth":16,"colorType":"grayscale","interlace":true} |
      | replace-palette | {"plte":[{"r":255,"g":0,"b":0},{"r":0,"g":255,"b":0},{"r":0,"g":0,"b":255},{"r":255,"g":255,"b":0}]} |
      | change-transparency | {"trns":null} |
      | change-gamma | {"gama":45455} |
      | change-chromaticities | {"chrm":{"whiteX":31270,"whiteY":32900,"redX":64000,"redY":33000,"greenX":30000,"greenY":60000,"blueX":15000,"blueY":6000}} |
      | change-srgb-intent | {"srgb":"perceptual"} |
      | change-physical-dims | {"phys":{"ppuX":2835,"ppuY":2835,"unitIsMeter":true}} |
      | change-timestamp | {"time":{"year":2024,"month":1,"day":2,"hour":3,"minute":4,"second":5}} |
      | change-background | {"bkgd":{"colorType":"rgb","r":255,"g":255,"b":255}} |
      | insert-text-chunk | {"index":0,"chunk":{"keyword":"Comment","value":"Wave 7 oracle probe","compressed":false,"kind":"text","languageTag":"","translatedKeyword":""}} |
      | remove-text-chunk | {"index":0} |
      | replace-text-chunk | {"index":0,"chunk":{"keyword":"Author","value":"replaces the arranged chunk outright","compressed":false,"kind":"text","languageTag":"","translatedKeyword":""}} |
      | insert-unknown-chunk | {"index":0,"chunk":{"kind":[119,97,86,101],"data":[119,97,118,101,55,45,112,114,111,98,101]}} |
      | remove-unknown-chunk | {"index":0} |

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to a small palette document
    Given the small palette input document shared://🎨️replace-palette-applied/⬅️before.png
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | replace-pixels | {"pixels":[200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.png
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own algebraic inverse is applied next
    Then the oracle and the subject agree on the semantic projection
    And that projection matches the untouched original document
    Examples:
      | id | params |
      | change-header | {"width":2334,"height":2560,"bitDepth":16,"colorType":"grayscale","interlace":true} |
      | replace-palette | {"plte":[{"r":255,"g":0,"b":0},{"r":0,"g":255,"b":0},{"r":0,"g":0,"b":255},{"r":255,"g":255,"b":0}]} |
      | change-transparency | {"trns":null} |
      | change-gamma | {"gama":45455} |
      | change-chromaticities | {"chrm":{"whiteX":31270,"whiteY":32900,"redX":64000,"redY":33000,"greenX":30000,"greenY":60000,"blueX":15000,"blueY":6000}} |
      | change-srgb-intent | {"srgb":"perceptual"} |
      | change-physical-dims | {"phys":{"ppuX":2835,"ppuY":2835,"unitIsMeter":true}} |
      | change-timestamp | {"time":{"year":2024,"month":1,"day":2,"hour":3,"minute":4,"second":5}} |
      | change-background | {"bkgd":{"colorType":"rgb","r":255,"g":255,"b":255}} |
      | insert-text-chunk | {"index":0,"chunk":{"keyword":"Comment","value":"Wave 7 oracle probe","compressed":false,"kind":"text","languageTag":"","translatedKeyword":""}} |
      | remove-text-chunk | {"index":0} |
      | replace-text-chunk | {"index":0,"chunk":{"keyword":"Author","value":"replaces the arranged chunk outright","compressed":false,"kind":"text","languageTag":"","translatedKeyword":""}} |
      | insert-unknown-chunk | {"index":0,"chunk":{"kind":[119,97,86,101],"data":[119,97,118,101,55,45,112,114,111,98,101]}} |
      | remove-unknown-chunk | {"index":0} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores a small palette document
    Given the small palette input document shared://🎨️replace-palette-applied/⬅️before.png
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own algebraic inverse is applied next
    Then the oracle and the subject agree on the semantic projection
    And that projection matches the untouched original document
    Examples:
      | id | params |
      | replace-pixels | {"pixels":[200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255,200,40,40,255]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.png
    When the document is decoded, printed to the text codec, reparsed and re-encoded
    Then the output is not a byte-for-byte copy of the input
    And the oracle and the subject agree on the semantic projection
