@capability-staffage-model-scales
Feature: One individually authored 3D model per scale and mode
  `modelOf` reads `volumes[scale]`: tone views are drawn from `body`, the thermal view from the `thermal` solids (one material per region and layer, painted from the solved field), forces and moments from the `structure` solids
  (member and joint solids coloured by their member's value, supports and masses neutral) plus the generated glyphs. `modelOfAsset` builds one document with one kernel scene per authored scale and mode over one skeleton.
  A scale that is not authored is a typed refusal, never a fallback to another scale.

  THE REFERENCES are the kernel validator and Ajv with the kernel JSON Schema for the documents.

  Scenario: Every scale and view is a valid document of the mode's own list
    Given the sample figure with its handcrafted four-scale volumes
    When the model of every scale and every view is built
    Then it validates, holds exactly the solids of the list of the view's mode, and carries the views of that mode as variants

  Scenario: The scales and the lists differ
    Given the four scales and three lists
    Then the body solid count falls from 1:50 to 1:200, the structure list differs from the body list, and a thermal solid that references a body solid shares its geometry

  Scenario: The thermal and structural looks follow the solved analyses
    Given the thermal and force views at 1:50
    Then every thermal region has a material, and a joint solid has the colour of its member

  Scenario: An unavailable or unknown scale is refused by name
    Given an asset with an unavailable 1:200 volume and a scale 77
    When the model is asked for
    Then the refusal names model-scale-unavailable or model-scale-unknown and no other scale answers

  Scenario: One document holds every scale and mode
    Given the authored scales
    When modelOfAsset builds the document
    Then it has a scene per scale and mode in order over one skeleton, the seven views as variants, skips an unavailable scale, tessellates scene by scene with falling triangles, is deterministic and cancellable
