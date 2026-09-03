@capability-gltf-2-0-mutate
@oracle-three-gltf-2-0-mutate-reader
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-asset
Feature: Apply every registered glTF 2.0 asset/document mutation to a real-world document
  The `gltf-2-0-asset` catalog (`../../🧪️oracle/🔣️.json`) declares the 14 kinds `document/asset`,
  `document/extensionsUsed`, `document/extensionsRequired`, `document/extensions` and
  `document/extras` own (§2.7/§3.9): `add-required-extension`, `add-used-extension`,
  `change-asset-descriptive-metadata`, `change-asset-extension-data`, `change-asset-extra-data`,
  `change-asset-version`, `change-document-extension-data`, `change-document-extra-data`,
  `move-required-extension`, `move-used-extension`, `remove-required-extension`,
  `remove-used-extension`, `reorder-required-extensions`, `reorder-used-extensions`. Shard A6
  already scaffolded this catalog and its 14 committed `before.gltf`/`after.gltf` fixture pairs
  (`../../🧫️fixtures/<kind>-applied/`), each derived from the same `gltf-2-0-any-reader-oracle` base
  document the artifact-root `mutate-gltf-2-0` case and the `✳️camera`/`✳️skin`/`✳️animation` cases
  (shard F4) use — this case claims it. Every real leaf directory
  (`../../../✳️any/🧬️schema/🧬️mutations/{✅️🧩️add-required-extension,…}/🦀️.rs`) stays physically
  owned by `✳️any`'s aggregate mutation root — `validate_mutation_leaf_source` requires a leaf's
  `owner` to be an immediate child of its aggregate root, so ownership here is declared through this
  catalog and the manifest's per-mutation `subset` override, never through moving the directory.

  Simpler than `✳️camera`/`✳️skin`: `document/extensionsUsed`/`extensionsRequired` are plain string
  arrays with NO cross-reference from anywhere else in the document (unlike `nodes[].camera`/
  `nodes[].skin`), and `document/asset`/`document/extensions`/`document/extras` are plain scalar/
  object members — none of these 14 kinds needs `apply_node_ref_change`'s index-remap arithmetic at
  all, confirmed by reading `../../../✳️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/
  🦀️.rs`'s own match arms before writing anything (no `GltfTopLevelFamily` variant exists for either
  array).

  The independent oracle (`../../../✳️any/🧪️oracle/🦀️.rs`) is the SAME domain-blind `json`-crate
  GLB/JSON reader `✳️camera`/`✳️skin`/`✳️animation` already extended, extended again here with 14
  more kinds reimplemented from scratch against the parsed tree: `add_extension`/`remove_extension`/
  `move_extension`/`reorder_extensions` operate generically on either string array by KEY name
  (`extensionsUsed`/`extensionsRequired`), and `change_asset_descriptive_metadata`/
  `change_asset_version`/`change_asset_extension_data`/`change_asset_extra_data`/
  `change_document_extension_data`/`change_document_extra_data` read/write `document/asset`'s own
  object and the document-root `extensions`/`extras` members directly — plain field gets/sets, no
  index arithmetic. An optional member going from `Some` to `None` REMOVES the key entirely
  (`without_key`, built from `.iter()`/`Object::insert` alone), matching this subset's own
  `skip_serializing_if = "Option::is_none"` — confirmed by grepping a committed fixture for the
  literal `"extensions"` key substring (zero hits in `before.gltf`) before writing the removal
  primitive. `project_gltf` gained an `asset`/`extensionsUsed`/`extensionsRequired`/
  `documentExtensions`/`documentExtras` projection, generic and structural (`to_host_json`), the same
  bridge `create-camera`'s own `projection` param and `✳️camera`/`✳️skin`'s own array projections
  already use.

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
      | id                                | params                                                                                    |
      | add-required-extension            | {"extension":"KHR_materials_unlit","position":0}                                         |
      | add-used-extension                | {"extension":"ACME_marker","position":1}                                                 |
      | remove-used-extension             | {"extension":"KHR_materials_unlit"}                                                      |
      | remove-required-extension         | {"extension":"KHR_materials_unlit"}                                                      |
      | move-used-extension               | {"extension":"ACME_marker","position":0}                                                 |
      | move-required-extension           | {"extension":"ACME_marker","position":0}                                                 |
      | reorder-used-extensions           | {"order":["ACME_marker","KHR_materials_unlit"]}                                          |
      | reorder-required-extensions       | {"order":["ACME_marker","KHR_materials_unlit"]}                                          |
      | change-asset-descriptive-metadata | {"generator":"three.js GLTFExporter (mutated by semio fixture)","copyright":"2026 Ueli Saluz — mutated","minVersion":"2.0"} |
      | change-asset-version              | {"version":"2.0.1"}                                                                       |
      | change-asset-extension-data       | {"data":{"ACME_marker":{"on":true}}}                                                      |
      | change-asset-extra-data           | {"data":{"fixtureBase":"gltf-2-0-any-reader-oracle","revision":2}}                        |
      | change-document-extension-data    | {"data":{"ACME_marker":{"on":true}}}                                                      |
      | change-document-extra-data        | {"data":{"documentPurpose":"mutated by semio fixture"}}                                   |

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
      | id                                | params                                                                                    |
      | add-required-extension            | {"extension":"KHR_materials_unlit","position":0}                                         |
      | add-used-extension                | {"extension":"ACME_marker","position":1}                                                 |
      | remove-used-extension             | {"extension":"KHR_materials_unlit"}                                                      |
      | remove-required-extension         | {"extension":"KHR_materials_unlit"}                                                      |
      | move-used-extension               | {"extension":"ACME_marker","position":0}                                                 |
      | move-required-extension           | {"extension":"ACME_marker","position":0}                                                 |
      | reorder-used-extensions           | {"order":["ACME_marker","KHR_materials_unlit"]}                                          |
      | reorder-required-extensions       | {"order":["ACME_marker","KHR_materials_unlit"]}                                          |
      | change-asset-descriptive-metadata | {"generator":"three.js GLTFExporter (mutated by semio fixture)","copyright":"2026 Ueli Saluz — mutated","minVersion":"2.0"} |
      | change-asset-version              | {"version":"2.0.1"}                                                                       |
      | change-asset-extension-data       | {"data":{"ACME_marker":{"on":true}}}                                                      |
      | change-asset-extra-data           | {"data":{"fixtureBase":"gltf-2-0-any-reader-oracle","revision":2}}                        |
      | change-document-extension-data    | {"data":{"ACME_marker":{"on":true}}}                                                      |
      | change-document-extra-data        | {"data":{"documentPurpose":"mutated by semio fixture"}}                                   |
