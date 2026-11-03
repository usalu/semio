@capability-staffage-evaluation
Feature: One three-dimensional chain feeds every view
  Every evaluated state carries the full 3D chain of the rig: the frame of each bone (origin, longitudinal, normal and lateral axes), its tip, the lateral z of every anchor, the root and the
  centre-of-mass kinematics. The elevation, front and plan views are projections of that chain, so a part keeps its size in every view whatever its bones do. `timeS` is the absolute query time and
  `clipTimeS` the wrapped clip phase; accelerations are one-sided near the ends of a clip, so they are continuous there.

  Scenario: The state matches the solved chain
    Given the herring gull in the soaring pose
    When the pose is evaluated
    Then every bone frame, tip and anchor z equals the three-dimensional chain

  Scenario: Rotating a child bone leaves the solid of its parent unchanged
    Given the charcoal grill with its lid solid revolved about a fixed axis bone
    When the vent bone is turned by 70 degrees
    Then the lid has the same extent in the side, front and plan views

  Scenario: A revolved part on a laterally offset bone keeps its real radius
    Given the person with the umbrella in every pose
    When the three views are evaluated at 1:50
    Then the umbrella canopy is never wider than the real canopy diameter

  Scenario: Absolute time and clip phase are separate
    Given a looping clip queried in its third cycle
    When the clip is evaluated
    Then the time is the absolute query time and the clip time is the wrapped phase

  Scenario: Accelerations are continuous at the ends of a clip
    Given a non-looping transition
    When the accelerations are read just inside and exactly at an end
    Then they differ by less than the acceleration scale of the clip

  Scenario: Scrubbing equals playing
    Given a clip evaluated forwards on one copy of the asset and backwards on another
    Then both give the same state at every time
