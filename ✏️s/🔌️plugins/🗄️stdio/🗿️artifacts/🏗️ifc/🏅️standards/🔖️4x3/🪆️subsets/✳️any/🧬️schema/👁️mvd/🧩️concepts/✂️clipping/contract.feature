Feature: Explicit selected released Body Clipping representation clauses
  Scenario: Concept family remains narrower than the separate schema function
    Given caller applicability and Body Clipping labels
    When every item belongs to BooleanClippingResult
    Then the three selected clauses pass
    When an item belongs only to CsgSolid
    Then the selected concept fails even if the separate schema type function permits it
    And schema operator and operand rules and geometry remain not evaluated

  Scenario: Original and authored physical inputs have distinct provenance
    Given no positive Clipping item in the pinned official corpus
    When the original bath model is checked
    Then its selected Clipping clauses fail
    When the separately authored difference fixture is checked
    Then its selected clauses pass without an official-source claim
