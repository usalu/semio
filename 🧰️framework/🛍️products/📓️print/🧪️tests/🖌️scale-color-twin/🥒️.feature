@capability-viz-scale-color-twin
@oracle-d3-scale
@oracle-d3-interpolate
@oracle-d3-color
@comparison-viz-probe-v1
Feature: The TypeScript twin of the colour scales positions and interpolates as d3 does
  `scale-color` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `scale-color`'s adapter with the LaTeX probe. It gets its own case instead; the vectors below
  are the vectors of `scale-color` verbatim, and the adapter reuses that case's oracle handlers
  unchanged so the two subjects are measured against literally the same reference numbers.

  Only the vectors the twin has a routine for are carried over. The twin ships sRGB interpolation
  alone: it has no CIE Lab and no HCL conversion, so `scale-color`'s Lab, HCL and multi-stop ramp
  scenarios have no counterpart here and are absent rather than approximated.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target. Colours
  travel as strings prefixed with a vertical bar, so a hexadecimal such as `1234e5` is compared as
  those six digits rather than parsed as a number.

  @id-sequential-position
  @level-quick
  @mode-differential
  Scenario: A sequential scale normalizes its domain onto the unit interval
    Given the sequential scales
      | name     | domain | clamp | inputs                    |
      | seq      | 0,100  | false | -20,0,25,50,75,100,140    |
      | seqclamp | 0,100  | true  | -20,0,25,50,75,100,140    |
    Then the twin kernel and the reference implementation agree on every position

  @id-diverging-position
  @level-quick
  @mode-differential
  Scenario: A diverging scale puts its midpoint at one half
    Given the diverging scales
      | name   | domain    | inputs           |
      | div    | -10,0,30  | -10,-5,0,15,30   |
      | divoff | 0,2,10    | 0,1,2,6,10       |
    Then the twin kernel and the reference implementation agree on every position

  @id-interpolate-rgb
  @level-quick
  @mode-differential
  Scenario: The sRGB interpolator agrees with d3-interpolate
    Given the colour pairs
      | from    | to      | positions          |
      | ff344f  | 34d1bf  | 0,0.25,0.5,0.75,1  |
      | 000000  | ffffff  | 0.5                |
    Then the twin kernel and the reference implementation agree on every hexadecimal colour
