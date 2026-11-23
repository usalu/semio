Feature: Abutting context menu frames
  Scenario: Oversized menu keyboard visibility
    Given a menu frame larger than its scrolling viewport
    When arrow navigation focuses a row outside the viewport
    Then only that frame's menu scroller moves enough to reveal the focused row
  Scenario: Sequential submenu frames
    Given two open submenu wings
    When the first wing is measured or resized
    Then the next wing frame starts at its measured right edge so both frames share one coincident edge
  Scenario: Wing growth and viewport updates
    Given a pointer-anchored menu near the viewport edge
    When a submenu opens, closes or resizes, or the viewport resizes
    Then the primary frame is clamped again and every wing is placed again from its parent frame
  Scenario: Shell-owned unclipped chrome
    Given a clipped window inside a shell
    When its context menu opens
    Then the menu is portaled to the shell layer and its frame host does not clip it
