@capability-staffage-dimension-clearance
Feature: Dimension lines stand clear of the drawing in every view and at every scale
  The overall dimension (extension lines, dimension line, ticks, leader) is placed outside the drawn geometry with a clear gap, as ISO 129-1 asks: the extension lines start 1.5 mm from the
  drawing and the dimension line stands 6 mm beyond it, in the side, front and top view alike, so that at 1:500 and 1:200 a few millimetres of glyph are never crossed.

  THE REFERENCE is shapely (🐍️.py): it builds the bounding box of the drawn geometry and measures the distance of every dimension line to it.

  Scenario: Retractable bollard at 1:500 and street lamp at 1:200
    Given the retractable bollard at 1:500 and the street lamp at 1:200 in all three viewpoints
    When the dimension items are separated from the drawn geometry
    Then no dimension item intersects the geometry box and every one keeps at least 1 mm from it

  Scenario: Every authored scale of both assets
    Given every authored scale of both assets in all three viewpoints
    When the same measurement is made
    Then no dimension item intersects the geometry box
