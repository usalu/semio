@capability-semio-v1-graph-mutate
@oracle-semio-graph-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-graph
Feature: Apply every typed semio GRAPH mutation to the Nakagin Capsule Tower's port graph, against an independent Python implementation
  `s.stdio.semio.graph` is a semio-NATIVE format: no third party reads or writes `.dsl.semio` or
  `.pack.semio`, so the second producer a differential comparison needs is a second IMPLEMENTATION.
  `🐍️component.py` beside this file is that implementation — the carrier, the DSL grammar, the pack
  frame, the recursive `SemioValue` codec and all eleven verbs, written in Python from the committed
  specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/🔣️component.json`,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`, `✳️value`'s own snapshot schema for the
  `SemioValue` member names, and the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing from and
  transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-graph-python-independent` in `…/✳️graph/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  🏗️ **The graph under test is a real building's wiring.** The richest `s.stdio.semio.graph` document
  committed anywhere in this artifact is the two-node wires graph — 297 bytes, which is a fixture,
  not a graph. So the document every mutation row below runs on was derived ONCE — by
  `🐍️derive-graph-fixture.py` in the ticket folder — from the real committed IFC 4 file
  `../../../🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc`, Kisho Kurokawa's Nakagin Capsule Tower,
  2.5 MB and 24 792 entities, read with **IfcOpenShell 0.8.4**, a genuine third-party IFC
  implementation and the only reader involved in the provenance. Its `IfcElementAssembly` and 180
  `IfcBuildingElementProxy` capsules became the 181 nodes — each carrying its real `GlobalId` as id,
  its real entity type as `kind`, its real `Name` as `label` and its real placement translation in
  millimetres as `position`; its 364 `IfcDistributionPort`s became the 364 ports; its 179
  `IfcRelConnectsPorts` became the 179 edges; and 366 of its `IfcPropertySingleValue`s became the
  typed properties, `str` and `float` alike. The result is 131 964 bytes of DSL and 67 124 bytes of
  pack, against 297 and 183 for the wires graph the case used to rest on. IfcOpenShell reads IFC, not
  a semio envelope, and cannot express a single one of the eleven verbs, which is why it is the
  source of the ARTIFACT and never the oracle.

  🔌️ **All three port directions come out of the real connection graph, not out of a default.** The
  source declares `FlowDirection` as `NOTDEFINED` on every port, so the direction is read off what
  the model actually wires: a port the file uses as an `IfcRelConnectsPorts` `RelatingPort` is an
  `out` (179 of them), one used as a `RelatedPort` is an `in` (179), and the six the file connects in
  neither direction are `inOut`. That is derived from real data rather than assigned, and it is why
  the artifact itself exercises all three arms of `port-kind` where the wires graph exercised two.

  ⚖️ **A property of the vocabulary the parameters have to respect.** `create-node` and `create-edge`
  carry no index and APPEND, so the undo of a removal can only put the record back at the END of its
  collection. `delete-edge` therefore addresses `3NLh69tTrEpfV9iDbwoXYL`, the last edge, and
  `delete-node` addresses `1tZkmTaMP4R8yLkBdfebfl`, the last node — which the real model wires with
  exactly one connection, and that connection is the last edge, so the real cascade this deletion
  performs is also the one its inverse can restore in order. That coincidence is a fact about the
  source file, checked rather than assumed; this feature asserts ordered equality rather than
  weakening the comparison to a multiset.

  The remaining parameters are chosen against the tower's own shape, so a plausible wrong codec
  fails: `create-node` appends a node carrying all THREE port kinds at once and a `float` property;
  `change-node-kind` and `change-node-label` each retag one field of the real `IfcElementAssembly`
  root and must leave the other alone; `move-node` lifts the capsule `0POPlhUSnC1REPvcqnensi` from
  the origin to a real capsule position in millimetres; `add-node-port` inserts AHEAD of that node's
  two real `out` ports so an implementation that merely appended fails; `add-node-property` inserts a
  genuinely NESTED value — a list holding an int and a map — so a value codec that only handles
  scalars fails; `remove-node-port` detaches the real `in` port of the last node and
  `remove-node-property` the real `ComposePieceAttributes.name` of the capsule; and `create-edge`
  wires the last node back to that capsule.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each kind, now applied by BOTH
  implementations and checked against the committed after-snapshot by each of them in role. Nothing
  was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions, over FOUR
  files. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an
  exact re-emission is the CORRECT answer here and the wave's must-differ tripwire would be
  backwards, which is why the Rust side asserts `law::carrier_is_exact`. The wires graph's two
  encodings were written by the RUST codec and the Python side reproduces them byte for byte from the
  grammar alone — it is kept for exactly that reason, and nothing it proved was given up — while the
  capsule tower's two encodings were written by the PYTHON implementation and the Rust codec has to
  reproduce THOSE, 364 ports and 366 typed properties among them.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real Nakagin Capsule Tower port graph
    Given the real capsule tower graph local://🗣️nakagin-capsule-tower.dsl.semio
    When the <id> mutation is applied to the graph parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id                   | mutation |
      | create-node          | {"prepare":[],"mutation":{"CreateNode":{"id":{"value":"1RSTKAPSEL0000000000AA"},"kind":"IfcBuildingElementProxy","label":"Kapsel A1101","position":{"x":-15850.0,"y":-8100.0},"ports":[{"name":"b6b3121a-252b-4ba7-ac8d-152c1d0fece6","kind":"in"},{"name":"48558771-3860-4918-ba95-f8b4069326c2","kind":"out"},{"name":"bus","kind":"inOut"}],"properties":[{"key":"ComposeConnectionParams.rotation","value":{"kind":"float","lexeme":"90"}}]}}} |
      | delete-node          | {"prepare":[],"mutation":{"DeleteNode":{"id":{"value":"1tZkmTaMP4R8yLkBdfebfl"}}}} |
      | change-node-kind     | {"prepare":[],"mutation":{"ChangeNodeKind":{"id":{"value":"1o$D5QcDP68vy1YIk$DDV$"},"new_kind":"IfcBuilding"}}} |
      | change-node-label    | {"prepare":[],"mutation":{"ChangeNodeLabel":{"id":{"value":"1o$D5QcDP68vy1YIk$DDV$"},"new_label":"Nakagin Capsule Tower, Ginza"}}} |
      | move-node            | {"prepare":[],"mutation":{"MoveNode":{"id":{"value":"0POPlhUSnC1REPvcqnensi"},"new_position":{"x":-15850.0,"y":-8100.0}}}} |
      | add-node-port        | {"prepare":[],"mutation":{"AddNodePort":{"node_id":{"value":"0POPlhUSnC1REPvcqnensi"},"index":0,"port":{"name":"reset","kind":"inOut"}}}} |
      | remove-node-port     | {"prepare":[],"mutation":{"RemoveNodePort":{"node_id":{"value":"1tZkmTaMP4R8yLkBdfebfl"},"index":0}}} |
      | add-node-property    | {"prepare":[],"mutation":{"AddNodeProperty":{"node_id":{"value":"0POPlhUSnC1REPvcqnensi"},"index":0,"property":{"key":"extent","value":{"kind":"list","items":[{"kind":"int","lexeme":"3"},{"kind":"map","entries":[{"key":"unit","value":{"kind":"str","value":"mm"}}]}]}}}}} |
      | remove-node-property | {"prepare":[],"mutation":{"RemoveNodeProperty":{"node_id":{"value":"0POPlhUSnC1REPvcqnensi"},"index":0}}} |
      | create-edge          | {"prepare":[],"mutation":{"CreateEdge":{"id":{"value":"1RSTKANTE00000000000AA"},"source":{"value":"1tZkmTaMP4R8yLkBdfebfl"},"target":{"value":"0POPlhUSnC1REPvcqnensi"},"kind":"IfcRelConnectsElements","label":"Kapsel an Schacht"}}} |
      | delete-edge          | {"prepare":[],"mutation":{"DeleteEdge":{"id":{"value":"3NLh69tTrEpfV9iDbwoXYL"}}}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real Nakagin Capsule Tower port graph
    Given the real capsule tower graph local://🗣️nakagin-capsule-tower.dsl.semio
    When the <id> mutation is applied to the graph parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the graph and agree on the mutated and the restored snapshot
    Examples:
      | id                   | mutation |
      | create-node          | {"prepare":[],"mutation":{"CreateNode":{"id":{"value":"1RSTKAPSEL0000000000AA"},"kind":"IfcBuildingElementProxy","label":"Kapsel A1101","position":{"x":-15850.0,"y":-8100.0},"ports":[{"name":"b6b3121a-252b-4ba7-ac8d-152c1d0fece6","kind":"in"},{"name":"48558771-3860-4918-ba95-f8b4069326c2","kind":"out"},{"name":"bus","kind":"inOut"}],"properties":[{"key":"ComposeConnectionParams.rotation","value":{"kind":"float","lexeme":"90"}}]}}} |
      | delete-node          | {"prepare":[],"mutation":{"DeleteNode":{"id":{"value":"1tZkmTaMP4R8yLkBdfebfl"}}}} |
      | change-node-kind     | {"prepare":[],"mutation":{"ChangeNodeKind":{"id":{"value":"1o$D5QcDP68vy1YIk$DDV$"},"new_kind":"IfcBuilding"}}} |
      | change-node-label    | {"prepare":[],"mutation":{"ChangeNodeLabel":{"id":{"value":"1o$D5QcDP68vy1YIk$DDV$"},"new_label":"Nakagin Capsule Tower, Ginza"}}} |
      | move-node            | {"prepare":[],"mutation":{"MoveNode":{"id":{"value":"0POPlhUSnC1REPvcqnensi"},"new_position":{"x":-15850.0,"y":-8100.0}}}} |
      | add-node-port        | {"prepare":[],"mutation":{"AddNodePort":{"node_id":{"value":"0POPlhUSnC1REPvcqnensi"},"index":0,"port":{"name":"reset","kind":"inOut"}}}} |
      | remove-node-port     | {"prepare":[],"mutation":{"RemoveNodePort":{"node_id":{"value":"1tZkmTaMP4R8yLkBdfebfl"},"index":0}}} |
      | add-node-property    | {"prepare":[],"mutation":{"AddNodeProperty":{"node_id":{"value":"0POPlhUSnC1REPvcqnensi"},"index":0,"property":{"key":"extent","value":{"kind":"list","items":[{"kind":"int","lexeme":"3"},{"kind":"map","entries":[{"key":"unit","value":{"kind":"str","value":"mm"}}]}]}}}}} |
      | remove-node-property | {"prepare":[],"mutation":{"RemoveNodeProperty":{"node_id":{"value":"0POPlhUSnC1REPvcqnensi"},"index":0}}} |
      | create-edge          | {"prepare":[],"mutation":{"CreateEdge":{"id":{"value":"1RSTKANTE00000000000AA"},"source":{"value":"1tZkmTaMP4R8yLkBdfebfl"},"target":{"value":"0POPlhUSnC1REPvcqnensi"},"kind":"IfcRelConnectsElements","label":"Kapsel an Schacht"}}} |
      | delete-edge          | {"prepare":[],"mutation":{"DeleteEdge":{"id":{"value":"3NLh69tTrEpfV9iDbwoXYL"}}}} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️component.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                   | dir                    | fixture                                                    |
      | create-node          | 🏗️create-node         | appends-a-filter-node-to-the-end-of-the-node-set           |
      | delete-node          | 🗑️delete-node         | removes-the-sink-node-and-severs-the-edge-into-it          |
      | change-node-kind     | 🔧change-node-kind     | retypes-the-source-node-without-relabelling-it             |
      | change-node-label    | 🖍️change-node-label   | relabels-the-source-node-without-retyping-it               |
      | move-node            | 📍move-node            | moves-the-sink-node-to-a-new-canvas-position               |
      | add-node-port        | 🔌add-node-port        | inserts-an-in-port-ahead-of-the-existing-out-port          |
      | remove-node-port     | 🔚remove-node-port     | detaches-the-trailing-out-port-from-the-source-node        |
      | add-node-property    | ➕add-node-property    | inserts-a-weight-property-ahead-of-the-colour-property     |
      | remove-node-property | ➖remove-node-property | detaches-the-trailing-weight-property-from-the-source-node |
      | create-edge          | 🔗create-edge          | connects-the-source-node-to-the-sink-node                  |
      | delete-edge          | ✂️delete-edge         | removes-the-feedback-edge-and-keeps-both-endpoints         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both encodings of the committed wires graph and of the real capsule tower graph
    Given the real committed graph artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/📚️examples/🕸️wires/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/📚️examples/🕸️wires/🖼️assets/🎒️example.pack.semio
    And the real capsule tower graph local://🗣️nakagin-capsule-tower.dsl.semio
    And its binary twin local://🎒️nakagin-capsule-tower.pack.semio
    When each implementation parses all four files, prints both documents back and re-encodes both packs
    Then all four files are reproduced byte for byte and the two implementations agree on both graphs and on the digests of what they emitted
