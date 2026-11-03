Feature: Phase change of a heated region
  A language-agnostic specification; the cases and the scipy/CoolProp oracle live in 🧫️fixtures/🫕️phase-change/🔣️.json.

  Scenario Outline: Plateau and converted mass of <case>
    Given a region of <case> heated from inside with a constant power and a phase change declared on its material
    When the transient thermal response is solved at the times of the fixture
    Then the temperature equals the oracle temperature within 0.05 K and never exceeds the phase-change point
    And the temperature holds exactly at the point from the oracle crossing time on
    And the converted mass equals the oracle mass within 0.1 g
    And the conversion power equals the surplus of the heating over the losses at the point within 0.2 percent

    Examples:
      | case         |
      | kettle-water |
      | ice-plate    |

  Scenario: Energy balance
    When a region rests on its plateau
    Then the heat conducted into the region equals radiation plus convection plus latent heat including the conversion
    And the converted mass rate times the latent heat equals the conversion power
    And the energy supplied equals the stored energy plus the losses plus the latent heat of the converted mass

  Scenario: Only the phase change caps the temperature
    When the same region is solved without a phase change
    Then its temperature exceeds the point and no mass is converted

  Scenario: The region overrides the material
    Given a phase change declared on the region at another point
    When the transient thermal response is solved
    Then the region holds the point of the region

  Scenario: Constants of water
    Then the declared boiling point and latent heat of water match CoolProp at 1 atm

  Scenario: Kettle at a constant 2 kW
    Given the electric kettle boil clip with the element at its rated level throughout
    When the thermal and transport fields are solved
    Then the water reaches 100 degrees in about three minutes and holds it
    And the evaporation rate is about 0.85 g/s on the plateau
    And the steam vent flow follows the evaporation rate
