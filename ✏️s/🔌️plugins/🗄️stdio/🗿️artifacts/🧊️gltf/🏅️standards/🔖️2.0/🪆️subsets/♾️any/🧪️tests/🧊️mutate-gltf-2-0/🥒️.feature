@capability-gltf-2-0-mutate
@oracle-three-gltf-2-0-mutate-reader
@comparison-semantic-gltf-reader-v1
@mutations-gltf-2-0-any
Feature: Apply registered typed glTF 2.0 mutations to a real-world document
  The subset's `GltfMutation` vocabulary (`../../🧬️schema/🧬️mutations/🦀️.rs`) is judged kind by kind in the
  per-family subset cases against one synthetic three-exported base; this artifact-root case is the one that
  measures seven of those kinds against a REAL document instead — honestly a sample, chosen to cover the node
  hierarchy, the scene root list, the default-scene pointer and material flags. The input is the real 284 KB,
  271-node, 2-material `base.glb` export (asset://🌱️metabolism/🏙️base/🧊️.glb) with one minimal,
  real-data-preserving derivation applied once: node 1 was moved out of the scene's 271-entry root-node list and
  into node 0's own `children`, since the real export's whole node graph is otherwise flat (every node a direct
  scene root, none nested) and two of the seven kinds (`bind-node-child`/`unbind-node-child`) need an existing or
  creatable parent/child edge to exercise. Every other byte, including the whole BIN chunk (skinning/mesh
  geometry), is the untouched real export; both the derived fixture
  (shared://🧊️mutate-gltf-2-0/🌳️base-with-nested-node/🧊️.glb) and the pristine real source are committed, so the
  substitution is auditable. Every scenario copies the fixture into the case work directory before touching it;
  the committed files are never written to.

  The judge is `three`'s GLTFLoader, a third-party READER (`three-gltf-2-0-mutate-reader`): the expected document
  is not computed, it is the COMMITTED `➡️after.glb` of each row (the real input itself for an inverse row and for
  the identity round trip), and the `gltf-2-0-three-compare-v1` pipeline reads it and the subject's document with
  the same loader (`gltf-import` admits both, `gltf-compare` compares the `semantic-gltf-reader-v1` projection).
  Each row's after is written by the subset generator's real-input rows
  (`../../🏭️generator/📜️script.ts generate --only <kind>-metabolism-applied`): the row's edit applied to the real
  container's JSON chunk by the same generic structural operations every synthetic recipe uses, the BIN chunk
  carried byte for byte. The subject parses the GLB into `GltfSnapshot`, applies the row's production mutation
  (and, for an inverse row, that mutation's OWN computed inverse) and re-serializes from the model alone, never
  passing bytes through. The cross-semio supplement (`../../🔮️oracles/🦀️.rs`) performs every kind by
  independent GLB-container and JSON-tree manipulation with the `json` 0.12 crate as the JSON layer only.
  `create-scene` has no separate `delete-scene` kind of its own — production inverts it through the SAME
  mutation's computed inverse, not a different command.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🧊️mutate-gltf-2-0/🌳️base-with-nested-node/🧊️.glb
    And the committed after-document shared://🧊️mutate-gltf-2-0/<fixture>/➡️after.glb
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then three's GLTFLoader reads the subject's document and the committed after-document as the same glTF
    Examples:
      | id                           | fixture                          | params                                          |
      | bind-node-child              | 🔗️bind-node-child              | {"parent":2,"child":3,"position":0}             |
      | bind-scene-root-node         | 🌲️bind-scene-root-node         | {"scene":0,"node":1,"position":0}               |
      | change-material-alpha-mode   | 🎭️change-material-alpha-mode   | {"material":0,"alphaMode":"MASK"}               |
      | change-material-double-sided | 🪞️change-material-double-sided | {"material":0,"doubleSided":true}               |
      | create-scene                 | 🎬️create-scene                 | {"position":0}                                  |
      | unbind-node-child            | ✂️unbind-node-child            | {"parent":0,"child":1}                          |
      | unbind-scene-root-node       | 🍂️unbind-scene-root-node       | {"scene":0,"node":5}                            |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://🧊️mutate-gltf-2-0/🌳️base-with-nested-node/🧊️.glb
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then three's GLTFLoader reads the restored document and the real input as the same glTF
    Examples:
      | id                           | params                                          |
      | bind-node-child              | {"parent":2,"child":3,"position":0}             |
      | bind-scene-root-node         | {"scene":0,"node":1,"position":0}               |
      | change-material-alpha-mode   | {"material":0,"alphaMode":"MASK"}               |
      | change-material-double-sided | {"material":0,"doubleSided":true}               |
      | create-scene                 | {"position":0}                                  |
      | unbind-node-child            | {"parent":0,"child":1}                          |
      | unbind-scene-root-node       | {"scene":0,"node":5}                            |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://🧊️mutate-gltf-2-0/🌳️base-with-nested-node/🧊️.glb
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And three's GLTFLoader reads the re-encoded document and the real input as the same glTF
