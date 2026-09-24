@capability-viz-charts-polar
@oracle-d3-scale
@comparison-floating-point-v1
Feature: The polar coordinate places every radial mark where its angle and radius say
  Taxonomy §20's polar family and §5's radar family share one coordinate: an angle measured from
  twelve o'clock and growing clockwise, and a radius that a linear scale maps from the value domain
  onto the band between the inner radius fraction and the frame's inscribed circle. Every leaf of
  those sections differs only in which mark is placed there, so the conformance of the coordinate
  itself is what the whole section rests on.

  d3 has no polar coordinate, but it does have the radius scale: the oracle builds the radial
  extent with `d3-scale`'s linear scale and only adds the two trigonometric projections
  x = cx + r sin θ and y = cy + r cos θ. That keeps the domain handling — which is where an
  off-by-one in the inner radius or a wrong maximum would hide — under a third-party judgement.

  The bar scenario checks the same coordinate through the annular sector instead of the point: bar
  i spans the angular slot [i, i+1) of the full turn inset by the family's hairline gap, and its
  outer radius is the same radius scale, so a sector and a point of the same datum must agree. The
  radar scenario checks the closed profile, whose vertices divide the turn evenly and whose radius
  is the value over the maximum across every series column — the rule that makes several series
  comparable on one grid. The coxcomb scenario checks the equal-area rule: a rose whose radius is
  the square root of the value, so that a sector's AREA, not its radius, carries the number.

  @id-polar-scatter
  @level-long
  @mode-differential
  Scenario: Polar points sit on their angle and radius
    Given the committed probe document shared://🎡️charts-polar-radar/polar-scatter.tex and the frame
      | width | height | pad | inner |
      | 60    | 40     | 4   | 0.15  |
    Then the compiled probe and the reference implementation agree on every point

  @id-polar-bars
  @level-long
  @mode-differential
  Scenario: Radial bars span their angular slot out to the same radius
    Given the committed probe document shared://🎡️charts-polar-radar/polar-bars.tex and the frame
      | width | height | pad | inner |
      | 60    | 40     | 4   | 0.2   |
    Then the compiled probe and the reference implementation agree on every annular sector

  @id-radar
  @level-long
  @mode-differential
  Scenario: A radar profile closes over evenly divided axes
    Given the committed probe document shared://🎡️charts-polar-radar/radar.tex and the frame
      | width | height | pad |
      | 60    | 40     | 4   |
    Then the compiled probe and the reference implementation agree on every vertex

  @id-coxcomb
  @level-long
  @mode-differential
  Scenario: A coxcomb sector carries its value in the area, not the radius
    Given the committed probe document shared://🎡️charts-polar-radar/coxcomb.tex and the frame
      | width | height | pad |
      | 60    | 40     | 4   |
    Then the compiled probe and the reference implementation agree on every annular sector
