@capability-staffage-front
Feature: A component seen from the front keeps its own drawing
  The front geometry of a representation is authored in front coordinates (x across the width from the centre, y up from the floor, metres). A front shape
  follows the bone it is bound to only by the bone's y travel from the rest pose. The elevation stays the rig plane (the section or the side view), so a
  radiator hung on a rear wall is cast with the viewpoint `front` and one against a cut wall with the elevation.

  THE REFERENCE is gl-matrix `mat2d`, an independent forward kinematics of the rig; the dimensions of the radiator are the EN 442 type 22 catalogue values
  (1000 mm long, 600 mm high, 110 mm above the floor) and the cast sizes are read back from the exported index.

  Scenario: A front shape follows the y travel of its bone only
    Given a rectangle on the shank and a rectangle on the thigh of the sample figure
    When the stride pose is evaluated in the front elevation
    Then the shank rectangle is 0.032767 m higher than the thigh rectangle relative to the rest pose and not moved sideways

  Scenario: The radiator is drawn from the front at every scale
    Given the panel radiator and the split air conditioner indoor unit
    When the front elevation is evaluated at 1:50, 1:100 and 1:200
    Then every scale draws front shapes

  Scenario: The radiator front has its catalogue size and a vertical temperature gradient
    Given the front outline of the radiator at 1:50 and the rated pose
    When the outline is evaluated and the thermal field is solved
    Then the outline is 1.0 m wide and from 0.11 m to 0.71 m high and the upper third of the front plate is at least the `minSpreadK` of the vectors warmer than the lower third

  Scenario: A front cast is as wide and as high as the front
    Given a front cast and a side cast of the radiator
    When the casts are exported
    Then the front cast is 1.0 to 1.2 m wide and 0.69 to 0.73 m high and the side cast 0.3 to 0.45 m wide (0.135 m of radiator and brackets plus the ground overhang) and 0.69 to 0.73 m high (the radiator and its brackets, no wall)

  Scenario: A representation without front or plan geometry is refused
    Given the sample figure with its 1:100 front drawing emptied
    When the asset is validated
    Then the validator reports `front-empty` at the front geometry of 1:100, and `plan-empty` likewise for an emptied plan, with a German message
