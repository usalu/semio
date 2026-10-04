Feature: Inferred Visualization Layer Data
  Authored table transformations and layout options are resolved by the chart inference.

  Scenario Outline: Declarative transforms match independent numerical oracles
    Given a named visualization table and a declared <transform> transformation
    When its layer data is inferred without mutating the chart snapshot
    Then its projected output agrees with the corresponding independent oracle
    Examples:
      | transform  |
      | filter     |
      | sort       |
      | group      |
      | aggregate  |
      | rollup     |
      | summary    |
      | fold       |
      | pivot      |
      | join       |
      | window     |
      | normalize  |
      | cumulative |
      | quantile   |
      | bin        |
      | stack      |
      | kde        |
      | regression |

  Scenario Outline: Layout algorithms produce deterministic geometry
    Given a visualization layer with the <layout> layout and explicit options
    When its table is inferred twice
    Then both inferred tables are equal and the authored table is unchanged
    And its numerical projection agrees with the registered independent oracle
    Examples:
      | layout     |
      | stack      |
      | bin        |
      | hexbin     |
      | beeswarm   |
      | jitter     |
      | pie        |
      | arc        |
      | chord      |
      | sankey     |
      | alluvial   |
      | treemap    |
      | partition  |
      | pack       |
      | force      |
      | tree       |
      | cluster    |
      | dag        |
      | bundling   |
      | voronoi    |
      | delaunay   |
      | hull       |
      | contour    |
      | density    |
      | projection |

  Scenario: Invalid authored computations become diagnostics
    Given a transformation with an unknown kind or an absent required column
    When its layer table is inferred
    Then computation reports the invalid kind or column instead of silently ignoring it
