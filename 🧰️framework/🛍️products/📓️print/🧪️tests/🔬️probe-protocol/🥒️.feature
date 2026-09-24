@capability-viz-probe-round-trip
@oracle-d3-scale
@comparison-viz-probe-v1
Feature: A LaTeX probe document reports its numbers to the harness
  The print visualization library is written in expl3 and produces a PDF, so nothing about it is
  testable by reading its output. `semio-viz-probe.sty` is the way out: a probe document computes
  with the same arithmetic the renderers use and appends one JSON record per key to
  `\jobname.probe.jsonl`, which `🔨️modules/🧪️viz-probe` compiles with the repository tectonic,
  validates against `🧬️schema/🔣️.json` and projects into a comparable object.

  This case is the reference example for that round trip, and the template every other viz case
  copies. It measures the arithmetic the probe carries — the affine and power mappings a scale is
  built on — against `d3-scale`, which is the published reference for exactly those mappings.
  Two paths are shown on purpose: a scenario whose probe document is RENDERED from the data table
  below, and a scenario whose probe document is a COMMITTED fixture. Vectors live in the data table
  in both, because the feature is the normative half of the contract.

  Numbers are compared under `viz-probe-v1` (1e-6). That is the resolution expl3 fixed-point
  arithmetic at millimetre scale actually carries; asserting 1e-9 would measure TeX's arithmetic
  rather than the algorithm.

  @id-affine-mapping
  @level-quick
  @mode-differential
  Scenario: An affine domain-to-range mapping agrees with d3-scale
    Given the linear scale vectors
      | domainMin | domainMax | rangeMin | rangeMax | input |
      | 0         | 100       | 0        | 180      | 0     |
      | 0         | 100       | 0        | 180      | 42    |
      | 0         | 100       | 0        | 180      | 100   |
      | -50       | 50        | 10       | 20       | -50   |
      | -50       | 50        | 10       | 20       | 0     |
      | -50       | 50        | 10       | 20       | 12.5  |
      | 1         | 3         | 100      | 0        | 2     |
      | 0.5       | 2.5       | -30      | 30       | 1.75  |
    Then the probe document rendered from those vectors and the reference implementation agree on every mapped value

  @id-power-mapping
  @level-quick
  @mode-differential
  Scenario: A square-root mapping agrees with d3-scale
    Given the committed probe document shared://🔬️probe-protocol/power-mapping.tex and the inputs
      | input |
      | 0     |
      | 0.25  |
      | 1     |
      | 4     |
      | 9     |
      | 16    |
    Then the compiled probe and the reference implementation agree on every mapped value
