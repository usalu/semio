Feature: Leaf transpiration and passive surfaces
  A leaf balances absorbed solar radiation against radiation, convection and Penman-Monteith latent heat; a passive surface has no latent term.

  Scenario Outline: Leaf and panel against the brentq oracle
    Given the environment "<environment>"
    When the thermal field is solved
    Then the leaf temperature and latent flux match the oracle within 0.1 K and 1 percent
    And the panel temperature matches the oracle within 0.1 K

    Examples:
      | environment  |
      | windy-sun    |
      | still-sun    |
      | breezy-shade |

  Scenario: Closed stomata stop transpiration
    Given the leaf has zero stomatal conductance
    When the thermal field is solved
    Then the latent flux is zero and the leaf is warmer than the transpiring leaf
