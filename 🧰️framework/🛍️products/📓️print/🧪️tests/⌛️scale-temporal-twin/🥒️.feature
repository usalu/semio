@capability-viz-scale-temporal-twin
@oracle-d3-scale
@oracle-d3-time
@comparison-viz-probe-v1
Feature: The TypeScript twin of the temporal scale maps and inverts as scaleUtc does
  `scale-temporal` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `scale-temporal`'s adapter with the LaTeX probe. It gets its own case instead; the vectors
  below are the vectors of `scale-temporal` verbatim, and the adapter reuses that case's oracle
  handlers unchanged so the two subjects are measured against literally the same reference numbers.

  Only the vectors the twin has a routine for are carried over. The twin's calendar intervals floor
  on the host's local midnights rather than on the UTC ones the specification demands, so it has no
  counterpart to `scale-temporal`'s day, week, month and year tick scenarios; those are absent here
  rather than being compared against a weakened reference.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-map
  @level-quick
  @mode-differential
  Scenario: Timestamps map and invert as scaleUtc does
    Given the temporal scales
      | name    | domain                              | range |
      | tm      | 2026-01-01,2026-12-31               | 0,364 |
      | tmhours | 2026-03-01T00:00,2026-03-02T00:00   | 0,240 |
    Then the twin kernel and the reference implementation agree on every position and epoch
