@capability-bmp-3-mutate
@oracle-image-bmp-3-mutate-reader
@comparison-semantic-bmp-owned-v1
@mutations-bmp-3-any
Feature: Apply every typed BMP v3 mutation to a real-world document
  The real input is the independently derived 2334x2560 indexed architectural floor plan.
  Its complete owned image has native channel masks and profile, palette entries with reserved bytes,
  top-relative indices or precise direct samples, row order, resolution, header controls, gap and trailer.

  The mutation kinds operate on this image. Replace-image imports an exact typed image; region paints preserve the native profile and unpainted fields.
  Paint revisions are FNV-1a over the canonical owned field sequence with counted strings and arrays,
  native component scalars, palette and metadata. Encoded BMP octets are absent from that sequence.
  Each inverse is a concrete replace-samples row carrying the base pixels of exactly the region the mutation overwrote.

  The oracle uses image-rs to read native indices and visual pixels, and separately retains precise
  native component words and every owned header field. Its own BMP writer preserves these fields.
  Both sides compare the owned profile, masks, metadata, palette and native sample digests, exact owned
  revision and the independent image-rs visual digest. The logical DSL identity law compares owned
  semantics; native row padding and equivalent file size declarations are physical writer choices.

  Every params cell is the leaf's camelCase owned wire payload. The real and small indexed inputs and
  direct-colour input are copied into each scenario's work directory before applying any mutation.

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
      | paint-indexed-region | {"revision":"998e58b33016549e","x":0,"y":0,"width":64,"height":64,"paletteIndex":239} |

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
      | paint-direct-region | {"revision":"dbf8379c618ab77e","x":0,"y":1,"width":2,"height":1,"red":17,"green":34,"blue":51,"alpha":255} |

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
      | paint-indexed-region | {"revision":"998e58b33016549e","x":0,"y":0,"width":64,"height":64,"paletteIndex":239} |

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
      | paint-direct-region | {"revision":"dbf8379c618ab77e","x":0,"y":1,"width":2,"height":1,"red":17,"green":34,"blue":51,"alpha":255} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Preserve the complete owned image through logical and native carriers
    Given the real input document shared://🏛️rathaus-ahlen-grundriss/🖼️.bmp
    When the document is decoded, printed through the DSL text codec, reparsed and re-encoded
    Then the output preserves every owned field and precise native sample
    And the oracle and the subject agree on the semantic projection
