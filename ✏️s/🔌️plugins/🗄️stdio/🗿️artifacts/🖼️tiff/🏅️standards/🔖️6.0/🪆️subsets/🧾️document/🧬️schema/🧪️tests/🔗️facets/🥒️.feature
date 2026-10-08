Feature: Transparent TIFF Mutation Facets
  Scenario: Owned page and tag payloads have typed cross-language declarations
    Given InsertIfd owns an index and TiffIfd
    And ReplaceTag owns an IFD index, tag identity and TiffValues
    When independent GraphQL and Protocol Buffers readers resolve the declarations
    Then the IFD entries and exact sample blocks remain structurally declared
    And tag values declare their scalar domain and payload without an opaque byte envelope
