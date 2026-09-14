@capability-viz-spatial-quadtree
@oracle-d3-quadtree
@comparison-viz-probe-v1
Feature: The spatial index covers the same square and finds the same neighbour as d3-quadtree
  Every collision layout in the library is a nearest-neighbour question asked many times over:
  `beeswarm` pushes a point off its axis until nothing within the collision radius is left, the
  force layout's collide step does the same in two dimensions. Both were doing it by scanning every
  point already placed, which is the one part of the spatial package with no index behind it.

  `\SemioVizQuadtree{table}[x=,y=]` builds that index, and `d3-quadtree` is a real reference for it
  rather than an approximate one, because two things about a quadtree are observable from outside
  and both are exactly specified.

  The first is the **square it covers**. d3 does not fit a box to the data: it takes the floor of
  the first corner it is given, makes a unit square of it, and doubles that square towards any point
  that falls outside, choosing which corner stays put from the quadrant the point lies in. Two
  implementations that merely "cover the data" would agree on nothing; two that run the same
  doubling agree on the extent digit for digit. `d3.quadtree(data)` covers the bounding-box corners
  before it adds a single point, and this implementation does the same, which is why the shifted
  table — whose points straddle the origin — comes out on `[[-7,-7],[25,25]]` on both sides and not
  on the data's own bounds.

  The second is the **node count**, which follows from the tree's shape alone: a leaf holds a chain
  of coincident points, and a leaf that meets a point of its own subdivides until the two separate.
  Counting the nodes `d3.quadtree.visit` walks and the nodes this implementation allocated is a
  structural comparison, not a behavioural one — it says the two trees have the same shape and not
  merely the same answers.

  The answers are the third half of it. `find` walks the tree depth first, nearest quadrant first,
  pruning any quadrant that cannot hold anything closer than the best candidate so far, and keeps a
  candidate only when it is *strictly* closer. d3 does the same, but reaches the candidates in its
  own traversal order, so the two agree on every input in which no two candidates are equidistant —
  and the vectors below are chosen so that none is.

  @id-cover
  @level-quick
  @mode-differential
  Scenario: The covering square is grown by doubling exactly as d3-quadtree grows it
    Given the committed probe document local://spatial-quadtree.tex and the point tables
      | table      | x                          | y                            |
      | qt-scatter | 1;5;2;9;4;7.5;0.5          | 1;2;7;9;4;3.25;8.5           |
      | qt-shifted | -3;12;-7;2.5               | -2;6;11;-6.5                 |
    Then the compiled probe and d3-quadtree report the same extent
    And they report the same number of points and the same number of nodes

  @id-find
  @level-quick
  @mode-differential
  Scenario: An unbounded query finds the same nearest point as d3-quadtree's find
    Given the committed probe document local://spatial-quadtree.tex and the queries
      | x   | y   |
      | 4.4 | 4.1 |
      | 9   | 1   |
      | 0   | 0   |
      | 2.2 | 7.4 |
      | 6   | 2.5 |
      | 10  | 10  |
    Then the compiled probe and d3-quadtree name the same point for every query

  @id-find-radius
  @level-quick
  @mode-differential
  Scenario: A bounded query returns nothing when the nearest point lies beyond the radius
    Given the committed probe document local://spatial-quadtree.tex and the bounded queries
      | x | y | radius |
      | 9 | 1 | 2      |
      | 9 | 1 | 3      |
      | 0 | 0 | 1      |
      | 0 | 0 | 2      |
      | 6 | 6 | 2.5    |
    Then the compiled probe and d3-quadtree agree on which queries find nothing at all
