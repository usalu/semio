@capability-viz-charts-dashboard
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: A gauge sweeps the arc d3-shape's arc generator sweeps
  A gauge is an angle: the value's share of the domain decides how far the coloured arc travels
  between `startAngle` and `endAngle`, and every other gauge mode of taxonomy §18 — dial,
  speedometer, progress ring, completion donut — is that same sweep under different angle bounds and
  a different thickness. The family emits both the arc's parameters and the point at its mid-angle,
  so the sweep can be checked against `d3-shape`'s `arc()` rather than eyeballed.

  The two conventions differ and the case converts between them rather than hiding the difference:
  the family measures degrees counter-clockwise from the positive x axis, d3 measures radians
  clockwise from twelve o'clock, and d3's y axis points down while the millimetre canvas points up.
  A d3 angle is therefore `(90 - degrees) * pi / 180`, and a d3 centroid `[cx + x, cy - y]`.

  @id-radial-gauge
  @level-long
  @mode-differential
  Scenario: A radial gauge sweeps from 180 to 0 degrees
    Given the committed probe document local://radial-gauge.tex and the gauge
      | value | min | max | startAngle | endAngle |
      | 72    | 0   | 100 | 180        | 0        |
    Then the compiled probe and the reference implementation agree on the arc centroid

  @id-progress-ring
  @level-long
  @mode-differential
  Scenario: A progress ring sweeps a full turn from twelve o'clock
    Given the committed probe document local://progress-ring.tex and the gauge
      | value | min | max | startAngle | endAngle |
      | 72    | 0   | 100 | 90         | -270     |
    Then the compiled probe and the reference implementation agree on the arc centroid
