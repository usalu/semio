@capability-png-1-2-mutate
@oracle-png-png-1-2-mutate
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
  remove-text-chunk, set-text-chunk and remove-unknown-chunk are exercised on the real document
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

  ⚠️ Two of the seventeen kinds genuinely cannot reach the bytes, and the case says so rather than
  letting them pass as though they had:
    – set-header — IHDR must describe the IDAT that follows it, and both encoders always write
      colour type 6 / bit depth 8 / interlace 0 because `PngSnapshot.pixels` is a canonical RGBA
      buffer (`encode_png`'s own 🚫️EncodeScopeNote). `SetHeader` also does not resize `pixels`, so
      changing width or height would only make the snapshot unencodable. Every field of this kind
      is model-only.
    – set-transparency — §11.3.3 forbids tRNS alongside colour types 4 and 6, so at the colour type
      both encoders write, the chunk can never appear. `encode_png` used to emit it anyway from the
      snapshot, producing a file the reference decoder rejects outright (`ColorWithBadTrns`); it now
      omits it, with the source's alpha already resolved into `pixels`.
  Both are named in the adapter's observability exemption list. Every other kind must move the
  projection, and the oracle fails the scenario if it does not.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🖼️rathaus-ahlen-grundriss.png
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                    | params                                                                                                                       |
      | no-mutation           | {}                                                                                                                           |
      | set-snapshot          | {"width":3,"height":2,"fill":[64,128,192,255]}                                                                              |
      | set-header            | {"width":2334,"height":2560,"bitDepth":16,"colorType":"grayscale","interlace":true}                                         |
      | set-palette           | {"plte":[[255,0,0],[0,255,0],[0,0,255],[255,255,0]]}                                                                        |
      | set-transparency      | {"trns":null}                                                                                                                |
      | set-gamma             | {"gama":45455}                                                                                                               |
      | set-chromaticities    | {"whiteX":31270,"whiteY":32900,"redX":64000,"redY":33000,"greenX":30000,"greenY":60000,"blueX":15000,"blueY":6000}          |
      | set-srgb-intent       | {"srgb":"perceptual"}                                                                                                        |
      | set-physical-dims     | {"ppuX":2835,"ppuY":2835,"unitIsMeter":true}                                                                                 |
      | set-timestamp         | {"year":2024,"month":1,"day":2,"hour":3,"minute":4,"second":5}                                                              |
      | set-background        | {"r":255,"g":255,"b":255}                                                                                                    |
      | insert-text-chunk     | {"index":0,"keyword":"Comment","value":"Wave 7 oracle probe"}                                                               |
      | remove-text-chunk     | {"index":0}                                                                                                                  |
      | set-text-chunk        | {"index":0,"keyword":"Author","value":"replaces the arranged chunk outright"}                                               |
      | set-pixels            | {"fill":[200,40,40,255]}                                                                                                     |
      | insert-unknown-chunk  | {"index":0,"kind":"waVe","data":"wave7-probe"}                                                                              |
      | remove-unknown-chunk  | {"index":0}                                                                                                                  |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🖼️rathaus-ahlen-grundriss.png
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own algebraic inverse is applied next
    Then the oracle and the subject agree on the semantic projection
    And that projection matches the untouched original document
    Examples:
      | id                    | params                                                                                                                       |
      | no-mutation           | {}                                                                                                                           |
      | set-snapshot          | {"width":3,"height":2,"fill":[64,128,192,255]}                                                                              |
      | set-header            | {"width":2334,"height":2560,"bitDepth":16,"colorType":"grayscale","interlace":true}                                         |
      | set-palette           | {"plte":[[255,0,0],[0,255,0],[0,0,255],[255,255,0]]}                                                                        |
      | set-transparency      | {"trns":null}                                                                                                                |
      | set-gamma             | {"gama":45455}                                                                                                               |
      | set-chromaticities    | {"whiteX":31270,"whiteY":32900,"redX":64000,"redY":33000,"greenX":30000,"greenY":60000,"blueX":15000,"blueY":6000}          |
      | set-srgb-intent       | {"srgb":"perceptual"}                                                                                                        |
      | set-physical-dims     | {"ppuX":2835,"ppuY":2835,"unitIsMeter":true}                                                                                 |
      | set-timestamp         | {"year":2024,"month":1,"day":2,"hour":3,"minute":4,"second":5}                                                              |
      | set-background        | {"r":255,"g":255,"b":255}                                                                                                    |
      | insert-text-chunk     | {"index":0,"keyword":"Comment","value":"Wave 7 oracle probe"}                                                               |
      | remove-text-chunk     | {"index":0}                                                                                                                  |
      | set-text-chunk        | {"index":0,"keyword":"Author","value":"replaces the arranged chunk outright"}                                               |
      | set-pixels            | {"fill":[200,40,40,255]}                                                                                                     |
      | insert-unknown-chunk  | {"index":0,"kind":"waVe","data":"wave7-probe"}                                                                              |
      | remove-unknown-chunk  | {"index":0}                                                                                                                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🖼️rathaus-ahlen-grundriss.png
    When the document is decoded, printed to the text codec, reparsed and re-encoded
    Then the output is not a byte-for-byte copy of the input
    And the oracle and the subject agree on the semantic projection
