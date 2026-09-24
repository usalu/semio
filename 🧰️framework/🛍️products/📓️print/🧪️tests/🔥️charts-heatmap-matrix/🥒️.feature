@capability-viz-matrix-heatmap
@oracle-d3-scale
@comparison-floating-point-v1
Feature: The heatmap family lays its cells on two band scales and shades them by a normalised value
  Taxonomy §11 is one family seen twenty ways: a clustered heatmap, a confusion matrix, a distance
  matrix and a co-occurrence matrix differ in what the numbers mean, not in where the cells go. The
  cells go where two `d3-scale` band scales with no padding put them — the row levels down the y
  range, the column levels across the x range, both in the order the long table first mentions them
  — so the geometry of the whole section is one differential comparison.

  The shading is a second, separate claim: every cell also emits a `geometry/cell` record carrying
  its row, its column, its raw value and the value normalised into the matrix extent. That is what
  a colour scale would consume, and separating it from the rectangle means a wrong extent cannot be
  hidden by a right rectangle. `demo-matrix` deliberately holds its minimum and its maximum in
  different rows so a per-row normalisation would show up immediately.

  The padding key insets each cell by a FRACTION of the cell, not by a millimetre amount, so that a
  matrix keeps its proportions when the frame changes; the padded scenario pins that reading down,
  because a millimetre inset would agree with the unpadded scenario on the default and diverge only
  here.

  @id-heatmap
  @level-long
  @mode-differential
  Scenario: Cells sit on two unpadded band scales
    Given the committed probe document shared://🔥️charts-heatmap-matrix/heatmap.tex and the frame
      | width | height | pad |
      | 64    | 40     | 4   |
    Then the compiled probe and the reference implementation agree on every cell rectangle

  @id-cells
  @level-long
  @mode-differential
  Scenario: Every cell reports its raw value and its share of the matrix extent
    Given the committed probe document shared://🔥️charts-heatmap-matrix/cells.tex
    Then the compiled probe and the reference implementation agree on every cell record

  @id-heatmap-padded
  @level-long
  @mode-differential
  Scenario: The padding key insets each cell by a fraction of the cell
    Given the committed probe document shared://🔥️charts-heatmap-matrix/heatmap-padded.tex and the frame
      | width | height | pad | padding |
      | 64    | 40     | 4   | 0.2     |
    Then the compiled probe and the reference implementation agree on every cell rectangle
