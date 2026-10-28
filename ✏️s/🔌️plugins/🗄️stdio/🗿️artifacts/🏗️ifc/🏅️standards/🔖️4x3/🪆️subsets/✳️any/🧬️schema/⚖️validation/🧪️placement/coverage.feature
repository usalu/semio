Feature: Issued placement and point owners
  Scenario: exact epsilon boundaries
    Given actual Cartesian points and raw direction ratios
    Then released strict epsilon comparisons distinguish below and exact boundary
    And proportional direction ratios are not silently normalized
  Scenario: real derived placement axes
    Given source-owned 2D and 3D placements
    Then SameAxis2Placement reads real derived P axes and locations
    And original source entities stay unchanged
  Scenario: parent placement dimensionality
    Given an absent parent or a genuine local parent
    Then CorrectLocalPlacement preserves released TRUE, FALSE and indeterminate outcomes
  Scenario: basis-derived point dimensions
    Given actual curve, surface and distance-expression basis objects
    Then PointDim resolves their true derived dimensions
