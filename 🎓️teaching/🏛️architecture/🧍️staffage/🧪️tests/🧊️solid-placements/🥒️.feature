@capability-staffage-solid-placements
Feature: Revolved and extruded parts keep their solid shape in the front and plan views at every pose
  A front or plan shape may be a solid of its rig-plane profile. The profile is authored in the bone's local frame like an elevation shape and placed with the
  full bone transform. A revolved part (umbrella canopy, rib, lid, drum) turns about the vertical axis through the origin of an axis bone; its front paper x is
  the lateral distance from that axis and its plan is the circle of its widest lateral reach. An extruded part (blind slat, damper flap, door panel, boom)
  spans a lateral range z0..z1 and its front or plan is that range by the height or forward range of the transformed profile.

  THE REFERENCE is shapely: it rotates the profile polygon by the hand-computed world angle of the bone and builds the mirrored, circular and rectangular
  solids with its own geometry operations.

  Scenario: The hand-computed vectors give the extents of both kinds
    Given a rectangle profile on the left thigh of the sample figure as canopy, offset half profile, crown and slat
    When the rest pose and the stride pose are evaluated in the front and the plan
    Then every extent equals the hand-computed number relative to the pelvis origin

  Scenario: A third-party geometry library builds the same solids
    Given the same profile rotated by the world angle of its bone
    When shapely mirrors, buffers and extrudes it
    Then its bounds equal the extents of the evaluation
