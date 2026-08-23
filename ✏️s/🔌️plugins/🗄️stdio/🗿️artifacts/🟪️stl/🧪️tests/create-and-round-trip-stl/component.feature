@capability-stl-mesh
@oracle-stl-io
@comparison-semantic-mesh-v1
Feature: Create and round-trip a binary STL mesh
  STL is a triangle soup: it carries no vertex index, and the per-facet normal is derivable from the
  corners. The projection therefore compares the resolved corner positions per triangle, which is
  what the format actually fixes, rather than an index buffer the format never had.

  @id-tetrahedron-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A closed tetrahedron survives decode and re-encode
    Given the mesh
    """
    { "shape": "tetrahedron" }
    """
    When the mesh is written and read back
    Then every triangle's corners are unchanged

  @id-quad-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A two-triangle quad survives decode and re-encode
    Given the mesh
    """
    { "shape": "quad" }
    """
    When the mesh is written and read back
    Then every triangle's corners are unchanged
