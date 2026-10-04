@capability-png-1-2-mutate
@oracle-png-png-1-2-mutate-reader
@comparison-semantic-raster-v1
@mutations-png-1-2-any
Feature: Apply every typed PNG 1.2 mutation to a real-world document
  The real input is a 250 KB, 2334x2560, 8-bit COLORMAP architectural floor plan
  (rathaus-ahlen-grundriss.png) whose chunk chain is exactly IHDR (colour type 3, bit depth 8,
  non-interlaced), PLTE (233 entries), eight IDAT chunks and IEND. Every scenario copies its input
  into the case work directory; the committed documents are never written to.

  `PngSnapshot` is byte-authoritative (`{schema, bytes}`, the file's own octets) and the vocabulary
  has five kinds: `set-snapshot` installs a whole file, `patch-snapshot` is one RFC 6901 pointer
  operation on that reading, `change-gamma` sets, inserts or removes the gAMA chunk, `patch-pixels`
  paints one RGBA8 colour into a rectangle of an 8-bit non-interlaced RGBA document, and
  `paint-native-samples` carries a completed native-sample paint (`result`) that the subject admits
  only when it keeps the source profile and every byte outside the IDAT run. The guarded kinds carry
  the revision of the document they were authored against — 64-bit FNV-1a over the schema text
  `stdio.png` and then the octets — and a stale one is refused (`mutation.target-mismatch`); every
  revision below is that of its scenario's own input. The inverse of every kind is a whole
  `set-snapshot` of the base.

  The rows: on the real document, `change-gamma` inserts gAMA 45455 before PLTE, `set-snapshot`
  installs the committed 2x2 RGBA swatch, and `patch-snapshot` splices a complete, CRC-correct gAMA
  chunk into the octets right after IHDR. On the committed 4x2 COLORMAP document (palette black, red,
  green, blue; both rows index 0, 1, 2, 3), `paint-native-samples` paints palette index 3 into the 2x2
  rectangle at (1, 0), its `result` re-encoding the painted index rows as one IDAT. On the committed
  2x2 RGBA swatch, `patch-pixels` paints rgba(200, 40, 40, 255) into the pixel at (1, 1).

  The oracle performs every kind independently with the registered `png` reference crate's own
  Encoder/Decoder API, recomputing the revision guard and the paint from their definitions, and
  never reuses this repository's codec. Both results are read back by the INDEPENDENT `png` decoder;
  the compared projection is the whole document — geometry, a digest of the decoded RGBA samples,
  the palette, the five typed ancillary chunks, the timestamp, the background colour, the text chunks
  and the private chunks.

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
      | change-gamma | {"revision":"41266632732fbc10","gama":45455} |
      | set-snapshot | {"snapshot":{"schema":"stdio.png","bytes":[137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,2,0,0,0,2,8,6,0,0,0,114,182,13,36,0,0,0,19,73,68,65,84,120,218,99,248,207,192,240,31,12,129,52,8,52,0,0,73,73,9,120,156,81,23,146,0,0,0,0,73,69,78,68,174,66,96,130]}} |
      | patch-snapshot | {"patch":{"operation":"splice","path":"/bytes","offset":33,"remove":0,"value":[0,0,0,4,103,65,77,65,0,0,177,143,11,252,97,5]}} |

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
      | paint-native-samples | {"revision":"ef81220d6258bcb7","region":{"x":1,"y":0,"width":2,"height":2},"paint":{"profile":"indexed","first":3,"second":0,"third":0,"fourth":0},"result":{"schema":"stdio.png","bytes":[137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,4,0,0,0,2,8,3,0,0,0,72,118,141,81,0,0,0,12,80,76,84,69,0,0,0,255,0,0,0,255,0,0,0,255,155,192,19,220,0,0,0,15,73,68,65,84,120,218,99,96,96,102,102,102,0,17,0,0,91,0,19,131,18,229,83,0,0,0,0,73,69,78,68,174,66,96,130]}} |

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
      | patch-pixels | {"revision":"c8fb16d279c2e54f","x":1,"y":1,"width":1,"height":1,"red":200,"green":40,"blue":40,"alpha":255} |

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
      | change-gamma | {"revision":"41266632732fbc10","gama":45455} |
      | set-snapshot | {"snapshot":{"schema":"stdio.png","bytes":[137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,2,0,0,0,2,8,6,0,0,0,114,182,13,36,0,0,0,19,73,68,65,84,120,218,99,248,207,192,240,31,12,129,52,8,52,0,0,73,73,9,120,156,81,23,146,0,0,0,0,73,69,78,68,174,66,96,130]}} |
      | patch-snapshot | {"patch":{"operation":"splice","path":"/bytes","offset":33,"remove":0,"value":[0,0,0,4,103,65,77,65,0,0,177,143,11,252,97,5]}} |

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
      | paint-native-samples | {"revision":"ef81220d6258bcb7","region":{"x":1,"y":0,"width":2,"height":2},"paint":{"profile":"indexed","first":3,"second":0,"third":0,"fourth":0},"result":{"schema":"stdio.png","bytes":[137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,4,0,0,0,2,8,3,0,0,0,72,118,141,81,0,0,0,12,80,76,84,69,0,0,0,255,0,0,0,255,0,0,0,255,155,192,19,220,0,0,0,15,73,68,65,84,120,218,99,96,96,102,102,102,0,17,0,0,91,0,19,131,18,229,83,0,0,0,0,73,69,78,68,174,66,96,130]}} |

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
      | patch-pixels | {"revision":"c8fb16d279c2e54f","x":1,"y":1,"width":1,"height":1,"red":200,"green":40,"blue":40,"alpha":255} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document, reproducing it exactly
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.png
    When the document is decoded, printed to the text codec, reparsed and re-encoded
    Then the output reproduces the input byte for byte
    And the oracle and the subject agree on the semantic projection
