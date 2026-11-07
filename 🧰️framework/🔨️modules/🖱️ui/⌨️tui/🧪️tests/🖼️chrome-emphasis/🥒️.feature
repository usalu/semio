@capability-tui-chrome-emphasis
Feature: Window tab outlines and pointer ancestor emphasis
  Scenario Outline: Active tabs keep a continuous accent silhouette
    Given three raised tabs in the <corner> corner of a focused or unfocused window
    When the first, middle or last tab is active and the pointer hovers a tab
    Then the hovered tab perimeter uses the emphasized border color
    And inactive tab perimeters use normal borders or emphasized borders on direct hover
    And active tab interiors retain the window surface and ordinary foreground
    And inactive tab interiors retain their muted text and existing hover fill
    And only the active tab seam opens into the window
    Examples:
      | corner       |
      | top-left     |
      | top-right    |
      | bottom-left  |
      | bottom-right |
  Scenario: Hovering descendants emphasizes every enclosing chrome frame
    Given a widget nested in a window and widgets nested in navbar and footer
    When the pointer moves among descendants, siblings, empty bodies and overlays
    Then only the hovered lineage has emphasized chrome borders
    And widget hover identity remains the nearest interactive node
    And focus loss, removal and reparenting clear stale emphasized borders
  Scenario: Compact hover owns shared borders
    Given three compact tabs with the middle tab active
    When the pointer hovers the first tab
    Then both first-tab border endpoints use the emphasized border color
    And the unshared active-tab endpoint retains the active base color
    And an endpoint shared by hovered and active tabs uses the emphasized border color
  Scenario: Raised tabs share a wall and retain connected outlines
    Given adjacent raised tabs in any corner
    When one tab is active and the pointer hovers its neighbor
    Then the tabs use one shared wall with connected junctions
    And the hovered tab has priority on that wall
