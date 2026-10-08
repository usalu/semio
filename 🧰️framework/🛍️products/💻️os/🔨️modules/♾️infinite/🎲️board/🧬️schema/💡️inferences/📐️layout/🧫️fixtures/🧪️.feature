Feature: Admitted typed Board layout
  Scenario: Original tidy forest and layered geometry preserve labels and endpoint identity
    Given the shared directed root and two child snapshot
    When the admitted typed snapshot is laid out
    Then its coordinates equal the independent d3 hierarchy output
    And labels and handle endpoints remain unchanged
  Scenario: Physical admission and independent work budgets refuse without mutation
    Given malformed physical fields or insufficient work
    Then the borrowed source and original admitted snapshot remain unchanged
