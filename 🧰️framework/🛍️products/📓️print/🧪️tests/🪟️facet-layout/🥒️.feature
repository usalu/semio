@capability-viz-facet
@no-oracle-facet-grid
@comparison-viz-probe-v1
Feature: A facet computes its panel rectangles from the block and the groups
  `\SemioVizFacet` repeats one body over a list of groups. Where each repetition lands is arithmetic
  the library owns: the block's width minus the gaps, divided by the column count; the block's
  height minus the gaps, divided by the row count the group count and the column count imply;
  panels filled left to right, top to bottom. Nothing in d3 computes that — d3 has no faceting — so
  this case is a conformance case with the rectangles written out, and the arithmetic is the
  specification rather than a re-implementation of it.

  `sharedX`/`sharedY` do not move panels; they decide which panels draw an axis. With both shared,
  the x axis appears only on the bottom row and the y axis only on the leftmost column, which for a
  four-panel two-column grid is panels 2 and 3 for x and panels 0 and 2 for y.

  @id-wrap-grid
  @level-quick
  @mode-conformance
  Scenario: A wrapped facet fills a two-column grid on a 120 by 60 block
    Given the committed probe document local://wrap.tex with wrap=A,B,C,D columns=2 gap=4
    Then the compiled probe reports the specified panel rectangles
      | key    | values           |
      | panel0 | 0,0,32,58,28     |
      | panel1 | 1,62,32,58,28    |
      | panel2 | 2,0,0,58,28      |
      | panel3 | 3,62,0,58,28     |

  @id-shared-axes
  @level-quick
  @mode-conformance
  Scenario: Shared axes are drawn on the bottom row and the leftmost column only
    Given the committed probe document local://wrap.tex with sharedX=true sharedY=true
    Then the compiled probe reports the specified axis count
      | key        | values |
      | axisCount  | 4      |
