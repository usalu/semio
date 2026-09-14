@capability-viz-plot-grammar
@oracle-d3-scale
@oracle-print-viz-kernel-twin
@comparison-viz-probe-v1
Feature: The plot grammar binds every encoding channel and puts a row where the scales say
  `\SemioVizPlot` is the one call taxonomy §79 promises: a data table, a mark, a coordinate system
  and a set of encoding channels go in, and a drawn layer comes out. Every family of the library is
  a preset over the same pipeline, so a defect in the pipeline is invisible to the family cases —
  they would all draw the same wrong picture and agree with each other about it.

  Two halves of that pipeline can be measured without rasterising anything.

  The first is the **vocabulary**. The grammar declares sixteen channels, and any key the plot's own
  option list does not claim is one of them; a typo therefore has to become an unbound channel and
  not a silently accepted key. The probe reads the channel registry directly, before a plot runs,
  and reports which of the sixteen a given option list bound, plus the column and named scale one
  binding carries — `x = cat` binds the column and leaves the scale implicit, `y = {column = val,
  scale = plot-y}` binds both. This is a conformance scenario: the sixteen channels are the
  specification, written out in the table below, and no library outside this repository has an
  opinion about them.

  The second is the **placement**, and there `d3-scale` is a real reference. A numeric channel gets
  a linear scale over the column extent, anchored at zero on the vertical axis; a non-numeric one
  gets a band, and a row lands on the centre of its band, which is exactly `scaleBand`'s
  `scale(value) + bandwidth() / 2`. The frame both are mapped onto is the figure rectangle inset by
  the guide package's plot pad — `[8, width − 2] × [8, height − 2]` — so an axis drawn by
  `semio-viz-guide` and a mark drawn by the plot land on the same geometry.

  The last scenario measures the whole pipeline against its second implementation. The TypeScript
  twin's `planVizChart` resolves the same specification — the same table, the same channel
  bindings, the same frame, its own scale kernel — into drawing primitives, and the circle it places
  for a row must sit where the LaTeX probe put its point. The twin is a supplement and never
  independent evidence, which is why the domain rule it is handed is the one `d3-scale` already
  adjudicated in the scenarios above: what this scenario adds is the one comparison d3 cannot make,
  the two copies of the pipeline drifting apart.

  @id-channel-vocabulary
  @level-quick
  @mode-conformance
  Scenario: Every declared encoding channel binds, and only the channels an option list names
    Given the committed probe document local://plot-grammar.tex and the channel vocabulary
      | channels                                                                        |
      | x;y;x2;y2;angle;radius;size;shape;fill;stroke;opacity;text;dash;width;order;detail |
    Then an option list that names no channel binds none of them
    And an option list that names `x` and `y` binds exactly those two
    And an option list that names all sixteen binds all sixteen
    And a bare binding records its column with no scale, a braced one records both

  @id-linear-mapping
  @level-quick
  @mode-differential
  Scenario: A numeric channel is placed by a linear scale over the column extent, as d3-scale does
    Given the committed probe document local://plot-grammar.tex and the numeric columns
      | x       | y       | xDomain | yDomain | xRange | yRange |
      | 1;2;3;4;5 | 4;7;3;8;5 | 1;5     | 0;8     | 8;78   | 8;38   |
    Then the compiled probe and d3-scale place every row at the same millimetre

  @id-band-mapping
  @level-quick
  @mode-differential
  Scenario: A non-numeric channel is placed on the centre of its band, as d3-scale's band scale does
    Given the committed probe document local://plot-grammar.tex and the categorical column
      | x         | y         | yDomain | xRange | yRange |
      | A;B;C;D;E | 2;3;5;4;6 | 0;6     | 8;78   | 8;38   |
    Then the compiled probe and d3-scale place every row at the same millimetre

  @id-twin-plan
  @level-quick
  @mode-differential
  Scenario: The TypeScript twin resolves the same specification onto the same points
    Given the committed probe document local://plot-grammar.tex and the shared specification
      | width | height | marginTop | marginRight | marginBottom | marginLeft | xColumn | yColumn | xDomain | yDomain |
      | 80    | 40     | 8         | 2           | 2            | 8          | t       | val     | 1;5     | 0;8     |
    Then the twin's render plan and the compiled probe place every row at the same millimetre
