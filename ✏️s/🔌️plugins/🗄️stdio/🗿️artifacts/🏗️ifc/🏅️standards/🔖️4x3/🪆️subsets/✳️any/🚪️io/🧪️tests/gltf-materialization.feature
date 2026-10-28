Feature: Cooperatively materialize imported glTF triangles
  Scenario Outline: Expensive post-accessor work retains original authority
    Given the loaded pinned Khronos Box artifact with <route> indices
    When the original signal cancels inside <phase>
    And the callback replaces the caller control object
    Then import refuses with io.cancelled without publishing a candidate
    And the source artifact remains unchanged
    Examples:
      | route    | phase                      |
      | indexed  | gltf-materialize-positions  |
      | indexed  | gltf-materialize-indices    |
      | indexed  | gltf-materialize-triangles  |
      | implicit | gltf-materialize-positions  |
      | implicit | gltf-materialize-indices    |
      | implicit | gltf-materialize-triangles  |
  Scenario: Complete official fixture output remains independently equivalent
    Given the pinned indexed, interleaved, unindexed and reflected Khronos fixtures
    When their complete geometry is imported
    Then all supported positions and triangle indices match the independent Three.js loader
  Scenario: Reflected and terminal emission retains original authority
    Given indexed, implicit and reflected real loaded source variants
    When the original signal cancels at any materialization completion boundary
    Then no partial candidate is published
    And reflected triangle emission also cancels at an interior checkpoint
  Scenario Outline: Export scalar and final byte births remain cooperative
    Given the original pinned triangulated IFC artifact
    When the original signal cancels inside or at completion of <phase>
    Then no partial glTF artifact is published
    And original control replacement does not detach cancellation
    Examples:
      | phase             |
      | gltf-vertices     |
      | gltf-indices      |
      | gltf-buffer-bytes |
