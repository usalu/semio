@capability-gltf-2-0-mutate
@oracle-three-gltf-2-0-mutate-reader
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-skin
Feature: Apply every registered glTF 2.0 skin mutation to a real-world document
  The `gltf-2-0-skin` catalog (`../../🧪️oracle/🔣️.json`) declares the 4 kinds `document/skins` owns
  (§5.7.3/§20.7): `create-skin`, `delete-skin`, `move-skin`, `reorder-skins`. Shard A6 already
  scaffolded this catalog and its 4 committed `before.gltf`/`after.gltf` fixture pairs
  (`../../🧫️fixtures/<kind>-applied/`), each derived from the same `gltf-2-0-any-reader-oracle` base
  document the artifact-root `mutate-gltf-2-0` case uses — this case claims it. Every real leaf
  directory (`../../../✳️any/🧬️schema/🧬️mutations/{🌱️🧥️create-skin,🗑️🧥️delete-skin,🚚️🧥️move-skin,
  🔀️🧥️reorder-skins}/🦀️.rs`) stays physically owned by `✳️any`'s aggregate mutation root —
  `validate_mutation_leaf_source` requires a leaf's `owner` to be an immediate child of its aggregate
  root, so ownership here is declared through this catalog and the manifest's per-mutation `subset`
  override, never through moving the directory.

  The independent oracle (`../../../✳️any/🧪️oracle/🦀️.rs`) is the SAME domain-blind `json`-crate
  GLB/JSON reader the artifact-root case measures its 7 kinds through and the `✳️camera` case
  extends with 4 more; this case adds 4 further kinds sharing the SAME generic `IndexChange`/
  `remap_index`/`apply_node_ref_change` machinery camera's kinds introduced (the identical
  four-branch index-remap arithmetic `../../../✳️any/🔨️modules/🧬️mutation-support/
  🗂️top-level-collections/🦀️.rs`'s own `repair`/`family_ops!` apply to a `nodes/{i}/skin`
  reference — `document/skins` is the only other top-level family, besides `document/cameras`, a
  bare node scalar field points at). `create-skin`'s own payload
  (`GltfCreateSkinPayload { position }`) carries no field content, so `delete-skin`'s inverse cannot
  be expressed as a second `create-skin` call the way camera's content-bearing `create-camera` could
  invert `delete-camera` — production dispatches this inverse through `DeleteSkinMutation`'s own
  diff-based `Restore` variant instead, which this domain-blind reader has no typed access to.
  `undo_delete_skin` (`../../../✳️any/🧪️oracle/🦀️.rs`) reimplements the SAME "restore the exact
  removed content" law directly against the original document's own `skins`/`nodes[].skin` fields —
  documented in the oracle module alongside its sibling `undo_create_scene`, which the artifact-root
  case's own `create-scene` needs for the identical reason.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://<id>-applied/before.gltf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id             | params                       |
      | create-skin    | {"position":1}               |
      | delete-skin    | {"index":0}                  |
      | move-skin      | {"index":1,"position":0}     |
      | reorder-skins  | {"order":[1,0]}              |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://<id>-applied/before.gltf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the document matches its pre-mutation semantic projection
    Examples:
      | id             | params                       |
      | create-skin    | {"position":1}               |
      | delete-skin    | {"index":0}                  |
      | move-skin      | {"index":1,"position":0}     |
      | reorder-skins  | {"order":[1,0]}              |
