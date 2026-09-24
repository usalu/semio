@capability-viz-charts-line
@capability-viz-charts-area
@oracle-d3-shape
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: A line and an area draw the vertices d3-shape would draw
  A curve is not decoration: `curve=step-after` inserts a vertex that `curve=linear` does not, and a
  reader can tell a step chart from a line chart precisely because of those extra vertices. The
  `line` family therefore records the vertex list it actually emitted — not the data points it
  started from — so that the interpolator is measurable rather than merely visible.

  The oracle is `d3-shape`'s own `line()` with `curveLinear`, `curveStepAfter` and `curveStepBefore`
  over `d3-scale` linear scales; the adapter reads the vertices back out of the path d3 generates,
  so the comparison is against d3's output, not against a restatement of d3's rule.

  The area family stacks with `d3-shape`'s `stack()`: the upper boundary of layer k is the layer's
  `y1` and its lower boundary is `y0`, both mapped through the value scale. The family draws the
  band between the two and then strokes the upper boundary, so a stacked area emits three vertex
  lists per layer — upper, lower, upper again — and the case compares each of them.

  @id-linear
  @level-long
  @mode-differential
  Scenario: A linear curve emits exactly the mapped data points
    Given the committed probe document shared://📈️charts-line-area/linear.tex and the frame
      | width | height | padLeft | padRight | padTop | padBottom |
      | 80    | 40     | 10      | 4        | 4      | 8         |
    Then the compiled probe and the reference implementation agree on every vertex

  @id-step-after
  @level-long
  @mode-differential
  Scenario: A step-after curve inserts the horizontal vertex d3 inserts
    Given the committed probe document shared://📈️charts-line-area/step-after.tex and the frame
      | width | height | padLeft | padRight | padTop | padBottom |
      | 80    | 40     | 10      | 4        | 4      | 8         |
    Then the compiled probe and the reference implementation agree on every vertex

  @id-step-before
  @level-long
  @mode-differential
  Scenario: A step-before curve inserts the vertical vertex d3 inserts
    Given the committed probe document shared://📈️charts-line-area/step-before.tex and the frame
      | width | height | padLeft | padRight | padTop | padBottom |
      | 80    | 40     | 10      | 4        | 4      | 8         |
    Then the compiled probe and the reference implementation agree on every vertex

  @id-area-stack
  @level-long
  @mode-differential
  Scenario: A stacked area draws the boundaries of d3-shape's stack
    Given the committed probe document shared://📈️charts-line-area/area-stack.tex and the frame
      | width | height | padLeft | padRight | padTop | padBottom |
      | 80    | 40     | 10      | 4        | 4      | 8         |
    Then the compiled probe and the reference implementation agree on every layer boundary
