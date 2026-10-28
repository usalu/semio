Feature: Released Alignment Business Terminal Zero
  Scenario Outline: Ordered original source evidence
    Given the closed case "<id>" and explicit caller applicability
    When the selected business-terminal-zero predicate reads one original source graph
    Then its status equals the neutral expected status
    And only the last RelatedObjects segment can satisfy terminal zero
    And geometry, schema and full view conformance remain not-evaluated

  Scenario: Ambiguous source evidence
    Given duplicate entities, competing nesting lists or unresolved earlier members
    Then an applicable result is incomplete rather than a fabricated terminal proof

  Scenario: Original cancellation
    When the original signal cancels request, capture, index, relation, member or result work
    Then no partial report is published
