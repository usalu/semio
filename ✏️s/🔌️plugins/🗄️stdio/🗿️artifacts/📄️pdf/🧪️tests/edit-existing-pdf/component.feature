@capability-pdf-edit
@oracle-lopdf
@comparison-semantic-pdf-v1
Feature: Edit an existing PDF document
  Every scenario copies the immutable input fixture into the case work directory before touching it;
  the fixture itself is never written to. The reference implementation is used only by the test
  oracle, and both results are read back by an independent parser before comparison.

  @id-replace-document-metadata
  @level-long
  @mode-differential
  Scenario: Replace the document metadata of an existing document
    Given the two-page input document local://📄️two-pages.pdf
    When the document title is replaced with "Replaced Title" and the author with "Replaced Author"
    Then the document reports the replaced metadata
    And the two pages are unchanged

  @id-delete-the-second-page
  @level-long
  @mode-differential
  Scenario: Delete a page from an existing document
    Given the two-page input document local://📄️two-pages.pdf
    When the second page is deleted
    Then the document contains exactly one page
    And the remaining page media box is unchanged
