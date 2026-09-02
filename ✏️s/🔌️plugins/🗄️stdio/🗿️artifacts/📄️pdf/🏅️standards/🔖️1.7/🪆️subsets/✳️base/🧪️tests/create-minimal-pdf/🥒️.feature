@capability-pdf-create
@oracle-pdf-writer
@comparison-semantic-pdf-v1
Feature: Create a PDF document
  The reference implementation is used only by the test oracle. Production implementations must not
  depend on it — `bun ./📜️script.ts dependency` proves `pdf-writer` is never production-reachable.
  Both the oracle's and this repository's bytes are read back by an INDEPENDENT parser before they
  are compared, so no producer is ever checked against its own reading of what it wrote.

  Object numbers, cross-reference offsets, dictionary serialization order, compression choices,
  timestamps and document identifiers are not normative and are canonicalized away by the
  `semantic-pdf-v1` profile. Version, page count, media boxes, content operators, extracted text and
  the normative metadata fields are.

  @id-one-empty-a4-page
  @level-quick
  @mode-differential
  Scenario: Create one empty A4 page
    Given the page dimensions are 595 by 842 points
    When a PDF is created
    Then the document contains exactly one page
    And the page media box is 0 0 595 842
    And the document can be parsed by an independent PDF reader

  @id-three-empty-pages
  @level-quick
  @mode-differential
  Scenario: Create a multi-page document
    Given three pages of 595 by 842 points
    When a PDF is created
    Then the document contains exactly three pages
    And every page media box is 0 0 595 842

  @id-document-title-and-author
  @level-quick
  @mode-differential
  Scenario: Create a document carrying normative metadata
    Given one page of 595 by 842 points
    And the document title "Semio Conformance" and author "semio"
    When a PDF is created
    Then the document metadata reports that title and author
