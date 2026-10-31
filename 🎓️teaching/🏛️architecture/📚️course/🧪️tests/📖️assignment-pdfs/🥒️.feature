Feature: Published assignment PDFs
  Scenario: Every declared assignment is delivered in both explicit languages
    Given the course declares English and German print artifact documents for the parametrization and typology assignments
    And the parent retains the exact published documents
    When the site consumes its authenticated local PDF transport
    Then it receives all four PDF bodies in course order below assignments/, from architecture-parametric-parametrization-en.pdf to architecture-parametric-typology-de.pdf

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
