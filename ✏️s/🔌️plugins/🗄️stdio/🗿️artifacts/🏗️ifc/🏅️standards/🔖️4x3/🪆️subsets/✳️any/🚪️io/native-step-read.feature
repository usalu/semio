Feature: Original STEP source receiving
  Scenario: A real UTF-8 artifact imports into the actual IFC graph and native Pack
    Given the original UTF-8 artifact contains IFC4X3_ADD2 STEP source across declared line endings
    When the registered TXT to IFC route runs with the original native pair and five-axis wallet
    Then ordered headers, complex components, nested typed values, exact numeric values and binary bits survive
    And the original IFC Pack publisher returns the actual graph
    And no complete joined text, isolated record document or origin-token mirror is created

  Scenario Outline: Denial or cancellation retains partial receiving owners
    Given original STEP receiving owns a source, token, list, entity or reference-index prefix
    When the original <currency> is denied or its original observer cancels
    Then no snapshot is published
    And every partial owner drains in the original decoder ledger and recipient
    Examples:
      | currency |
      | items    |
      | copy     |
      | capacity |
      | release  |
      | depth    |

  Scenario: Lexical refusals identify original UTF-8 bytes
    Given multibyte text, ignored controls and declared CRLF endings precede a malformed token
    When the original lexer refuses the token
    Then the diagnostic identifies the original source byte offset

  Scenario: Existing STEP lexical witnesses remain normative
    Given the nine ISO 8859 pages and all 855 neutral positions
    And print directives, doubled slash, arbitrary-byte and Unicode escapes, surrogate pairs and ignored controls
    When original STEP receiving decodes the source
    Then values match the owned lexical contracts and independent reader witnesses
