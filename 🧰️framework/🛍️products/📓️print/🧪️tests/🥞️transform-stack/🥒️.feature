@capability-viz-transform-stack
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: The stack transform produces the baselines d3-shape's stack produces
  `\SemioVizTransform{out}{in}[stack=<x>:<series>:<value>]` is `d3-shape`'s stack over a long-form
  table: it widens the table by series, decides in which sequence the series are laid on top of
  each other, cumulates them, and then moves the whole column onto a baseline. Order and offset are
  separate decisions, so they are separate scenarios, and both `y0` and `y1` of every series are
  measured — a stack whose tops are right and whose bottoms are wrong is still wrong.

  The orders are d3's own: `none` keeps the series order, `ascending` sorts by series total,
  `descending` reverses that, `reverse` reverses the given order, `appearance` sorts by the
  position of each series' largest value, and `inside-out` lays the appearance order alternately
  outwards from the middle. The offsets are `none`, `expand`, `diverging`, `silhouette` and
  `wiggle`.

  d3's `stack` returns its series in key order and records the stacking sequence separately, so the
  order scenarios compare the stacking sequence itself — `stackOrderAscending` applied to the same
  series — rather than the order of the returned array.

  The data is the shipped `demo-series` table: three series over eight time points.

  @id-order-none
  @level-quick
  @mode-differential
  Scenario: Without an order the series stack in the order they appear
    Given the committed probe document local://transform-stack.tex and the series
      | series | values                |
      | alpha  | 4,7,6,9,8,12,11,14    |
      | beta   | 2,3,5,4,7,6,9,8       |
      | gamma  | 6,5,3,5,2,4,3,5       |
    Then the compiled probe and the reference implementation agree on every baseline

  @id-order-ascending
  @level-quick
  @mode-differential
  Scenario: The ascending order stacks the smallest total first
    Given the committed probe document local://transform-stack.tex and the same series
      | order     |
      | ascending |
    Then the compiled probe and the reference implementation agree on the stacking order and every baseline

  @id-order-reverse
  @level-quick
  @mode-differential
  Scenario: The reverse order stacks the given order backwards
    Given the committed probe document local://transform-stack.tex and the same series
      | order   |
      | reverse |
    Then the compiled probe and the reference implementation agree on the stacking order and every baseline

  @id-offset-expand
  @level-quick
  @mode-differential
  Scenario: The expand offset rescales every column onto the unit interval
    Given the committed probe document local://transform-stack.tex and the same series
      | offset |
      | expand |
    Then the compiled probe and the reference implementation agree on every baseline

  @id-offset-silhouette
  @level-quick
  @mode-differential
  Scenario: The silhouette offset centres every column on the horizontal axis
    Given the committed probe document local://transform-stack.tex and the same series
      | offset     |
      | silhouette |
    Then the compiled probe and the reference implementation agree on every baseline

  @id-offset-diverging
  @level-quick
  @mode-differential
  Scenario: The diverging offset stacks negative values below the axis
    Given the committed probe document local://transform-stack.tex and the signed series
      | series | values  |
      | up     | 3,4,2   |
      | down   | -2,-5,-1|
      | mixed  | 1,-3,4  |
    Then the compiled probe and the reference implementation agree on every baseline
