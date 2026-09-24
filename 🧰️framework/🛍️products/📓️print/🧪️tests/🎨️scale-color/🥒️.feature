@capability-viz-scale-color
@oracle-d3-scale
@oracle-d3-interpolate
@oracle-d3-color
@comparison-viz-probe-v1
Feature: Sequential and diverging scales position and interpolate colour as d3 does
  A colour scale is two independent pieces and this case keeps them apart. The first is the
  position: a sequential scale normalizes its two-point domain onto `[0,1]`, a diverging scale
  normalizes its three-point domain onto `[0,0.5,1]`, and both honour `clamp`. Measuring that
  against `d3-scale` with the identity interpolator isolates the domain arithmetic from the colour.

  The second is the interpolator. `semio-viz-scale` converts between sRGB, CIE Lab on the D50 white
  point and HCL with shortest-path hue exactly as `d3-color` does, so `interpolateRgb`,
  `interpolateLab` and `interpolateHcl` are real references, not approximations — the comparison is
  on the rendered hexadecimal, character by character. The multi-stop ramp is the same interpolator
  applied piecewise, which is `d3-interpolate`'s `piecewise`.

  The library's own default colour space is OKLab, for which d3 has no counterpart at this version;
  it is not asserted here and carries its own specification vectors instead.

  Colours travel as strings prefixed with a vertical bar, so a hexadecimal such as `1234e5` is
  compared as those six digits rather than parsed as a number.

  @id-sequential-position
  @level-quick
  @mode-differential
  Scenario: A sequential scale normalizes its domain onto the unit interval
    Given the committed probe document shared://🎨️scale-color/scale-color.tex and the sequential scales
      | name     | domain | clamp | inputs                    |
      | seq      | 0,100  | false | -20,0,25,50,75,100,140    |
      | seqclamp | 0,100  | true  | -20,0,25,50,75,100,140    |
    Then the compiled probe and the reference implementation agree on every position

  @id-diverging-position
  @level-quick
  @mode-differential
  Scenario: A diverging scale puts its midpoint at one half
    Given the committed probe document shared://🎨️scale-color/scale-color.tex and the diverging scales
      | name   | domain    | inputs           |
      | div    | -10,0,30  | -10,-5,0,15,30   |
      | divoff | 0,2,10    | 0,1,2,6,10       |
    Then the compiled probe and the reference implementation agree on every position

  @id-interpolate-rgb
  @level-quick
  @mode-differential
  Scenario: The sRGB interpolator agrees with d3-interpolate
    Given the committed probe document shared://🎨️scale-color/scale-color.tex and the colour pairs
      | from    | to      | positions          |
      | ff344f  | 34d1bf  | 0,0.25,0.5,0.75,1  |
      | 000000  | ffffff  | 0.5                |
    Then the compiled probe and the reference implementation agree on every hexadecimal colour

  @id-interpolate-lab
  @level-quick
  @mode-differential
  Scenario: The CIE Lab interpolator agrees with d3-interpolate
    Given the committed probe document shared://🎨️scale-color/scale-color.tex and the colour pairs
      | from    | to      | positions          |
      | ff344f  | 34d1bf  | 0,0.25,0.5,0.75,1  |
      | fa9500  | 001117  | 0.5                |
    Then the compiled probe and the reference implementation agree on every hexadecimal colour

  @id-interpolate-hcl
  @level-quick
  @mode-differential
  Scenario: The HCL interpolator takes the shortest hue path, as d3-interpolate does
    Given the committed probe document shared://🎨️scale-color/scale-color.tex and the colour pairs
      | from    | to      | positions          |
      | ff344f  | 34d1bf  | 0,0.25,0.5,0.75,1  |
      | 7eb77f  | a60009  | 0.5                |
    Then the compiled probe and the reference implementation agree on every hexadecimal colour

  @id-ramp-stops
  @level-quick
  @mode-differential
  Scenario: A multi-stop ramp is the interpolator applied piecewise
    Given the committed probe document shared://🎨️scale-color/scale-color.tex and the ramp
      | stops                     | space | positions            |
      | ff344f,ffffff,34d1bf      | rgb   | 0,0.25,0.5,0.75,1    |
      | ff344f,ffffff,34d1bf      | lab   | 0.125                |
    Then the compiled probe and the reference implementation agree on every hexadecimal colour
