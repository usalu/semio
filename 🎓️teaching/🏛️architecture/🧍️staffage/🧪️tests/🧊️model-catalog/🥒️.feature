@capability-staffage-model-catalog
Feature: Every asset of the catalog has a valid 3D model set
  The catalog-wide guard covers all 152 assets. For each asset it checks the authored volume and the 3D formats offered in `exports`, the procedural model (valid, tessellated without
  non-finite or degenerate triangles, watertight where the solid kind is closed), the extent of the solids in the presentation pose against `dimensions` (the authoring gate of
  `volume-drafting`: 8 % per axis, at least 15 mm; where the presentation pose misses, the best-matching named pose is reported and the rest pose is never compared), the triangle budget
  per preset scale, and the files of the batch export read back from disk. A limitation is stated per asset with its reason; nothing else is excused.

  THE REFERENCES are the Khronos gltf-validator, three's GLTFLoader and FBXLoader, assimp (assimpjs), Ajv with the semio.model/v1 schema, fflate, an independent weld-and-count of edges
  for watertightness and the physical dimensions of the asset.

  Scenario: The guard finds every asset
    Given the catalog
    When the asset folders are listed
    Then every asset is found once

  Scenario: Each asset offers and validates its models
    Given an asset and its batch export
    When the volume, the exports, the geometry and the files are audited
    Then no check reports a finding that is not a stated limitation

  Scenario: Each pose is a valid GLB with seven variants
    Given every named pose of the asset
    When its GLB is validated and loaded
    Then the Khronos validator reports no error and no warning and the seven variants are present

  Scenario: Each clip is a loadable FBX
    Given every clip of the asset
    When its FBX is loaded by three and by assimp
    Then both find every rig bone and a finite animation of the clip's duration

  Scenario: The model files and the ZIP are consistent
    Given the model JSON files and the ZIP of the asset
    When they are validated with Ajv and compared with their files
    Then every document conforms, and the ZIP has each file exactly once
