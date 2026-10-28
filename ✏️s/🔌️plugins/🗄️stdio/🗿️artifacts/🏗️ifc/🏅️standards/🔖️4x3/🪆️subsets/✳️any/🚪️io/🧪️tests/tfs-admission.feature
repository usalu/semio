Feature: Triangulated Face Set Source Admission
  Scenario: Preserve distinct point and normal domains
    Given all 32 source cases satisfy the closed admission schema
    When each original triangulation is projected through its declared product
    Then CoordIndex addresses optional normals directly
    And optional PnIndex resolves only coordinates
    And INTEGER indices and optional BOOLEAN closure retain their source carriers

  Scenario: Respect an explicit closure claim
    Given a triangulation declares Closed true
    When exact coordinate aliases are used only for private geometric validation
    Then the triangles form a finite outward closed manifold
    And the original coordinate roster and STEP values remain unchanged

  Scenario: Preserve open surface scope
    Given Closed is false or unset
    Then finite nondegenerate triangles can remain independent crossing surfaces
    And global solid closure and intersection certificates are not inferred

  Scenario: Edit and exchange represented triangulations
    Given four unchanged official IFC fixtures have pinned hashes
    When a coordinate mutation yields a diff applied by the central applier
    Then mutation and diff inverses restore the exact original STEP
    And GLB exchange retains geometry with explicit normal omission diagnostics
    And original observer failures and cancellation prevent completed publication
