Feature: Two-node thermoregulation against pythermalcomfort
  The Gagge two-node model regulates skin blood flow, sweating and wettedness from core and skin temperature; started from the neutral state it is run for sixty minutes like the library.

  Scenario Outline: Core and mean skin temperature after one hour
    Given a clothed body of 70 kg and 1.8258 m2 with <clo> clo in air of <air> degrees Celsius at <rh> percent humidity and <wind> m/s
    And an activity of <met> met
    When the body is simulated for 3600 seconds from the neutral state
    Then the core temperature is <core> degrees Celsius within 0.15 K and the mean skin temperature <skin> degrees Celsius within 1 K

    Examples:
      | air | rh | wind | met | clo | core  | skin  |
      | 24  | 50 | 0.1  | 1.0 | 0.6 | 36.83 | 33.79 |
      | 35  | 50 | 0.2  | 1.2 | 0.4 | 37.02 | 35.49 |
      | 10  | 60 | 1.0  | 1.2 | 1.5 | 36.83 | 30.48 |
