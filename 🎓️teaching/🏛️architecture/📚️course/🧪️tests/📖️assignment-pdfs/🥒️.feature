Feature: Published assignment PDFs
  Scenario: Both explicit language documents are delivered
    Given the course declares English and German print artifact documents
    And the parent retains the exact published documents
    When the site consumes its authenticated local PDF transport
    Then it receives both PDF bodies at assignments/parametrization-en.pdf and assignments/parametrization-de.pdf

  Scenario: Missing or invalid PDFs fail the build
    Given the transport is missing or sends an oversized or non-PDF body
    When the site reads assignment PDFs
    Then the build fails before publishing a download

  Scenario Outline: Shared publications retain all public filenames
    Given the publication vector "<name>" in the shared fixture
    Then exact duplicates are retained once and conflicting producers fail

    Examples:
      | name |
      | Same producer retains both language files |
      | Exact duplicate publication is retained once |
      | Different producers cannot overwrite a public file |
