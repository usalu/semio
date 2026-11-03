@capability-schema-twins
Feature: A schema document has one canonical layout
  The JSON Schema document is embedded verbatim in the crates of its module and counts against the 262144 bytes and 4000 lines of the repo source boundary. `schema twins generate` writes it, `schema twins check --strict` compares against it.

  Scenario: A short document is indented
    Given a schema document whose indented text has at most 1500 lines
    When the canonical layout is rendered
    Then the text is the two-space indented JSON of the document

  Scenario: A long document is compact
    Given a schema document whose indented text has more than 1500 lines
    When the canonical layout is rendered
    Then every top-level key and every definition takes one line and the values are unchanged

  Scenario: Rendering is idempotent
    Given the canonical text of any schema document
    When the canonical layout of its parsed value is rendered again
    Then the text is identical
