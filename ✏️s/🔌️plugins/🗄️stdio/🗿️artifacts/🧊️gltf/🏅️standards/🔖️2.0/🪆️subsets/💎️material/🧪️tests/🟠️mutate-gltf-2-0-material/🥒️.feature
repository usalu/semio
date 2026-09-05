@capability-gltf-2-0-mutate
@oracle-three-gltf-2-0-mutate-reader
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-material
Feature: Apply every registered glTF 2.0 material mutation to a real-world document
  The `gltf-2-0-material` catalog (`../../🔮️oracle/🔣️.json`) declares the 18 kinds the 4 families
  `document/materials` (§5.20), `document/textures` (§5.31), `document/images` (§5.24) and
  `document/samplers` (§5.29) own — `create`/`delete`/`move`/`reorder` per family, plus
  `change-material-alpha-mode`/`change-material-double-sided` (already claimed by the
  artifact-root's own 7-kind case's oracle functions, reused here unmodified). Shard A6 already
  scaffolded this catalog and its 18 committed `before.gltf`/`after.gltf` fixture pairs
  (`../../🧫️fixtures/<kind>-applied/`), each derived from the same `gltf-2-0-any-reader-oracle` base
  document the artifact-root `🐞️mutate-gltf-2-0` case uses — this case claims it. Every real leaf
  directory stays physically owned by `♾️any`'s aggregate mutation root —
  `validate_mutation_leaf_source` requires a leaf's `owner` to be an immediate child of its
  aggregate root, so ownership here is declared through this catalog and the manifest's per-mutation
  `subset` override, never through moving the directory.

  Read `../../../♾️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs`'s own `repair`
  match before writing anything: `materials`/`images`/`samplers` are each a SINGLE simple
  `Option<usize>` reference site (`meshes[].primitives[].material`, `textures[].source`,
  `textures[].sampler` respectively) — structurally identical in difficulty to `🎥️camera`/`🦴️skin`,
  reusing the SAME generic `IndexChange`/`remap_index` arithmetic those kinds introduced, generalized
  to an arbitrary top-level container (`apply_ref_change_in`) or a nested one
  (`apply_primitive_material_ref_change`). `textures` is genuinely harder: FIVE reference sites per
  material (`pbrMetallicRoughness.{baseColorTexture,metallicRoughnessTexture}.index`,
  `normalTexture.index`, `occlusionTexture.index`, `emissiveTexture.index`), each wrapped in its own
  `Option<TextureInfo>` CLEARED ENTIRELY (not just the index field) when the referenced texture is
  deleted — `apply_texture_info_ref_change` (`../../../♾️any/🔮️oracle/🦀️.rs`) reimplements this
  cascading-clear shape independently.

  All four `create-*` payloads (`GltfCreate{Material,Texture,Image,Sampler}Payload { position }`)
  carry no field content — the same shape `create-skin`/`create-animation` already established — so
  every `delete-*`'s inverse is special-cased through a bespoke `undo_delete_*`
  (`../../../♾️any/🔮️oracle/🦀️.rs`, restoring the exact removed content AND every reference straight
  off the original document) rather than a second `create-*` call, mirroring `undo_delete_skin`/
  `undo_delete_animation` exactly.

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
      | id                           | params                                                |
      | create-material              | {"position":1}                                        |
      | delete-material              | {"index":1}                                           |
      | move-material                | {"index":0,"position":1}                               |
      | reorder-materials            | {"order":[1,0]}                                        |
      | create-texture               | {"position":1}                                         |
      | delete-texture               | {"index":2}                                            |
      | move-texture                 | {"index":2,"position":0}                                |
      | reorder-textures             | {"order":[2,1,0]}                                      |
      | create-image                 | {"position":1}                                         |
      | delete-image                 | {"index":2}                                            |
      | move-image                   | {"index":2,"position":0}                                |
      | reorder-images               | {"order":[2,1,0]}                                      |
      | create-sampler               | {"position":1}                                         |
      | delete-sampler               | {"index":2}                                            |
      | move-sampler                 | {"index":2,"position":0}                                |
      | reorder-samplers             | {"order":[2,1,0]}                                      |
      | change-material-alpha-mode   | {"material":1,"alphaMode":"BLEND"}                     |
      | change-material-double-sided | {"material":1,"doubleSided":true}                      |

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
      | id                           | params                                                |
      | create-material              | {"position":1}                                        |
      | delete-material              | {"index":1}                                           |
      | move-material                | {"index":0,"position":1}                               |
      | reorder-materials            | {"order":[1,0]}                                        |
      | create-texture               | {"position":1}                                         |
      | delete-texture               | {"index":2}                                            |
      | move-texture                 | {"index":2,"position":0}                                |
      | reorder-textures             | {"order":[2,1,0]}                                      |
      | create-image                 | {"position":1}                                         |
      | delete-image                 | {"index":2}                                            |
      | move-image                   | {"index":2,"position":0}                                |
      | reorder-images               | {"order":[2,1,0]}                                      |
      | create-sampler               | {"position":1}                                         |
      | delete-sampler               | {"index":2}                                            |
      | move-sampler                 | {"index":2,"position":0}                                |
      | reorder-samplers             | {"order":[2,1,0]}                                      |
      | change-material-alpha-mode   | {"material":1,"alphaMode":"BLEND"}                     |
      | change-material-double-sided | {"material":1,"doubleSided":true}                      |
