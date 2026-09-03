@capability-dag-1-mutate
@oracle-dag-1-python-independent
@comparison-ordered-json-v1
@mutations-dag-1-any
Feature: Apply every typed DAG mutation to the real committed pipeline, to its rejection vectors, and against an independent Python implementation
  `dag.dag` is a semio-NATIVE port-directed computation graph. Nothing third-party reads
  `.dag.dsl.semio`, and no graph format holds an opinion about an edge whose endpoints are named
  PORTS owned by two nodes. The second producer a differential comparison needs is therefore a second
  IMPLEMENTATION, and `🐍️component.py` beside this file is it: written in Python from this subset's
  own committed `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and each mutation's
  own payload schema, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `create`/`delete`/`rename`/`change`/`move`/`resize`/`replace`/`connect`/`disconnect`/`reorder` verb
  entries. It imports nothing from the Rust it judges and transliterates none of it. The no-oracle
  decision this replaces (`dag-1-port-directed-graph-mutation-semantics`) is narrowed to an empty
  `capabilities` list rather than deleted, because its own investigation remains the honest record of
  what was checked.

  ⚠️ Honest boundary. `DagSnapshot` persists neither nodes nor edges — one content-addressed child
  handle only — so no committed fixture carries a decodable graph, and the REAL committed pipeline
  this feature's own Rust adapter additionally exercises is reached only by parsing the real
  `.dsl.semio` example through PRODUCTION's own `parse_dag_dsl`, which this Python reference does not
  reimplement. What the Python side DOES cover, and cross-checks against a real committed fixture, is
  the REJECTION half every one of this vocabulary's fourteen kinds commits to: every `(before,
  mutation, outcome)` triad below is now a declared `asset://` fixture rather than an
  `include_str!`-only literal, for BOTH `@id-mutate` (its own committed rejection vector) and
  `@id-inverse` (the SAME rejection vector, restated — a rejection has nothing to invert, so
  `taxonomy.md`'s "Missing target ⇒ inverse returns `Vec::new()`" is what both implementations assert
  there). The real-pipeline halves of both scenarios (`the real application moves the handle`, `the
  document equals the pipeline again`) stay exactly as they were, asserted by the Rust subject alone.

  What distinguishes this subset from every node-and-edge vocabulary in the repository is that
  `DagSnapshot` PERSISTS NEITHER NODES NOR EDGES. It carries `schema` plus one composed
  `s.stdio.semio.graph` child handle, and that handle's `childId` is a content digest of the child.
  Two consequences run through everything below. An applied mutation mints a DIFFERENT handle and a
  refused one leaves the handle alone, so the persisted projection is a perfectly sharp
  observability surface — it moves if and only if the working scene moved. And a committed
  `➡️after` for an APPLIED mutation would have to carry a hand-forged
  `std::collections::hash_map::DefaultHasher` digest, a value the standard library explicitly
  refuses to specify, which is why all fourteen committed specification vectors under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/` are REJECTION vectors
  and say so in their own leaf tests.

  Each `mutate-<kind>` scenario therefore does two things the committed vectors alone cannot do
  between them. It replays that kind's committed vector and requires the exact `(code, severity,
  target)` triple the vector's `🎯️outcome/🔣️.json` declares — three different triples live
  in this vocabulary and they are not interchangeable: a missing node is the Error-level
  `mutation.target-missing`, a colliding id is the Fatal `mutation.duplicate-id`, and a duplicate
  entry in a reorder list is the Fatal `mutation.invariant`. Then it applies the real-effect payload
  in the `params` column to the real committed example document — a five-node signal pipeline,
  `slider → scale → combine → screen` with `mode` feeding `combine`'s second input over four edges —
  and requires the content handle to MOVE. A kind that reached nothing would pass the first half
  and fail the second.

  The second half is also what makes this a DAG rather than a graph. `connect-nodes` refuses a
  self-loop and refuses any edge whose target can already reach its source, so the `e5` row below
  deliberately connects `slider@out` to `combine@a` — a second path to a node already downstream,
  which is legal — rather than anything closing the cycle `screen → slider` would close. Endpoints
  are `node@port` strings this plugin splits itself; `rename-node` on `scale` is in the table
  precisely because it has to rewrite the `e1` target and the `e2` source as well as the node.

  `mutate-<kind>`/`inverse-<kind>` now dispatch BOTH an oracle role (the Python implementation,
  reached through this plugin's `oracleHostPackages` entry, checking the REJECTION half) and a
  subject role (this repository's own real dispatch, unaffected — the same real-pipeline assertions
  as before). A handler that merely ran the mutation and returned would report a pass having checked
  nothing.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed rejection vector and to the real pipeline
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed rejection vector for the <id> kind and the real committed example pipeline
    When <id> is replayed against its vector and then applied to the pipeline
      """
      {"kind": "<id>", "code": "<code>", "params": <params>}
      """
    Then the vector is refused with exactly <code>, its content handle is untouched, the real application moves the handle, and the two implementations agree on the rejection
    Examples:
      | id                        | dir                          | fixture                                                       | code                    | params                                                                                                                                                                                            |
      | create-node               | 🌱create-node                | rejects-a-duplicate-node-id                               | mutation.duplicate-id   | {"mutation": "createNode", "node": {"id": "gain", "name": "Gain", "abbreviation": "Gain", "icon": "", "x": -260.0, "y": 180.0, "width": 104.0, "height": 14.0, "properties": {}, "kind": "computation", "inputs": [], "outputs": [], "variadicInputs": false, "variadicOutputs": false}} |
      | delete-node               | 🗑️delete-node               | rejects-deleting-a-missing-node                           | mutation.target-missing | {"mutation": "deleteNode", "id": "mode"}                                                                                                                                                          |
      | rename-node               | 🏷️rename-node               | rejects-renaming-a-missing-node                           | mutation.target-missing | {"mutation": "renameNode", "id": "scale", "newId": "gain"}                                                                                                                                        |
      | change-node-name          | 🔤change-node-name           | rejects-renaming-the-label-of-a-missing-node              | mutation.target-missing | {"mutation": "changeNodeName", "id": "combine", "newName": "Merge"}                                                                                                                               |
      | move-node                 | ↔️move-node                 | rejects-moving-a-missing-node                             | mutation.target-missing | {"mutation": "moveNode", "id": "screen", "x": 520.0, "y": 40.0}                                                                                                                                   |
      | resize-node               | 📐resize-node                | rejects-resizing-a-missing-node                           | mutation.target-missing | {"mutation": "resizeNode", "id": "screen", "width": 240.0, "height": 160.0}                                                                                                                       |
      | change-node-icon          | 🖼️change-node-icon          | rejects-reiconing-a-missing-node                          | mutation.target-missing | {"mutation": "changeNodeIcon", "id": "slider", "newIcon": "emoji:spark"}                                                                                                                          |
      | change-node-abbreviation  | 🔡change-node-abbreviation   | rejects-reabbreviating-a-missing-node                     | mutation.target-missing | {"mutation": "changeNodeAbbreviation", "id": "slider", "newAbbreviation": "Amt"}                                                                                                                   |
      | change-node-operator-kind | 🧮change-node-operator-kind  | rejects-rebinding-the-operator-of-a-missing-node          | mutation.target-missing | {"mutation": "changeNodeOperatorKind", "id": "combine", "newOperatorKind": "sum"}                                                                                                                  |
      | replace-node-kind         | 🔁replace-node-kind          | rejects-rekinding-a-missing-node                          | mutation.target-missing | {"mutation": "replaceNodeKind", "id": "scale", "newKind": {"kind": "computation", "inputs": [], "outputs": [], "variadicInputs": true, "variadicOutputs": false}}                                  |
      | replace-node-properties   | 🗃️replace-node-properties   | rejects-repropertying-a-missing-node                      | mutation.target-missing | {"mutation": "replaceNodeProperties", "id": "scale", "newProperties": {"units": "mm"}}                                                                                                             |
      | reorder-nodes             | 🔀reorder-nodes              | rejects-a-duplicate-id-in-the-order                       | mutation.invariant      | {"mutation": "reorderNodes", "order": ["screen", "combine", "scale", "mode", "slider"]}                                                                                                            |
      | connect-nodes             | 🔗connect-nodes              | rejects-a-missing-source-node                             | mutation.target-missing | {"mutation": "connectNodes", "id": "e5", "source": "slider@out", "target": "combine@a", "routeStyle": "bezier", "properties": {}}                                                                   |
      | disconnect-nodes          | ✂️disconnect-nodes          | rejects-disconnecting-a-missing-edge                      | mutation.target-missing | {"mutation": "disconnectNodes", "id": "e3"}                                                                                                                                                       |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real committed pipeline
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the real committed example pipeline
    When <id> is applied to it and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "code": "<code>", "params": <params>}
      """
    Then the document equals the pipeline again, content handle included — which for a content-addressed child means the whole scene came back — and, on the same kind's own committed rejection vector, both implementations agree there is nothing to invert
    Examples:
      | id                        | dir                          | fixture                                                       | code                    | params                                                                                                                                                                                            |
      | create-node               | 🌱create-node                | rejects-a-duplicate-node-id                               | mutation.duplicate-id   | {"mutation": "createNode", "node": {"id": "gain", "name": "Gain", "abbreviation": "Gain", "icon": "", "x": -260.0, "y": 180.0, "width": 104.0, "height": 14.0, "properties": {}, "kind": "computation", "inputs": [], "outputs": [], "variadicInputs": false, "variadicOutputs": false}} |
      | delete-node               | 🗑️delete-node               | rejects-deleting-a-missing-node                           | mutation.target-missing | {"mutation": "deleteNode", "id": "mode"}                                                                                                                                                          |
      | rename-node               | 🏷️rename-node               | rejects-renaming-a-missing-node                           | mutation.target-missing | {"mutation": "renameNode", "id": "scale", "newId": "gain"}                                                                                                                                        |
      | change-node-name          | 🔤change-node-name           | rejects-renaming-the-label-of-a-missing-node              | mutation.target-missing | {"mutation": "changeNodeName", "id": "combine", "newName": "Merge"}                                                                                                                               |
      | move-node                 | ↔️move-node                 | rejects-moving-a-missing-node                             | mutation.target-missing | {"mutation": "moveNode", "id": "screen", "x": 520.0, "y": 40.0}                                                                                                                                   |
      | resize-node               | 📐resize-node                | rejects-resizing-a-missing-node                           | mutation.target-missing | {"mutation": "resizeNode", "id": "screen", "width": 240.0, "height": 160.0}                                                                                                                       |
      | change-node-icon          | 🖼️change-node-icon          | rejects-reiconing-a-missing-node                          | mutation.target-missing | {"mutation": "changeNodeIcon", "id": "slider", "newIcon": "emoji:spark"}                                                                                                                          |
      | change-node-abbreviation  | 🔡change-node-abbreviation   | rejects-reabbreviating-a-missing-node                     | mutation.target-missing | {"mutation": "changeNodeAbbreviation", "id": "slider", "newAbbreviation": "Amt"}                                                                                                                   |
      | change-node-operator-kind | 🧮change-node-operator-kind  | rejects-rebinding-the-operator-of-a-missing-node          | mutation.target-missing | {"mutation": "changeNodeOperatorKind", "id": "combine", "newOperatorKind": "sum"}                                                                                                                  |
      | replace-node-kind         | 🔁replace-node-kind          | rejects-rekinding-a-missing-node                          | mutation.target-missing | {"mutation": "replaceNodeKind", "id": "scale", "newKind": {"kind": "computation", "inputs": [], "outputs": [], "variadicInputs": true, "variadicOutputs": false}}                                  |
      | replace-node-properties   | 🗃️replace-node-properties   | rejects-repropertying-a-missing-node                      | mutation.target-missing | {"mutation": "replaceNodeProperties", "id": "scale", "newProperties": {"units": "mm"}}                                                                                                             |
      | reorder-nodes             | 🔀reorder-nodes              | rejects-a-duplicate-id-in-the-order                       | mutation.invariant      | {"mutation": "reorderNodes", "order": ["screen", "combine", "scale", "mode", "slider"]}                                                                                                            |
      | connect-nodes             | 🔗connect-nodes              | rejects-a-missing-source-node                             | mutation.target-missing | {"mutation": "connectNodes", "id": "e5", "source": "slider@out", "target": "combine@a", "routeStyle": "bezier", "properties": {}}                                                                   |
      | disconnect-nodes          | ✂️disconnect-nodes          | rejects-disconnecting-a-missing-edge                      | mutation.target-missing | {"mutation": "disconnectNodes", "id": "e3"}                                                                                                                                                       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed pipeline through its own DSL carrier and print it back
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed, printed back to `.dag.dsl.semio` and parsed again
    Then every decoding agrees on the same five-node pipeline over four edges, and the printed text reproduces the committed file byte for byte
