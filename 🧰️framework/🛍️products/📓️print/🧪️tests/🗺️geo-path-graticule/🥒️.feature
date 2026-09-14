@capability-viz-geo-path
@oracle-d3-geo
@comparison-viz-probe-v1
Feature: geoPath and the graticule of semio-viz-geo agree with d3-geo
  A projection alone does not draw a map. `semio-viz-geo.sty` also carries d3-geo's path pipeline:
  the ring stream that treats a polygon as closed and drops the repeated last vertex, and the
  adaptive resampler `resampleLineTo`, which subdivides a segment while its projected midpoint
  deviates by more than `precision²` from the straight chord, while the midpoint sits away from the
  chord's centre, or while the two endpoints are more than thirty degrees apart on the sphere —
  d3's three criteria, at d3's `maxDepth` of sixteen.

  Resampling is what makes a straight meridian bend correctly on a conic and what puts the extra
  vertices into a world outline; it is also the part that silently changes the vertex count, so it
  is tested by comparing the whole projected vertex list, not a bounding box. `precision = 0`
  reproduces d3's `resampleNone` and is tested separately, because a family that turns resampling
  off must still emit the same corner points.

  `\SemioVizGraticule` reimplements `d3.geoGraticule`: major meridians every `stepMajor.x`
  sampled every ninety degrees of latitude, major parallels every `stepMajor.y` sampled every
  `precision` degrees of longitude, and the minor lines that are not already major. The line list
  is compared before projection, so a disagreement is a disagreement about the graticule and not
  about the projection.

  Probe documents: local://path-none.tex, local://path-resampled.tex and local://graticule.tex are all committed.

  Antimeridian clipping is out of scope — `semio-viz-geo` does not split geometry at ±180°, and
  every vector below stays inside one hemisphere of longitude so that d3's default `clipAntimeridian`
  is a no-op on both sides.

  @id-path-without-resampling
  @level-quick
  @mode-differential
  Scenario: Unresampled ring vertices agree with d3-geo's geoPath
    Given the committed probe document local://path-none.tex and the projections
      | kind            | precision |
      | equirectangular | 0         |
      | mercator        | 0         |
      | equal-earth     | 0         |
    Then the compiled probe and the reference implementation agree on every projected ring vertex

  @id-path-with-resampling
  @level-quick
  @mode-differential
  Scenario: Adaptively resampled ring vertices agree with d3-geo's geoPath
    Given the committed probe document local://path-resampled.tex and the projections
      | kind             | precision          |
      | mercator         | 0.7071067811865476 |
      | conic-equal-area | 0.7071067811865476 |
    Then the compiled probe and the reference implementation agree on every resampled vertex

  @id-graticule-lines
  @level-quick
  @mode-differential
  Scenario: The graticule line list agrees with d3-geo
    Given the committed probe document local://graticule.tex and the graticule steps
      | stepMinorX | stepMinorY | stepMajorX | stepMajorY | precision |
      | 20         | 20         | 90         | 360        | 30        |
    Then the compiled probe and the reference implementation agree on every graticule vertex
