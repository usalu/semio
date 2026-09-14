@capability-viz-composition
@no-oracle-composition-geometry
@comparison-viz-probe-v1
Feature: Concatenation, insets and dashboard cells hand out sub-frames
  Every composition operator in `semio-viz-composition` does one thing: it gives its body a
  rectangle of the current frame and rebinds the frame variables to it, so a plot drawn in a concat
  cell, an inset or a dashboard cell is the same code as a plot drawn in a whole figure. The
  rectangles are what this case fixes. d3 has no composition layer, so the arithmetic is specified
  here rather than adjudicated by an oracle.

  A horizontal `VizConcat` advances a cursor by each item's declared width plus the gap. A
  `VizDashboard` cell takes its rectangle from the grid and a spanning cell consumes as many column
  slots as it spans, so the cell after a `colspan=2` starts two slots later. An inset takes its own
  `at`, `width` and `height` unchanged, since an inset is placed, not flowed.

  @id-concat-horizontal
  @level-quick
  @mode-conformance
  Scenario: Two horizontal items advance by width plus gap
    Given the committed probe document local://concat.tex with direction=horizontal gap=5 width=55
    Then the compiled probe reports the specified item rectangles
      | key   | values        |
      | item0 | 0,0,0,55,60   |
      | item1 | 1,60,0,55,60  |

  @id-inset-rectangle
  @level-quick
  @mode-conformance
  Scenario: An inset keeps the rectangle it was given
    Given the committed probe document local://inset.tex with at=70,30 width=45 height=25
    Then the compiled probe reports the specified inset rectangle
      | key   | values      |
      | inset | 70,30,45,25 |

  @id-dashboard-span
  @level-quick
  @mode-conformance
  Scenario: A spanning dashboard cell consumes the column slots it spans
    Given the committed probe document local://dashboard.tex with columns=3 rows=2 gap=3
    Then the compiled probe reports the specified cell rectangles
      | key   | values             |
      | cell0 | 0,0,31.5,38,28.5   |
      | cell1 | 1,41,31.5,79,28.5  |
      | cell2 | 3,0,0,38,28.5      |
      | cell3 | 4,41,0,38,28.5     |
      | cell4 | 5,82,0,38,28.5     |
