Feature: Window chrome publishes the levels it paints
  Scenario: Content and its edge chips use their own surface levels
    Given a pane frame with an opaque window-level content surface
    When content and edge-chip controls are mounted
    Then content inherits a window-level opaque surface
    And the title, controls and footer inherit pane-level glass
    And the content surface is painted once

  Scenario: A collapsed frame publishes the chip surface
    Given a chip-only window frame
    Then the title inherits pane-level glass
