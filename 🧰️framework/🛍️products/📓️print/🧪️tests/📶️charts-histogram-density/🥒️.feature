@capability-viz-charts-distribution
@oracle-d3-array
@comparison-floating-point-v1
Feature: The distribution transforms bin, estimate and accumulate exactly like d3-array
  `semio-viz-charts-distribution` carries the statistical kernel every distribution family reads:
  the binning of taxonomy §3's histograms, the tick generation that produces their thresholds, the
  kernel density estimate behind the curve, ridgeline and violin families, and the empirical
  distribution function behind the CDF families. None of that is drawing — it is arithmetic that a
  reader trusts without being able to check it, which is exactly why it is measured here against
  the implementation it was ported from rather than against itself.

  Binning is `d3-array`'s `bin().thresholds(n)` in full: the extent is niced by repeated
  `tickIncrement` until the step is stable, `ticks` lays the thresholds over the niced domain, and
  the last threshold is either popped or the upper bound is pushed out by one increment — the rule
  that decides between the two is whether the DATA maximum reaches the niced bound, and getting it
  wrong costs one bin, so both branches have a scenario. The thresholds that fall on the domain
  edges are then trimmed, and every observation is placed by a bisect-right.

  The kernel density estimate has no d3 equivalent, so the oracle is a TypeScript reimplementation
  of the same textbook formula — a gaussian kernel on a regular grid with Silverman's rule of thumb
  h = 1.06 * min(sd, IQR/1.349) * n^(-1/5) — whose two inputs, the deviation and the quartiles, come
  from `d3-array`. That keeps the parts a reader would get wrong (the spread estimate and the
  quantile interpolation) under a third-party judgement, and leaves only the summation to the
  oracle. A second scenario fixes the bandwidth explicitly so a bandwidth bug cannot hide inside a
  compensating rule-of-thumb bug.

  @id-bins
  @level-quick
  @mode-differential
  Scenario: Ten thresholds over demo-distribution reproduce d3's bins
    Given the committed probe document shared://📶️charts-histogram-density/bins.tex
    Then the compiled probe and the reference implementation agree on every bin edge and count

  @id-bins-coarse
  @level-quick
  @mode-differential
  Scenario: Four thresholds take the branch that pops the last threshold
    Given the committed probe document shared://📶️charts-histogram-density/bins-coarse.tex
    Then the compiled probe and the reference implementation agree on every bin edge and count

  @id-ticks
  @level-quick
  @mode-differential
  Scenario: The tick generator agrees with d3 on integer, fractional and sub-unit steps
    Given the committed probe document shared://📶️charts-histogram-density/ticks.tex
    Then the compiled probe and the reference implementation agree on every tick

  @id-density
  @level-quick
  @mode-differential
  Scenario: The gaussian kernel density estimate matches the textbook formula
    Given the committed probe document shared://📶️charts-histogram-density/density.tex
    Then the compiled probe and the reference implementation agree on the bandwidth and the curve

  @id-ecdf
  @level-quick
  @mode-differential
  Scenario: The empirical CDF steps once per sorted observation
    Given the committed probe document shared://📶️charts-histogram-density/ecdf.tex
    Then the compiled probe and the reference implementation agree on every step
