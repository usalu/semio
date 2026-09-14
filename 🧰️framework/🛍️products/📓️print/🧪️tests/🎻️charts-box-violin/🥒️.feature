@capability-viz-charts-distribution
@oracle-d3-array
@comparison-floating-point-v1
Feature: The box and violin families summarise a sample the way d3-array's quantiles do
  Taxonomy §3's box family and violin family are the same five-number summary seen twice: the box
  draws it as a rectangle with two whiskers, the violin wraps a kernel density silhouette around it.
  A box plot that draws its quartiles with the wrong interpolation rule still looks like a box plot,
  so the summary is emitted as a `geometry/box` record — lower whisker, first quartile, median,
  third quartile, upper whisker — and compared against `d3-array`'s `quantileSorted`, which is the
  R-7 rule the family claims to implement.

  Whiskers are Tukey's: the most extreme observation still inside 1.5 IQR of the nearer quartile,
  never the fence itself, so a group whose extremes lie inside the fence must report its own
  minimum and maximum. `demo-distribution` has three group levels whose spreads differ, which is
  what makes that visible.

  The letter-value (boxen) depth is the same quantile function evaluated at 2^-(k+1) and its
  complement, so a boxen plot is a stack of quantile pairs and the oracle can state it directly.
  The violin scenario checks the composition rather than the silhouette: a violin with an inner box
  must emit the same five-number summary as the box family does for the same group, so the two
  families cannot drift apart.

  @id-quartiles
  @level-quick
  @mode-differential
  Scenario: The quantile function is d3's R-7 interpolation
    Given the committed probe document local://quartiles.tex
    Then the compiled probe and the reference implementation agree on every quantile

  @id-whiskers
  @level-long
  @mode-differential
  Scenario: Every group reports its Tukey five-number summary
    Given the committed probe document local://whiskers.tex
    Then the compiled probe and the reference implementation agree on every box summary

  @id-letter
  @level-long
  @mode-differential
  Scenario: A boxen plot reports the quantile pairs of its letter depths
    Given the committed probe document local://letter.tex
    Then the compiled probe and the reference implementation agree on every letter value

  @id-violin
  @level-long
  @mode-differential
  Scenario: A boxed violin summarises its groups exactly like the box family
    Given the committed probe document local://violin.tex
    Then the compiled probe and the reference implementation agree on every box summary
