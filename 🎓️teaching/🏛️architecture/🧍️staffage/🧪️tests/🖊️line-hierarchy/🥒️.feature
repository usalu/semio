Feature: Line hierarchy
  Outline weights follow the role of the shape and the drawing scale.

  Scenario: pen table
    Given a figure with body, detail, texture and glazing shapes
    When it is drawn as a line view at 1:500, 1:200, 1:100 and 1:50
    Then body outlines use 0.18, 0.25, 0.35 and 0.50 mm
    And detail and glazing outlines use 0.13, 0.13, 0.18 and 0.25 mm
    And texture outlines are absent at 1:500 and use 0.13 mm otherwise
    And the ground line uses 0.25, 0.35, 0.50 and 0.70 mm
