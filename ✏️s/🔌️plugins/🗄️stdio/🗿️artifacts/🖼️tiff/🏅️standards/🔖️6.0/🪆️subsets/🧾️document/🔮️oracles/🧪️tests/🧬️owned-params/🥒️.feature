Feature: Authored TIFF Oracle Parameters
  Scenario: Canonical owned page content survives an independent baseline writer
    Given a page with entries and exact word blocks
    And authored ASCII empty strings and IEEE NaN payload words
    When the reference writer exports native TIFF and the third-party decoder reads it
    Then RGB samples are 1,2,3 and 4,5,6
    And authored ASCII boundaries and IEEE bits remain exact
    And physical storage or byte order cannot be authored parameters
