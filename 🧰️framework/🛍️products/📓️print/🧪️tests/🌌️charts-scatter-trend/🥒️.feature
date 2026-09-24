@capability-viz-charts-scatter
@oracle-d3-regression
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: A scatter's trend line is the least-squares fit d3-regression computes
  `trend=linear` on the `scatter` family is ordinary least squares over the raw data, computed in
  data space and only then mapped through the scales — so the fit does not depend on the plot's
  padding, its aspect ratio or the millimetre grid. The family emits the slope and the intercept it
  computed, which is exactly the pair `d3-regression`'s `regressionLinear` returns as `a` and `b`.

  The size encoding is the second measurable thing here: `size=<column>` is a square-root scale from
  the column's extent onto a radius, because a bubble's AREA is what a reader compares. The bubble
  scenario checks the radii the family drew against that construction on `d3-scale`.

  @id-linear-trend
  @level-long
  @mode-differential
  Scenario: The trend line is d3-regression's least-squares fit
    Given the committed probe document shared://🌌️charts-scatter-trend/linear-trend.tex and the points
      | x  | y   |
      | 1  | 2.4 |
      | 2  | 3.1 |
      | 3  | 3.0 |
      | 4  | 4.6 |
      | 5  | 4.4 |
      | 6  | 6.1 |
      | 7  | 6.0 |
      | 8  | 7.4 |
      | 9  | 7.1 |
      | 10 | 8.6 |
    Then the compiled probe and the reference implementation agree on the slope and the intercept

  @id-bubble-size
  @level-long
  @mode-differential
  Scenario: A size encoding is a square-root scale onto the radius
    Given the committed probe document shared://🌌️charts-scatter-trend/bubble-size.tex and the sizes
      | values                | rmin | rmax |
      | 3,5,2,7,4,6,3,8,5,4   | 0.7  | 2.8  |
    Then the compiled probe and the reference implementation agree on every radius
