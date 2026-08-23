@capability-bmp-3-mutate
@oracle-image-bmp-3-mutate
@comparison-semantic-raster-v1
@mutations-bmp-3-any
Feature: Apply every typed BMP v3 mutation to a real-world document
  The input is a real 250 KB, 2334x2560, 8-bit palette architectural floor plan
  (rathaus-ahlen-grundriss), not a synthetic gradient. It is derived ONCE from
  🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png (PNG IHDR
  color type 3/PLTE indexed, 233-entry palette): the independent `png` 0.18 decoder recovers its
  genuine index buffer and palette table, and the `image` 0.25 reference encoder's palette-aware
  `BmpEncoder::encode_with_palette` writes them back as an 8-bit indexed BITMAPINFOHEADER BMP v3 —
  so the committed fixture genuinely exercises BMP's palette decode path rather than being
  downgraded to 24-bit RGB. That derivation is a one-off (not a test step); the committed result,
  shared://🖼️rathaus-ahlen-grundriss.bmp, is what every scenario below reads. Each scenario copies it
  into the case work directory before touching it; the committed document is never written to.

  The oracle applies each mutation independently against the registered `image` reference crate's
  own decode/encode API; the subject fully parses the artifact into the typed `BmpSnapshot` and
  re-serializes from it. Both results are read back by the INDEPENDENT `image` decoder before the
  `semantic-raster-v1` profile compares dimensions and a digest of the decoded samples — BMP is
  lossless, so exact sample equality is legitimate, and a digest avoids inlining the real fixture's
  ~24 million decoded bytes as JSON.

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
      | no-mutation            | {}                                                               |
      | set-snapshot           | {"width":3,"height":2,"fill":[64,128,192,255]}                  |
      | set-header-fields      | {"row_order":"top-down"}                                         |
      | insert-palette-entry   | {"index":0,"entry":{"b":10,"g":20,"r":30,"reserved":0}}          |
      | remove-palette-entry   | {"index":0}                                                      |
      | set-palette-entry      | {"index":0,"entry":{"b":1,"g":2,"r":3,"reserved":0}}             |
      | set-pixel-data         | {"fill":[200,40,40,255]}                                         |

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
      | no-mutation            | {}                                                               |
      | set-snapshot           | {"width":3,"height":2,"fill":[64,128,192,255]}                  |
      | set-header-fields      | {"row_order":"top-down"}                                         |
      | insert-palette-entry   | {"index":0,"entry":{"b":10,"g":20,"r":30,"reserved":0}}          |
      | remove-palette-entry   | {"index":0}                                                      |
      | set-palette-entry      | {"index":0,"entry":{"b":1,"g":2,"r":3,"reserved":0}}             |
      | set-pixel-data         | {"fill":[200,40,40,255]}                                         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🖼️rathaus-ahlen-grundriss.bmp
    When the document is decoded, printed through the DSL text codec, reparsed and re-encoded
    Then the output is not a byte-for-byte copy of the input
    And the oracle and the subject agree on the semantic projection
