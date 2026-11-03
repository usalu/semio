Feature: Overall dimension of small objects
  The value of the overall dimension never sits on its extension lines or ticks.

  Scenario: a tealight at 1:50
    Given the tealight drawn in elevation at 1:50 with dimensions
    When the SVG is read back
    Then the extension lines are less than 3 mm apart
    And a leader lifts the label "0.04 m" above the upper extension line, right of the ticks

  Scenario: the thermostatic radiator valve at 1:50
    Given the same drawing of the thermostatic radiator valve
    Then the label stands clear of both extension lines and both ticks

  Scenario: an adult at 1:50
    Given the standing adult drawn in elevation at 1:50
    Then the label stays between the extension lines and there is no leader
