@capability-staffage-model-silhouette
Feature: The volume agrees with the authored silhouettes
  In its presentation pose the volume projected into the elevation, front and plan covers the authored closed shapes (open strokes stroked with their pen width) with an IoU of at least
  0.80 and fills `dimensions` within 8 % on every axis (the authoring gate of volume-drafting). The sample figure and three catalog assets (a table, a bed, a beam) are checked here; the whole catalog is measured
  by the ticket report.

  THE REFERENCES are polygon-clipping (exact union and intersection areas) and the dimensions of the asset; an owned scanline raster must agree with the exact IoU.

  Scenario: The extent fills the dimensions
    Given an asset in its presentation pose
    When the solids are tessellated and carried to the world
    Then width, height and depth are within 8 % of `dimensions`

  Scenario: Each view agrees
    Given the authored shapes and the projected solids of a view
    When their exact IoU is computed
    Then it is at least 0.80 and the owned raster IoU is within 0.03 of it
