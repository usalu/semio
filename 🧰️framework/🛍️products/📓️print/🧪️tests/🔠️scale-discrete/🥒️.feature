@capability-viz-scale-discrete
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: Discrete and binned scales place their bands and buckets exactly as d3-scale does
  Taxonomy §78's discrete kinds are `band`, `point` and `ordinal`; its binned kinds are `quantize`,
  `quantile` and `threshold`. A band scale is not one number but three — where a band starts, how
  wide it is and how far the next one sits — because a bar chart needs all three, so all three are
  measured. `point` is the band scale with an inner padding of one, which is exactly how d3 builds
  it, so the same three answers are measured there too.

  The binned kinds are measured through the range entry they assign, and separately through the
  thresholds they derive: `quantize` splits its domain into `((i+1)·x1 − (i−n)·x0) / (n+1)` cuts,
  `quantile` uses the R-7 quantiles of its domain sample, and `threshold` uses its domain verbatim.
  Assigning the right bucket for the wrong reason is then still a failure.

  @id-band
  @level-quick
  @mode-differential
  Scenario: Band starts, widths and steps follow the padding, alignment and range direction
    Given the committed probe document shared://🔠️scale-discrete/scale-discrete.tex and the band scales
      | name     | domain    | range | paddingInner | paddingOuter | align |
      | plain    | A,B,C,D,E | 0,100 | 0            | 0            | 0.5   |
      | padded   | A,B,C,D   | 0,120 | 0.1          | 0.2          | 0.5   |
      | aligned  | A,B,C     | 0,90  | 0.5          | 0.5          | 0     |
      | reversed | A,B,C,D   | 120,0 | 0.2          | 0            | 0.5   |
    Then the compiled probe and the reference implementation agree on every band start, width and step

  @id-point
  @level-quick
  @mode-differential
  Scenario: A point scale is the band scale with an inner padding of one
    Given the committed probe document shared://🔠️scale-discrete/scale-discrete.tex and the point scales
      | name  | domain  | range | padding |
      | pt    | A,B,C,D | 0,120 | 0       |
      | ptpad | A,B,C,D | 0,120 | 0.5     |
    Then the compiled probe and the reference implementation agree on every point position

  @id-ordinal
  @level-quick
  @mode-differential
  Scenario: An ordinal scale cycles its range over its domain
    Given the committed probe document shared://🔠️scale-discrete/scale-discrete.tex and the ordinal scale
      | domain    | range    | inputs    |
      | A,B,C,D,E | 10,20,30 | A,B,C,D,E |
    Then the compiled probe and the reference implementation agree on every assigned entry

  @id-quantize
  @level-quick
  @mode-differential
  Scenario: A quantize scale cuts its domain into uniform buckets
    Given the committed probe document shared://🔠️scale-discrete/scale-discrete.tex and the quantize scales
      | name   | domain | range   | inputs                            |
      | qz     | 0,1    | 1,2,3,4 | -1,0,0.1,0.25,0.5,0.6,0.9,1,2     |
      | qzwide | -10,10 | a,b,c   | -10,-4,0,4,10                     |
    Then the compiled probe and the reference implementation agree on every bucket and threshold

  @id-quantile
  @level-quick
  @mode-differential
  Scenario: A quantile scale cuts its domain sample at its R-7 quantiles
    Given the committed probe document shared://🔠️scale-discrete/scale-discrete.tex and the quantile scale
      | domain               | range          | inputs       |
      | 1,2,3,4,5,6,7,8,9,10 | 10,20,30,40    | 1,3,5,7,9,10 |
    Then the compiled probe and the reference implementation agree on every bucket and threshold

  @id-threshold
  @level-quick
  @mode-differential
  Scenario: A threshold scale bisects its explicit cuts
    Given the committed probe document shared://🔠️scale-discrete/scale-discrete.tex and the threshold scale
      | domain | range        | inputs         |
      | 0,1    | low,mid,high | -1,0,0.5,1,2   |
    Then the compiled probe and the reference implementation agree on every bucket and threshold
