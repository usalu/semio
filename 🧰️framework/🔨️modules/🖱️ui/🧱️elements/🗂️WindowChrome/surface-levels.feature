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

  Scenario: Page sections retain their owner's surface
    Given a window containing an overview card, a canvas and a second card
    Then each in-flow body shares the window background
    And each card's edge chips use pane glass
    And a card floating above the overview veil owns a dialog surface

  Scenario: Nested content keeps a single paint even through an unpainted wrapper
    Given a panel with a glass background
    When its content includes nested surfaces at the same level
    Then only the owner paints the panel
    And descendants resolve the owner's glass

  Scenario: A native widget is repainted after a transient
    Given a base, window, dialog or menu surface
    When a chip is drawn and subsequently replaced with ordinary content
    Then the chip uses the next level capped at menu
    And the content restores the owner background in light and dark appearances

  Scenario: Inline chips publish their glass surface
    Given a window-level surface containing a badge, status chip and tab list
    Then each chip paints pane-level glass
    And descendants inherit pane-level glass
    And a neutral badge does not paint a muted background
    And semantic status remains visible in the ink
