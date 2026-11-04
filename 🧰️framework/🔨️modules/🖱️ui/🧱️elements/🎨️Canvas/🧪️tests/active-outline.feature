Feature: Controlled Mode Window Outline
  Scenario: Initial active window is visible without pointer or focus interaction
    Given the App design mode contains adjacent left and right window stacks
    And each stack measures 584 by 370 pixels with a 108 by 30 pixel title chip
    And activeWindowId is left
    Then the left tab is selected and its complete chip and body outline is active
    And the right outline is neutral
    And the shared splitter reserves zero layout space

  Scenario: Activation follows the controlled window selection
    When the right tab is selected
    Then onActiveWindowChange emits right
    And the host updates activeWindowId to right
    Then the entire right chip and body outline is active
    And the left outline is neutral
    When the pointer enters the right tab
    Then hover replaces active across the entire right outline
    When the pointer leaves the stack
    Then the entire right outline is active again
