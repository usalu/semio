Feature: View synchrony
  Every view is a restyling of the same evaluated state.

  Scenario: identical geometry
    Given the reference walker in a pose
    When it is drawn in dark-line, dark-solid, light-line, light-solid, thermal, forces and moments
    Then every shape id present in a view has the same model-space coordinates in every other view

  Scenario: language-independent geometry
    Given the same state drawn in English and in German
    Then every shape id has the same model-space coordinates in both languages whatever the width of the labels

  Scenario: labels
    Given the forces view with labels on
    Then label texts do not overlap one another
    And no label prints a raw floating point number
