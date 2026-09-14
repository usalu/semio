@capability-viz-scale-discrete-twin
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: The TypeScript twin of the discrete scales places its bands and buckets as d3-scale does
  `scale-discrete` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `scale-discrete`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `scale-discrete` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-band
  @level-quick
  @mode-differential
  Scenario: Band starts, widths and steps follow the padding, alignment and range direction
    Given the band scales
      | name     | domain    | range | paddingInner | paddingOuter | align |
      | plain    | A,B,C,D,E | 0,100 | 0            | 0            | 0.5   |
      | padded   | A,B,C,D   | 0,120 | 0.1          | 0.2          | 0.5   |
      | aligned  | A,B,C     | 0,90  | 0.5          | 0.5          | 0     |
      | reversed | A,B,C,D   | 120,0 | 0.2          | 0            | 0.5   |
    Then the twin kernel and the reference implementation agree on every band start, width and step

  @id-point
  @level-quick
  @mode-differential
  Scenario: A point scale is the band scale with an inner padding of one
    Given the point scales
      | name  | domain  | range | padding |
      | pt    | A,B,C,D | 0,120 | 0       |
      | ptpad | A,B,C,D | 0,120 | 0.5     |
    Then the twin kernel and the reference implementation agree on every point position

  @id-ordinal
  @level-quick
  @mode-differential
  Scenario: An ordinal scale cycles its range over its domain
    Given the ordinal scale
      | domain    | range    | inputs    |
      | A,B,C,D,E | 10,20,30 | A,B,C,D,E |
    Then the twin kernel and the reference implementation agree on every assigned entry

  @id-quantize
  @level-quick
  @mode-differential
  Scenario: A quantize scale cuts its domain into uniform buckets
    Given the quantize scales
      | name   | domain | range   | inputs                            |
      | qz     | 0,1    | 1,2,3,4 | -1,0,0.1,0.25,0.5,0.6,0.9,1,2     |
      | qzwide | -10,10 | a,b,c   | -10,-4,0,4,10                     |
    Then the twin kernel and the reference implementation agree on every bucket and threshold

  @id-quantile
  @level-quick
  @mode-differential
  Scenario: A quantile scale cuts its domain sample at its R-7 quantiles
    Given the quantile scale
      | domain               | range          | inputs       |
      | 1,2,3,4,5,6,7,8,9,10 | 10,20,30,40    | 1,3,5,7,9,10 |
    Then the twin kernel and the reference implementation agree on every bucket and threshold

  @id-threshold
  @level-quick
  @mode-differential
  Scenario: A threshold scale bisects its explicit cuts
    Given the threshold scale
      | domain | range        | inputs         |
      | 0,1    | low,mid,high | -1,0,0.5,1,2   |
    Then the twin kernel and the reference implementation agree on every bucket and threshold
