Feature: Real IFC representation geometry
  Scenario: Placed rectangle extrusion
    Given the language-neutral placed-extrusion IFC4 fixture
    When its Body representation is projected
    Then product 16 has 8 vertices and 12 triangles
    And the world bounds are [8,19,0] to [12,21,3]
    And the closed mesh volume is 24
  Scenario: Tessellation preserves actual coordinates
    Given the language-neutral triangulated IFC4 fixture
    When its Body representation is projected
    Then product 5 has 3 vertices and 1 triangle
  Scenario: Empty and unsupported documents stay honest
    Given an empty or unsupported representation graph
    When it is projected
    Then no substitute shape is produced
  Scenario: Projection can be cancelled or limited
    Given a real representation graph
    When the controller cancels or the work budget is exhausted
    Then projection stops with explicit status and no partial product mesh
