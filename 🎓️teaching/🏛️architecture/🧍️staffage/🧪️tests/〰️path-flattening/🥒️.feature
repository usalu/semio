@capability-staffage-evaluation
Feature: Paths flatten to polylines within 0.5 millimetres
  An SVG path (M L H V C S Q T A Z, absolute and relative, y up, metres) becomes closed rings or open polylines. Straight commands keep their exact vertices.
  Cubic and quadratic Béziers are subdivided adaptively, elliptical arcs by the angle whose sagitta is below the tolerance, so that every flattened
  point lies on the analytic curve and every analytic point lies within 0.0005 m of the polyline.

  THE REFERENCE is three.js: `CubicBezierCurve`, `QuadraticBezierCurve` and `EllipseCurve` sampled densely; the test measures the distance both ways.

  Scenario: A cubic bump
    Given the path "M 0 0 C 0 2 3 2 3 0"
    When it is flattened
    Then no point of the dense reference curve is farther than 0.5 mm from the polyline

  Scenario: Relative and absolute forms agree
    Given "m 0 0 l 2 0 l 0 1 z" and "M 0 0 L 2 0 L 2 1 Z"
    When both are flattened
    Then both give the ring (0, 0), (2, 0), (2, 1), closed
