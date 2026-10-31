@capability-staffage-representation
Feature: A requested scale resolves to an authored, derived or unavailable representation
  A preset scale (500, 200, 100, 50) takes exactly the representation the asset authored for it, or its explicit unavailable decision with reason, limiting
  feature, supported scales and alternatives. A custom scale inside the valid range of an authored representation resolves to it as a derived use that names the
  source representation and its range; outside every range it is unavailable. Nothing is substituted or enlarged silently, paper size always follows the requested
  scale (1.80 m = 3.6 / 9 / 18 / 36 mm), and viewport zoom magnifies the finished drawing without changing which geometry is drawn.

  Scenario: Exact preset
    Given a figure with authored drawings for 1:200, 1:100 and 1:50
    When 1:100 is requested
    Then the authored 1:100 drawing resolves, marked exact

  Scenario: Custom scale inside a valid range
    Given the valid range 75…150 of the 1:100 drawing
    When 1:120 is requested
    Then the 1:100 drawing resolves as derived with requested 120 and its valid range, and the paper height of 1.80 m is 15 mm

  Scenario: Scale without a drawing
    Given 1:500 declared unavailable and 1:1000 outside every valid range
    When either is requested
    Then the resolution is unavailable, lists the supported scales 200, 100, 50 and offers alternatives, and drawing it throws

  Scenario: Zoom does not select geometry
    Given the figure at 1:100
    When it is shown at zoom 0.5, 1, 2 and 4
    Then the drawn shapes and the paper scene are identical and only the screen size scales with the zoom
