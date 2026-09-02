@capability-dag-1-mutate
@no-oracle-dag-1-port-directed-graph-mutation-semantics
@comparison-ordered-json-v1
@mutations-dag-1-any
Feature: Apply every typed DAG mutation to the real committed pipeline and to its rejection vectors
  `dag.dag` is a semio-NATIVE port-directed computation graph. Nothing third-party reads
  `.dag.dsl.semio`, and no graph format holds an opinion about an edge whose endpoints are named
  PORTS owned by two nodes, so no reference library is registered — recorded as the
  `dag-1-port-directed-graph-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-puzzle-2d-1` and `mutate-puzzle-3d-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, the rules of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` and the
  committed vectors this feature already replays. What blocks it TODAY is stated in the decision and
  is one edit: this case's vectors are not declared as `asset://` fixtures — the `Examples` table
  carries the payloads inline and the adapter reads the committed files through `include_str!` — so
  the plan pins none of their digests and a Python reference cannot read them at all. Separately, `identity-round-trip` would still be refused: this subset's committed
  snapshot text grammar is the repository-wide placeholder `payload = OCTET+`, whose header production
  declares `"schema" SP "stdio.json"` against an artifact whose own first line says otherwise.
  Until that is done, every assertion below still lives in the SUBJECT role, and the ceiling is the
  one this decision has always had: no second producer runs beside it, so a mistake shared by the
  handcrafted vector and the production code passes unseen.

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

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler. A handler that merely ran the mutation and returned
  would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed rejection vector and to the real pipeline
    Given the committed rejection vector for the <id> kind and the real committed example pipeline
    When <id> is replayed against its vector and then applied to the pipeline
      """
      {"kind": "<id>", "code": "<code>", "params": <params>}
      """
    Then the vector is refused with exactly <code>, its content handle is untouched, and the real application moves the handle
    Examples:
      | id                        | code                    | params                                                                                                                                                                                            |
      | create-node               | mutation.duplicate-id   | {"mutation": "createNode", "node": {"id": "gain", "name": "Gain", "abbreviation": "Gain", "icon": "", "x": -260.0, "y": 180.0, "width": 104.0, "height": 14.0, "properties": {}, "kind": "computation", "inputs": [], "outputs": [], "variadicInputs": false, "variadicOutputs": false}} |
      | delete-node               | mutation.target-missing | {"mutation": "deleteNode", "id": "mode"}                                                                                                                                                          |
      | rename-node               | mutation.target-missing | {"mutation": "renameNode", "id": "scale", "newId": "gain"}                                                                                                                                        |
      | change-node-name          | mutation.target-missing | {"mutation": "changeNodeName", "id": "combine", "newName": "Merge"}                                                                                                                               |
      | move-node                 | mutation.target-missing | {"mutation": "moveNode", "id": "screen", "x": 520.0, "y": 40.0}                                                                                                                                   |
      | resize-node               | mutation.target-missing | {"mutation": "resizeNode", "id": "screen", "width": 240.0, "height": 160.0}                                                                                                                       |
      | change-node-icon          | mutation.target-missing | {"mutation": "changeNodeIcon", "id": "slider", "newIcon": "emoji:spark"}                                                                                                                          |
      | change-node-abbreviation  | mutation.target-missing | {"mutation": "changeNodeAbbreviation", "id": "slider", "newAbbreviation": "Amt"}                                                                                                                   |
      | change-node-operator-kind | mutation.target-missing | {"mutation": "changeNodeOperatorKind", "id": "combine", "newOperatorKind": "sum"}                                                                                                                  |
      | replace-node-kind         | mutation.target-missing | {"mutation": "replaceNodeKind", "id": "scale", "newKind": {"kind": "computation", "inputs": [], "outputs": [], "variadicInputs": true, "variadicOutputs": false}}                                  |
      | replace-node-properties   | mutation.target-missing | {"mutation": "replaceNodeProperties", "id": "scale", "newProperties": {"units": "mm"}}                                                                                                             |
      | reorder-nodes             | mutation.invariant      | {"mutation": "reorderNodes", "order": ["screen", "combine", "scale", "mode", "slider"]}                                                                                                            |
      | connect-nodes             | mutation.target-missing | {"mutation": "connectNodes", "id": "e5", "source": "slider@out", "target": "combine@a", "routeStyle": "bezier", "properties": {}}                                                                   |
      | disconnect-nodes          | mutation.target-missing | {"mutation": "disconnectNodes", "id": "e3"}                                                                                                                                                       |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real committed pipeline
    Given the real committed example pipeline
    When <id> is applied to it and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "code": "<code>", "params": <params>}
      """
    Then the document equals the pipeline again, content handle included — which for a content-addressed child means the whole scene came back
    Examples:
      | id                        | code                    | params                                                                                                                                                                                            |
      | create-node               | mutation.duplicate-id   | {"mutation": "createNode", "node": {"id": "gain", "name": "Gain", "abbreviation": "Gain", "icon": "", "x": -260.0, "y": 180.0, "width": 104.0, "height": 14.0, "properties": {}, "kind": "computation", "inputs": [], "outputs": [], "variadicInputs": false, "variadicOutputs": false}} |
      | delete-node               | mutation.target-missing | {"mutation": "deleteNode", "id": "mode"}                                                                                                                                                          |
      | rename-node               | mutation.target-missing | {"mutation": "renameNode", "id": "scale", "newId": "gain"}                                                                                                                                        |
      | change-node-name          | mutation.target-missing | {"mutation": "changeNodeName", "id": "combine", "newName": "Merge"}                                                                                                                               |
      | move-node                 | mutation.target-missing | {"mutation": "moveNode", "id": "screen", "x": 520.0, "y": 40.0}                                                                                                                                   |
      | resize-node               | mutation.target-missing | {"mutation": "resizeNode", "id": "screen", "width": 240.0, "height": 160.0}                                                                                                                       |
      | change-node-icon          | mutation.target-missing | {"mutation": "changeNodeIcon", "id": "slider", "newIcon": "emoji:spark"}                                                                                                                          |
      | change-node-abbreviation  | mutation.target-missing | {"mutation": "changeNodeAbbreviation", "id": "slider", "newAbbreviation": "Amt"}                                                                                                                   |
      | change-node-operator-kind | mutation.target-missing | {"mutation": "changeNodeOperatorKind", "id": "combine", "newOperatorKind": "sum"}                                                                                                                  |
      | replace-node-kind         | mutation.target-missing | {"mutation": "replaceNodeKind", "id": "scale", "newKind": {"kind": "computation", "inputs": [], "outputs": [], "variadicInputs": true, "variadicOutputs": false}}                                  |
      | replace-node-properties   | mutation.target-missing | {"mutation": "replaceNodeProperties", "id": "scale", "newProperties": {"units": "mm"}}                                                                                                             |
      | reorder-nodes             | mutation.invariant      | {"mutation": "reorderNodes", "order": ["screen", "combine", "scale", "mode", "slider"]}                                                                                                            |
      | connect-nodes             | mutation.target-missing | {"mutation": "connectNodes", "id": "e5", "source": "slider@out", "target": "combine@a", "routeStyle": "bezier", "properties": {}}                                                                   |
      | disconnect-nodes          | mutation.target-missing | {"mutation": "disconnectNodes", "id": "e3"}                                                                                                                                                       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed pipeline through its own DSL carrier and print it back
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed, printed back to `.dag.dsl.semio` and parsed again
    Then every decoding agrees on the same five-node pipeline over four edges, and the printed text reproduces the committed file byte for byte
