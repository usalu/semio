Feature: Legends state units and conventions
  Scenario: analytic views
    Given the reference walker
    When it is drawn in the thermal, forces and moments views
    Then each scene has legends with a unit, a sign convention and an interpretation in English and German
    And the legend colour stops lie between 0 and 1
    And plain line and solid views have no legends
