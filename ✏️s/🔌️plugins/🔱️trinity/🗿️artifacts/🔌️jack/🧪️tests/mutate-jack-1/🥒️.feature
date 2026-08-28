@capability-jack-1-mutate
@oracle-jack-python-independent
@comparison-ordered-json-v1
@mutations-jack-1-any
Feature: Apply every typed jack scene mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.trinity.jack` assembly scene, of its `.dsl.semio` carrier and of
  all eight typed mutations, written in Python from
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json` (the document — nodes with
  ports, edges over `node@port` endpoints, a manifest, a camera and a root node id), from
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (the eight verbs and their argument
  lists, including `entity = "node" ":" id / "edge" ":" id`) and from the eight committed
  specification vectors. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library. A jack scene is a labelled property
  graph whose edges address PORTS, not vertices, over a manifest that closes the kind vocabulary.
  `networkx`, `igraph` and `petgraph` model vertices and edges and have no notion of a port-addressed
  endpoint, of a kind manifest, or of the untyped property bag the last two verbs edit — and none of
  them reads `.dsl.semio`. What a reference can genuinely adjudicate is this vocabulary's own
  algebra, and that is what this one does.

  THE ARTIFACT IS REAL, AND IT IS THE REAL ONE. `identity-round-trip` and all sixteen real-document
  scenarios read the artifact's own committed example — the Nakagin Capsule Tower scene, nine nodes
  and six edges with their real UUIDs, real port ids, real transform properties and a real camera —
  through its own carrier, in BOTH languages. The Python parses the `.dsl.semio` file directly (every
  member is the hex of its UTF-8 bytes, with `camera`, `nodes` and `edges` the hex of their compact
  JSON) and re-encodes it byte for byte, which is what pins its reading of a layout no prose document
  describes.

  TWO DEFECTS AND ONE GAP, found while writing the reference and reported rather than worked around.
  First: `…/🧬️schema/🧬️mutations/🔣️component.json` does not describe the mutations at all — it is a
  verbatim copy of the snapshot schema with `title` changed to `JackMutation`. The wire form was
  therefore read off the committed vectors, which spell it internally tagged and inconsistently mix
  camelCase discriminators with snake_case arguments (`new_name`, `new_value`). Second: nothing in
  the specification says whether deleting a node also deletes the edges that name it; the reference
  cascades, on the grounds that `create-edge`'s own committed vector is rejected with
  `mutation.invariant` when its endpoints are absent, so a delete that left a dangling edge would
  produce a document the format refuses to construct — and the `delete-node` row below nevertheless
  addresses `jack_orphan`, which no edge names, so the comparison never rests on that inference.
  Third, and the largest: ALL EIGHT committed vectors are NEGATIVE — three rejections and five
  accepted no-ops — so before this conversion the accepting direction of this entire vocabulary had
  no committed evidence at all. The sixteen real-document scenarios below are the first.

  A LIMIT OF THIS VOCABULARY, found by the second implementation and recorded rather than hidden.
  `create-node` carries a whole `Node` and NO index — the grammar writes
  `create-node id text text number number number number port-table` — so a created node can only ever
  land at the end of the list. The inverse of `delete-node` is therefore exact only for a TRAILING
  node, and undoing the deletion of any other one puts it back in the wrong place. Both
  implementations share that limit, so a differential alone would report a comfortable green over a
  violated law; it is caught here because both sides assert the restoring law IN ROLE, index for
  index. The two tables below therefore differ on purpose and neither is softened: `mutate-` deletes
  `jack_orphan`, which sits in the MIDDLE of the nine, and `inverse-` deletes `jack_spare`, the last
  of them — and both are nodes no edge names, so neither rests on the cascade inference above.

  The committed vectors were KEPT, not replaced. `spec-vector-<kind>` replays each one through both
  implementations, and its `verdict` column states which refusal that vector commits to: `refused`
  must be refused outright with the scene left alone, and `noop` must be ACCEPTED while leaving the
  scene exactly where it was — a distinction a handler that simply compared before with after could
  not make, since both vectors commit the same before- and after-scene.

  A FOURTH FINDING, and why one scenario below is RED. The committed vectors are not self-contained.
  Their before-scene carries a composed `content` child instead of inline `nodes` and `edges`, so the
  entities every one of them addresses are ABSENT from the snapshot that is actually committed. Seven
  of the eight still replay, because what their outcomes really specify turns out to be that the four
  in-place verbs answer an absent target with an accepted `mutation.no-op` while the two `delete-`
  verbs reject one — and the reference implements exactly that, on their authority. The eighth cannot:
  `rejects-a-node-id-the-scene-already-holds` commits a before-scene holding NO nodes at all, so the
  id it calls a duplicate is not there and no implementation reading only what is committed can
  refuse it. `spec-vector-create-node` is therefore left RED. Nothing was softened to hide it: the
  fixture is the thing that needs fixing, by committing the scene the vector claims to start from.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real Nakagin Capsule Tower scene
    Given the real committed scene asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same scene
    Examples:
      | id                    | mutation                                                                                                                                                                                                                                                                                                             |
      | create-node           | {"mutation":"createNode","node":{"id":"jack-annex-0001","kind":"Piece","name":"jack_annex","x":300.0,"y":-40.0,"width":88.0,"height":40.0,"properties":{"label":"jack-annex","tier":9.0},"ports":[{"id":"annex-in-0001","kind":"Connector","direction":"in","properties":{}}]}}                                        |
      | delete-node           | {"mutation":"deleteNode","id":"a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6"}                                                                                                                                                                                                                                                |
      | create-edge           | {"mutation":"createEdge","edge":{"id":"e-jack-spare-orphan","kind":"Connection","source":"caf5e4d3-6d7e-8f90-a1b2-c3d4e5f6a7b8@f6a7b8c9-d0e1-2345-f678-90abcdef0123","target":"a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6@d4e5f6a7-b8c9-0123-def4-567890abcdef","properties":{"gap":0.0,"rise":0.0,"rotation":90.0,"shift":0.0,"tilt":0.0,"turn":0.0,"u":0.6,"v":-0.6}}} |
      | delete-edge           | {"mutation":"deleteEdge","id":"e-jack-prune"}                                                                                                                                                                                                                                                                        |
      | rename-node           | {"mutation":"renameNode","id":"b9f4e3d2-5c6d-7e8f-90a1-b2c3d4e5f6a7","new_name":"jack_pruned_capsule"}                                                                                                                                                                                                               |
      | move-node             | {"mutation":"moveNode","id":"5f0266bc-856b-4ef2-9eb0-16ef5e1fb952","x":-260.5,"y":120.25}                                                                                                                                                                                                                            |
      | change-data-property  | {"mutation":"changeDataProperty","entity":{"entity":"node","id":"7dc5b737-3b6b-4068-b315-b7bacc91c2e1"},"key":"tier","new_value":3.5}                                                                                                                                                                                |
      | remove-data-property  | {"mutation":"removeDataProperty","entity":{"entity":"edge","id":"e-shaft-1"},"key":"tilt"}                                                                                                                                                                                                                           |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real tower scene and land back on it
    Given the real committed scene asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated scene AND on the restored one, member for member and index for index
    Examples:
      | id                    | mutation                                                                                                                                                                                                                                                                                                             |
      | create-node           | {"mutation":"createNode","node":{"id":"jack-annex-0001","kind":"Piece","name":"jack_annex","x":300.0,"y":-40.0,"width":88.0,"height":40.0,"properties":{"label":"jack-annex","tier":9.0},"ports":[{"id":"annex-in-0001","kind":"Connector","direction":"in","properties":{}}]}}                                        |
      | delete-node           | {"mutation":"deleteNode","id":"caf5e4d3-6d7e-8f90-a1b2-c3d4e5f6a7b8"}                                                                                                                                                                                                                                                |
      | create-edge           | {"mutation":"createEdge","edge":{"id":"e-jack-spare-orphan","kind":"Connection","source":"caf5e4d3-6d7e-8f90-a1b2-c3d4e5f6a7b8@f6a7b8c9-d0e1-2345-f678-90abcdef0123","target":"a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6@d4e5f6a7-b8c9-0123-def4-567890abcdef","properties":{"gap":0.0,"rise":0.0,"rotation":90.0,"shift":0.0,"tilt":0.0,"turn":0.0,"u":0.6,"v":-0.6}}} |
      | delete-edge           | {"mutation":"deleteEdge","id":"e-jack-prune"}                                                                                                                                                                                                                                                                        |
      | rename-node           | {"mutation":"renameNode","id":"b9f4e3d2-5c6d-7e8f-90a1-b2c3d4e5f6a7","new_name":"jack_pruned_capsule"}                                                                                                                                                                                                               |
      | move-node             | {"mutation":"moveNode","id":"5f0266bc-856b-4ef2-9eb0-16ef5e1fb952","x":-260.5,"y":120.25}                                                                                                                                                                                                                            |
      | change-data-property  | {"mutation":"changeDataProperty","entity":{"entity":"node","id":"7dc5b737-3b6b-4068-b315-b7bacc91c2e1"},"key":"tier","new_value":3.5}                                                                                                                                                                                |
      | remove-data-property  | {"mutation":"removeDataProperty","entity":{"entity":"edge","id":"e-shaft-1"},"key":"tilt"}                                                                                                                                                                                                                           |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-scene asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json
    And the committed after-scene asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️component.json
    When the committed mutation is applied to the committed before-scene
      """
      {"verdict": "<verdict>"}
      """
    Then each implementation gives the committed <verdict> answer in role, and the two agree
    Examples:
      | id                    | verdict | dir                     | fixture                                             |
      | create-node           | refused | 🌱️create-node           | rejects-a-node-id-the-scene-already-holds           |
      | delete-node           | refused | 🗑️delete-node           | rejects-deleting-a-node-the-scene-never-had         |
      | create-edge           | refused | 🔗️create-edge           | rejects-an-edge-whose-endpoints-are-absent          |
      | delete-edge           | refused | ✂️delete-edge           | rejects-cutting-an-edge-the-scene-never-had         |
      | rename-node           | noop    | ✏️rename-node           | keeps-the-name-a-node-already-carries               |
      | move-node             | noop    | 📍️move-node             | keeps-a-node-at-the-point-it-already-occupies       |
      | change-data-property  | noop    | 🔧️change-data-property  | keeps-a-node-property-at-the-value-it-already-holds |
      | remove-data-property  | noop    | 🧹️remove-data-property  | keeps-an-edge-without-the-property-it-never-had     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed Nakagin tower scene in both languages and agree on it
    Given the real committed scene asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When each implementation parses it, prints it back through its own carrier and parses it again
    Then both languages read the same nine nodes and six edges out of the same real bytes, the Python reproduces the file byte for byte, and the Rust holds its own canonical printing to ArtifactDsl's fixpoint law
