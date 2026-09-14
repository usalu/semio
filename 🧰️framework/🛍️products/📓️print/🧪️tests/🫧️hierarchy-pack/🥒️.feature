@capability-viz-hierarchy-pack
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: Circle packing places and sizes circles exactly as d3-hierarchy does
  `semio-viz-hierarchy` implements taxonomy §78's `pack` transform as `d3-hierarchy`'s `pack()`:
  a postorder walk packs each node's children with the front-chain algorithm — place the next
  circle tangent to the current pair, walk the chain in both directions for the nearest
  intersection, splice the chain and re-pick the pair closest to the centroid — and closes with
  Welzl's smallest-enclosing-circle over the resulting front chain. A preorder walk then rescales
  every circle and re-anchors it on its parent.

  Welzl's algorithm is randomised, and d3 makes it reproducible with a linear congruential
  generator seeded with one, shared by every enclosure of a single `pack()` call. The order in
  which that stream is consumed is therefore part of the algorithm: a layout that packs the same
  circles in a different order gets different, equally valid, coordinates. Matching d3's numbers is
  only possible by consuming the same stream in the same order, which is what this case measures.

  The default radius branch (`sqrt(value)`) and the explicit radius branch are different code paths
  in d3 — the first packs twice, once without padding to learn the root radius and once with the
  padding scaled by it, the second packs once with half the padding — so both are measured, and
  padding is measured on its own because it is added to and taken off every child radius around the
  packing rather than applied afterwards.

  Values are the centre and the radius of every node, listed in the layout's own preorder.

  @id-default-radius
  @level-quick
  @mode-differential
  Scenario: A pack over sqrt(value) matches d3 pack() on a square and an oblong frame
    Given the committed probe document local://hierarchy-pack.tex and the pack extents
      | key    | width | height |
      | plain  | 100   | 100    |
      | oblong | 120   | 80     |
    Then the compiled probe and the reference implementation agree on every value

  @id-padding
  @level-quick
  @mode-differential
  Scenario: Padding separates the circles as d3 pack().padding() does
    Given the committed probe document local://hierarchy-pack.tex and the pack extents
      | key    | width | height | padding |
      | padded | 100   | 100    | 3       |
    Then the compiled probe and the reference implementation agree on every value

  @id-explicit-radius
  @level-quick
  @mode-differential
  Scenario: An explicit radius takes d3's single-pass branch
    Given the committed probe document local://hierarchy-pack.tex and the radii
      | key      | width | height | radius |
      | value    | 100   | 100    | value  |
      | constant | 100   | 100    | 4      |
    Then the compiled probe and the reference implementation agree on every value

  @id-unbalanced
  @level-quick
  @mode-differential
  Scenario: An unbalanced hierarchy consumes the same random stream as d3
    Given the committed probe document local://hierarchy-pack.tex and the pack extents
      | key    | width | height | padding |
      | plain  | 90    | 90     |         |
      | padded | 90    | 90     | 2       |
    Then the compiled probe and the reference implementation agree on every value
