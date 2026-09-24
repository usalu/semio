@capability-gltf-2-0-mutate
@oracle-json-rust-gltf-2-0-mutate
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-mesh
Feature: Apply every registered glTF 2.0 mesh mutation to a real-world document
  The `gltf-2-0-mesh` catalog (`../../🔮️oracles/🔣️.json`) declares the 35 kinds that own
  document/meshes, their primitives and morph targets, and document/accessors. Each kind runs against its own committed `⬅️before.gltf`, written by the three.js
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
      | bind-morph-target-attribute | 🎚️morph-attribute/🔗️bind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "semantic": "NORMAL", "accessor": 1} |
      | bind-primitive-attribute | 🔤️primitive-attribute/🔗️bind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "semantic": "COLOR_0", "accessor": 3} |
      | bind-primitive-indices | 🔢️primitive-indices/🔗️bind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "accessor": 4} |
      | bind-primitive-material | 🧱️primitive-material/🔗️bind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "material": 1} |
      | change-mesh-extension-data | 🕸️mesh/🧩️change-extensions/⬅️before.gltf | {"mesh": 0, "data": {"state": "present", "value": {"ACME_marker": {"on": true}}}} |
      | change-mesh-extra-data | 🕸️mesh/📝️change-extras/⬅️before.gltf | {"mesh": 0, "data": {"state": "present", "value": {"meshKind": "fixture", "state": "mutated"}}} |
      | change-mesh-morph-weights | 🕸️mesh/⚖️change-weights/⬅️before.gltf | {"mesh": 0, "weights": [0.75]} |
      | change-mesh-name | 🕸️mesh/🏷️rename/⬅️before.gltf | {"mesh": 0, "value": "renamedMesh0"} |
      | change-primitive-extension-data | 🔺️primitive/🧩️change-extensions/⬅️before.gltf | {"mesh": 0, "primitive": 0, "data": {"state": "present", "value": {"ACME_marker": {"on": true}}}} |
      | change-primitive-extra-data | 🔺️primitive/📝️change-extras/⬅️before.gltf | {"mesh": 0, "primitive": 0, "data": {"state": "present", "value": {"state": "mutated"}}} |
      | change-primitive-topology-mode | 🔺️primitive/📐️change-topology/⬅️before.gltf | {"mesh": 0, "primitive": 0, "mode": 0} |
      | create-accessor | 📐️accessor/🌱️create/⬅️before.gltf | {"position": 1, "componentType": 5126, "count": 1, "kind": "SCALAR"} |
      | create-mesh | 🕸️mesh/🌱️create/⬅️before.gltf | {"position": 1} |
      | create-morph-target | 🧬️morph-target/🌱️create/⬅️before.gltf | {"mesh": 0, "primitive": 0, "position": 1} |
      | create-primitive | 🔺️primitive/🌱️create/⬅️before.gltf | {"mesh": 0, "position": 1} |
      | delete-accessor | 📐️accessor/🗑️delete/⬅️before.gltf | {"index": 18} |
      | delete-mesh | 🕸️mesh/🗑️delete/⬅️before.gltf | {"index": 1} |
      | delete-morph-target | 🧬️morph-target/🗑️delete/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0} |
      | delete-primitive | 🔺️primitive/🗑️delete/⬅️before.gltf | {"mesh": 0, "primitive": 0} |
      | move-accessor | 📐️accessor/🚚️move/⬅️before.gltf | {"index": 18, "position": 0} |
      | move-mesh | 🕸️mesh/🚚️move/⬅️before.gltf | {"index": 0, "position": 1} |
      | move-morph-target | 🧬️morph-target/🚚️move/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "position": 1} |
      | move-morph-target-attribute | 🎚️morph-attribute/🚚️move/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "semantic": "POSITION", "position": 1} |
      | move-primitive | 🔺️primitive/🚚️move/⬅️before.gltf | {"mesh": 0, "primitive": 0, "position": 1} |
      | move-primitive-attribute | 🔤️primitive-attribute/🚚️move/⬅️before.gltf | {"mesh": 0, "primitive": 0, "semantic": "POSITION", "position": 2} |
      | reorder-accessors | 📐️accessor/🔀️reorder/⬅️before.gltf | {"order": [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]} |
      | reorder-meshs | 🕸️mesh/🔀️reorder/⬅️before.gltf | {"order": [2, 1, 0]} |
      | reorder-morph-target-attributes | 🎚️morph-attribute/🔀️reorder/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "order": ["NORMAL", "POSITION"]} |
      | reorder-morph-targets | 🧬️morph-target/🔀️reorder/⬅️before.gltf | {"mesh": 0, "primitive": 0, "order": [1, 0]} |
      | reorder-primitive-attributes | 🔤️primitive-attribute/🔀️reorder/⬅️before.gltf | {"mesh": 0, "primitive": 0, "order": ["TEXCOORD_0", "NORMAL", "POSITION"]} |
      | reorder-primitives | 🔺️primitive/🔀️reorder/⬅️before.gltf | {"mesh": 0, "order": [1, 0]} |
      | unbind-morph-target-attribute | 🎚️morph-attribute/✂️unbind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "semantic": "POSITION"} |
      | unbind-primitive-attribute | 🔤️primitive-attribute/✂️unbind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "semantic": "NORMAL"} |
      | unbind-primitive-indices | 🔢️primitive-indices/✂️unbind/⬅️before.gltf | {"mesh": 0, "primitive": 0} |
      | unbind-primitive-material | 🧱️primitive-material/✂️unbind/⬅️before.gltf | {"mesh": 0, "primitive": 0} |

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
      | bind-morph-target-attribute | 🎚️morph-attribute/🔗️bind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "semantic": "NORMAL", "accessor": 1} | [{"kind": "unbind-morph-target-attribute", "params": {"mesh": 0, "primitive": 0, "target": 0, "semantic": "NORMAL"}}] | null |
      | bind-primitive-attribute | 🔤️primitive-attribute/🔗️bind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "semantic": "COLOR_0", "accessor": 3} | [{"kind": "unbind-primitive-attribute", "params": {"mesh": 0, "primitive": 0, "semantic": "COLOR_0"}}] | null |
      | bind-primitive-indices | 🔢️primitive-indices/🔗️bind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "accessor": 4} | [{"kind": "unbind-primitive-indices", "params": {"mesh": 0, "primitive": 0}}] | null |
      | bind-primitive-material | 🧱️primitive-material/🔗️bind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "material": 1} | [{"kind": "bind-primitive-material", "params": {"mesh": 0, "primitive": 0, "material": 0}}] | null |
      | change-mesh-extension-data | 🕸️mesh/🧩️change-extensions/⬅️before.gltf | {"mesh": 0, "data": {"state": "present", "value": {"ACME_marker": {"on": true}}}} | [{"kind": "change-mesh-extension-data", "params": {"mesh": 0, "data": {"state": "absent"}}}] | null |
      | change-mesh-extra-data | 🕸️mesh/📝️change-extras/⬅️before.gltf | {"mesh": 0, "data": {"state": "present", "value": {"meshKind": "fixture", "state": "mutated"}}} | [{"kind": "change-mesh-extra-data", "params": {"mesh": 0, "data": {"state": "present", "value": {"meshKind": "fixture"}}}}] | null |
      | change-mesh-morph-weights | 🕸️mesh/⚖️change-weights/⬅️before.gltf | {"mesh": 0, "weights": [0.75]} | [{"kind": "change-mesh-morph-weights", "params": {"mesh": 0, "weights": [0.5]}}] | null |
      | change-mesh-name | 🕸️mesh/🏷️rename/⬅️before.gltf | {"mesh": 0, "value": "renamedMesh0"} | [{"kind": "change-mesh-name", "params": {"mesh": 0, "value": null}}] | null |
      | change-primitive-extension-data | 🔺️primitive/🧩️change-extensions/⬅️before.gltf | {"mesh": 0, "primitive": 0, "data": {"state": "present", "value": {"ACME_marker": {"on": true}}}} | [{"kind": "change-primitive-extension-data", "params": {"mesh": 0, "primitive": 0, "data": {"state": "absent"}}}] | null |
      | change-primitive-extra-data | 🔺️primitive/📝️change-extras/⬅️before.gltf | {"mesh": 0, "primitive": 0, "data": {"state": "present", "value": {"state": "mutated"}}} | [{"kind": "change-primitive-extra-data", "params": {"mesh": 0, "primitive": 0, "data": {"state": "absent"}}}] | null |
      | change-primitive-topology-mode | 🔺️primitive/📐️change-topology/⬅️before.gltf | {"mesh": 0, "primitive": 0, "mode": 0} | [{"kind": "change-primitive-topology-mode", "params": {"mesh": 0, "primitive": 0, "mode": 4}}] | null |
      | create-accessor | 📐️accessor/🌱️create/⬅️before.gltf | {"position": 1, "componentType": 5126, "count": 1, "kind": "SCALAR"} | [{"kind": "delete-accessor", "params": {"index": 1}}] | null |
      | create-mesh | 🕸️mesh/🌱️create/⬅️before.gltf | {"position": 1} | [{"kind": "delete-mesh", "params": {"index": 1}}] | null |
      | create-morph-target | 🧬️morph-target/🌱️create/⬅️before.gltf | {"mesh": 0, "primitive": 0, "position": 1} | null | ["meshes"] |
      | create-primitive | 🔺️primitive/🌱️create/⬅️before.gltf | {"mesh": 0, "position": 1} | [{"kind": "delete-primitive", "params": {"mesh": 0, "primitive": 1}}] | null |
      | delete-accessor | 📐️accessor/🗑️delete/⬅️before.gltf | {"index": 18} | null | ["meshes", "skins", "animations", "accessors"] |
      | delete-mesh | 🕸️mesh/🗑️delete/⬅️before.gltf | {"index": 1} | null | ["nodes", "meshes"] |
      | delete-morph-target | 🧬️morph-target/🗑️delete/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0} | null | ["meshes"] |
      | delete-primitive | 🔺️primitive/🗑️delete/⬅️before.gltf | {"mesh": 0, "primitive": 0} | null | ["meshes"] |
      | move-accessor | 📐️accessor/🚚️move/⬅️before.gltf | {"index": 18, "position": 0} | [{"kind": "move-accessor", "params": {"index": 0, "position": 18}}] | null |
      | move-mesh | 🕸️mesh/🚚️move/⬅️before.gltf | {"index": 0, "position": 1} | [{"kind": "move-mesh", "params": {"index": 1, "position": 0}}] | null |
      | move-morph-target | 🧬️morph-target/🚚️move/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "position": 1} | [{"kind": "move-morph-target", "params": {"mesh": 0, "primitive": 0, "target": 1, "position": 0}}] | null |
      | move-morph-target-attribute | 🎚️morph-attribute/🚚️move/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "semantic": "POSITION", "position": 1} | [{"kind": "move-morph-target-attribute", "params": {"mesh": 0, "primitive": 0, "target": 0, "semantic": "POSITION", "position": 0}}] | null |
      | move-primitive | 🔺️primitive/🚚️move/⬅️before.gltf | {"mesh": 0, "primitive": 0, "position": 1} | [{"kind": "move-primitive", "params": {"mesh": 0, "primitive": 1, "position": 0}}] | null |
      | move-primitive-attribute | 🔤️primitive-attribute/🚚️move/⬅️before.gltf | {"mesh": 0, "primitive": 0, "semantic": "POSITION", "position": 2} | [{"kind": "move-primitive-attribute", "params": {"mesh": 0, "primitive": 0, "semantic": "POSITION", "position": 0}}] | null |
      | reorder-accessors | 📐️accessor/🔀️reorder/⬅️before.gltf | {"order": [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]} | [{"kind": "reorder-accessors", "params": {"order": [18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]}}] | null |
      | reorder-meshs | 🕸️mesh/🔀️reorder/⬅️before.gltf | {"order": [2, 1, 0]} | [{"kind": "reorder-meshs", "params": {"order": [2, 1, 0]}}] | null |
      | reorder-morph-target-attributes | 🎚️morph-attribute/🔀️reorder/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "order": ["NORMAL", "POSITION"]} | [{"kind": "reorder-morph-target-attributes", "params": {"mesh": 0, "primitive": 0, "target": 0, "order": ["POSITION", "NORMAL"]}}] | null |
      | reorder-morph-targets | 🧬️morph-target/🔀️reorder/⬅️before.gltf | {"mesh": 0, "primitive": 0, "order": [1, 0]} | [{"kind": "reorder-morph-targets", "params": {"mesh": 0, "primitive": 0, "order": [1, 0]}}] | null |
      | reorder-primitive-attributes | 🔤️primitive-attribute/🔀️reorder/⬅️before.gltf | {"mesh": 0, "primitive": 0, "order": ["TEXCOORD_0", "NORMAL", "POSITION"]} | [{"kind": "reorder-primitive-attributes", "params": {"mesh": 0, "primitive": 0, "order": ["POSITION", "NORMAL", "TEXCOORD_0"]}}] | null |
      | reorder-primitives | 🔺️primitive/🔀️reorder/⬅️before.gltf | {"mesh": 0, "order": [1, 0]} | [{"kind": "reorder-primitives", "params": {"mesh": 0, "order": [1, 0]}}] | null |
      | unbind-morph-target-attribute | 🎚️morph-attribute/✂️unbind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "target": 0, "semantic": "POSITION"} | [{"kind": "bind-morph-target-attribute", "params": {"mesh": 0, "primitive": 0, "target": 0, "semantic": "POSITION", "accessor": 3}}] | null |
      | unbind-primitive-attribute | 🔤️primitive-attribute/✂️unbind/⬅️before.gltf | {"mesh": 0, "primitive": 0, "semantic": "NORMAL"} | [{"kind": "bind-primitive-attribute", "params": {"mesh": 0, "primitive": 0, "semantic": "NORMAL", "accessor": 1}}, {"kind": "move-primitive-attribute", "params": {"mesh": 0, "primitive": 0, "semantic": "NORMAL", "position": 1}}] | null |
      | unbind-primitive-indices | 🔢️primitive-indices/✂️unbind/⬅️before.gltf | {"mesh": 0, "primitive": 0} | [{"kind": "bind-primitive-indices", "params": {"mesh": 0, "primitive": 0, "accessor": 4}}] | null |
      | unbind-primitive-material | 🧱️primitive-material/✂️unbind/⬅️before.gltf | {"mesh": 0, "primitive": 0} | [{"kind": "bind-primitive-material", "params": {"mesh": 0, "primitive": 0, "material": 0}}] | null |
