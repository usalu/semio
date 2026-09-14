@capability-viz-transform-statistics-twin
@oracle-d3-array
@oracle-d3-regression
@no-oracle-kernel-density
@comparison-viz-probe-v1
Feature: The TypeScript twin's aggregates, quantiles and density estimates agree with their references
  `transform-statistics` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `transform-statistics`'s adapter with the LaTeX probe. It gets its own case instead; the
  vectors below are the vectors of `transform-statistics` verbatim, and the adapter reuses that case's
  oracle handlers unchanged so the two subjects are measured against literally the same reference
  numbers.

  The twin carries `linearRegression` and `polynomialRegression` only, so the least-squares scenario
  of `transform-statistics` — which also fits d3-regression's logarithmic, power and weighted
  exponential models — has no second subject yet and is not restated here.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @no-oracle-kernel-density: d3 ships no kernel density estimator at this version. The `kde`
  operation is therefore checked against an independent second implementation of the same
  specification in the adapter — the estimate is `(1/(n·h)) · Σ K((x − xᵢ)/h)` over an evenly
  spaced grid across the data extent, with the standard Gaussian and Epanechnikov kernels — rather
  than against a re-derivation of the LaTeX code.

  @id-aggregates
  @level-quick
  @mode-differential
  Scenario: Every rollup reduces its group as d3-array does
    Given the twin kernel and the aggregates
      | table             | group | column | rollups                          |
      | demo-distribution | grp   | value  | mean,median,sum,min,max,count    |
    Then the twin kernel and the reference implementation agree on every aggregate

  @id-quantiles
  @level-quick
  @mode-differential
  Scenario: Quantiles follow the R-7 definition d3-array implements
    Given the twin kernel and the probabilities
      | probabilities            |
      | 0,0.25,0.5,0.75,1        |
      | 0.1,0.33,0.66,0.9        |
    Then the twin kernel and the reference implementation agree on every quantile

  @id-kde
  @level-quick
  @mode-differential
  Scenario: The kernel density estimate matches an independent implementation of the same formula
    Given the twin kernel and the kernels
      | kernel       | bandwidth | samples |
      | gaussian     | 1         | 9       |
      | epanechnikov | 1.5       | 9       |
    Then the twin kernel and the reference implementation agree on every density
