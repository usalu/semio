Feature: Grey-body radiation exchange
  Emitted flux is epsilon * sigma * T^4; the net longwave exchange is the sum over sky, ground and surroundings of epsilon * sigma * F * (Ts^4 - T^4), positive when the surface loses heat.

  Scenario Outline: Net exchange and split against the CODATA oracle
    Given a surface of emissivity <emissivity> held at <surface> degrees Celsius facing <geometry> with sky view factor <sky>
    And a sky at -10 degrees Celsius, ground at 30 degrees Celsius and surroundings at 20 degrees Celsius
    When the thermal field is solved
    Then the emitted flux is <emitted> W/m2 and the net radiative flux is <net> W/m2 within 0.2 percent

    Examples:
      | emissivity | surface | geometry              | sky | emitted  | net       |
      | 0.95       | 33      | horizontal-plate-up   | 0.8 | 473.2301 | 187.0130  |
      | 0.95       | 5       | vertical-plate        | 0.3 | 322.4420 | -53.5253  |
      | 0.9        | 25      | horizontal-plate-down | 0.0 | 403.2678 | -27.7395  |
