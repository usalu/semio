Feature: Represented Faceted BRep Geometry
  Scenario: Original authored shell topology reaches editable geometry and GLB
    Given privately admitted IFC4X3_ADD2 faces, polygon rings and declared bound orientations
    When original-controlled extraction triangulates all outer and void shells
    Then face holes, unmarked outer rings, source point sharing and inward void normals are retained
    And a central point mutation yields a reversible diff and exact STEP restoration
    And original official models agree with independent physical geometry observations

  Scenario: A failed topology or original operation cannot publish a partial item
    Given malformed geometry or an exhausted original value grant
    When topology, triangulation or emission is refused or cancelled
    Then the complete item is refused and the exact original operation failure is retained
