Feature: Clothed human surface temperature
  A clothed body is a core-resistance region: (Tcore - Ts) / R = radiation + convection + evaporation (ISO 7730 / Fanger convection).

  Scenario: Clothing surface temperature against the brentq oracle
    Given a body of 70 kg and 1.75 m in air of 20 degrees Celsius, radiant temperature 20 degrees Celsius, relative air speed 0.1 m/s
    And 1.2 met with 0.8 clo clothing and a core temperature of 33.746 degrees Celsius
    When the thermal field is solved
    Then the clothing surface temperature is 26.502 degrees Celsius within 0.1 K
    And the predicted mean vote computed from it matches pythermalcomfort within 0.02

  Scenario: Respiration is assigned to the breath emitter
    Given the same body with a breath emitter at full strength
    When the thermal field is solved
    Then the emitter sensible power is 2.52 W and the emitter latent power is 10.46 W within 2 percent

  Scenario: Activity warms the skin
    Given the same body at 3 met instead of 1.2 met
    When the thermal field is solved
    Then the skin region is warmer than at 1.2 met
