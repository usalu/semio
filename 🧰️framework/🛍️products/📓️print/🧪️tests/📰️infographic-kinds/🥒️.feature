@capability-viz-infographic-kinds
@no-oracle-infographic-kinds
@comparison-viz-probe-v1
Feature: Every infographic chart kind draws geometry of its own
  Taxonomy section 44 is the infographic vocabulary: the number card, the badge list, the unit
  chart and the annotated silhouette. `semio-viz-infographic` serves its eleven catalogue kinds
  from four families, and every one of them is a promise that the library can draw that kind and
  not merely caption a fifth copy of the same picture.

  There is no third-party library to adjudicate what an infographic should look like — d3 has no
  infographic module, and no other package in the registry lays out a badge list or punches a
  cutaway wedge out of a silhouette — so the catalogue is the specification, exactly as it is for
  `showcase-families`. Every kind the catalogue registers for these four families must emit
  geometry through the probe, and no two kinds of one family may emit the same geometry. The
  arithmetic underneath the figures — the scales the number card reads, the grid the unit chart
  wraps into — is adjudicated against the registered d3 oracles by the scale and layout cases, so
  this case covers presence and distinctness only, never numbers.

  Four kinds separate on a single option, which is what makes the distinctness half of the
  specification worth asserting: `list-infographic` and `timeline-infographic` differ only in
  `badge`, and `anatomical-infographic` and `cutaway-diagram` only in `cutaway`. The probe record
  the badge list emits therefore carries the badge code, and the silhouette's record carries the
  model unit, the cut fraction, the explode factor and whether an outline was given.

  The case runs at the long level: the probe document loads the diagram kernel behind the
  infographic package and draws eleven figures, which costs more than the quick budget in the
  typesetter alone.

  @id-kinds
  @level-long
  @mode-conformance
  Scenario: The eleven infographic kinds emit geometry and are pairwise distinct inside their family
    Given the committed probe document shared://📰️infographic-kinds/infographic-kinds.tex and the infographic families
      | family                   | kinds |
      | infographic-number       | 2     |
      | infographic-list         | 2     |
      | infographic-icon         | 2     |
      | infographic-illustration | 5     |
    Then the compiled probe and the catalogue agree on which kinds drew and on how many of them are distinct
