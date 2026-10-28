Feature: IFC4 circular and elliptical swept profiles
  Scenario: Parameterized and full conic outlines preserve their declared axes
    Given the language-neutral IFC4 circle and ellipse profile fixtures
    When their Body representations are projected
    Then every outline uses 72 counterclockwise samples with implicit closure
    And chord deviation is at most 0.001 times the larger source semi-axis
    And profile positions preserve their two-dimensional translation and rotation
    And closed extrusion volumes have outward triangle winding for either direction
    And coordinates, triangle counts, area and volume match independent Three.js curve and extrusion output
  Scenario: Invalid curve profile definitions stay explicit
    Given a zero radius, negative semi-axis, CURVE profile type or three-dimensional profile position
    When its swept area is projected
    Then no partial product is published and the invalid profile is diagnosed
  Scenario: Curve sampling remains controlled by the original caller
    Given a real circular extrusion graph
    When the original controller cancels or its work limit is reached during sampling or triangulation
    Then the scene reports cancelled or limited and publishes no partial product
