@capability-viz-charts-statistical
@oracle-d3-array
@comparison-floating-point-v1
Feature: The evaluation family sweeps a classifier's thresholds the way the textbook definitions do
  Taxonomy §24's ROC, precision-recall, DET, lift and gains curves are one sweep seen through
  different projections: sort the scored observations descending, walk them, and after each step
  report a pair of rates. Every one of those curves is therefore wrong or right in the same place —
  the ordering and the running counts — which is what this case measures.

  d3 has no classifier metrics, so the oracle reimplements the sweep in TypeScript with `d3-array`
  supplying the pieces a reimplementation gets wrong: the descending order over the score column
  and, for the confusion matrix, the median threshold through `quantileSorted`. The sweep itself is
  four counters, and stating them in the oracle is the point: the feature, not the implementation,
  owns the definition that the ROC starts at the origin, that the PR curve does not, and that the
  gains curve counts population rather than false positives.

  The AUC is the same walk accumulated rather than plotted: each negative observation contributes
  the true positives seen so far over the product of the class sizes, which is the rank-sum
  definition and not a trapezoid over the plotted points, so a curve with the right shape but the
  wrong ordering still fails.

  @id-roc
  @level-quick
  @mode-differential
  Scenario: The ROC sweep and its rank-sum AUC
    Given the committed probe document shared://🎯️charts-evaluation-curves/roc.tex
    Then the compiled probe and the reference implementation agree on every rate and on the area

  @id-pr
  @level-quick
  @mode-differential
  Scenario: The precision-recall sweep starts at the first ranked observation
    Given the committed probe document shared://🎯️charts-evaluation-curves/pr.tex
    Then the compiled probe and the reference implementation agree on every rate

  @id-gain
  @level-quick
  @mode-differential
  Scenario: The cumulative-gains sweep counts population against captured positives
    Given the committed probe document shared://🎯️charts-evaluation-curves/gain.tex
    Then the compiled probe and the reference implementation agree on every rate

  @id-confusion
  @level-long
  @mode-differential
  Scenario: The confusion matrix counts the four outcomes at the median threshold
    Given the committed probe document shared://🎯️charts-evaluation-curves/confusion.tex
    Then the compiled probe and the reference implementation agree on the four counts
