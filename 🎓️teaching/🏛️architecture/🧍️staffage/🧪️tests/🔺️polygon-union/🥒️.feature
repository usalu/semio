@capability-staffage-evaluation
Feature: The polygon union never silently returns nothing
  `unionPolygons` returns the boundary of the union of polygons. Nearly collinear overlapping edges, knees and duplicate points (the profile of an agave leaf seen nearly edge-on) used to produce an empty result.
  The fast edge-splitting union is checked against what a union can be, the exact slab union takes over when it fails, and polygons with area that still cannot be united raise a named error.

  Scenario: The agave leaf profiles
    Given the sweeps of two agave leaf profiles that used to return no ring
    When their union is built
    Then its area and its number of rings equal the shapely union

  Scenario: A long thin leaf whose boundary has runs of closely spaced vertices
    Given the 19 sweep polygons of an agave leaf in the frost-sag pose at 1:100, which made the exact slab union fail by a hair
    When their union is built
    Then its area equals the union of the independent library polygon-clipping, and the straight vertices dropped from its boundary never collapse a curve

  Scenario: Slivers thinner than the resolution of the union
    Given the strokes of a herring gull in the landing clip and of a seated worker in the settle-back clip, which contain quads only 20 to 40 nanometres wide
    When their union is built
    Then it neither fails nor changes the area beyond the independent library polygon-clipping, and a lone sliver gives an empty union

  Scenario: Edges that coincide to within a few nanometres
    Given strips of a swing chain and a vacuum nozzle that lie edge to edge with offsets of 7 to 70 nanometres, and a whisker stroke flattened into steps of nanometres
    When their union is built
    Then it is one ring of the area polygon-clipping finds

  Scenario: Overlapping edges a few nanometres apart that are not exactly parallel
    Given the prism faces of an extruded sleeve of a kneeling gardener and of an extruded shape of a parent with a stroller seen along their profile, whose overlapping long edges lie 2 to 65 nanometres apart at an angle above a billionth
    When their union is built
    Then the edges are cut where the other edge ends instead of at a crossing far outside both, and the union is one ring of the area polygon-clipping finds

  Scenario: Edges that lie a fraction of a micrometre apart, at the limit of the resolution
    Given two faces of a skinned quilt prism of a sleeping person that share one edge and whose opposite edges lie 0.22, 0.49 and 0.55 micrometres apart, the same faces moved so that corners nearly meet or poke through an edge, and five strips of a rain poncho
    When their union is built
    Then a piece counts as lying on an edge of another polygon only with both ends and its middle, vertices within the resolution of another polygon are joined to it before the union is repeated, and the union is one ring of the area polygon-clipping finds

  Scenario: Prism faces that stand within nanometres of the vertical
    Given the faces of a shank prism of a pregnant person swept along the bone normal with one profile edge along the bone, whose sweep edges stand 0.5 to 23 nanometres off the vertical in the front view, and the faces of a hand prism of a tour guide seen along its profile with its two end outlines as slivers of 2 micrometres
    When their union is built
    Then the slab union that cannot close with vertical slabs is repeated on the polygons turned by a fixed angle and turned back, and the union is one ring of the area shapely and polygon-clipping find

  Scenario: Duplicate points do not matter
    Given a square with a doubled corner and a closing point
    Then its union is the square

  Scenario: Polygons without area give an empty union and no error
    Given three collinear points
    Then the union is empty

  Scenario: A mass of overlapping beads loses no ellipsoid
    Given masses of two to seven ellipsoids, each a bead of elliptic sections 2, 4 or 13 degrees apart across a tilted view, and the convex hull of every two consecutive sections
    When the bodies are united
    Then the area and the extent equal those of the shapely union and the area equals that of the polygon-clipping union
