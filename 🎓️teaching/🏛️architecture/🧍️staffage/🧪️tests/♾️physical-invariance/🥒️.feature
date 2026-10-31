@capability-staffage-representation
Feature: The physical state does not depend on the drawing
  Dimensions, bones, anchors, loads, thermal inputs, respiration, the thermal field, the structural solution and the air transport are identical for every
  representation selector and every viewport zoom; only the drawn geometry differs between scales.

  Scenario: Same physics at every scale
    Given the fixture figure evaluated at 1:200, 1:100, 1:50 and 1:120 and the symbol
    When bones, anchors, loads, thermal inputs, respiration and kinematics are compared
    Then they are identical, while the drawn shape sets differ between 1:200, 1:100 and 1:50

  Scenario: Same thermal and structural solutions
    Given the same states
    When the thermal field and the structure are solved from each
    Then the results are identical

  Scenario: Zoom is irrelevant
    Given the 1:100 state
    When the scene is depicted for several zoom values
    Then the scene and the state are identical
