@capability-viz-geo-path-twin
@oracle-d3-geo
@comparison-viz-probe-v1
Feature: The TypeScript twin of geoPath agrees with d3-geo
  `geo-path-graticule` measures the LaTeX kernel. This case measures the second subject of the same
  specification, the TypeScript twin `@semio-tech/print-viz-kernel`, on the same vectors and against
  the same oracle, because a kernel that exists twice is only a kernel if both copies answer alike.

  The platform gives a case one adapter per language and one subject per scenario, so the twin cannot
  share `geo-path-graticule`'s adapter with the LaTeX probe. It gets its own case instead; the
  vectors below are the vectors of `geo-path-graticule` verbatim, and the adapter reuses that case's
  oracle handlers unchanged so the two subjects are measured against literally the same reference
  numbers.

  Everything the base case says about the path pipeline holds here word for word: the ring stream
  treats a polygon as closed and drops the repeated last vertex, and `precision = 0` reproduces d3's
  `resampleNone`, so a family that turns resampling off must still emit the same corner points. The
  whole projected vertex list is compared, not a bounding box, because that is the number resampling
  would silently change.

  Two scenarios of the base case have no counterpart here. The twin projects the vertices it is given
  and carries no `resampleLineTo`, so the adaptive resampling scenario cannot be measured on it; and
  `vizGraticule` takes one extent for both the major and the minor grid, while d3 holds the minor
  lines to ±80° of latitude and only the major ones to the poles, so the graticule scenario cannot be
  measured on it either. Neither vector table is the twin's to shorten, so the twin case measures the
  unresampled ring vertices alone.

  Antimeridian clipping is out of scope — the twin does not split geometry at ±180°, and every vector
  below stays inside one hemisphere of longitude so that d3's default `clipAntimeridian` is a no-op
  on both sides.

  Numbers are rounded onto the probe protocol's nine-decimal emission grid on both sides, so the twin
  is held to the grid the LaTeX subject is held to and neither side is given a wider target.

  @id-path-without-resampling
  @level-quick
  @mode-differential
  Scenario: Unresampled ring vertices agree with d3-geo's geoPath
    Given the projections
      | kind            | precision |
      | equirectangular | 0         |
      | mercator        | 0         |
      | equal-earth     | 0         |
    Then the twin kernel and the reference implementation agree on every projected ring vertex
