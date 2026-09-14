@capability-viz-hierarchy-treemap
@oracle-d3-hierarchy
@comparison-viz-probe-v1
Feature: The treemap layout tiles rectangles exactly as d3-hierarchy does
  `semio-viz-hierarchy` implements taxonomy §78's `treemap` transform as `d3-hierarchy`'s
  `treemap()`: a preorder walk that takes the depth padding off every rectangle, pushes the inner
  padding onto a per-depth stack, and hands the remaining rectangle to a tiling function.

  A tiling is a choice of algorithm, not a style, so each of d3's six is measured separately on the
  same fixture: `squarify` picks greedy rows whose worst aspect ratio stays closest to the golden
  ratio, `resquarify` is the same on a first application, `slice` and `dice` split along one axis,
  `slice-dice` alternates by depth, and `binary` recursively halves the value prefix sums along the
  longer side. Two tilings are also measured on `demo-hierarchy-unbalanced`, whose deeper branch
  makes the depth padding stack observable.

  Padding is measured on its own because d3 applies it in two places that are easy to conflate:
  the outer padding shrinks the node's own rectangle before tiling, while half the inner padding is
  pushed onto the stack and taken off every child. The `paddingTop` vector is the header-row idiom,
  where the parent keeps a strip for its own label.

  `ratio` and `round` are measured together because both change squarify's output without changing
  its structure: the ratio moves the row breaks, `round` snaps every corner to a whole unit.

  Values are the four corners of every node, listed in the layout's own preorder.

  @id-tiling
  @level-quick
  @mode-differential
  Scenario: Every d3 tiling splits demo-hierarchy-deep into the same rectangles
    Given the committed probe document local://hierarchy-treemap.tex and the tilings
      | tile       | width | height |
      | squarify   | 100   | 60     |
      | resquarify | 100   | 60     |
      | slice      | 100   | 60     |
      | dice       | 100   | 60     |
      | slice-dice | 100   | 60     |
      | binary     | 100   | 60     |
    Then the compiled probe and the reference implementation agree on every value

  @id-padding
  @level-quick
  @mode-differential
  Scenario: Inner and outer padding shrink the rectangles as d3 treemap padding does
    Given the committed probe document local://hierarchy-treemap.tex and the paddings
      | key     | padding | paddingInner | paddingTop | paddingRight | paddingBottom | paddingLeft |
      | padding | 2       |              |            |              |               |             |
      | nested  |         | 1.5          | 6          | 1            | 1             | 1           |
    Then the compiled probe and the reference implementation agree on every value

  @id-ratio-and-round
  @level-quick
  @mode-differential
  Scenario: A custom squarify ratio and rounded corners follow d3 treemap ratio and round
    Given the committed probe document local://hierarchy-treemap.tex and the variants
      | key   | ratio | round |
      | ratio | 1     | false |
      | round |       | true  |
    Then the compiled probe and the reference implementation agree on every value

  @id-unbalanced
  @level-quick
  @mode-differential
  Scenario: An unbalanced hierarchy tiles as d3 does at every depth
    Given the committed probe document local://hierarchy-treemap.tex and the tilings
      | tile     | width | height |
      | squarify | 80    | 50     |
      | binary   | 80    | 50     |
    Then the compiled probe and the reference implementation agree on every value
