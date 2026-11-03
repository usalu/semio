@capability-staffage-plan
Feature: A figure seen from above follows its pose
  The plan geometry of a representation is authored in plan coordinates (x ahead, y sideways, metres). A plan shape follows the bone it is bound to only by the
  bone's x travel from the rest pose. A plan capsule with `followTip` stretches: its first end follows the origin of the bone and its second end the tip of the
  bone, so an arm, a leg or a backrest seen from above grows and shortens with the pose. Plan casts are sized to the plan, not to the elevation.

  THE REFERENCE is gl-matrix `mat2d`, an independent forward kinematics of the rig; the cast depth is read back from the exported index.

  Scenario: A limb capsule stretches with a stride
    Given a capsule of radius 0.08 m on a thigh whose tip travels 0.168573 m ahead at a stride
    When the stride pose is evaluated in plan
    Then the capsule reaches 0.228573 m ahead and a rigid capsule on the same bone has not moved

  Scenario: A shape of a child bone follows the parent tip
    Given a plan triangle bound to the shank
    When the stride pose is evaluated in plan
    Then the triangle is shifted by the thigh tip travel

  Scenario: The people and the car are drawn from above at every scale
    Given the standing adult, the walking commuter, the seated office worker and the city car
    When the default pose is evaluated at 1:50, 1:100, 1:200 and 1:500
    Then every scale draws plan shapes, a person is about half a metre across the arms and the car is 3.9 m long

  Scenario: A foot and the shoulders track the pose
    Given the walking commuter at contact and in passing, and the seated office worker typing and leaning back
    When the plan is evaluated
    Then the shoe capsule matches the independent kinematics and the shoulders move backwards with the lean

  Scenario: A plan cast is as deep as the plan
    Given a plan cast of the commuter and of the car and an elevation cast of the car
    When the casts are exported
    Then the plan casts are 0.55 to 0.8 m and 1.68 to 2.05 m deep and the elevation cast 1.4 to 1.6 m
