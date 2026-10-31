Feature: Leaf transpiration with stomatal control
  A leaf balances absorbed sunlight against two-faced longwave exchange, convection and transpiration; transpiration is vapour transport through the stomata and the boundary layer in series (Penman-Monteith) and carries latent heat.

  Scenario: Hot sunny dry air
    Given a C3 leaf with maximum stomatal conductance 8 mm/s in full sun of 850 W/m2, air of 32 degrees Celsius at 35 percent humidity and wind of 1.5 m/s
    When the thermal field is solved
    Then the leaf temperature matches the brentq oracle within 0.2 K and the latent flux within 3 percent
    And the stomata are partly closed by the vapour pressure deficit

  Scenario: Clear night
    Given a CAM leaf in air of 15 degrees Celsius under a sky of minus 6 degrees Celsius
    When the thermal field is solved
    Then the stomata are open, the leaf is colder than the air and dew forms when it is below the dew point
