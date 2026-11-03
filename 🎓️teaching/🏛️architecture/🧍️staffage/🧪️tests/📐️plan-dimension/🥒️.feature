Feature: Overall dimension per viewpoint
  The dimension line measures what the viewpoint draws along the paper's vertical axis: the height in the elevation, the depth in the plan.

  Scenario: the elevation of the gas hob
    Given the gas hob drawn in elevation at 1:50 with dimensions
    When the SVG is read back
    Then the dimension is a height of 0.088 m and the label reads "0.09 m"

  Scenario: the plan of the gas hob
    Given the gas hob drawn in plan at 1:50 with dimensions
    When the SVG is read back
    Then the dimension is a depth of 0.52 m and the label reads "0.52 m"
    And the extension lines are 0.52 m apart at 20 mm per metre
