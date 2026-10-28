Feature: Explicit released Body SurfaceModel representation clauses
  Scenario: The selected concept admits exactly three declared item families
    Given caller-declared applicability and Body SurfaceModel labels
    When every resolved item is a known tessellated item or shell-based or face-based model
    Then the three selected clauses pass
    And indexed polygonal faces use the actual TessellatedItem family
    And no schema cardinality, geometry or whole-view admission follows

  Scenario: Late unresolved source evidence remains visible
    Given a privately captured original source and request
    When any later constituent or qualified owner remains unknown
    Then the selected report remains incomplete with exact pointers
    And original cancellation and observer exceptions retain authority
