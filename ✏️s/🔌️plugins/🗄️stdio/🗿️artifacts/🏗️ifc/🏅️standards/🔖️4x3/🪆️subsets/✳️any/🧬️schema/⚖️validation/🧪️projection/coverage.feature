Feature: Issued projection axis owner boundaries
  Scenario: Concrete axes retain partial indeterminate results
    Given the unchanged pinned IFC4X3_ADD2 functions and actual direction source
    When two-dimensional and three-dimensional projection branches execute
    Then source entity values remain unchanged
    And known LIST members and indeterminate members retain their positions
    And exact issued defaults and handedness flips remain observable
    And SDK tuple storage discrepancies and exceptions are retained separately
    And independent SQL projections retain their declared support boundary
    And original nested cancellation, failure and work denial never produce success
    And no complete branch or native execution claim follows
