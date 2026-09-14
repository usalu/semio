@capability-viz-shape-symbol
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: Every d3 symbol type is drawn from its own area, not from a radius
  `\SemioVizSymbol` carries the thirteen `d3-shape/src/symbol/*` draw routines with their own
  constants, so `size` means AREA and symbols of different types read as equally heavy on the page --
  a circle and a square of the same `size` cover the same ink. The scenario therefore compares the
  full path of every type at three areas, which catches both a wrong constant and a radius that was
  used where an area was meant.

  The circle exercises the buffer's full-circle arc and the square its `rect` shorthand, so this
  scenario also pins the two path commands no interpolator ever emits.

  @id-symbol-paths
  @level-quick
  @mode-differential
  Scenario: Thirteen symbol types at three areas draw the paths d3-shape draws
    Given the committed probe document local://shape-symbols.tex and the symbols
      | types                                                                                            | sizes        |
      | circle, cross, diamond, diamond2, plus, square, square2, star, times, triangle, triangle2, wye, asterisk | 64, 17.5, 200 |
    Then the compiled probe and the reference implementation agree on every value
