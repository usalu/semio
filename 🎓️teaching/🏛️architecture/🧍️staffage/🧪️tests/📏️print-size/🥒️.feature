@capability-staffage-units
Feature: Model lengths become paper millimetres at every drawing scale
  A drawing scale 1:d maps one model metre to 1000 / d paper millimetres, so a 1.80 m person measures 3.6, 9, 18 and 36 mm at 1:500, 1:200,
  1:100 and 1:50 and 90 mm at 1:20. The level of detail grows as the scale gets larger (denominator > 200 → 0, > 100 → 1, > 50 → 2, otherwise 3).
  ISO 128 pen widths depend on the line role and the nearest preset scale, found by the smallest |ln d − ln preset|; texture is not drawn at 1:500.
  Forces, pressures, moments, powers, flux densities and temperatures are written with the prefix that keeps the number below 1000 (3 significant
  digits, decimal comma in German).

  Scenario: A person of 1.80 m on paper
    Given the drawing scales 1:500, 1:200, 1:100, 1:50 and 1:20
    When the height 1.80 m is converted to paper millimetres
    Then the results are 3.6, 9, 18, 36 and 90 millimetres

  Scenario: Line weights follow role and preset
    Given every shape role and the four preset scales
    When the pen width is looked up
    Then it matches the ISO 128 table, and is absent for texture at 1:500

  Scenario: A custom scale snaps to the nearest preset
    Given the denominators 70 and 320
    When the pen width of a body line is looked up
    Then 1:70 draws 0.70 mm (preset 1:50) and 1:320 draws 0.25 mm (preset 1:500; the boundary between 1:500 and 1:200 is their geometric mean, about 316)
