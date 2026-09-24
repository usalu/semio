@capability-gltf-2-0-mutate
@oracle-json-rust-gltf-2-0-mutate
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-buffer
Feature: Apply every registered glTF 2.0 buffer mutation to a real-world document
  The `gltf-2-0-buffer` catalog (`../../🔮️oracles/🔣️.json`) declares the 8 kinds that own
  document/buffers and document/bufferViews. Each kind runs against its own committed `⬅️before.gltf`, written by the three.js
  fixture generator (`../../../♾️any/🏭️generator/📜️script.ts`), with the parameters that recipe
  names.

  The oracle is `json-rust-gltf-2-0-mutate`: a second reading of the glTF 2.0 specification over
  a domain-blind JSON tree, including every reference a top-level family change must move — scene
  roots, children, skin joints and skeleton, animation targets, node meshes, primitive attributes,
  indices and morph targets, inverse-bind matrices, sampler input and output, accessor and image
  buffer views, and each buffer view's buffer. It is a cross-semio implementation, a supplement to
  a third-party reader rather than a substitute for one. The subject parses the same document into
  `GltfSnapshot`, applies the leaf's own typed `apply()` and serializes from the model alone.

  Every kind is also undone. A kind whose payload can carry the undo is inverted by the kinds the
  inverse column names; a kind whose payload cannot (every delete, a node weight override, a new
  morph target) restores the listed top-level members from the original document.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://<input>
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>, "fixture": "shared://<input>"}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id | input | params |
      | create-buffer | 🌱️create-buffer-applied/⬅️before.gltf | {"position": 1, "bytes": [0, 0, 0, 0]} |
      | create-buffer-view | 🪟️create-buffer-view-applied/⬅️before.gltf | {"position": 1, "buffer": 0, "byteOffset": 0, "byteLength": 4} |
      | delete-buffer | 🗑️delete-buffer-applied/⬅️before.gltf | {"index": 1} |
      | delete-buffer-view | ✂️delete-buffer-view-applied/⬅️before.gltf | {"index": 18} |
      | move-buffer | 🚚️move-buffer-applied/⬅️before.gltf | {"index": 1, "position": 0} |
      | move-buffer-view | ➡️move-buffer-view-applied/⬅️before.gltf | {"index": 18, "position": 0} |
      | reorder-buffer-views | 🔢️reorder-buffer-views-applied/⬅️before.gltf | {"order": [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]} |
      | reorder-buffers | 🔀️reorder-buffers-applied/⬅️before.gltf | {"order": [1, 0]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://<input>
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>, "fixture": "shared://<input>", "inverse": <inverse>, "restore": <restore>}
      """
    And the mutation's own inverse is applied to the result
    Then the document matches its pre-mutation semantic projection
    Examples:
      | id | input | params | inverse | restore |
      | create-buffer | 🌱️create-buffer-applied/⬅️before.gltf | {"position": 1, "bytes": [0, 0, 0, 0]} | [{"kind": "delete-buffer", "params": {"index": 1}}] | null |
      | create-buffer-view | 🪟️create-buffer-view-applied/⬅️before.gltf | {"position": 1, "buffer": 0, "byteOffset": 0, "byteLength": 4} | [{"kind": "delete-buffer-view", "params": {"index": 1}}] | null |
      | delete-buffer | 🗑️delete-buffer-applied/⬅️before.gltf | {"index": 1} | null | ["bufferViews", "buffers"] |
      | delete-buffer-view | ✂️delete-buffer-view-applied/⬅️before.gltf | {"index": 18} | null | ["accessors", "images", "bufferViews"] |
      | move-buffer | 🚚️move-buffer-applied/⬅️before.gltf | {"index": 1, "position": 0} | [{"kind": "move-buffer", "params": {"index": 0, "position": 1}}] | null |
      | move-buffer-view | ➡️move-buffer-view-applied/⬅️before.gltf | {"index": 18, "position": 0} | [{"kind": "move-buffer-view", "params": {"index": 0, "position": 18}}] | null |
      | reorder-buffer-views | 🔢️reorder-buffer-views-applied/⬅️before.gltf | {"order": [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]} | [{"kind": "reorder-buffer-views", "params": {"order": [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]}}] | null |
      | reorder-buffers | 🔀️reorder-buffers-applied/⬅️before.gltf | {"order": [1, 0]} | [{"kind": "reorder-buffers", "params": {"order": [1, 0]}}] | null |
