@capability-gltf-2-0-mutate
@oracle-json-rust-gltf-2-0-mutate
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-scene
Feature: Apply every registered glTF 2.0 scene mutation to a real-world document
  The `gltf-2-0-scene` catalog (`../../🔮️oracles/🔣️.json`) declares the 33 kinds that own
  document/scenes, document/nodes and every node binding. Each kind runs against its own committed `⬅️before.gltf`, written by the three.js
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
      | bind-default-scene | 🏠️default-scene/🔗️bind/⬅️before.gltf | {"scene": 1} |
      | bind-node-camera | 📷️node-camera/🔗️bind/⬅️before.gltf | {"node": 6, "camera": 0} |
      | bind-node-child | 🌿️node-child/🔗️bind/⬅️before.gltf | {"parent": 1, "child": 6, "position": 0} |
      | bind-node-mesh | 🏗️node-mesh/🔗️bind/⬅️before.gltf | {"node": 6, "mesh": 0} |
      | bind-node-skin | 🩻️node-skin/🔗️bind/⬅️before.gltf | {"node": 6, "skin": 0} |
      | bind-scene-root-node | 🌲️scene-root/🔗️bind/⬅️before.gltf | {"scene": 1, "node": 6, "position": 0} |
      | change-node-extension-data | 🌳️node/🧩️change-extensions/⬅️before.gltf | {"node": 6, "data": {"state": "present", "value": {"ACME_marker": {"on": true}}}} |
      | change-node-extra-data | 🌳️node/📝️change-extras/⬅️before.gltf | {"node": 6, "data": {"state": "present", "value": {"nodeKind": "fixture", "state": "mutated"}}} |
      | change-node-morph-weights | 🌳️node/⚖️change-weights/⬅️before.gltf | {"node": 1, "weights": [0.9]} |
      | change-node-name | 🌳️node/🏷️rename/⬅️before.gltf | {"node": 6, "value": "renamedEmptyNode"} |
      | change-node-transform | 🌳️node/📐️transform/⬅️before.gltf | {"node": 6, "transform": {"kind": "trs", "translation": [2, 3, 4]}} |
      | change-scene-extension-data | 🎬️scene/🧩️change-extensions/⬅️before.gltf | {"scene": 1, "data": {"state": "present", "value": {"ACME_marker": {"on": true}}}} |
      | change-scene-extra-data | 🎬️scene/📝️change-extras/⬅️before.gltf | {"scene": 1, "data": {"state": "present", "value": {"sceneKind": "fixture", "state": "mutated"}}} |
      | change-scene-name | 🎬️scene/🏷️rename/⬅️before.gltf | {"scene": 1, "value": "renamedSceneB"} |
      | create-node | 🌳️node/🌱️create/⬅️before.gltf | {"position": 1} |
      | create-scene | 🎬️scene/🌱️create/⬅️before.gltf | {"position": 1} |
      | delete-node | 🌳️node/🗑️delete/⬅️before.gltf | {"index": 7} |
      | delete-scene | 🎬️scene/🗑️delete/⬅️before.gltf | {"index": 1} |
      | move-node | 🌳️node/🚚️move/⬅️before.gltf | {"index": 6, "position": 0} |
      | move-node-child | 🌿️node-child/🚚️move/⬅️before.gltf | {"parent": 0, "child": 6, "position": 0} |
      | move-node-parent | 🌳️node/🌿️reparent/⬅️before.gltf | {"parent": 7, "child": 6, "position": 0} |
      | move-scene | 🎬️scene/🚚️move/⬅️before.gltf | {"index": 0, "position": 1} |
      | move-scene-root-node | 🌲️scene-root/🚚️move/⬅️before.gltf | {"scene": 0, "node": 6, "position": 0} |
      | reorder-node-children | 🌿️node-child/🔀️reorder/⬅️before.gltf | {"parent": 0, "order": [6, 4, 3, 2, 1]} |
      | reorder-nodes | 🌳️node/🔀️reorder/⬅️before.gltf | {"order": [7, 6, 5, 4, 3, 2, 1, 0]} |
      | reorder-scene-root-nodes | 🌲️scene-root/🔀️reorder/⬅️before.gltf | {"scene": 0, "order": [6, 0]} |
      | reorder-scenes | 🎬️scene/🔀️reorder/⬅️before.gltf | {"order": [1, 0]} |
      | unbind-default-scene | 🏠️default-scene/✂️unbind/⬅️before.gltf | {} |
      | unbind-node-camera | 📷️node-camera/✂️unbind/⬅️before.gltf | {"node": 3} |
      | unbind-node-child | 🌿️node-child/✂️unbind/⬅️before.gltf | {"parent": 0, "child": 6} |
      | unbind-node-mesh | 🏗️node-mesh/✂️unbind/⬅️before.gltf | {"node": 1} |
      | unbind-node-skin | 🩻️node-skin/✂️unbind/⬅️before.gltf | {"node": 4} |
      | unbind-scene-root-node | 🌲️scene-root/✂️unbind/⬅️before.gltf | {"scene": 1, "node": 7} |

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
      | bind-default-scene | 🏠️default-scene/🔗️bind/⬅️before.gltf | {"scene": 1} | [{"kind": "bind-default-scene", "params": {"scene": 0}}] | null |
      | bind-node-camera | 📷️node-camera/🔗️bind/⬅️before.gltf | {"node": 6, "camera": 0} | [{"kind": "unbind-node-camera", "params": {"node": 6}}] | null |
      | bind-node-child | 🌿️node-child/🔗️bind/⬅️before.gltf | {"parent": 1, "child": 6, "position": 0} | [{"kind": "unbind-node-child", "params": {"parent": 1, "child": 6}}] | null |
      | bind-node-mesh | 🏗️node-mesh/🔗️bind/⬅️before.gltf | {"node": 6, "mesh": 0} | [{"kind": "unbind-node-mesh", "params": {"node": 6}}] | null |
      | bind-node-skin | 🩻️node-skin/🔗️bind/⬅️before.gltf | {"node": 6, "skin": 0} | [{"kind": "unbind-node-skin", "params": {"node": 6}}] | null |
      | bind-scene-root-node | 🌲️scene-root/🔗️bind/⬅️before.gltf | {"scene": 1, "node": 6, "position": 0} | [{"kind": "unbind-scene-root-node", "params": {"scene": 1, "node": 6}}] | null |
      | change-node-extension-data | 🌳️node/🧩️change-extensions/⬅️before.gltf | {"node": 6, "data": {"state": "present", "value": {"ACME_marker": {"on": true}}}} | [{"kind": "change-node-extension-data", "params": {"node": 6, "data": {"state": "absent"}}}] | null |
      | change-node-extra-data | 🌳️node/📝️change-extras/⬅️before.gltf | {"node": 6, "data": {"state": "present", "value": {"nodeKind": "fixture", "state": "mutated"}}} | [{"kind": "change-node-extra-data", "params": {"node": 6, "data": {"state": "present", "value": {"nodeKind": "fixture"}}}}] | null |
      | change-node-morph-weights | 🌳️node/⚖️change-weights/⬅️before.gltf | {"node": 1, "weights": [0.9]} | null | ["nodes"] |
      | change-node-name | 🌳️node/🏷️rename/⬅️before.gltf | {"node": 6, "value": "renamedEmptyNode"} | [{"kind": "change-node-name", "params": {"node": 6, "value": "emptyNode"}}] | null |
      | change-node-transform | 🌳️node/📐️transform/⬅️before.gltf | {"node": 6, "transform": {"kind": "trs", "translation": [2, 3, 4]}} | [{"kind": "change-node-transform", "params": {"node": 6, "transform": {"kind": "trs"}}}] | null |
      | change-scene-extension-data | 🎬️scene/🧩️change-extensions/⬅️before.gltf | {"scene": 1, "data": {"state": "present", "value": {"ACME_marker": {"on": true}}}} | [{"kind": "change-scene-extension-data", "params": {"scene": 1, "data": {"state": "absent"}}}] | null |
      | change-scene-extra-data | 🎬️scene/📝️change-extras/⬅️before.gltf | {"scene": 1, "data": {"state": "present", "value": {"sceneKind": "fixture", "state": "mutated"}}} | [{"kind": "change-scene-extra-data", "params": {"scene": 1, "data": {"state": "present", "value": {"sceneKind": "fixture"}}}}] | null |
      | change-scene-name | 🎬️scene/🏷️rename/⬅️before.gltf | {"scene": 1, "value": "renamedSceneB"} | [{"kind": "change-scene-name", "params": {"scene": 1, "value": "sceneB"}}] | null |
      | create-node | 🌳️node/🌱️create/⬅️before.gltf | {"position": 1} | [{"kind": "delete-node", "params": {"index": 1}}] | null |
      | create-scene | 🎬️scene/🌱️create/⬅️before.gltf | {"position": 1} | [{"kind": "delete-scene", "params": {"index": 1}}] | null |
      | delete-node | 🌳️node/🗑️delete/⬅️before.gltf | {"index": 7} | null | ["scenes", "nodes", "skins", "animations"] |
      | delete-scene | 🎬️scene/🗑️delete/⬅️before.gltf | {"index": 1} | null | ["scene", "scenes"] |
      | move-node | 🌳️node/🚚️move/⬅️before.gltf | {"index": 6, "position": 0} | [{"kind": "move-node", "params": {"index": 0, "position": 6}}] | null |
      | move-node-child | 🌿️node-child/🚚️move/⬅️before.gltf | {"parent": 0, "child": 6, "position": 0} | [{"kind": "move-node-child", "params": {"parent": 0, "child": 6, "position": 4}}] | null |
      | move-node-parent | 🌳️node/🌿️reparent/⬅️before.gltf | {"parent": 7, "child": 6, "position": 0} | [{"kind": "move-node-parent", "params": {"parent": 0, "child": 6, "position": 4}}] | null |
      | move-scene | 🎬️scene/🚚️move/⬅️before.gltf | {"index": 0, "position": 1} | [{"kind": "move-scene", "params": {"index": 1, "position": 0}}] | null |
      | move-scene-root-node | 🌲️scene-root/🚚️move/⬅️before.gltf | {"scene": 0, "node": 6, "position": 0} | [{"kind": "move-scene-root-node", "params": {"scene": 0, "node": 6, "position": 1}}] | null |
      | reorder-node-children | 🌿️node-child/🔀️reorder/⬅️before.gltf | {"parent": 0, "order": [6, 4, 3, 2, 1]} | [{"kind": "reorder-node-children", "params": {"parent": 0, "order": [1, 2, 3, 4, 6]}}] | null |
      | reorder-nodes | 🌳️node/🔀️reorder/⬅️before.gltf | {"order": [7, 6, 5, 4, 3, 2, 1, 0]} | [{"kind": "reorder-nodes", "params": {"order": [7, 6, 5, 4, 3, 2, 1, 0]}}] | null |
      | reorder-scene-root-nodes | 🌲️scene-root/🔀️reorder/⬅️before.gltf | {"scene": 0, "order": [6, 0]} | [{"kind": "reorder-scene-root-nodes", "params": {"scene": 0, "order": [0, 6]}}] | null |
      | reorder-scenes | 🎬️scene/🔀️reorder/⬅️before.gltf | {"order": [1, 0]} | [{"kind": "reorder-scenes", "params": {"order": [1, 0]}}] | null |
      | unbind-default-scene | 🏠️default-scene/✂️unbind/⬅️before.gltf | {} | [{"kind": "bind-default-scene", "params": {"scene": 0}}] | null |
      | unbind-node-camera | 📷️node-camera/✂️unbind/⬅️before.gltf | {"node": 3} | [{"kind": "bind-node-camera", "params": {"node": 3, "camera": 0}}] | null |
      | unbind-node-child | 🌿️node-child/✂️unbind/⬅️before.gltf | {"parent": 0, "child": 6} | [{"kind": "bind-node-child", "params": {"parent": 0, "child": 6, "position": 4}}] | null |
      | unbind-node-mesh | 🏗️node-mesh/✂️unbind/⬅️before.gltf | {"node": 1} | [{"kind": "bind-node-mesh", "params": {"node": 1, "mesh": 0}}] | null |
      | unbind-node-skin | 🩻️node-skin/✂️unbind/⬅️before.gltf | {"node": 4} | [{"kind": "bind-node-skin", "params": {"node": 4, "skin": 0}}] | null |
      | unbind-scene-root-node | 🌲️scene-root/✂️unbind/⬅️before.gltf | {"scene": 1, "node": 7} | [{"kind": "bind-scene-root-node", "params": {"scene": 1, "node": 7, "position": 0}}] | null |
