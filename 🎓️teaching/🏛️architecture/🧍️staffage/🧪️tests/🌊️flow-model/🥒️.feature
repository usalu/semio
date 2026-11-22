@capability-staffage-flow-model
Feature: The 3D model carries the air flow
  Every 3D file of an asset with flowing air (GLB, FBX, the neutral model JSON, the ZIP and the batch) carries, in its thermal scenes, the developed flow field of its static instant as geometry: stream tubes along the
  kept streams with arrow heads in the direction of flow, coloured by the temperature-excess bins of the flow paint, and vapour and fog puffs as translucent spheres. The marks are generalised per scale by the rule of the
  drawings and show in the thermal variant only. There is no emitter cone.

  THE REFERENCES are a numpy and trimesh oracle on a synthetic field, the Khronos gltf-validator, three's GLTFLoader, three's Line3 and three's colour management.

  Scenario Outline: The flow parts of a synthetic field
    Given a synthetic flow field of a warm rising stream, a straight ambient stream, a cool sinking arc and five puffs
    When its flow parts are built at 1:<scale> for an asset of 1.73 m
    Then every stream is one tube per run of equal temperature-excess bin whose sections are the stream points Ramer-Douglas-Peucker keeps at 0.08 mm of paper, in the neutral frame (x, y, z) -> (z, y, x)
    And the radius of a section is half the paper width of its speed, at most 0.012 of the extent
    And every arrow of the stream is a cone with its tip on the stream point and its axis along the flow
    And every vapour and fog puff is an icosphere of its clamped radius with the volume and area of trimesh's icosphere, in the material of its bin

    Examples:
      | scale |
      | 50    |
      | 200   |
      | 500   |

  Scenario: The model of a fixture asset
    Given the sample figure in its own air and the reference walker in cold humid air
    When the model of a pose is built with every authored scale
    Then it is valid, reports progress up to 1, and its thermal scenes, and only they, hold a tube for every stream the drawings keep at that scale, each on points of the field the drawings develop for 14 s
    And every flow material is transparent at rest and bound by the thermal variant alone, in the colour of its bin
    And the cold humid air gives fog puffs
    And every other pose of the document hides the flow, which belongs to the static instant
    And no node, solid or material is named for an emitter or a cone

  Scenario: Budgets and integrity
    Given the model of the sample figure with every pose and every authored scale
    When the geometry audit of the model tests tessellates every scene at the quality of its scale
    Then every flow solid is watertight, outward and free of degenerate triangles, and the triangles of the thermal scenes fall with the scale

  Scenario: A clip
    Given the walking clip of the sample figure, which travels one metre per cycle
    When the thermal model of the clip is built
    Then the flow is the field developed for 14 whole cycles, carried back by the travel of the root onto the figure of the first instant
    And it is static geometry: no animation channel moves a flow node

  Scenario: The caller animates the flow itself
    Given a model request that asks for no flow geometry
    Then the model has no flow node, solid or material
    And a cancelled operation stops the developing flow with the kernel's cancellation error

  Scenario: The GLB a viewer reads
    Given the GLB of a pose of the sample figure at 1:50 and 1:200
    When the Khronos validator and three's GLTFLoader read it
    Then it has no error and no warning, the thermal scenes hold the flow nodes and no other scene does
    And every vertex of a stream node lies within the tube, arrow head and chord reach of its stream, and the node spans its stream
    And the flow shows in the thermal variant and in no other
