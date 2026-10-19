@capability-tui-window-contour
Feature: Continuous window and tab contour strokes
  Scenario: Focused body stroke passes under inactive tabs
    Given a focused window with first, middle and last tabs in any corner
    When the middle tab is active
    Then the body outline retains its active color across every closed seam
    And only the active tab cap and walls use active color
    And the active tab seam opens into the body
  Scenario: Pointer ancestry emphasizes the entire exterior
    Given a hovered window with inactive tabs
    Then its body, inactive caps and inactive junctions use emphasized borders
    And the selected tab retains its active cap and junctions
  Scenario: Junctions preserve every connected direction
    Given right-only, four-corner, short, compact and flat windows
    Then every visible box glyph matches the union of incident contour segments
    And the contour forms one connected graph
    And unused raised notches retain the parent surface
