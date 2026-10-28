Feature: Explicit released Body AdvancedSweptSolid representation clauses
  Scenario: Released concept and schema function have distinct family scope
    Given an explicitly applicable Body AdvancedSweptSolid representation
    When every item is a known swept-area or swept-disk subtype
    Then the three selected concept clauses pass
    When an item is a horizontal sectioned solid
    Then the selected concept fails while the separate released schema function may pass
    And no full schema, view or geometry conformance is claimed

  Scenario: Original evidence and callback authority remain retained
    Given one privately captured source and exact caller-authored applicability
    When late unknown constituents or ambiguous qualified owners are encountered
    Then the result remains incomplete with exact editable pointers
    And cancellation and original observer failures propagate without redirected authority
