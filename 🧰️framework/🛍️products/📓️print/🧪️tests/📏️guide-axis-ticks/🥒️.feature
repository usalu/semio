@capability-viz-guide-axis
@oracle-d3-scale
@oracle-d3-array
@oracle-d3-format
@comparison-viz-probe-v1
Feature: An axis puts its ticks where d3-axis would
  `\SemioVizAxis` is d3-axis over `semio-viz-scale`: the tick VALUES come from the scale's own tick
  algorithm, the tick POSITIONS come from mapping those values through the scale, and the label text
  comes from `semio-viz-format`. Each of the three is measurable on its own, so this case measures
  each of the three on its own — a wrong axis is then a named failure rather than a picture that
  looks off.

  The tick algorithm is `d3-array`'s `ticks(start, stop, count)`: the step is 1, 2, 5 or 10 times a
  power of ten, chosen by the sqrt(2)/sqrt(10)/sqrt(50) thresholds, and the ticks are the multiples
  of that step inside the domain. A log axis uses `d3-scale`'s log ticks instead: every integer
  multiple of a power of the base while the decades are few, linear ticks in exponent space when
  they are many. A band axis has one tick per domain entry, at the band's centre.

  Positions and labels are read out of a compiled probe document through `semio-viz-probe`, so what
  is compared is what the renderer actually drew, not a re-derivation of it.

  @id-linear-ticks
  @level-quick
  @mode-differential
  Scenario: Linear tick values agree with d3-array
    Given the committed probe document local://linear-ticks.tex and the domains
      | start | stop | count |
      | 0     | 10   | 5     |
      | 0     | 1    | 5     |
      | 0     | 1    | 10    |
      | -5    | 5    | 4     |
      | 1     | 3    | 5     |
      | 0     | 100  | 7     |
      | 0.5   | 0.9  | 4     |
      | 10    | 0    | 5     |
      | 2     | 7    | 3     |
    Then the compiled probe and the reference implementation agree on every tick value

  @id-log-ticks
  @level-quick
  @mode-differential
  Scenario: Logarithmic tick values agree with d3-scale
    Given the committed probe document local://log-ticks.tex and the domains
      | start | stop    | count |
      | 1     | 100     | 10    |
      | 1     | 1000000 | 10    |
    Then the compiled probe and the reference implementation agree on every tick value

  @id-band-ticks
  @level-quick
  @mode-differential
  Scenario: Band tick positions are the band centres d3-scale computes
    Given the committed probe document local://band-ticks.tex and the band scale
      | domain    | rangeMin | rangeMax |
      | A,B,C,D,E | 8        | 78       |
    Then the compiled probe and the reference implementation agree on every tick position

  @id-axis-geometry
  @level-quick
  @mode-conformance
  Scenario: A bottom axis draws its domain line, ticks and grid on the specified geometry
    Given the committed probe document local://axis-geometry.tex
    Then the compiled probe reports the specified axis geometry
      | key                 | values             |
      | axis-domain         | 8,8,78,8           |
      | axis-tick-positions | 8,22,36,50,64,78   |
      | grid-line-first     | 8,8,8,38           |

  @id-tick-format-labels
  @level-quick
  @mode-differential
  Scenario: Tick labels are formatted by the d3-format grammar in both locales
    Given the committed probe document local://tick-format-labels.tex and the specifier
      | specifier | start | stop | count | locales |
      | .1f       | 0     | 1    | 5     | en,de   |
    Then the compiled probe and the reference implementation agree on every tick label
    # An English label such as `0.0` reaches the harness as a JSON number, because the probe
    # protocol classifies a bare decimal as numeric; a German label such as `0,0` reaches it as a
    # string. The adapter mirrors that on the oracle side rather than weakening the comparison.
