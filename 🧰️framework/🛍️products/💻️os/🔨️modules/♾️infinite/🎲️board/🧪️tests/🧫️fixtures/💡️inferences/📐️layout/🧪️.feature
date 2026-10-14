Feature: Admitted typed Board layout
  Scenario: Original tidy forest and layered geometry preserve labels and endpoint identity
    Given the shared directed root and two child snapshot
    When the admitted typed snapshot is laid out
    Then its coordinates equal the independent d3 hierarchy output
    And labels and handle endpoints remain unchanged
  Scenario: Physical admission and independent work budgets refuse without mutation
    Given malformed physical fields or insufficient work
    Then the borrowed source and original admitted snapshot remain unchanged

  Scenario: Explicit optional absence remains distinct from required option null
    Given the shared nullable fixture declares optional snapshot and redraw fields
    Then both admissions preserve their absent semantic options
    And six required force/tree option null values are refused
    And two required layered option null values are refused
    And independent Ajv and Serde observe the same outcomes
