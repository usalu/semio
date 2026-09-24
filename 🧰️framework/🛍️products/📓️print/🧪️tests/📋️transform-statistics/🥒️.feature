@capability-viz-transform-statistics
@oracle-d3-array
@oracle-d3-regression
@no-oracle-kernel-density
@comparison-viz-probe-v1
Feature: Aggregates, quantiles, density estimates and least-squares fits agree with their references
  The statistical half of `semio-viz-transform` is measured against the two libraries that define
  it. `group`/`rollup` reduces to `d3-array`'s `mean`, `median`, `sum`, `min`, `max` and length;
  `quantile` is `d3-array`'s R-7 `quantile` at the requested probabilities; and the five
  least-squares fits are `d3-regression`'s `regressionLinear`, `regressionPoly`, `regressionLog`,
  `regressionPow` and `regressionExp` — including d3's *weighted* exponential fit, which is not the
  naive fit of `ln y` and is reimplemented as d3 does it.

  The library names its coefficients by role rather than by position: `a` is always the constant
  term of the model and `b` the coefficient of the (possibly transformed) `x`, with `c`, `d`, …
  continuing the polynomial. d3-regression names the slope `a` and the intercept `b` for its linear
  and logarithmic models, and the multiplier `a` and the exponent `b` for its power and exponential
  models, so the adapter maps the two namings onto each other explicitly.

  @no-oracle-kernel-density: d3 ships no kernel density estimator at this version. The `kde`
  operation is therefore checked against an independent second implementation of the same
  specification in the adapter — the estimate is `(1/(n·h)) · Σ K((x − xᵢ)/h)` over an evenly
  spaced grid across the data extent, with the standard Gaussian and Epanechnikov kernels — rather
  than against a re-derivation of the LaTeX code.

  @id-aggregates
  @level-quick
  @mode-differential
  Scenario: Every rollup reduces its group as d3-array does
    Given the committed probe document shared://📋️transform-statistics/transform-statistics.tex and the aggregates
      | table             | group | column | rollups                          |
      | demo-distribution | grp   | value  | mean,median,sum,min,max,count    |
    Then the compiled probe and the reference implementation agree on every aggregate

  @id-quantiles
  @level-quick
  @mode-differential
  Scenario: Quantiles follow the R-7 definition d3-array implements
    Given the committed probe document shared://📋️transform-statistics/transform-statistics.tex and the probabilities
      | probabilities            |
      | 0,0.25,0.5,0.75,1        |
      | 0.1,0.33,0.66,0.9        |
    Then the compiled probe and the reference implementation agree on every quantile

  @id-kde
  @level-quick
  @mode-differential
  Scenario: The kernel density estimate matches an independent implementation of the same formula
    Given the committed probe document shared://📋️transform-statistics/transform-statistics.tex and the kernels
      | kernel       | bandwidth | samples |
      | gaussian     | 1         | 9       |
      | epanechnikov | 1.5       | 9       |
    Then the compiled probe and the reference implementation agree on every density

  @id-regression
  @level-quick
  @mode-differential
  Scenario: The five least-squares fits agree with d3-regression
    Given the committed probe document shared://📋️transform-statistics/transform-statistics.tex and the points
      | x | y    |
      | 1 | 2.1  |
      | 2 | 3.9  |
      | 3 | 6.2  |
      | 4 | 7.8  |
      | 5 | 10.3 |
      | 6 | 11.9 |
    Then the compiled probe and the reference implementation agree on every coefficient
