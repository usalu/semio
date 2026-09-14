@capability-diagram-grid-placement
@no-oracle-diagram-grid-placement
@comparison-viz-probe-v1
Feature: The diagram kernel places nodes and lane bands on a declared grid
  Every family of `semio-viz-diagram-*` shares one placement rule, implemented once in the
  `🔖️DiagramKernel`/`🔖️Layout` regions of `semio-viz-diagram-flowchart.sty`. A node table carries a
  `row` and a `col` per node; the kernel derives the grid extent from the largest index it sees,
  divides the frame by it, and puts every node in the centre of its cell:

      nodeWidth  = (width  - 2·pad - (cols - 1)·colGap) / cols
      nodeHeight = (height - 2·pad - (rows - 1)·rowGap) / rows
      cx = pad + (col - 1)·(nodeWidth  + colGap) + nodeWidth  / 2
      cy = height - pad - (row - 1)·(nodeHeight + rowGap) - nodeHeight / 2

  `direction=right` transposes the two table columns before the extent is measured, so the same
  table grows to the right instead of downward, and `lanes=row` bands each grid row across the full
  frame width, the band running from half a row gap below the row to half a gap above it.

  **Why there is no third-party oracle.** No published library computes this grid: d3 has no
  diagram layer at all, and dagre/elkjs place nodes by rank and barycentre rather than from
  explicitly declared row/column indices, so neither can adjudicate a coordinate that is defined by
  the table itself. The arithmetic is fully specified by the four formulas above, so every scenario
  is a conformance scenario whose expected millimetres are computed from them and written out. The
  probe is emitted by `\semio_viz_probe_geometry:nn` as `geometry/diagram-node` records of
  `cx, cy, width, height` in node-table order and `geometry/diagram-lane` records of
  `bottom, top`.

  @id-grid-placement
  @level-quick
  @mode-conformance
  Scenario: A node table with row and column indices fills the frame in equal cells
    Given the diagram nodes
      | id | row | col | lane    | cx   | cy      | w  | h       |
      | a  | 1   | 1   | Intake  | 20.5 | 39.3333 | 35 | 11.3333 |
      | b  | 1   | 2   | Intake  | 59.5 | 39.3333 | 35 | 11.3333 |
      | c  | 2   | 2   | Review  | 59.5 | 24      | 35 | 11.3333 |
      | d  | 3   | 1   | Record  | 20.5 | 8.6667  | 35 | 11.3333 |
    Then every node centre and size matches the placement formula

  @id-direction-transposes-the-grid
  @level-quick
  @mode-conformance
  Scenario: direction=right reads the row column as the column index and grows to the right
    Given the diagram nodes
      | id | row | col | lane    | cx | cy   | w  | h  |
      | a  | 1   | 1   | Intake  | 14 | 35.5 | 22 | 19 |
      | b  | 1   | 2   | Intake  | 14 | 12.5 | 22 | 19 |
      | c  | 2   | 2   | Review  | 40 | 12.5 | 22 | 19 |
      | d  | 3   | 1   | Record  | 66 | 35.5 | 22 | 19 |
    Then every node centre and size matches the placement formula

  @id-lane-bands
  @level-quick
  @mode-conformance
  Scenario: lanes=row bands each grid row across the full frame width
    Given the diagram nodes
      | id | row | col | lane   | bottom  | top     |
      | a  | 1   | 1   | Intake | 31.6667 | 47      |
      | b  | 1   | 2   | Intake | 16.3333 | 31.6667 |
      | c  | 2   | 2   | Review | 1       | 16.3333 |
      | d  | 3   | 1   | Record |         |         |
    Then every lane band spans its grid row
