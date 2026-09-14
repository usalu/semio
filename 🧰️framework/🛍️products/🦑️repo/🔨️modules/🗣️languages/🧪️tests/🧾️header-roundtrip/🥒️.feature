@capability-repo-header-roundtrip
@no-oracle-repo-file-header
@comparison-ordered-json-v1
Feature: A file header is written and read back without losing a byte
  The header region carries the artifact identity line, the contributors, the licence and the
  optional summary and requirements blocks, each behind the language's own comment prefix and wrapped
  in that language's own section markers. No third-party formatter emits or reads this shape, so the
  recorded no-oracle decision `repo-file-header` names the metamorphic law below and two
  independently written formatters as the substitutes.

  @id-header-per-language
  @level-fundamental
  @mode-differential
  Scenario: Every header-carrying language writes the same header text
    Given one set of header fields
    When each implementation formats a header for every language of the table
    Then every implementation produces the same text per language, and the empty string for a language without headers

  @id-header-region-is-parseable
  @level-fundamental
  @mode-round-trip
  Scenario: A formatted header is a well-formed section that reads back to the same fields
    Given one set of header fields
    When each implementation formats a header and parses the result back with the same language
    Then the header region is the only section, and the identity line, contributors and licence come back unchanged
