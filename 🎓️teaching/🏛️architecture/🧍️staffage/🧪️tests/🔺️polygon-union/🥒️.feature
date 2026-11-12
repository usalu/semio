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

  Scenario: Duplicate points do not matter
    Given a square with a doubled corner and a closing point
    Then its union is the square

  Scenario: Polygons without area give an empty union and no error
    Given three collinear points
    Then the union is empty
