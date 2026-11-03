@capability-staffage-evaluation
Feature: The polygon union never silently returns nothing
  `unionPolygons` returns the boundary of the union of polygons. Nearly collinear overlapping edges, knees and duplicate points (the profile of an agave leaf seen nearly edge-on) used to produce an empty result.
  The fast edge-splitting union is checked against what a union can be, the exact slab union takes over when it fails, and polygons with area that still cannot be united raise a named error.

  Scenario: The agave leaf profiles
    Given the sweeps of two agave leaf profiles that used to return no ring
    When their union is built
    Then its area and its number of rings equal the shapely union

  Scenario: Duplicate points do not matter
    Given a square with a doubled corner and a closing point
    Then its union is the square

  Scenario: Polygons without area give an empty union and no error
    Given three collinear points
    Then the union is empty
