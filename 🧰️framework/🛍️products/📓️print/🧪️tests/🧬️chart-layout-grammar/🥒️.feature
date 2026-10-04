Feature: Authored Native Layout Geometry
  Scenario: All layout kinds consume authored tables in place
    Given neutral authored tables and options for all twenty-four chart layouts
    When canonical native layout inference compiles with the pinned TeX engine
    Then every layout publishes numeric columns consumable by the plot grammar
    And independent D3 produces the same numeric geometry
  Scenario: Authored customization reaches the numerical native owners
    Given eighteen neutral scenarios varying centers, radii, rotations, padding, graph controls, selectors, raster controls and reflections
    When their native layouts compile in place
    Then independent D3 and dagre produce the same customized geometry
