@capability-staffage-rig
Feature: A rig is solved parent-first into world transforms
  A bone's local frame has its origin at the joint and +x along the bone; its transform in the parent frame is translate(rest) · rotate(rest angle +
  offset). The root bone hangs from the root placement (x, y, angle). Revolute offsets are clamped to [minDeg, maxDeg]; prismatic offsets are projected
  on the axis and clamped to [minM, maxM]; fixed joints ignore offsets. A missing parent, a cycle or several roots are errors.

  THE REFERENCE is gl-matrix `mat2d`, which uses the same [a b c d tx ty] layout; the test composes the chain with `mat2d.multiply`/`rotate`/`translate`
  and compares to the solver within 1e-12.

  Scenario: A two-link arm with a bent elbow
    Given a shoulder of 1 m and an elbow of 0.5 m placed at (1, 2) with the root rotated 90 degrees
    When the elbow is bent by 90 degrees
    Then the elbow origin is (1, 3), its tip (0.5, 3) and its world angle 180 degrees

  Scenario: Joint limits clamp offsets
    Given a shoulder limited to ±90 degrees
    When an offset of 200 degrees is applied
    Then the shoulder solves to 90 degrees
