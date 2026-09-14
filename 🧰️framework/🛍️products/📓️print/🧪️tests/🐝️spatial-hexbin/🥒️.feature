@capability-viz-spatial-hexbin
@oracle-d3-hexbin
@comparison-viz-probe-v1
Feature: Hexagonal binning in semio-viz-spatial agrees with d3-hexbin
  `\SemioVizHexbin` reimplements `d3-hexbin`'s binning exactly: `dy = 1.5 r`, `dx = 2 r sin 60°`,
  the row index from `Math.round(y / dy)` with JavaScript's half-away-from-minus-infinity rounding,
  the column index shifted by a half cell on odd rows, and d3's correction step — when the point
  sits more than a third of a row from its row centre, the neighbouring hexagon is tested and the
  nearer one wins. Bin centres come back as `(i + odd/2) dx` and `j dy`.

  The rounding rule is the whole point of reproducing this by hand: `\fp_eval` rounds half away from
  zero and JavaScript rounds half towards positive infinity, and a point sitting exactly on a
  boundary lands in a different hexagon under the two rules. The probe therefore computes
  `floor(v + 0.5)`, which is what `Math.round` is.

  Bins are compared as centre x, centre y and point count, sorted by centre: `d3-hexbin` returns
  them in first-touch order, which is an artefact of iteration, not of the binning.

  Probe document: local://hexbin.tex, over `demo-points-dense` at two radii.

  @id-hexagonal-binning
  @level-quick
  @mode-differential
  Scenario: Bin centres and counts agree with d3-hexbin
    Given the committed probe document local://hexbin.tex and the radii
      | radius |
      | 6      |
      | 10     |
    Then the compiled probe and the reference implementation agree on every bin centre and count
