@capability-tui-window-contour
Feature: One rectangular window box with chips inside it
  Scenario: The box outline is a plain rectangle
    Given a window with chips in any corner, in compact mode, or without chips
    Then its outline is the four edges of one rectangle of cells
    And no cell of the rectangle is cut out, left to the floor or closed as a sliver
  Scenario: Chips lie inside the box on the first or last interior row
    Given a focused or emphasized window with first, middle and last tabs in any corner
    Then every chip lies within the box bounds on an interior row
    And a chip draws only its inner walls, which meet the outline in one junction each
    And an outer chip edge is the frame outline itself
    And the outline keeps the highest priority state of the frame and the chips at every shared cell
  Scenario: Chip rows paint content where no chip covers them
    Given a chip row with chips and free cells
    Then every chip cell uses the opaque chip surface one elevation above the content
    And every uncovered cell paints the content surface and hits the content
  Scenario: Junctions preserve every connected direction
    Given right-only, four-corner, short, compact and flat windows
    Then every visible box glyph matches the union of incident contour segments
    And the contour forms one connected graph
