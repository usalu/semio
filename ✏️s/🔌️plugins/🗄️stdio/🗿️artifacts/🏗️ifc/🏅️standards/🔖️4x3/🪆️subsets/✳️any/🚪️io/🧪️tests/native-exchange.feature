Feature: Native IFC4x3 artifact geometry exchange
  Scenario: Registered native IFC artifact pack exports a real glTF artifact pack
    Given the official triangulated IFC4X3_ADD2 fixture
    When its native artifact pack is composed through the registered glTF target route
    Then the native glTF artifact contains eight vertices and twelve triangles
    And glTF Y-up world bounds in metres are (0.5, 0, -0.5) through (1.5, 2, 0.5)
    And authored and analytical semantic losses are explicit diagnostics

  Scenario: Registered glTF artifact import creates real IFC4x3 geometry
    Given the official Khronos Box GLB and its native glTF snapshot pack
    When composed through the registered IFC4x3 target route
    Then the target native IFC artifact contains a metre unit and actual triangulated face sets
    And native export to STEP declares IFC4X3_ADD2
    And geometry retains twelve triangles and cube bounds of plus or minus half a metre

  Scenario Outline: Native exchange never publishes unsupported partial work
    Given native exchange containing <defect>
    When controlled geometry interchange is requested
    Then no destination artifact is returned
    And the source artifact remains unchanged
    Examples:
      | defect                               |
      | cancellation                         |
      | a geometry work or allocation limit   |
      | missing or ambiguous project units    |
      | unsupported georeferencing            |
      | required glTF extensions or animation |
      | malformed accessor or triangle data   |
      | a cyclic or multiply-parented node    |
      | misaligned accessor or invalid stride |
      | cancellation during unit assignment   |
      | cancellation during graph validation  |
