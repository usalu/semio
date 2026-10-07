@capability-png-1-2-mutate
@oracle-png-png-1-2-mutate-reader
@comparison-semantic-png-owned-v1
@mutations-png-1-2-any
Feature: Apply every typed PNG 1.2 mutation to a real-world document
  The real input is a 250 KB, 2334x2560, 8-bit COLORMAP architectural floor plan
  (rathaus-ahlen-grundriss.png) whose chunk chain is exactly IHDR (colour type 3, bit depth 8,
  non-interlaced), PLTE (233 entries), eight IDAT chunks and IEND. Every scenario copies its input
  into the case work directory; the committed documents are never written to.

  `PngSnapshot` owns exact native samples, precision, color profile and metadata. Its five mutation
  kinds edit those values: `set-snapshot`, `patch-snapshot`, `change-gamma`, `patch-pixels` and
  `paint-native-samples`. Guarded kinds name the structural revision of the owned image. Native
  compression, filtering and chunk materialization belong to I/O. Every inverse restores the owned base.

  The rows: on the real document, `change-gamma` inserts gAMA 45455 before PLTE, `set-snapshot`
  installs the committed 2x2 RGBA swatch, and `patch-snapshot` sets its owned gamma value. On the
  committed 4x2 COLORMAP document (palette black, red,
  green, blue; both rows index 0, 1, 2, 3), `paint-native-samples` paints palette index 3 into the 2x2
  rectangle at (1, 0), its `result` carrying the painted exact index samples. On the committed
  2x2 RGBA swatch, `patch-pixels` paints rgba(200, 40, 40, 255) into the pixel at (1, 1).

  The oracle performs every kind independently with the registered `png` reference crate's own
  Encoder/Decoder API, recomputing the revision guard and the paint from their definitions, and
  never reuses this repository's codec. Both results are read back by the INDEPENDENT `png` decoder;
  the compared projection includes exact native sample digest and count, source precision/profile,
  palette, transparency, all owned metadata, ordered text and unknown ancillary chunks.

  Every Examples `params` cell is exactly the leaf's wire payload — camelCase, `Option`s as `null`,
  no aggregate tag — decoded by the subject through the derive-generated `from_payload_value` and
  read by the `png` oracle by the same field names.

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
      | change-gamma | {"revision":"8a303b603d1fa1ff","gama":45455} |
      | set-snapshot | {"snapshot":{"schema":"stdio.png","image":{"width":2,"height":2,"bitDepth":8,"colorType":"rgba","interlace":false,"samples":[255,0,0,255,0,255,0,255,0,0,255,255,255,255,255,128],"palette":null,"transparency":null,"gamma":null,"chromaticities":null,"srgb":null,"physicalDims":null,"timestamp":null,"background":null,"textChunks":[],"ancillaryChunks":[]}}} |
      | patch-snapshot | {"patch":{"operation":"set","path":"/image/gamma","value":45455}} |

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
      | paint-native-samples | {"revision":"7dc7413fbf2c0731","region":{"x":1,"y":0,"width":2,"height":2},"paint":{"profile":"indexed","first":3,"second":0,"third":0,"fourth":0},"result":{"schema":"stdio.png","image":{"width":4,"height":2,"bitDepth":8,"colorType":"palette","interlace":false,"samples":[0,3,3,3,0,3,3,3],"palette":[{"r":0,"g":0,"b":0},{"r":255,"g":0,"b":0},{"r":0,"g":255,"b":0},{"r":0,"g":0,"b":255}],"transparency":null,"gamma":null,"chromaticities":null,"srgb":null,"physicalDims":null,"timestamp":null,"background":null,"textChunks":[],"ancillaryChunks":[]}}} |

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to a small RGBA document
    Given the small RGBA input document shared://🟥️rgba8-swatch/🖼️.png
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | patch-pixels | {"revision":"2989b8e438c3fb2d","x":1,"y":1,"width":1,"height":1,"red":200,"green":40,"blue":40,"alpha":255} |

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
      | change-gamma | {"revision":"8a303b603d1fa1ff","gama":45455} |
      | set-snapshot | {"snapshot":{"schema":"stdio.png","image":{"width":2,"height":2,"bitDepth":8,"colorType":"rgba","interlace":false,"samples":[255,0,0,255,0,255,0,255,0,0,255,255,255,255,255,128],"palette":null,"transparency":null,"gamma":null,"chromaticities":null,"srgb":null,"physicalDims":null,"timestamp":null,"background":null,"textChunks":[],"ancillaryChunks":[]}}} |
      | patch-snapshot | {"patch":{"operation":"set","path":"/image/gamma","value":45455}} |

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
      | paint-native-samples | {"revision":"7dc7413fbf2c0731","region":{"x":1,"y":0,"width":2,"height":2},"paint":{"profile":"indexed","first":3,"second":0,"third":0,"fourth":0},"result":{"schema":"stdio.png","image":{"width":4,"height":2,"bitDepth":8,"colorType":"palette","interlace":false,"samples":[0,3,3,3,0,3,3,3],"palette":[{"r":0,"g":0,"b":0},{"r":255,"g":0,"b":0},{"r":0,"g":255,"b":0},{"r":0,"g":0,"b":255}],"transparency":null,"gamma":null,"chromaticities":null,"srgb":null,"physicalDims":null,"timestamp":null,"background":null,"textChunks":[],"ancillaryChunks":[]}}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores a small RGBA document
    Given the small RGBA input document shared://🟥️rgba8-swatch/🖼️.png
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own algebraic inverse is applied next
    Then the oracle and the subject agree on the semantic projection
    And that projection matches the untouched original document
    Examples:
      | id | params |
      | patch-pixels | {"revision":"2989b8e438c3fb2d","x":1,"y":1,"width":1,"height":1,"red":200,"green":40,"blue":40,"alpha":255} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document preserving exact owned values
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.png
    When the document is decoded, printed to the text codec, reparsed and re-encoded
    Then the output preserves every owned native sample and metadata value
    And the oracle and the subject agree on the semantic projection

  @id-owned-native-samples
  @level-long
  @mode-differential
  Scenario: Preserve precise native samples and paint the neutral owned vectors
    Given the committed language-neutral native sample vectors
    When each owned image is encoded, decoded, painted and restored
    Then the oracle and the subject agree on exact owned image values
