@capability-png-1-2-mutate
@oracle-png-png-1-2-mutate
@comparison-semantic-raster-v1
@mutations-png-1-2-any
Feature: Apply every typed PNG 1.2 mutation to a real-world document
  The input is a real 250 KB, 2334x2560, 8-bit COLORMAP architectural floor plan
  (rathaus-ahlen-grundriss.png, IHDR color type 3/PLTE indexed, 233-entry palette, no ancillary
  chunks), not a synthetic fixture — it exercises the PLTE/palette decode path, not just RGBA.
  Every scenario copies it into the case work directory before touching it; the committed document
  is never written to. The oracle applies each mutation independently against the registered `png`
  reference crate's own Encoder/Decoder API; the subject fully parses the artifact into the typed
  `PngSnapshot` and re-serializes from it. Both results are read back by the INDEPENDENT `png`
  decoder before the `semantic-raster-v1` profile compares dimensions and decoded samples.

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
      | set-header            | {"width":2334,"height":2560,"bitDepth":8,"colorType":"grayscale","interlace":true}                                          |
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
      | set-text-chunk        | {"index":0,"keyword":"Comment","value":"ignored no-op"}                                                                     |
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
      | set-header            | {"width":2334,"height":2560,"bitDepth":8,"colorType":"grayscale","interlace":true}                                          |
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
      | set-text-chunk        | {"index":0,"keyword":"Comment","value":"ignored no-op"}                                                                     |
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
