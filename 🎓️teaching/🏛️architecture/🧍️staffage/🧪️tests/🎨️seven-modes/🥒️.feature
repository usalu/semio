@capability-staffage-depiction
Feature: All seven modes draw the selected representation
  Dark line, dark solid, light line, light solid, thermodynamic, structural load and structural moment render from the representation selected for the scale: the same shape
  set per scale in every mode, representation-specific pens, analytic overlays on that geometry, and the air transport overlay in any mode.

  Scenario: Same shapes in every mode, different shapes per scale
    Given the fixture figure at 1:200, 1:100 and 1:50
    When all seven modes are depicted
    Then each mode carries the fill shapes of its scale's representation and the shape sets differ between scales

  Scenario: Pens belong to the representation
    Given 1:200 and 1:50
    When the outline widths of the line view are read
    Then they are the body pen of the respective representation

  Scenario: Transport overlay
    Given the transport layers parcels and streamlines
    When the line and thermal views are depicted
    Then the scene gains transport items and a legend
