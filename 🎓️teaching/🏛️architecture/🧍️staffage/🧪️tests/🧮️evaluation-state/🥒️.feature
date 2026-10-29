@capability-staffage-evaluation
Feature: One evaluated state feeds every view
  `evaluate(asset, query)` returns the world-space state of a pose, a clip at any time or a transition: bone transforms, shapes flattened to rings (capsule ≥ 24,
  ellipse ≥ 32 points), anchors, contacts, loads as world forces, thermal inputs, bounds and kinematics. Distributed loads become eight equivalent point forces at
  the midpoints of eight equal sub-segments, so their resultant and centroid are exact. Load tracks scale loads. The result is a pure function of the input:
  evaluating twice gives bit-identical state. The mean centre-of-mass velocity of a planted looping clip equals advanceM / duration.

  Scenario: A distributed load is converted to eight resultants
    Given 100 N/m over 0.4 m of the torso
    When the pose is evaluated
    Then eight forces act at the sub-segment midpoints and sum to 40 N

  Scenario: Unknown ids are reported clearly
    Given an unknown pose id
    When the pose is evaluated
    Then an error names the asset and the poses it has
