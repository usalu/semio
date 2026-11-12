@capability-staffage-model-digests
Feature: GLB and FBX structure digests of the sample figure
  The sample figure is exported as a GLB per pose and an FBX per clip, and each file is summarised as a language-agnostic digest: its byte length and SHA-256, for a GLB the counts of
  the glTF document (triangles, line segments, primitives, nodes, meshes, materials, skins, accessors) and the seven variants, for an FBX the object counts of the file and the notices
  of the encoder. A second implementation reproduces the counts and the loaded structure.

  THE REFERENCES are the Khronos gltf-validator and three's GLTFLoader for each GLB, three's FBXLoader and assimp (assimpjs) for each FBX and the SHA-256 of node:crypto.

  Scenario: The vectors cover the sample figure
    Given the digest fixture
    Then it names every pose and every clip of the sample figure

  Scenario: A pose is a stable valid GLB
    Given a pose
    When its GLB is written
    Then its digest equals the vector and the Khronos validator and three's loader accept it

  Scenario: A clip is a stable loadable FBX
    Given a clip
    When its FBX is written
    Then its digest equals the vector and three's loader and assimp load it with every rig bone and a finite animation of the clip's duration
