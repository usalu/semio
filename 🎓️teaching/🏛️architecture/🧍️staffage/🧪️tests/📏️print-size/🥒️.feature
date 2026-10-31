@capability-staffage-units
Feature: Model lengths become paper millimetres at every drawing scale
  A drawing scale 1:d maps one model metre to 1000 / d paper millimetres, so a 1.80 m person measures 3.6, 9, 18 and 36 mm at 1:500, 1:200,
  1:100 and 1:50 and 90 mm at 1:20. Pens, hatches and detail belong to the representation authored for each scale, never to the units module.
  Forces, pressures, moments, powers, flux densities and temperatures are written with the prefix that keeps the number below 1000 (3 significant
  digits, decimal comma in German).

  Scenario: A person of 1.80 m on paper
    Given the drawing scales 1:500, 1:200, 1:100, 1:50 and 1:20
    When the height 1.80 m is converted to paper millimetres
    Then the results are 3.6, 9, 18, 36 and 90 millimetres

