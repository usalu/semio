@domain-print @case-chart-transform-grammar @oracle-d3-array @oracle-d3-shape
Feature: Schema Transform Inference Through Native Print Grammar
  Every authored transform produces the same column values as independent D3 computations.

  @id-all-transform-kinds @mode-differential @level-long
  Scenario: All seventeen schema transform kinds
    Given the neutral JSON tables and authored transformations
    When the LaTeX inference grammar evaluates each transformation
    Then every output column equals its independent reference
