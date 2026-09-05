@capability-obj-mesh
@oracle-tobj
@comparison-semantic-mesh-v1
Feature: Create and round-trip a Wavefront OBJ mesh
  OBJ is a plain-text grammar, so whitespace, numeric formatting and statement order are writer
  freedom. The vertex set, the face topology and the counts are normative. Both the reference text
  and this repository's re-encoded text are read back by the INDEPENDENT `tobj` reader.

  @id-tetrahedron-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A closed tetrahedron survives decode and re-encode
    Given the mesh
    """
    { "shape": "tetrahedron" }
    """
    When the mesh is written and read back
    Then every vertex and face is unchanged

  @id-quad-round-trips
  @level-quick
  @mode-round-trip
  Scenario: A two-triangle quad survives decode and re-encode
    Given the mesh
    """
    { "shape": "quad" }
    """
    When the mesh is written and read back
    Then every vertex and face is unchanged
