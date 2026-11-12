@capability-staffage-model-glyph-edges
Feature: The feature edges of glyphs follow their glyph
  A force arrow, a moment arc or an emitter cone is drawn only by the variant of its view. The feature-edge pass gives the edges of the glyph solids a material of their own that each
  variant maps exactly as it maps the glyph surface, so the outline of a glyph never shows under a view that hides the glyph.

  THE REFERENCES are the Khronos gltf-validator, three's GLTFLoader (the parsed glTF) and the solid provenance of the kernel.

  Scenario: A GLB is valid
    Given the GLB of a pose of an asset with glyphs
    When the Khronos validator reads it
    Then it reports no error and no warning

  Scenario: No hidden glyph has a visible outline
    Given the GLB of a pose of an asset with force arrows, moment arcs or emitter cones
    When, per variant, the primitives with a visible material are collected by their glyph solid
    Then every visible glyph outline has its visible glyph surface in the same variant
