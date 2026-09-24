@capability-viz-shape-curves
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: Every curve interpolator emits the control points d3-shape emits
  `semio-viz-mark` carries a real transcription of every interpolator in `d3-shape/src/curve`, not
  a cosmetic TikZ `smooth` alias: the same `_point` state machine, the same `_line` flag, the same
  cubic control points. A curve is therefore only correct when the whole SVG path it produces --
  command letters included -- is the path d3 would have written, which is what these scenarios
  compare token for token.

  The probe records the path exactly as `d3-path` serialises it, so the reference side only has to
  tokenise `d3.line().curve(c).digits(15)(points)`. Numbers are compared on the six-decimal
  emission grid with the print numeric profile.

  @id-interpolators
  @level-quick
  @mode-differential
  Scenario: The eighteen d3 interpolators agree on one six-point series
    Given the committed probe document shared://➰️shape-curves/shape-curves.tex and the series
      | points                       | curves                                                                                                                                                                                                                    |
      | 0,0; 1,3; 2,1; 3,4; 4,2; 5,5 | linear, linear-closed, step, step-before, step-after, basis, basis-open, basis-closed, bundle, cardinal, cardinal-open, cardinal-closed, catmull-rom, catmull-rom-open, catmull-rom-closed, monotone-x, monotone-y, natural |
    Then the compiled probe and the reference implementation agree on every value

  @id-parameters
  @level-quick
  @mode-differential
  Scenario: Tension, alpha and beta move the control points the way d3 moves them
    Given the committed probe document shared://➰️shape-curves/shape-curves.tex and the parameterised curves
      | curve       | parameter   |
      | cardinal    | tension=0.5 |
      | catmull-rom | alpha=0     |
      | catmull-rom | alpha=1     |
      | bundle      | beta=0.5    |
    Then the compiled probe and the reference implementation agree on every value

  @id-degenerate
  @level-quick
  @mode-differential
  Scenario: One- and two-point lines degrade exactly as d3 degrades them
    Given the committed probe document shared://➰️shape-curves/shape-curves.tex and the short series
      | curve       | points        |
      | linear      | 2,7           |
      | basis       | 2,7; 4,9      |
      | natural     | 2,7; 4,9      |
      | catmull-rom | 2,7; 4,9; 6,3 |
    Then the compiled probe and the reference implementation agree on every value
