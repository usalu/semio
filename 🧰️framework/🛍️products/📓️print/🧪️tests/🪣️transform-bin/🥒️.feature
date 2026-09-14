@capability-viz-transform-bin
@oracle-d3-array
@comparison-viz-probe-v1
Feature: The bin transform lays out the bins d3-array's bin lays out
  `\SemioVizTransform{out}{in}[bin=<column>]` is `d3-array`'s `bin`: the extent of the column is
  rounded by `nice` to the requested bin count, the ticks of that rounded extent become the
  thresholds, a threshold coincident with the upper bound extends the extent by one increment when
  the data maximum reaches it and is dropped otherwise, and the thresholds outside the extent are
  trimmed. That whole rule is what makes bins uniform, so the case measures the edges `x0` and `x1`
  as well as the counts — a histogram with the right counts on the wrong edges is still wrong.

  The default bin count is Sturges' rule, `ceil(log2(n)) + 1`; `thresholds=count:<n>` asks for a
  count; and an explicit `a;b;c` list bypasses the rule entirely, keeping the data extent as the
  outer edges. The data is the shipped `demo-distribution` table, 48 values in three clusters.

  @id-sturges
  @level-quick
  @mode-differential
  Scenario: Without a threshold option the bin count follows Sturges' rule
    Given the committed probe document local://transform-bin.tex and the sample
      | table             | column | thresholds |
      | demo-distribution | value  | sturges    |
    Then the compiled probe and the reference implementation agree on every edge and count

  @id-count-five
  @level-quick
  @mode-differential
  Scenario: A requested count of five produces the niced bins d3 produces
    Given the committed probe document local://transform-bin.tex and the sample
      | table             | column | thresholds |
      | demo-distribution | value  | count:5    |
    Then the compiled probe and the reference implementation agree on every edge and count

  @id-count-twenty
  @level-quick
  @mode-differential
  Scenario: A requested count of twenty produces half-unit bins
    Given the committed probe document local://transform-bin.tex and the sample
      | table             | column | thresholds |
      | demo-distribution | value  | count:20   |
    Then the compiled probe and the reference implementation agree on every edge and count

  @id-explicit-thresholds
  @level-quick
  @mode-differential
  Scenario: An explicit threshold list keeps the data extent as the outer edges
    Given the committed probe document local://transform-bin.tex and the sample
      | table             | column | thresholds |
      | demo-distribution | value  | 4;6;8      |
    Then the compiled probe and the reference implementation agree on every edge and count
