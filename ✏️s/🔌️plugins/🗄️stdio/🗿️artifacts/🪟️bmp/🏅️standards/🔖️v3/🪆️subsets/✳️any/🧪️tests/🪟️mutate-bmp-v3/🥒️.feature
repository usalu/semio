@capability-bmp-3-mutate
@oracle-image-bmp-3-mutate-reader
@comparison-semantic-raster-v1
@mutations-bmp-3-any
Feature: Apply every typed BMP v3 mutation to a real-world document
  The real input is a 2334x2560, 8-bit indexed architectural floor plan (rathaus-ahlen-grundriss),
  derived ONCE from
  🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/🏛️rathaus-ahlen-grundriss/🖼️.png: the
  independent `png` 0.18 decoder recovers its genuine index buffer and 233-entry palette, and the
  `image` 0.25 reference encoder's palette-aware `BmpEncoder::encode_with_palette` writes them back as
  an 8-bit indexed BITMAPINFOHEADER BMP v3 (the derivation is `#[ignore]`d in the subset's own oracle
  module). The table is padded to 240 entries with colours no pixel resolves to; the real-document
  paint targets spare entry 239, so the index buffer and the resolved samples both move.

  `BmpSnapshot` is byte-authoritative (`{schema, bytes}`, the file's own octets) and the vocabulary has
  four kinds: `set-snapshot` installs a whole file, `patch-snapshot` is one RFC 6901 pointer operation
  on that reading, and `paint-indexed-region` / `paint-direct-region` write one palette index or one
  colour into an image-top-relative rectangle of an indexed or a direct-colour document. Each paint
  carries the revision of the document it was authored against — 64-bit FNV-1a over the schema text
  `stdio.bmp` and then the octets — and a stale one is refused (`mutation.target-mismatch`); every
  revision below is that of its scenario's own input. The inverse of every kind is a whole
  `set-snapshot` of the base.

  Each scenario copies its input into the case work directory; the committed documents are never
  written to. The small indexed input is the committed 4x4, 7-entry document whose entries 5 and 6 no
  pixel references; the direct-colour input is the committed 2x2 top-down 24-bit `BI_RGB` document of
  the canonical byte-authority set. `set-snapshot` installs the small indexed document onto the real
  one.

  The oracle performs every kind independently with the registered `image` reference crate, keeping
  the INDEXED layer intact (`BmpDecoder::set_indexed_color`, `get_palette`, `encode_with_palette`) and
  recomputing the revision guard from its definition. The subject decodes into `BmpSnapshot`, applies
  the typed mutation and re-encodes. Both results are read back by the INDEPENDENT `image` decoder
  before the `semantic-raster-v1` profile compares geometry, row order, both pixels-per-metre fields,
  the colour table's length and digest, the raw index buffer's digest and the resolved samples'
  digest. BMP is lossless, so every one of those is an exact claim.

  Every Examples `params` cell is exactly the leaf's wire payload — camelCase, no aggregate tag —
  decoded by the subject through the derive-generated `from_payload_value` and read by the oracle by
  the same field names.

  The identity round trip asserts EXACT bytes: an uncompressed BMP v3 leaves a writer no choice, and
  the committed fixture was authored by the reference encoder itself, so a byte that moves is a codec
  defect. On the subject side the ONLY channel from input to output is decode_bmp → the DSL text codec
  → parse_dsl → encode_bmp, so a byte that survives did so by being modelled.

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
      | paint-indexed-region | {"revision":"dde0b3ff1648aa21","x":0,"y":0,"width":64,"height":64,"paletteIndex":239} |
      | set-snapshot | {"snapshot":{"schema":"stdio.bmp","bytes":[66,77,98,0,0,0,0,0,0,0,82,0,0,0,40,0,0,0,4,0,0,0,4,0,0,0,1,0,8,0,0,0,0,0,16,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,0,255,0,0,255,0,0,255,0,0,0,0,255,255,0,255,0,255,0,0,0,0,0,30,20,10,0,2,3,4,0,3,4,0,1,4,0,1,2,0,1,2,3]}} |

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
      | patch-snapshot | {"patch": {"operation": "set", "path": "/bytes/82", "value": 5}} |

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to a direct-colour document
    Given the direct-colour input document shared://🧬️canonical-byte-authority/direct-rgb24-top-down.bmp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | params |
      | paint-direct-region | {"revision":"30fee1c26d0ecc44","x":0,"y":1,"width":2,"height":1,"red":17,"green":34,"blue":51,"alpha":255} |

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
      | paint-indexed-region | {"revision":"dde0b3ff1648aa21","x":0,"y":0,"width":64,"height":64,"paletteIndex":239} |
      | set-snapshot | {"snapshot":{"schema":"stdio.bmp","bytes":[66,77,98,0,0,0,0,0,0,0,82,0,0,0,40,0,0,0,4,0,0,0,4,0,0,0,1,0,8,0,0,0,0,0,16,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,0,255,0,0,255,0,0,255,0,0,0,0,255,255,0,255,0,255,0,0,0,0,0,30,20,10,0,2,3,4,0,3,4,0,1,4,0,1,2,0,1,2,3]}} |

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
      | patch-snapshot | {"patch": {"operation": "set", "path": "/bytes/82", "value": 5}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores a direct-colour document
    Given the direct-colour input document shared://🧬️canonical-byte-authority/direct-rgb24-top-down.bmp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own algebraic inverse is applied next
    Then the oracle and the subject agree on the semantic projection
    And that projection matches the untouched original document
    Examples:
      | id | params |
      | paint-direct-region | {"revision":"30fee1c26d0ecc44","x":0,"y":1,"width":2,"height":1,"red":17,"green":34,"blue":51,"alpha":255} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document, reproducing it exactly
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp
    When the document is decoded, printed through the DSL text codec, reparsed and re-encoded
    Then the output reproduces the input byte for byte
    And the oracle and the subject agree on the semantic projection
