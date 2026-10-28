Feature: Exact IFC binary scalar persistence
  Scenario Outline: Binary content survives the typed native and SQLite graph
    Given an IFC binary scalar with canonical STEP content <content>
    When the snapshot is exported to SQLite and independently queried
    Then its kind is binary and its literal content is <content>
    And reconstruction preserves binary scalars in headers, aggregates, typed values and complex members
    Examples:
      | content  |
      | 0        |
      | 0F       |
      | 17       |
      | 23       |
      | 31       |
      | 00ABCDEF |
  Scenario: Invalid unused-bit padding is refused
    Given binary content with a first hexadecimal digit that uses a left padding bit
    When the snapshot is projected or reconstructed
    Then the operation is rejected
