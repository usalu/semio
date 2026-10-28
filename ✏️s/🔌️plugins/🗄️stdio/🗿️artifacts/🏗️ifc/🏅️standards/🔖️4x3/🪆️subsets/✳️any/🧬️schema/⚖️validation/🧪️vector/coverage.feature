Feature: Concrete issued vector owners
  Scenario: source addressed derived dimensionality
    Given the original authored IFC directions and vectors
    When the unchanged compiler-issued vector function runs in the real population context
    Then actual derived Dim and constructor attributes determine the output
    And original source entity values remain unchanged
  Scenario: meaningful undefined vector boundaries
    Given a zero-magnitude vector or dimension-mismatched directions
    Then the actual released function returns indeterminate
    And no whole model or view conformance is inferred
  Scenario: original controls
    Given cancellation or an exact callback failure during source preparation or function work
    Then execution refuses under the original control without returning a result
