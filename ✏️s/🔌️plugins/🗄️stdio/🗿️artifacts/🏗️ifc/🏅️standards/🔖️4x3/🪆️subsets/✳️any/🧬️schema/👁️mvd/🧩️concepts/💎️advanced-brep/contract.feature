Feature: Explicit released Body AdvancedBrep representation clauses
  Scenario: Both released concrete manifold branches are admitted by the selected family clause
    Given explicit applicability and Body AdvancedBrep labels
    When every item belongs to AdvancedBrep or FacetedBrep including their with-voids subtypes
    Then the selected three clauses pass
    And an abstract manifold-only projection does not become physical validity

  Scenario: Original labels and source errors remain visible
    Given unmodified official cube, basic Brep and basin inputs
    When the selected AdvancedBrep clauses are evaluated
    Then only the original cube carries the required AdvancedBrep label
    And the basin's unrelated schema error remains an independent recorded source observation
    And no topology, geometry or full-view admission is inferred
