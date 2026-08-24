@capability-jack-1-mutate
@no-oracle-jack-1-assembly-scene-mutation-semantics
@comparison-ordered-json-v1
@mutations-jack-1-any
Feature: Apply every typed assembly-scene mutation to its committed vector and to the Nakagin tower
  `trinity.graph` is a semio-NATIVE assembly scene — pieces joined at named connector ports.
  Nothing third-party reads `.jack.dsl.semio`, so there is no reference implementation to register
  (recorded as the `jack-1-assembly-scene-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`).

  Two facts shape everything below. First, `JackSnapshot` PERSISTS NEITHER PIECES NOR CONNECTIONS:
  it carries a manifest, a camera, a root node id and one composed `s.stdio.semio.graph` child
  handle whose `childId` is a content digest of the child. So the persisted projection moves if and
  only if the scene moved — an exact observability surface, and the reason a committed `➡️after` for
  an APPLIED mutation cannot be hand-authored. Second, all eight committed specification vectors
  under `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/` leave the
  document byte-identical, and they do so for FOUR different reasons: a missing node or edge is the
  Error-level `mutation.target-missing`, an id the scene already holds is the Fatal
  `mutation.duplicate-id`, an edge naming an absent endpoint is the Fatal `mutation.invariant` — an
  assembly claim, not a lookup failure — and four kinds instead degrade to a `Warning`-level
  `mutation.no-op` when the piece already carries the name, the point, the property value, or does
  not carry the property at all. The `code` and `level` columns carry that distinction per kind and
  the handler requires the exact pair, because the document alone cannot tell the four apart.

  Each `mutate-<kind>` scenario then applies the real-effect payload in `params` to the real
  committed artifact — the Nakagin Capsule Tower: a service core, five stacked capsule pieces, three
  unattached jacks and six connections — and requires the content handle to MOVE. The payloads are
  addressed at that document's own uuids on purpose: `delete-node` removes `jack_orphan`, the one
  piece no connection touches, so the delete is not also an edge cascade; `create-edge` joins
  `jack_spare`'s out connector to `jack_orphan`'s in connector, the only pair of free ports in the
  tower; and `delete-edge` cuts `e-jack-prune`, the one connection whose removal leaves both
  endpoints standing.

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler. A handler that merely ran the mutation and returned
  would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed vector and then to the Nakagin tower
    Given the committed specification vector for the <id> kind and the real committed Nakagin tower
    When <id> is replayed against its vector and then applied to the tower
      """
      {"kind": "<id>", "code": "<code>", "level": "<level>", "params": <params>}
      """
    Then the vector reports exactly <code> at <level> with the content handle untouched, and the real application moves the handle
    Examples:
      | id                   | code                    | level   | params |
      | create-node          | mutation.duplicate-id   | Fatal   | {"mutation": "createNode", "node": {"id": "capsule-new", "kind": "Piece", "name": "capsule_new", "x": 300.0, "y": 420.0, "width": 88.0, "height": 40.0, "properties": {"label": "capsule-new", "tier": 0.0}, "ports": [{"id": "port-new-in", "kind": "Connector", "direction": "in", "properties": {}}]}} |
      | delete-node          | mutation.target-missing | Error   | {"mutation": "deleteNode", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6"} |
      | create-edge          | mutation.invariant      | Fatal   | {"mutation": "createEdge", "edge": {"id": "e-spare-to-orphan", "kind": "Connection", "source": "caf5e4d3-6d7e-8f90-a1b2-c3d4e5f6a7b8@f6a7b8c9-d0e1-2345-f678-90abcdef0123", "target": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6@d4e5f6a7-b8c9-0123-def4-567890abcdef", "properties": {}}} |
      | delete-edge          | mutation.target-missing | Error   | {"mutation": "deleteEdge", "id": "e-jack-prune"} |
      | rename-node          | mutation.no-op          | Warning | {"mutation": "renameNode", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6", "new_name": "jack_orphan_renamed"} |
      | move-node            | mutation.no-op          | Warning | {"mutation": "moveNode", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6", "x": 360.0, "y": 260.0} |
      | change-data-property | mutation.no-op          | Warning | {"mutation": "changeDataProperty", "entity": {"entity": "node", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6"}, "key": "label", "new_value": "orphan-relabelled"} |
      | remove-data-property | mutation.no-op          | Warning | {"mutation": "removeDataProperty", "entity": {"entity": "node", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6"}, "key": "label"} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the Nakagin tower
    Given the real committed Nakagin tower
    When the real <id> payload is applied to it and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "code": "<code>", "level": "<level>", "params": <params>}
      """
    Then the document equals the tower again, content handle included — which for a content-addressed child means the whole scene came back
    Examples:
      | id                   | code                    | level   | params |
      | create-node          | mutation.duplicate-id   | Fatal   | {"mutation": "createNode", "node": {"id": "capsule-new", "kind": "Piece", "name": "capsule_new", "x": 300.0, "y": 420.0, "width": 88.0, "height": 40.0, "properties": {"label": "capsule-new", "tier": 0.0}, "ports": [{"id": "port-new-in", "kind": "Connector", "direction": "in", "properties": {}}]}} |
      | delete-node          | mutation.target-missing | Error   | {"mutation": "deleteNode", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6"} |
      | create-edge          | mutation.invariant      | Fatal   | {"mutation": "createEdge", "edge": {"id": "e-spare-to-orphan", "kind": "Connection", "source": "caf5e4d3-6d7e-8f90-a1b2-c3d4e5f6a7b8@f6a7b8c9-d0e1-2345-f678-90abcdef0123", "target": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6@d4e5f6a7-b8c9-0123-def4-567890abcdef", "properties": {}}} |
      | delete-edge          | mutation.target-missing | Error   | {"mutation": "deleteEdge", "id": "e-jack-prune"} |
      | rename-node          | mutation.no-op          | Warning | {"mutation": "renameNode", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6", "new_name": "jack_orphan_renamed"} |
      | move-node            | mutation.no-op          | Warning | {"mutation": "moveNode", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6", "x": 360.0, "y": 260.0} |
      | change-data-property | mutation.no-op          | Warning | {"mutation": "changeDataProperty", "entity": {"entity": "node", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6"}, "key": "label", "new_value": "orphan-relabelled"} |
      | remove-data-property | mutation.no-op          | Warning | {"mutation": "removeDataProperty", "entity": {"entity": "node", "id": "a8f3e2d1-4b5c-6d7e-8f90-a1b2c3d4e5f6"}, "key": "label"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed Nakagin tower through its own DSL carrier and print it back
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed, printed back to `.jack.dsl.semio` and parsed again
    Then every decoding agrees on the same tower — nine pieces over six connections, rooted at the service core — and the printed text reproduces the committed file byte for byte
