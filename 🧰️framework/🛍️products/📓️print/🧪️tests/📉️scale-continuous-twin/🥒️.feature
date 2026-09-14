@capability-viz-scale-continuous-twin
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: The TypeScript twin of the continuous scales answers exactly as d3-scale does
  `scale-continuous` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `scale-continuous`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `scale-continuous` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-linear
  @level-quick
  @mode-differential
  Scenario: A linear scale maps, clamps, inverts and ticks as d3-scale does
    Given the linear scales
      | name     | domain | range | options | inputs             |
      | lin      | 0,100  | 0,180 |         | 0,25,42,100,150    |
      | linrev   | -5,5   | 100,0 |         | -5,0,2.5,5         |
      | linclamp | 0,10   | 0,1   | clamp   | -3,0,5,10,17       |
    Then the twin kernel and the reference implementation agree on every value

  @id-log
  @level-quick
  @mode-differential
  Scenario: A logarithmic scale maps, inverts and lists its decade ticks as d3-scale does
    Given the logarithmic scales
      | name    | domain  | range | inputs           |
      | lg      | 1,1000  | 0,300 | 1,10,100,1000,42 |
      | lgsmall | 0.001,1 | 0,100 |                  |
    Then the twin kernel and the reference implementation agree on every value

  @id-pow
  @level-quick
  @mode-differential
  Scenario: Power and square-root scales honour their exponent
    Given the power scales
      | name    | kind | domain | range | exponent |
      | pw      | pow  | 0,10   | 0,100 | 2        |
      | pwhalf  | pow  | 0,16   | 0,64  | 0.5      |
      | sq      | sqrt | 0,100  | 0,10  | 0.5      |
    Then the twin kernel and the reference implementation agree on every value

  @id-symlog
  @level-quick
  @mode-differential
  Scenario: A symmetric-logarithmic scale stays linear around zero
    Given the symlog scales
      | name | domain   | range | constant |
      | sl   | -100,100 | 0,200 | 1        |
      | slc  | -100,100 | 0,200 | 10       |
    Then the twin kernel and the reference implementation agree on every value

  @id-nice
  @level-quick
  @mode-differential
  Scenario: The nice option extends a domain to round tick values
    Given the domains
      | name | domain    | count |
      | n1   | 0.1,0.9   | 10    |
      | n2   | 1.1,10.9  | 10    |
      | n3   | -0.5,17.3 | 5     |
      | n4   | 12,87     | 4     |
    Then the twin kernel and the reference implementation agree on every value
