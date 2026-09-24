@capability-viz-charts-statistical
@capability-viz-table
@oracle-d3-scale
@no-oracle-table-layout
@comparison-floating-point-v1
Feature: Quadrant matrices and viz tables place their cells by the rules their presets state
  Taxonomy §42's quadrant matrices and §12's tables are the two families of this namespace that
  carry meaning in their LAYOUT rather than in a statistic: a Boston matrix is a Boston matrix
  because of which corner means what, and a league table is a league table because of the order and
  the renderers of its columns. Both are therefore conformance cases against the rules the preset
  keys state, with `d3-scale`'s band scale judging the part that is a scale.

  A quadrant grid is an unpadded band scale per axis, so the `cells` key turns a 2×2 matrix into a
  3×3 risk matrix without any other change, and both are the same comparison. What the preset adds
  is the caption of each cell, which is why the caption count and their anchor positions are part
  of the scenario: a preset that forgets its captions still draws the right rectangles.

  The table family has no third-party equivalent — d3 does not lay out tables — so its scenarios
  state the geometry the family claims: the header row sits on the frame's top edge, a rule follows
  it at 0.6 row heights, the body starts one further row height below, and each body row descends
  by exactly one row height. The bar-in-cell scenario adds the second claim: an in-cell bar is as
  long as the value's share of ITS OWN column's range, so the smallest value in a column draws no
  bar at all, which is the reading that distinguishes a bar-in-cell table from a heatmapped one.

  @id-quadrant
  @level-long
  @mode-differential
  Scenario: A two-by-two matrix is an unpadded band scale per axis
    Given the committed probe document shared://🔲️charts-quadrant-table/quadrant.tex and the frame
      | width | height | cells |
      | 40    | 40     | 2     |
    Then the compiled probe and the reference implementation agree on every quadrant rectangle

  @id-risk
  @level-long
  @mode-differential
  Scenario: The cells key turns the same matrix into a nine-box risk grid
    Given the committed probe document shared://🔲️charts-quadrant-table/risk.tex and the frame
      | width | height | cells |
      | 45    | 45     | 3     |
    Then the compiled probe and the reference implementation agree on every quadrant rectangle

  @id-table
  @level-long
  @mode-conformance
  Scenario: A plain table stacks its header, rule and body rows by the row height
    Given the committed probe document shared://🔲️charts-quadrant-table/table.tex
    Then the compiled probe reports the specified anchors
      | key        | x    | y    |
      | header-1   | 0    | 40   |
      | header-2   | 15   | 40   |
      | header-3   | 30   | 40   |
      | header-4   | 45   | 40   |
      | rule-left  | 0    | 37.6 |
      | rule-right | 60   | 37.6 |
      | row-1-text | 0    | 32   |
      | row-1-num  | 28.8 | 32   |
      | row-2-text | 0    | 28   |
      | row-6-text | 0    | 12   |
      | row-6-num  | 58.8 | 12   |

  @id-table-bars
  @level-long
  @mode-conformance
  Scenario: An in-cell bar is the value's share of its own column range
    Given the committed probe document shared://🔲️charts-quadrant-table/table-bars.tex
    Then the compiled probe reports the specified bars
      | x  | y    | w       | h   |
      | 20 | 31.1 | 6.2     | 2.2 |
      | 20 | 27.1 | 18.6    | 2.2 |
      | 20 | 23.1 | 0       | 2.2 |
      | 20 | 19.1 | 14.3375 | 2.2 |
      | 20 | 15.1 | 9.6875  | 2.2 |
      | 20 | 11.1 | 11.2375 | 2.2 |
    # The `estimate` column of `demo-interval` runs 0.66..1.14, so Study C draws nothing and the
    # bar width is 18.6 mm (one column of the 60 mm frame less the 1.4 mm cell inset) times the
    # value's share of that range.
