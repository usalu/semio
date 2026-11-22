Feature: The flow view answers the environment
  Temperature and humidity of the ambient air decide which way air moves, how much vapour it carries and whether fog forms.

  Scenario Outline: Dew point and fog in a reference environment
    Given a breathing person in the reference environment <preset>
    When the flow field is developed
    Then the dew point of its legend matches CoolProp and psychrolib within 0.2 K (below 0 °C within 0.5 K under their frost point, the model has no ice phase)
    And fog is shown exactly when CoolProp's mixing line of exhaled and ambient air is supersaturated

    Examples:
      | preset               |
      | office-neutral       |
      | winter-clear-night   |
      | summer-humid         |
      | summer-heatwave-calm |

  Scenario Outline: A body's plume follows the temperature difference
    Given a body of 28 °C surface temperature in the reference environment <preset>
    When the flow field is developed
    Then its plume rises where the air is colder than the body and sinks where it is warmer

    Examples:
      | preset               |
      | office-neutral       |
      | winter-clear-night   |
      | summer-hot-sun       |
      | summer-heatwave-calm |

  Scenario: No temperature difference, no flow
    Given a body as warm as the air, and a machine that is switched off
    When the flow field is developed
    Then there are no streams and no puffs
