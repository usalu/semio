@capability-staffage-model-scale-files
Feature: The file set of a four-scale asset passes every independent oracle
  A GLB per pose holds a glTF scene per authored scale and mode (the seven views as variants), an FBX per clip, scale and view is one scene, the model JSON holds the same kernel scenes, the ZIP equals the files.

  THE REFERENCES are the Khronos gltf-validator, three's GLTFLoader and FBXLoader, glTF-Transform, assimp, Ajv with the semio.model/v1 JSON Schema and fflate.

  Scenario: The geometry of every scene is valid
    Given the handcrafted four-scale sample figure
    Then every scene is finite and watertight where its solid kind is closed and its triangles never grow towards coarser scales

  Scenario: The exported files are valid and consistent
    Given the whole file set exported
    Then every GLB has the scenes in order and is read by three and glTF-Transform, every FBX loads in three and assimp and equals the triangles of its GLB scene (body and thermal), every model JSON validates with the same scenes, the body extent of every scale fits the dimensions and the ZIP equals its files
