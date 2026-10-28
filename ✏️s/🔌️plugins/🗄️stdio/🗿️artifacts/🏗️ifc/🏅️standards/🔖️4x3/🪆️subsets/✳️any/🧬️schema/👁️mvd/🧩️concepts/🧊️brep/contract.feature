Feature: Explicit released basic Body Brep representation clauses
  Scenario: Basic Brep uses the exact faceted family
    Given explicit applicability and Body Brep labels
    When every item belongs to FacetedBrep including its with-voids subtype
    Then the selected three clauses pass
    When an item is only an AdvancedBrep or manifold base
    Then the basic Brep clause fails
    And no topology, schema or geometry admission is inferred

  Scenario: Original source and incomplete evidence retain authority
    Given privately captured original input and control
    When later constituents or qualified owners remain unknown
    Then diagnostics remain incomplete with exact pointers
    And original official labels and declared families remain unchanged
