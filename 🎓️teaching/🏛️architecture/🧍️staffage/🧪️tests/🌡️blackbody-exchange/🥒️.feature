Feature: Grey-body radiation exchange
  Radiant flux of a surface follows q = epsilon * sigma * (Ts^4 - Tmrt^4); positive values leave the surface.

  Scenario Outline: Radiant flux against the CODATA oracle
    Given a surface of emissivity 0.95 held at <surface> degrees Celsius
    And a mean radiant temperature of 20 degrees Celsius
    When the thermal field is solved
    Then the radiant flux is <flux> W/m2 within 0.5 percent

    Examples:
      | surface | flux     |
      | 33      | 75.4025  |
      | 5       | -75.3856 |
      | 20      | 0        |
