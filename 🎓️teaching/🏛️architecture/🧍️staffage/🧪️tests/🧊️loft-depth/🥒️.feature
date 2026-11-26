@capability-staffage-loft-depth
Feature: A loft in the side elevation is the depth twin of its bone
  A flat stroke of the front elevation or the plan has no depth of its own: it is carried at the depth of the flat strokes that its bone draws in the side elevation,
  so a part far from the bone origin pivots about its true place. A foliage mass drawn as one loft in the three views leaves the bone without a flat stroke in the side
  elevation; the loft is its silhouette there and counts as the twin exactly as an authored flat outline of the same silhouette does.

  THE REFERENCE is the engine's own path for a flat outline in the side elevation, fed with the rest outline of the very loft in the bone frame; the lofts themselves are
  held against gl-matrix, shapely and the kernel by the loft-shape scenarios.

  Scenario: The loft and its flat outline carry a stroke alike
    Given a loft on the left thigh of the sample figure in the three views and a flat stroke in the front and the plan
    And the same asset with the rest outline of that loft as a flat polygon in the side elevation
    When every pose is evaluated
    Then the rings of the stroke are the same in the front and the plan

  Scenario: The twin is read
    Given the same stroke on a bone without a shape in the side elevation
    When the bone turns in a pose
    Then the stroke is carried to another place than with the loft as its twin
