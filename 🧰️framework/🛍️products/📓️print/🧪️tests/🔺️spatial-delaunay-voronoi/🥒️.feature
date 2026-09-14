@capability-viz-spatial-delaunay
@oracle-d3-delaunay
@comparison-viz-probe-v1
Feature: The triangulation and the Voronoi diagram of semio-viz-spatial agree with d3-delaunay
  `semio-viz-spatial.sty` triangulates with Bowyer–Watson: points are inserted in input order, every
  triangle whose circumcircle contains the new point is removed, and the boundary edges of the
  resulting hole become triangles with it. `d3-delaunay` uses Delaunator's sweep hull. The two
  algorithms are different programs with the same specification, so for a point set in general
  position they must produce the *same set of triangles* — and that is what is compared.

  Neither the order of the triangles nor which of its three vertices a triangle starts at is part of
  the contract; a Bowyer–Watson triangle and a Delaunator triangle for the same three sites are the
  same triangle. The probe therefore canonicalises: the three one-based indices are sorted
  ascending, packed as `i*10000 + j*100 + k`, and the codes are sorted. The oracle canonicalises
  the identical way. That is a normalisation of representation, not of geometry — a missing,
  extra or differently shaped triangle still fails.

  The Voronoi diagram is built from those triangles: a cell is the circumcentres of the triangles
  incident to its site, ordered by angle about the site, with two far points along the outward edge
  normals for a hull site, Sutherland–Hodgman clipped to the rectangle. `d3-delaunay`'s
  `voronoi(bounds).cellPolygon(i)` builds the same polygon by a different route and reports it with
  its own starting vertex, its own winding and a repeated closing vertex, so both sides are compared
  as a lexicographically sorted vertex set.

  Probe documents: local://delaunay.tex and local://voronoi.tex are both committed. The point set is
  `demo-points`, declared in `semio-viz-spatial.sty` and repeated in the tables below so that the
  feature owns the vectors.

  @id-delaunay-triangles
  @level-quick
  @mode-differential
  Scenario: The triangle set agrees with d3-delaunay
    Given the committed probe document local://delaunay.tex and the point set demo-points
      | x  | y  |
      | 12 | 14 |
      | 28 | 9  |
      | 41 | 26 |
      | 19 | 33 |
      | 55 | 17 |
      | 63 | 31 |
      | 34 | 41 |
      | 8  | 27 |
      | 47 | 7  |
      | 58 | 44 |
      | 25 | 20 |
      | 39 | 13 |
    Then the compiled probe and the reference implementation agree on the canonical triangle set

  @id-voronoi-cells
  @level-quick
  @mode-differential
  Scenario: The clipped Voronoi cells agree with d3-delaunay
    Given the committed probe document local://voronoi.tex and the clip rectangle
      | x0 | y0 | x1 | y1 |
      | 0  | 0  | 70 | 50 |
    And the point set demo-points
      | x  | y  |
      | 12 | 14 |
      | 28 | 9  |
      | 41 | 26 |
      | 19 | 33 |
      | 55 | 17 |
      | 63 | 31 |
      | 34 | 41 |
      | 8  | 27 |
      | 47 | 7  |
      | 58 | 44 |
      | 25 | 20 |
      | 39 | 13 |
    Then the compiled probe and the reference implementation agree on the vertices of every cell
