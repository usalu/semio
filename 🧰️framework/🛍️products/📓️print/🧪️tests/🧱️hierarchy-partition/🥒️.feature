@capability-viz-hierarchy-partition
@capability-viz-hierarchy-sunburst
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: The partition layout bands and the sunburst angles follow d3-hierarchy
  `semio-viz-hierarchy` implements taxonomy §78's `partition` transform as `d3-hierarchy`'s
  `partition()`: the root spans the full width and the first of `height + 1` rows, and a preorder
  walk dices every parent's children over the parent's own abscissa range while moving them one row
  down. That single transform is both the icicle plot and the sunburst — the sunburst is the same
  rectangles read as (angle, radius) instead of (x, y), which is why this case measures the second
  as a partition over the extent `2π × radius` rather than as a separate algorithm.

  Padding and rounding are measured with the bands because d3 applies the partition padding to the
  right and bottom edge only, collapsing a rectangle onto its own midline when the padding exceeds
  it — a rule that is invisible on a wide fixture and decides the layout on a narrow one.

  The unbalanced fixture is measured because the number of rows comes from the root height, so a
  hierarchy with one deep and one shallow branch is the case where a wrong height silently rescales
  every band.

  Values are the four corners of every node, listed in the layout's own preorder.

  @id-icicle
  @level-quick
  @mode-differential
  Scenario: Partition bands, padded bands and rounded bands match d3 partition
    Given the committed probe document local://hierarchy-partition.tex and the partitions
      | key     | width | height | padding | round |
      | plain   | 100   | 60     |         | false |
      | padded  | 100   | 60     | 1.5     | false |
      | rounded | 100   | 60     |         | true  |
    Then the compiled probe and the reference implementation agree on every value

  @id-sunburst
  @level-quick
  @mode-differential
  Scenario: A partition over a full turn gives the sunburst angles and radii of d3
    Given the committed probe document local://hierarchy-partition.tex and the polar extent
      | turn             | radius |
      | 6.28318530717958 | 24     |
    Then the compiled probe and the reference implementation agree on every value

  @id-unbalanced
  @level-quick
  @mode-differential
  Scenario: An unbalanced hierarchy gets its row count from the root height, as d3 does
    Given the committed probe document local://hierarchy-partition.tex and the partitions
      | key   | width | height |
      | plain | 80    | 50     |
    Then the compiled probe and the reference implementation agree on every value
