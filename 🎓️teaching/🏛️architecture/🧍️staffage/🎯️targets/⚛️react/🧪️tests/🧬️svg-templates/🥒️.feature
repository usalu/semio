Feature: Parsed SVG templates stay within a weight budget
  Scenario: Many heavy frames
    Given figures whose every frame is a different, heavy SVG markup
    When more frames are shown than the budget holds
    Then the kept templates never weigh more than the budget in markup characters
    And the element shows the latest frame

  Scenario: A short loop
    Given a looping clip of a few light frames
    When the loop is shown twice
    Then each frame is parsed once

  Scenario: A frame heavier than the whole budget
    When a markup heavier than the budget is shown
    Then the element shows it and no template of it is kept
