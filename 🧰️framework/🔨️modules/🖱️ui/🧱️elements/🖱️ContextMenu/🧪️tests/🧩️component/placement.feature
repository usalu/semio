Feature: Fused context menu placement
  Scenario: Oversized menu keyboard visibility
    Given a fused menu larger than its scrolling viewport
    When arrow navigation focuses a row outside the viewport
    Then only the menu scroller moves enough to reveal the focused row
  Scenario: Sequential submenu geometry
    Given two open submenu columns
    When the first wing is measured or resized
    Then the next wing starts at its measured right edge, without overlap
  Scenario: Fusion growth and viewport updates
    Given a pointer-anchored menu near the viewport edge
    When a submenu opens, closes or resizes, or the viewport resizes
    Then the entire fused menu is clamped again using its current dimensions
  Scenario: Shell-owned unclipped chrome
    Given a clipped window inside a shell
    When its context menu opens
    Then the menu is portaled to the shell layer and its silhouette host does not clip its outline
