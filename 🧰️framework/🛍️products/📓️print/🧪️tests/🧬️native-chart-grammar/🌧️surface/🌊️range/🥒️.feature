Feature: Authored ranges of scientific geological signals
  Scenario Outline: Authored windows map native seismogram and river vertices
    Given a linear ground-motion signal or a three-point river profile
    And the authored SemioT signal samples 401 vertices from zero to sixteen seconds
    And an omitted, authored, reversed, default-equal, or combined physical range
    When canonical mutation, replay, inversion, and inference compile in English light and German dark
    Then omitted ranges retain the native automatic fit
    And every authored pair including -6,2 remains effective
    And the actual marked PDF body vertices match independent D3 scales
    And the source snapshots and neutral fixture remain unchanged