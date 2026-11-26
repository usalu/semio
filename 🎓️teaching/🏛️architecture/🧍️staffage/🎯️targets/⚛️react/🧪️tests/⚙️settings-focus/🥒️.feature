Feature: Keyboard access to display settings
  Scenario: Open desktop display settings and return to the trigger
    Given the archive on a desktop
    When the reader opens Display settings with the keyboard
    Then the selected Text size option receives focus
    When the reader tabs to the next setting
    Then the selected Motion option receives focus
    When the reader presses Escape
    Then the settings close and focus returns to their trigger

  Scenario: Open the phone navigation and display settings
    Given the archive on a phone
    When the reader opens Display settings with the keyboard
    Then the first archive navigation link receives focus

  Scenario: Display settings have one owner
    When a reader opens display settings
    Then there is one glass menu surface
    And its options share the menu surface and their borders
