@capability-gltf-2-0-mutate
@oracle-three-gltf-2-0-mutate-reader
@comparison-semantic-gltf-v1
@mutations-gltf-2-0-camera
Feature: Apply every registered glTF 2.0 camera mutation to a real-world document
  The `gltf-2-0-camera` catalog (`../../🔮️oracle/🔣️.json`) declares the 4 kinds `document/cameras`
  owns (§5.10): `create-camera`, `delete-camera`, `move-camera`, `reorder-cameras`. Shard A6 already
  scaffolded this catalog and its 4 committed `⬅️before.gltf`/`➡️after.gltf` fixture pairs
  (the exact `../../🧫️fixtures/<fixture>/` coordinates below), each derived from the same `gltf-2-0-any-reader-oracle` base
  document the artifact-root `🐞️mutate-gltf-2-0` case uses — this case claims it. Every real leaf
  directory (`../../../♾️any/🧬️schema/🧬️mutations/🎥️camera/
  {🌱️create,🗑️delete,🚚️move,🔀️reorder}/🦀️.rs`) stays physically owned by `♾️any`'s aggregate
  mutation root — the exact registered domain/operation owners are declared through this catalog and the manifest's
  per-mutation `subset` override, never through moving the directory.

  The independent oracle (`../../../♾️any/🔮️oracle/🦀️.rs`) is the SAME domain-blind `json`-crate
  GLB/JSON reader the artifact-root case measures its 7 kinds through, extended here with 4 more
  kinds reimplemented from scratch against the parsed tree: `create_camera`/`delete_camera`/
  `move_camera`/`reorder_cameras` re-derive the exact four-branch index-remap arithmetic
  `../../../♾️any/🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs`'s own `repair`/
  `family_ops!` apply to every `nodes/{i}/camera` reference (insert bumps refs at or past the new
  position, delete drops refs to the removed index and shifts refs past it down, move follows the
  same displacement a `Vec::remove`+`Vec::insert` produces, reorder maps each ref to its new position
  in the permutation) — read from that module's own doc comments and verified against the 4 committed
  fixture pairs' own before/after diffs, never by calling that module (which is the subject's own
  production code). `project_gltf` gained a `cameras`/`nodes[].camera` projection, generic and
  structural (`to_host_json`/`from_host_json`), so this comparison surface stays symmetric with how
  `create-camera`'s own `projection` param is carried into the parsed document.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://<fixture>/⬅️before.gltf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id               | fixture                       | params |
      | create-camera    | 🌱️create-camera-applied      | {"position":1,"projection":{"type":"perspective","perspective":{"yfov":1,"znear":0.1}}} |
      | delete-camera    | 🗑️delete-camera-applied      | {"index":0} |
      | move-camera      | 🚚️move-camera-applied        | {"index":1,"position":0} |
      | reorder-cameras  | 🔀️reorder-cameras-applied    | {"order":[1,0]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document shared://<fixture>/⬅️before.gltf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the document matches its pre-mutation semantic projection
    Examples:
      | id               | fixture                       | params |
      | create-camera    | 🌱️create-camera-applied      | {"position":1,"projection":{"type":"perspective","perspective":{"yfov":1,"znear":0.1}}} |
      | delete-camera    | 🗑️delete-camera-applied      | {"index":0} |
      | move-camera      | 🚚️move-camera-applied        | {"index":1,"position":0} |
      | reorder-cameras  | 🔀️reorder-cameras-applied    | {"order":[1,0]} |
