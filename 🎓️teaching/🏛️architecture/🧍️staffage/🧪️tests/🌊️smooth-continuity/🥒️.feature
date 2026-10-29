@capability-staffage-animation
Feature: Smooth keys have continuous velocity
  A key with ease `smooth` starts a cubic Hermite (Catmull-Rom) segment through the neighbouring keys: the tangent at a key is the finite difference
  (v[next] − v[previous]) / (t[next] − t[previous]), wrapping around for looping clips and zero at the ends of non-looping clips. Bone offsets (angles
  in degrees, translations in metres), the authored root and scalar tracks follow this rule; `linear`, `in`, `out` and `step` keep blending the two keys
  by the eased weight. A looping clip drops a last key at or beyond its duration that repeats the first key. Planted root motion and the central
  differences of the kinematics inherit the continuity.

  THE REFERENCE is three.js `CatmullRomCurve3` (closed, `catmullrom`, tension 0.5), which is the uniform Catmull-Rom spline through the same keys.

  Scenario: Velocity does not jump at a key
    Given the sparse walk with four smooth keys
    When a bone angle is differenced on both sides of every key
    Then the two velocities agree within 0.05 degrees per second

  Scenario: The loop seam is seamless
    Given a looping clip
    When it is sampled just before and just after its duration
    Then position and velocity agree

  Scenario: Linear keys do jump
    Given the same clip with ease linear
    When it is differenced across a key
    Then the velocities differ by more than 1 degree per second
