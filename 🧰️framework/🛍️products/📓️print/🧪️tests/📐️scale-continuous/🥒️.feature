@capability-viz-scale-continuous
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: Continuous scales map, invert, tick and nice exactly as d3-scale does
  `semio-viz-scale` implements taxonomy §78's continuous kinds — linear, log, pow, sqrt and symlog —
  as a transform pair applied around a normalized position, which is how `d3-scale` builds them.
  A scale is therefore only correct if all four of its answers are correct, so every scenario
  measures all four separately: where a value lands, which value a position came from, which ticks
  the domain produces, and which domain `nice` rounds it to.

  The tick algorithm is `d3-array`'s: the step is 1, 2, 5 or 10 times a power of ten chosen by the
  sqrt(2)/sqrt(10)/sqrt(50) thresholds. A log scale instead lists every integer multiple of each
  power of the base inside the domain, and `pow`, `sqrt` and `symlog` are "linearish" — their ticks
  are the linear ticks of the untransformed domain, exactly as in d3.

  Numbers are emitted rounded to six decimals, which is the emission grid of the probe protocol,
  and compared with the print numeric profile.

  @id-linear
  @level-quick
  @mode-differential
  Scenario: A linear scale maps, clamps, inverts and ticks as d3-scale does
    Given the committed probe document shared://📐️scale-continuous/scale-continuous.tex and the linear scales
      | name     | domain | range | options | inputs             |
      | lin      | 0,100  | 0,180 |         | 0,25,42,100,150    |
      | linrev   | -5,5   | 100,0 |         | -5,0,2.5,5         |
      | linclamp | 0,10   | 0,1   | clamp   | -3,0,5,10,17       |
    Then the compiled probe and the reference implementation agree on every value

  @id-log
  @level-quick
  @mode-differential
  Scenario: A logarithmic scale maps, inverts and lists its decade ticks as d3-scale does
    Given the committed probe document shared://📐️scale-continuous/scale-continuous.tex and the logarithmic scales
      | name    | domain  | range | inputs           |
      | lg      | 1,1000  | 0,300 | 1,10,100,1000,42 |
      | lgsmall | 0.001,1 | 0,100 |                  |
    Then the compiled probe and the reference implementation agree on every value

  @id-pow
  @level-quick
  @mode-differential
  Scenario: Power and square-root scales honour their exponent
    Given the committed probe document shared://📐️scale-continuous/scale-continuous.tex and the power scales
      | name    | kind | domain | range | exponent |
      | pw      | pow  | 0,10   | 0,100 | 2        |
      | pwhalf  | pow  | 0,16   | 0,64  | 0.5      |
      | sq      | sqrt | 0,100  | 0,10  | 0.5      |
    Then the compiled probe and the reference implementation agree on every value

  @id-symlog
  @level-quick
  @mode-differential
  Scenario: A symmetric-logarithmic scale stays linear around zero
    Given the committed probe document shared://📐️scale-continuous/scale-continuous.tex and the symlog scales
      | name | domain   | range | constant |
      | sl   | -100,100 | 0,200 | 1        |
      | slc  | -100,100 | 0,200 | 10       |
    Then the compiled probe and the reference implementation agree on every value

  @id-nice
  @level-quick
  @mode-differential
  Scenario: The nice option extends a domain to round tick values
    Given the committed probe document shared://📐️scale-continuous/scale-continuous.tex and the domains
      | name | domain    | count |
      | n1   | 0.1,0.9   | 10    |
      | n2   | 1.1,10.9  | 10    |
      | n3   | -0.5,17.3 | 5     |
      | n4   | 12,87     | 4     |
    Then the compiled probe and the reference implementation agree on every niced domain
