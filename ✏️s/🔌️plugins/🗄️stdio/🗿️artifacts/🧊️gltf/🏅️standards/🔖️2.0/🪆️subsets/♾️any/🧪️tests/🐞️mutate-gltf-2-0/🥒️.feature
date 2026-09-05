@capability-gltf-2-0-mutate
@oracle-three-gltf-2-0-mutate-reader
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-any
Feature: Apply every registered typed glTF 2.0 mutation to a real-world document
  glTF has no `pub enum GltfMutation` -- its vocabulary is a descriptor table,
  `GLTF_MUTATION_LEAF_DESCRIPTORS`. 120 real leaf directories exist on disk with complete
  mutation/diff/inverse code and a committed fixture each, but only 7 are mounted as production
  modules AND listed in that descriptor table today; this case covers exactly those 7, honestly
  smaller than the 120 that exist. The input is the real 284 KB, 271-node, 2-material `base.glb`
  export (asset://📚️examples/🌱️metabolism/🖼️assets/🧪️base/🧊️.glb) with
  one minimal, real-data-preserving derivation applied once: node 1 was moved out of the scene's
  271-entry root-node list and into node 0's own `children`, since the real export's whole node
  graph is otherwise flat (every node a direct scene root, none nested) and two of the seven
  registered kinds (`bind-node-child`/`unbind-node-child`) need an existing or creatable parent/child
  edge to exercise. Every other byte, including the whole BIN chunk (skinning/mesh geometry), is the
  untouched real export; both the derived fixture (local://🧪️base-with-nested-node/🧊️.glb) and the
  pristine real source are committed, so the substitution is auditable. Every scenario copies the
  fixture into the case work directory before touching it; the committed files are never written to.
  The oracle performs every kind by direct, independent GLB-container and JSON-tree manipulation
  (../../🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔮️oracle/🦀️component.rs, using the already-linked `json`
  0.12 crate as the JSON layer only -- every mutation's own semantics are reimplemented from scratch,
  never delegated to `json`'s domain-blind reader/writer); the subject fully parses into `GltfSnapshot`
  and re-serializes from it alone (no byte pass-through). `create-scene` has no separate
  `delete-scene` kind of its own -- production inverts it through the SAME descriptor's own
  `phase: Inverse`, not a different command -- so its inverse scenario below is discharged by this
  oracle's own `undo_create_scene`, not by another catalog kind.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document local://🧪️base-with-nested-node/🧊️.glb
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                           | params                                          |
      | bind-node-child              | {"parent":2,"child":3,"position":0}             |
      | bind-scene-root-node         | {"scene":0,"node":1,"position":0}               |
      | change-material-alpha-mode   | {"material":0,"alphaMode":"MASK"}               |
      | change-material-double-sided | {"material":0,"doubleSided":true}               |
      | create-scene                 | {"position":0}                                  |
      | unbind-node-child            | {"parent":0,"child":1}                          |
      | unbind-scene-root-node       | {"scene":0,"node":5}                            |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document local://🧪️base-with-nested-node/🧊️.glb
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the document matches its pre-mutation semantic projection
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
    Given the real input document local://🧪️base-with-nested-node/🧊️.glb
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the oracle and the subject agree on the semantic projection
