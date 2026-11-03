@capability-staffage-three-views
Feature: Every staffage has a top, a front and a side view for every pose, clip and scale
  Every authored representation draws the side view (elevation), the front view and the top view (plan), each in rest-pose model metres bound to the same bones.
  The elevation x is forward and y up. The front x is lateral (the left of the asset is +x, centred on its symmetry plane) and y up. The plan x is forward
  (the elevation x) and the plan y is lateral (the left is +y, the front x). The three views of the default pose are one object: they share their ranges and
  match the dimensions of the asset (`widthM` the forward extent, `depthM` the lateral extent, `heightM` the height) within the larger of 2 cm and 3 % of the
  largest dimension. Ground shapes do not count.

  THE REFERENCE is shapely: it recomputes the extents of the hand-computed vectors and of the exported evaluated rings.

  Scenario: The hand-computed vectors give the extents of each view
    Given a synthetic asset on the sample figure rig with a block, a stretching thigh and a limb capsule in every view
    When the rest pose and the stride pose are evaluated in elevation, front and plan
    Then the extents of every view equal the hand-computed numbers

  Scenario: A third-party geometry library recomputes the same extents
    Given the exported evaluated rings of the vector cases
    When shapely measures the union of the rings of every view
    Then its bounds equal the extents of the evaluation and of the vectors

  Scenario: Every authored representation draws all three views
    Given every asset of the catalog and every authored scale
    When the views are counted
    Then no asset has an authored representation without elevation, front or plan shapes

  Scenario: Every pose and clip instant is drawn in every view
    Given every pose and six instants of every clip of every authored representation
    When the three views are evaluated
    Then each view has geometry with finite bounds

  Scenario: The three views of the default pose are one object
    Given the default pose of every authored representation
    When the extents of the views are compared
    Then the elevation x equals the plan x, the elevation y equals the front y, the front x equals the plan y and the sizes match the dimensions

  Scenario: The dimensions are read as forward, lateral and vertical extent
    Given the complete references solar-thermal-collector and split-ac-indoor-unit
    When the views of the default pose are measured
    Then widthM is the elevation x extent, depthM the lateral extent and heightM the height at every scale
