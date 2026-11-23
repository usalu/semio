@capability-tui-chrome-emphasis
Feature: Window tab outlines and pointer ancestor emphasis
  Scenario Outline: Active chips keep a continuous accent outline
    Given three chips in the <corner> corner of a focused or unfocused window
    When the first, middle or last tab is active and the pointer hovers a tab
    Then the hovered chip walls and outline segment use the emphasized border color
    And inactive chip walls use normal borders or emphasized borders on direct hover
    And active chip interiors use the chip surface and ordinary foreground
    And inactive chip interiors use the chip surface with muted text and the existing hover fill
    And every chip wall meets the window outline in one junction, a stated chip in its own state color
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
    Then both first-tab walls use the emphasized border color
    And the unshared active-tab wall retains the active base color
    And a wall shared by hovered and active tabs uses the emphasized border color
  Scenario: Abutting chips share a wall and retain connected outlines
    Given adjacent chips in any corner
    When one tab is active and the pointer hovers its neighbor
    Then the chips use one shared wall with a connected junction
    And the hovered chip has priority on that wall
