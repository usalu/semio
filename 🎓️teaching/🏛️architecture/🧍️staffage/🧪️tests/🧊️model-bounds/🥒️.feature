@capability-staffage-model-bounds
Feature: The 3D model has the extent of the asset and of its 2D views
  The volume solids of an asset fill the physical envelope its `dimensions` state: at rest the extent ahead is `widthM`, up `heightM`, lateral `depthM`, and in every named pose the extent of the
  posed, skinned model equals the extent of the elevation, front and plan shapes of the same evaluated state. Tolerance is the catalog's D18: the larger of 2 % and 15 mm.

  THE REFERENCES are three.js (the precise skinned extent of the GLB loaded by GLTFLoader) and the 2D shapes of `evaluate`.

  Scenario: At rest the model fills the dimensions
    Given the sample figure
    When the solids are tessellated and carried by the rest placements
    Then the extent ahead, up and lateral equals widthM, heightM and depthM within the tolerance

  Scenario: In every pose the model has the extent of the 2D views
    Given each named pose of the sample figure exported as a GLB and loaded by three.js
    When its precise extent is compared with the elevation, front and plan shapes of the same state
    Then the ahead extent equals the elevation and plan, the up extent the elevation and the lateral extent the front and plan, within the tolerance

  Scenario: A standing pose stands on the ground
    Given the standing pose
    When its GLB is loaded
    Then its lowest point is the ground within 15 mm
