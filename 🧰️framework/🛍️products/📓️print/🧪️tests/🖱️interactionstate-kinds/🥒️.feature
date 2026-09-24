@capability-viz-interactionstate-kinds
@no-oracle-interactionstate-kinds
@comparison-viz-probe-v1
Feature: Every frozen interaction-state kind draws geometry of its own
  Taxonomy section 45 is what an interactive view looks like once it has been printed: the
  overview beside its detail panel, the brush rectangle that links them, one selection frozen on a
  node-link diagram, and the storyboard of frames a transition would have animated through.
  `semio-viz-interactionstate` serves its fifteen catalogue kinds from three families.

  No third-party library adjudicates this. d3 has interaction behaviours, not depictions of them:
  nothing in the registry lays out an overview-plus-detail pair on paper or draws the ghost of a
  previous keyframe. The catalogue is therefore the specification, as it is for `showcase-families`
  and `infographic-kinds`: every kind the catalogue registers must emit geometry through the probe,
  and no two kinds of one family may emit the same geometry. Everything numeric underneath — the
  scale the overview series is drawn on, the layout the selection diagram is placed by — is
  adjudicated against the registered d3 oracles by the scale and layout cases, so this case covers
  presence and distinctness only.

  Distinctness is the load-bearing half here, because these kinds are option sets over one
  renderer: seven of the eight `state-sequence` kinds differ only in `depiction`, `columns`,
  `arrows` or `ghost`, and the three `state-selection` kinds only in `mode`. The selection record
  therefore carries both the kept flag of every node and the fill rule the mode chose, and a
  callout emits its own tooltip record, so `filtered`, `callout` and `neighbourhood` cannot
  collapse onto one another.

  The case runs at the long level: the probe document loads the diagram kernel behind the package
  and draws fifteen figures, more than the quick budget allows in the typesetter alone.

  @id-kinds
  @level-long
  @mode-conformance
  Scenario: The fifteen interaction-state kinds emit geometry and are pairwise distinct inside their family
    Given the committed probe document shared://🖱️interactionstate-kinds/interactionstate-kinds.tex and the state families
      | family          | kinds |
      | state-overview  | 5     |
      | state-selection | 3     |
      | state-sequence  | 7     |
    Then the compiled probe and the catalogue agree on which kinds drew and on how many of them are distinct
