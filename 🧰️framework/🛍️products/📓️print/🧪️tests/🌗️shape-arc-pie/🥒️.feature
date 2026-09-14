@capability-viz-shape-arc
@oracle-d3-shape
@comparison-viz-probe-v1
Feature: Arc geometry and pie angles are the ones d3-shape computes
  `\SemioVizArc` is a transcription of `d3-shape/src/arc.js`, padding and rounded corners included:
  the pad angle is taken out at the pad radius, the corner radius is shrunk to what the sector's own
  opening admits, and the corner tangent circles are placed by the same closed form. Because every
  one of those branches changes the emitted path, the scenarios walk them one by one: plain sector,
  annulus, padding, padding at an explicit pad radius, rounded corners, both together, rounded
  corners on a solid wedge, the full ring, the full disc, a counter-clockwise sector, and a sector
  that padding collapses to a line.

  `\SemioVizPie` is `d3-shape/src/pie.js`: the pad angle comes out of the available turn before the
  values are scaled, sorting reorders the INDEX and never the result, and a datum whose value is not
  positive receives no angle of its own. Angles are radians clockwise from twelve o'clock, as in d3.

  @id-arc-geometry
  @level-quick
  @mode-differential
  Scenario: Every branch of d3's arc generator produces the same path
    Given the committed probe document local://shape-arc-pie.tex and the arcs
      | tag          | innerRadius | outerRadius | startAngle | endAngle | padAngle | padRadius | cornerRadius |
      | plain        | 0           | 100         | 0          | 1.2      | 0        |           | 0            |
      | annular      | 40          | 100         | 0.3        | 2.5      | 0        |           | 0            |
      | pad          | 40          | 100         | 0.3        | 2.5      | 0.05     |           | 0            |
      | pad-radius   | 40          | 100         | 0.3        | 2.5      | 0.05     | 120       | 0            |
      | corner       | 40          | 100         | 0.3        | 2.5      | 0        |           | 12           |
      | pad-corner   | 40          | 100         | 0.3        | 2.5      | 0.05     |           | 12           |
      | corner-solid | 0           | 100         | 0.3        | 2.5      | 0        |           | 12           |
      | full         | 40          | 100         | 0          | 6.283185307179586 | 0 |      | 0            |
      | full-disc    | 0           | 100         | 0          | 6.283185307179586 | 0 |      | 0            |
      | counter      | 40          | 100         | 2.5        | 0.3      | 0        |           | 0            |
      | collapsed    | 40          | 100         | 0          | 0.2      | 0.5      |           | 0            |
    Then the compiled probe and the reference implementation agree on every value

  @id-arc-centroid
  @level-quick
  @mode-differential
  Scenario: The arc centroid is the midpoint in both radius and angle
    Given the committed probe document local://shape-arc-pie.tex and the arcs
      | tag     | innerRadius | outerRadius | startAngle | endAngle |
      | annular | 40          | 100         | 0.3        | 2.5      |
      | wedge   | 0           | 60          | 1          | 1.9      |
      | counter | 10          | 30          | 2.5        | 0.3      |
    Then the compiled probe and the reference implementation agree on every value

  @id-pie-angles
  @level-quick
  @mode-differential
  Scenario: Pie start and end angles follow d3's ordering, padding and zero handling
    Given the committed probe document local://shape-arc-pie.tex and the pies
      | tag        | value   | startAngle | endAngle          | padAngle | sort       |
      | descending | 1,2,3,4 | 0          | 6.283185307179586 | 0        | descending |
      | padded     | 1,2,3,4 | 0          | 6.283185307179586 | 0.05     | descending |
      | unsorted   | 5,1,4,2 | 0.5        | 4                 | 0        | none       |
      | ascending  | 5,1,4,2 | 0          | 6.283185307179586 | 0        | ascending  |
      | all-zero   | 0,0,0   | 0          | 6.283185307179586 | 0        | descending |
    Then the compiled probe and the reference implementation agree on every value
