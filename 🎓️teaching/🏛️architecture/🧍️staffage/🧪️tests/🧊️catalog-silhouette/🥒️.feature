@capability-staffage-catalog-silhouette
Feature: Every asset of the catalog is one body in its views and its volume
  One physical body per asset: in `presentation.defaultPose` at 1:50 the side elevation, the front elevation, the plan and the volume are projections of the same body. The bounding box of the volume is within 8 % of
  `dimensions` on every axis and the projection of the volume agrees with each depicted view with an IoU of at least 0.80, for all 152 assets, without an exemption (the fixture states none).
  The drawing side is what `evaluate` draws: filled shapes as areas, open strokes and unfilled outlines at the true width of the member they draw (the volume gives that width; a stroke on the boundary of a body counts two
  cells wide inside it, one on nothing keeps its pen width). A view declared as a section is compared with the volume cut by the plane: the drawing shows every cut face, may fill what remains visible beyond the plane and
  may outline what lies in front of it (overhead and hidden lines), and shows nothing that is not there.

  THE REFERENCES are sharp (librsvg rasterises the same polygons; within 0.06 of the owned raster), polygon-clipping (exact areas for five assets), a numpy and shapely mesh-plane section for the cut faces, the hand-computed
  areas and widths of the fixture and the physical dimensions of the asset.

  Scenario: Every asset is found once and none is excused
    Given the catalog
    Then 152 assets or more are found, each once, and the fixture excuses none

  Scenario: The cut face of a solid
    Given a box, a plate with a hole, a cylinder cut through its axis and a sphere cut off its centre
    When the plane cuts the tessellated solid
    Then the net area of the loops is the hand-computed area and equals the numpy and shapely section

  Scenario: The width of a drawn member
    Given a body mask and a stroke
    When the member of the stroke is measured
    Then a stroke on a bar is as wide as the bar, on the boundary of a plate two cells wide inside it, on nothing its pen width, beside a filled shape two cells wide and on a wide body the cap

  Scenario: Each asset agrees in each view
    Given an asset in its presentation pose
    When its volume is tessellated, carried by the bone frames and projected into the three views
    Then the bounding box is within 8 % of the dimensions, the IoU of each view is at least 0.80 and the sharp rasterisation agrees within 0.06
