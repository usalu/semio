@capability-gltf-2-0-mutate
@oracle-three-gltf-2-0-mutate-reader
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-animation
Feature: Apply every registered glTF 2.0 animation mutation to a real-world document
  The `gltf-2-0-animation` catalog (`../../🧪️oracle/🔣️.json`) declares the 4 kinds
  `document/animations` owns (§5.5): `create-animation`, `delete-animation`, `move-animation`,
  `reorder-animations`. Shard A6 already scaffolded this catalog and its 4 committed
  `before.gltf`/`after.gltf` fixture pairs (`../../🧫️fixtures/<kind>-applied/`), each derived from
  the same `gltf-2-0-any-reader-oracle` base document the artifact-root `mutate-gltf-2-0` case uses —
  this case claims it. Every real leaf directory (`../../../✳️any/🧬️schema/🧬️mutations/
  {🌱️🎞️create-animation,🗑️🎞️delete-animation,🚚️🎞️move-animation,🔀️🎞️reorder-animations}/🦀️.rs`)
  stays physically owned by `✳️any`'s aggregate mutation root — `validate_mutation_leaf_source`
  requires a leaf's `owner` to be an immediate child of its aggregate root, so ownership here is
  declared through this catalog and the manifest's per-mutation `subset` override, never through
  moving the directory.

  Unlike `✳️camera`'s/`✳️skin`'s own kinds, `document/animations` is the ONE top-level family
  `../../../✳️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs`'s own `repair` has an
  EMPTY match arm for (`GltfTopLevelFamily::Animations => {}`) — no node scalar field, nor any other
  family, ever points AT an animation by index (only the reverse: `animations[i].channels[j].
  target.node` points at a node), so these 4 kinds need no `apply_node_ref_change` step at all.
  `create-animation`'s own payload carries no field content (same shape as `✳️skin`'s
  `create-skin`), so `delete-animation`'s inverse is special-cased through `undo_delete_animation`
  (`../../../✳️any/🧪️oracle/🦀️.rs`) rather than a second `create-animation` call, for the identical
  reason `✳️skin`'s own feature file documents for `undo_delete_skin`.

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
      | id                    | params                       |
      | create-animation      | {"position":1}               |
      | delete-animation      | {"index":0}                  |
      | move-animation        | {"index":1,"position":0}     |
      | reorder-animations    | {"order":[1,0]}              |

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
      | id                    | params                       |
      | create-animation      | {"position":1}               |
      | delete-animation      | {"index":0}                  |
      | move-animation        | {"index":1,"position":0}     |
      | reorder-animations    | {"order":[1,0]}              |
