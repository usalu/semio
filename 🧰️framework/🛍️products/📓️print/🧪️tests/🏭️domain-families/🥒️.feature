@capability-viz-domain-families
@no-oracle-domain-families
@oracle-d3-array
@comparison-viz-probe-exact-v1
Feature: The sixteen long-tail domain families draw every catalogue kind they own, and each kind draws its own geometry
  `semio-viz-text` and `semio-viz-domain` carry the domain families of the long tail: the text and
  language charts of taxonomy §13, the placement algorithms of §47, the delivery charts of §16, the
  profile shapes of §62, the repository charts of §64, the machine schematics of §36, the
  operations-research drawings of §40, the cause-effect analysis of §42, the three read notations of
  §56, the freight charts of §59, the four orphans of §61, the running-system views of §65, the
  activation grids of §67, the answer grids of §69, the constituency maps of §70 and the survey maps
  of §34. Seventy-one catalogue kinds run through sixteen renderers.

  **The contract this case measures.** Architecture §4 says every family must actually consume its
  data and its options: two kinds of the same family with different options must render
  differently. That is what the first scenario asserts, and it asserts it the only way that cannot
  be faked — by compiling every catalogue kind of these families through `\SemioVizRunFamily` with
  exactly the option set the catalogue holds, and comparing the geometry records the probe writes.
  A kind that draws nothing fails; two kinds of one family whose complete record streams coincide
  fail. The vectors are the catalogue entries themselves rather than a table copied beside them,
  because a table copied beside them would be a second source of truth for the same fact and would
  stop measuring the catalogue.

  **Why there is no third-party oracle for that scenario.** No library renders a gear train, a
  cause-effect diagram or a constituency cartogram, and none defines what makes two drawings
  different. The oracle is therefore the specification written in this feature: for a family that
  owns *n* catalogue kinds the probe must yield *n* non-empty record streams and *n* distinct
  signatures. That is `@no-oracle-domain-families`, registered with its rationale in
  `🔮️oracles/🔣️.json`.

  **What d3-array is the reference for.** The one part of these families that a third party does
  define is the term scale of the §13 word charts: the glyph body height of a term is an affine map
  of its frequency over the extent of every frequency in the table, and the reading-order grid
  places the terms in descending frequency. `d3.extent` and `d3.sort` with `d3.descending` compute
  that domain and that order, so the second scenario is a genuine differential: the oracle sizes and
  orders the same terms with d3-array and the documented `sizemin`/`sizemax` range, and the subject
  is the `word` geometry records the family writes. Only the glyph size is compared, exactly, at
  three decimals, because that is the number both sides derive; the placement arithmetic of the grid
  is the family's own and is covered by the first scenario.

  @id-every-domain-kind-draws-its-own-geometry
  @level-long
  @mode-conformance
  Scenario: Every catalogue kind of a domain family draws a non-empty geometry no sibling kind repeats
    Given the domain families and the number of catalogue kinds each of them owns
      | family              | kinds |
      | analytical          | 1     |
      | biology             | 2     |
      | election            | 3     |
      | engineering-diagram | 7     |
      | logistics           | 2     |
      | monitoring          | 4     |
      | neural              | 3     |
      | niche               | 4     |
      | notation            | 3     |
      | optimization        | 5     |
      | performance         | 6     |
      | schedule            | 5     |
      | spatial-layout      | 10    |
      | survey              | 2     |
      | text-viz            | 12    |
      | version-control     | 2     |
    Then every family yields as many non-empty record streams as it owns kinds, as many distinct signatures, and no empty stream

  @id-term-glyph-size-follows-the-frequency-extent
  @level-long
  @mode-differential
  Scenario: The glyph body height of a term is d3-array's extent scale over the term frequencies
    Given the term frequencies and the glyph size range
      | term   | count | sizemin | sizemax |
      | model  | 42    | 2       | 8       |
      | data   | 36    |         |         |
      | graph  | 31    |         |         |
      | layout | 27    |         |         |
      | scale  | 24    |         |         |
      | theme  | 21    |         |         |
      | probe  | 15    |         |         |
      | axis   | 13    |         |         |
      | mark   | 11    |         |         |
      | token  | 5     |         |         |
    Then every glyph body height equals the size d3-array's extent scale gives its term
