@capability-viz-charts-bar
@oracle-d3-scale
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: The bar family lays its rectangles out where d3 would
  `semio-viz-charts-bar`'s `bar` family is one renderer for every rectangular categorical comparison
  of taxonomy §1: the `mode` key decides how several series share one category band, `orient`
  decides which axis carries the categories, and `padding`, `barWidth` and `cornerRadius` shape the
  individual bar. None of that is a picture: every bar is a rectangle with four numbers, and those
  four numbers are what this case measures.

  The category band is `d3-scale`'s band scale over the plot's x range with no padding, so the band
  of category i starts at `range[0] + i * step`. Inside the band the family reserves
  `bandwidth * (1 - padding)` and splits that evenly between the series, which is a second band
  scale — a grouped bar chart is therefore two nested d3 band scales, and the oracle builds it that
  way rather than restating the family's arithmetic.

  A stacked mode is `d3-shape`'s `stack()` with the series as keys: the rectangle of one series in
  one category runs from the layer's lower boundary to its upper one, both mapped through the value
  scale. `mode=percent` is the same stack under `stackOffsetExpand`; the family scales the shares to
  0–100 and gives the value scale the matching domain, so the pixels are identical either way.

  `mode=diverging` is deliberately NOT `stackOffsetDiverging`: d3 splits a stack by the SIGN of the
  value, while the family splits it by the POSITION of the series in the group list, so that a
  Likert scale with only positive counts still opens to both sides of the baseline. d3 cannot
  adjudicate that rule, so the diverging scenario is a conformance scenario against the
  specification vectors below.

  @id-grouped
  @level-long
  @mode-differential
  Scenario: Grouped bars sit in nested band scales
    Given the committed probe document local://grouped.tex and the frame
      | width | height | padLeft | padRight | padTop | padBottom | padding |
      | 80    | 40     | 10      | 4        | 4      | 8         | 0.2     |
    Then the compiled probe and the reference implementation agree on every rectangle

  @id-horizontal
  @level-long
  @mode-differential
  Scenario: A horizontal orientation swaps the band and the value axis
    Given the committed probe document local://horizontal.tex and the frame
      | width | height | padLeft | padRight | padTop | padBottom | padding |
      | 80    | 40     | 10      | 4        | 4      | 8         | 0.2     |
    Then the compiled probe and the reference implementation agree on every rectangle

  @id-stacked
  @level-long
  @mode-differential
  Scenario: Stacked bars reproduce d3-shape's stack layout
    Given the committed probe document local://stacked.tex and the frame
      | width | height | padLeft | padRight | padTop | padBottom | padding |
      | 80    | 40     | 10      | 4        | 4      | 8         | 0.2     |
    Then the compiled probe and the reference implementation agree on every rectangle

  @id-percent
  @level-long
  @mode-differential
  Scenario: A percent stack reproduces d3-shape's expand offset
    Given the committed probe document local://percent.tex and the frame
      | width | height | padLeft | padRight | padTop | padBottom | padding |
      | 80    | 40     | 10      | 4        | 4      | 8         | 0.2     |
    Then the compiled probe and the reference implementation agree on every rectangle

  @id-diverging
  @level-long
  @mode-conformance
  Scenario: A diverging stack opens around the baseline by series position
    Given the committed probe document local://diverging.tex
    Then the compiled probe reports the specified rectangles
      | x     | y     | w     | h      |
      | 11.32 | 22    | 10.56 | -7     |
      | 11.32 | 22    | 10.56 | 5.25   |
      | 11.32 | 27.25 | 10.56 | 3.5    |
      | 24.52 | 22    | 10.56 | -12.25 |
      | 24.52 | 22    | 10.56 | 3.5    |
      | 24.52 | 25.5  | 10.56 | 8.75   |
      | 37.72 | 22    | 10.56 | -5.25  |
      | 37.72 | 22    | 10.56 | 10.5   |
      | 37.72 | 32.5  | 10.56 | 1.75   |
      | 50.92 | 22    | 10.56 | -14    |
      | 50.92 | 22    | 10.56 | 7      |
      | 50.92 | 29    | 10.56 | 5.25   |
      | 64.12 | 22    | 10.56 | -8.75  |
      | 64.12 | 22    | 10.56 | 12.25  |
      | 64.12 | 34.25 | 10.56 | 7      |
    # The rule the vectors state: series j of m stacks upwards from the baseline when 2j > m and
    # downwards otherwise, both by absolute value. `demo-cartesian` has three series, so North goes
    # down while South and East stack up. The baseline sits at the centre of a domain that the
    # family widens symmetrically to the largest per-category total, hence y = 22 on an 8..36 range.
