Feature: Exhaust plume in wind
  A vertical exhaust jet bends downwind; calm air is symmetric and opposite winds mirror each other.

  Scenario: Calm air
    Given a vertical exhaust stack of 195 degrees Celsius in calm air
    When the thermal field is solved
    Then the plume stays on the vertical axis and rises monotonically

  Scenario Outline: Wind bends the plume
    Given a wind of 3 m/s blowing <direction>
    When the thermal field is solved
    Then the plume drifts <direction> and stays below the calm-air plume

    Examples:
      | direction |
      | right     |
      | left      |

  Scenario: Mirror symmetry
    Given winds of 3 m/s to the right and to the left
    When both fields are solved
    Then the plumes are mirror images about the stack axis

  Scenario: Oracle path
    When the streamline is compared with the RK45 reference
    Then every point lies within 5 mm of the reference polyline
