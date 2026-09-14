@capability-viz-interpolate-kinds
@oracle-d3-interpolate
@comparison-viz-probe-v1
Feature: The public interpolator dispatches onto the interpolator each kind names
  `d3-interpolate` is one function with a type switch: numbers interpolate linearly, colours
  interpolate in a colour space, arrays interpolate element by element, and `interpolateRound`
  rounds the linear result. The kernel already had all four — the ramp code has interpolated colours
  since the scale package existed — but nothing outside the kernel could reach them, so a document
  that wanted a number half way between two others had to declare a scale to get it.

  `\SemioVizInterpolate{kind}{a}{b}{t}` is that surface, and what this case measures is the switch:
  every kind must land on the interpolator it promises and on no other. `number` and `round` are
  `interpolateNumber` and `interpolateRound`; `rgb`, `lab` and `hcl` are `interpolateRgb`,
  `interpolateLab` and `interpolateHcl` on the rendered hexadecimal, character by character; `array`
  is `interpolateArray` over two equally long lists of numbers. The kernel's fourth colour space,
  OKLab, has no reference in d3 and is measured in `theme-palettes` instead, against the law an
  interpolation space owes its own endpoints.

  The command typesets and is therefore not expandable, exactly like `\SemioVizFormat`; the probe
  calls the protected form behind it and emits the token list it fills, which is the value the
  document would have set. Numbers travel as numbers so the tolerance profile applies to them;
  colours travel as strings, where a tolerance means nothing and the comparison is exact anyway.

  @id-number
  @level-quick
  @mode-differential
  Scenario: The number interpolator agrees with d3-interpolate's interpolateNumber
    Given the committed probe document local://interpolate-kinds.tex and the number pairs
      | a  | b | t    |
      | 10 | 20 | 0    |
      | 10 | 20 | 0.25 |
      | 10 | 20 | 0.5  |
      | 10 | 20 | 1    |
      | -4 | 6  | 0.3  |
      | 7  | 7  | 0.75 |
    Then the compiled probe and d3-interpolate agree on every value

  @id-round
  @level-quick
  @mode-differential
  Scenario: The rounding interpolator agrees with d3-interpolate's interpolateRound
    Given the committed probe document local://interpolate-kinds.tex and the number pairs
      | a  | b  | t    |
      | 10 | 21 | 0    |
      | 10 | 21 | 0.25 |
      | 10 | 21 | 0.5  |
      | 10 | 21 | 1    |
      | -4 | 7  | 0.3  |
    Then the compiled probe and d3-interpolate agree on every value

  @id-colour
  @level-quick
  @mode-differential
  Scenario: Each colour kind dispatches onto d3-interpolate's interpolator of the same name
    Given the committed probe document local://interpolate-kinds.tex and the colour pairs
      | space | a      | b      | t         |
      | rgb   | ff344f | 34d1bf | 0.25;0.75 |
      | lab   | ff344f | 34d1bf | 0.25;0.75 |
      | hcl   | ff344f | 34d1bf | 0.25;0.75 |
    Then the compiled probe and d3-interpolate agree on every rendered colour

  @id-array
  @level-quick
  @mode-differential
  Scenario: The array interpolator walks two lists element by element, as d3-interpolate does
    Given the committed probe document local://interpolate-kinds.tex and the array pairs
      | a        | b          | t    |
      | 0;10;100 | 10;20;200  | 0    |
      | 0;10;100 | 10;20;200  | 0.5  |
      | 0;10;100 | 10;20;200  | 1    |
      | -2;0;3   | 2;8;-1     | 0.25 |
    Then the compiled probe and d3-interpolate agree on every element
