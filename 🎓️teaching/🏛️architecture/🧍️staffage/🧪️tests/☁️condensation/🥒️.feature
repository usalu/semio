Feature: Condensation
  Exhaled air mixing with colder ambient air becomes supersaturated along the mixing line and condenses.

  Scenario Outline: Mixing line against CoolProp
    Given exhaled air at 34 degrees and 95 percent humidity mixed with ambient air at <ambient> degrees by a dry-air mass fraction
    When the mixture is brought to saturation equilibrium
    Then its temperature and liquid water match CoolProp within 0.15 K and 0.03 g/kg

    Examples:
      | ambient |
      | 0       |
      | 5       |
      | 10      |
      | 20      |

  Scenario: Conservation
    When the saturation adjustment conserves total enthalpy and total water per kg of dry air
    Then vapour plus liquid equals the total water and the enthalpy with liquid equals the mixing enthalpy
