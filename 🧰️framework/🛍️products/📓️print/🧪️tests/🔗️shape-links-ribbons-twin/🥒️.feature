@capability-viz-shape-link-twin
@oracle-d3-shape
@oracle-d3-chord
@comparison-viz-probe-v1
Feature: The TypeScript twin's links bump the way d3 draws them
  `shape-links-ribbons` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `shape-links-ribbons`'s adapter with the LaTeX probe. It gets its own case instead; the
  vectors below are the vectors of `shape-links-ribbons` verbatim, and the adapter reuses that case's
  oracle handlers unchanged so the two subjects are measured against literally the same reference
  numbers.

  The twin's `vizRibbon` takes no pad angle, so the chord-ribbon scenario of `shape-links-ribbons` —
  whose padded row takes the pad off each end before the arcs are drawn — has no second subject yet
  and is not restated here.

  Numbers are rounded onto the probe protocol's six-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-links
  @level-quick
  @mode-differential
  Scenario: Horizontal, vertical and radial links bump on the right axis
    Given the twin kernel and the links
      | tag        | kind       | source | target |
      | horizontal | horizontal | 10,20  | 80,60  |
      | vertical   | vertical   | 10,20  | 80,60  |
      | radial     | radial     | 0.5,30 | 2.5,90 |
      | flat       | horizontal | 10,20  | 10,20  |
    Then the twin kernel and the reference implementation agree on every value
