Feature: Active chrome uses the design system
  The current page is shown in the primary color. A pressed toggle uses the active background.

  Scenario: The current page is primary
    Given the archive is open on the overview
    Then the home link is the current page
    And that link uses the primary color
    And that link is not underlined

  Scenario: Another page takes the primary color
    Given the archive is open on all assets
    Then the all-assets link is the current page
    And that link uses the primary color
    And that link is not underlined

  Scenario: A pressed toggle uses the active background
    Given the archive is open on all assets
    When the sort direction is pressed
    Then that toggle uses the active background
    And that toggle does not use the emphasized background

  Scenario: A selected segment uses the active background
    Given the archive is open
    Then every selected segment uses the active background
    And no selected segment uses the emphasized background
