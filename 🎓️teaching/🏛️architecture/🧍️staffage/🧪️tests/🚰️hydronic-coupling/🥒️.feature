Feature: Water-bearing thermal regions
  A language-agnostic specification; the cases and the scipy/CoolProp oracle live in 🧫️fixtures/🚰️hydronic-coupling/🔣️.json.

  Scenario Outline: A plate heated by the water of a heating circuit (<case>)
    Given a plate that conducts from circuit water at a place along the flow, interpolated from the supply and return rows at the operating level
    When its transient thermal response is solved at the times of the fixture
    Then the surface temperature equals the oracle within 0.05 K
    And the steady temperature equals the oracle steady state within 0.05 K
    And no surface temperature exceeds the water temperature or the supply temperature

    Examples:
      | case           |
      | upper-rated    |
      | lower-rated    |
      | middle-comfort |
      | middle-night   |
      | middle-quarter |

  Scenario: Catalog panel radiator
    When the rated, comfort, night and off poses are solved
    Then every surface is at or below the supply temperature of 75 degrees
    And the upper front band is warmer than the middle band and the middle band warmer than the lower band
    And the total conducted output at the rated pose is between 1.05 and 1.3 kW

  Scenario: A wall coupled to the water below the fill level
    Given a wall whose wetted share conducts from the water of another region and a water region heated by a constant power with a boiling cap
    When the transient thermal response is solved at the times of the fixture
    Then the water and wall temperatures equal the oracle within 0.6 K and 0.6 K
    And the wall stays within 6 K of the water on the plateau and never exceeds it
    And the conversion power on the plateau equals the heating less the water losses and the heat taken by the wall within 3 percent

  Scenario: Catalog kettle and pot
    When the clips are solved on the plateau
    Then the wall follows the water within 6 K and stays below it
