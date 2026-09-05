@capability-semio-v1-flow-mutate
@oracle-semio-flow-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-flow
Feature: Apply every typed semio FLOW mutation to the Nakagin Capsule Tower's 180-node connection network, against an independent Python implementation
  `stdio.semio.flow` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio` or `.pack.semio`, so the second producer a differential comparison needs is a second
  IMPLEMENTATION. `🐍️component.py` beside this file is that implementation — the envelope, the DSL
  grammar, the LEB128 pack frame with its little-endian `f64` coordinates, and all thirteen verbs
  with their inverses, written in Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
  the committed `(before, mutation, after)` vectors in this case's own `🧫️fixtures/`, and the semio
  envelope region of `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing
  from and transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-flow-python-independent` in `…/🌊️flow/🔮️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against. That decision
  also recorded json-rust as surveyed and declined — a generic JSON DOM is not a reference
  implementation of the vocabulary carried inside the container — and that judgement still stands;
  what changed is that a second implementation of the vocabulary itself now exists.

  🏗️ **The graph under test is a real building.** The richest `stdio.semio.flow` document committed
  anywhere in this artifact is the two-node demo pipeline, which is a fixture, not a network. So the
  flow this case mutates was derived ONCE — by `🐍️derive-flow-fixture.py` in the ticket folder — from
  the real committed IFC 4 model `../../../🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc`, Kisho
  Kurokawa's Nakagin Capsule Tower, 2.5 MB and 24 792 entities, read with **IfcOpenShell 0.8.4** — a
  genuine third-party IFC implementation, and the only reader involved in the provenance. Its 180
  `IfcBuildingElementProxy` capsules and cores became the nodes, keyed by their real IFC GlobalIds
  and labelled with their real names; their 366 `IfcPropertySingleValue` properties became the
  params; their `IfcLocalPlacement` coordinates became the positions, taking the placement's X and Z
  because a flow canvas is two-dimensional and the tower's pieces are distributed in plan and in
  elevation; and the 179 `IfcRelConnectsPorts` relations between their 364 `IfcDistributionPort`s
  became the edges, each endpoint resolved to the element that nests the port through `IfcRelNests`
  and each edge kinded by the relating port's own human description. The result is 131 252 bytes of
  DSL and 67 184 bytes of pack, against 249 and 160 for the demo pipeline the case used to rest on.

  ⚖️ **A property of the vocabulary the parameters have to respect.** `insert-node` and `insert-edge`
  carry no index and append, so the undo of a removal can only put the record back at the END of its
  collection: removing a node or an edge that is not the last one is not invertible in this
  vocabulary at all. `remove-node`, `remove-edge` and `remove-node-param` therefore address the last
  record of their collection, exactly as the committed vectors do. This is stated rather than worked
  around; the same constraint binds both implementations.

  The remaining parameters are chosen against the network's own shape, so a plausible wrong codec
  fails: `set-node-param` writes a key the addressed node does NOT carry, so it exercises the append
  branch that `remove-node-param`'s inverse exercises in reverse, `set-node-kind` and
  `set-node-label` retag one capsule out of 180 while its 179 siblings stay put,
  `set-node-position` moves a capsule to a fractional coordinate that only survives an exact `f64`
  round trip, `set-edge-endpoints` reverses one connection's direction without touching its kind,
  `set-edge-kind` rewrites one edge's description to a non-ASCII one, and `remove-node` deletes a
  capsule that 179 edges do NOT cascade from — the committed vectors' own semantics.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each kind, whose before-snapshot is the real demo pipeline
  artifact decoded, now applied AND undone by BOTH implementations and checked against the committed
  after- and before-snapshots by each of them in role. Nothing was removed to make room for the
  oracle.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions. `.dsl.semio`
  is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact re-emission is
  the CORRECT answer here and the wave's must-differ tripwire would be backwards, which is why the
  Rust side asserts `law::carrier_is_exact`. What stops that being a codec agreeing with itself is
  that the demo pipeline's two encodings were written by the RUST codec and the Python side
  reproduces them byte for byte from the grammar alone, while the capsule network's two encodings
  were written by the PYTHON implementation and the Rust codec has to reproduce THOSE — each
  implementation is measured against bytes the other one emitted, and the digests are compared
  across the two languages.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real 180-node capsule network
    Given the real capsule network local://📝️nakagin-capsule-tower.dsl.semio
    When the <id> mutation is applied to the flow parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting flow
    Examples:
      | id                 | mutation |
      | set-snapshot       | {"mutation":"setSnapshot","snapshot":{"schema":"stdio.semio.flow","nodes":[{"id":"0POPlhUSnC1REPvcqnensi","kind":"IfcElementAssembly","label":"Kernrest","params":[{"key":"ComposeConnectionParams.rotation","value":"270.0"}],"position":{"x":0.0,"y":0.0}}],"edges":[]}} |
      | insert-node        | {"mutation":"insertNode","node":{"id":"1RSTKAPSEL0000000000AA","kind":"IfcBuildingElementProxy","label":"Ersatzkapsel A1101","params":[{"key":"ComposePieceAttributes.name","value":"Ersatzkapsel A1101"},{"key":"ComposeConnectionParams.shift","value":"-12.5"}],"position":{"x":-2650.0,"y":42283.33}}} |
      | remove-node        | {"mutation":"removeNode","id":"1tZkmTaMP4R8yLkBdfebfl"} |
      | set-node-kind      | {"mutation":"setNodeKind","id":"0POPlhUSnC1REPvcqnensi","kind":"IfcElementAssembly"} |
      | set-node-label     | {"mutation":"setNodeLabel","id":"3GOXMcqS9E287ioto$RIXo","label":"Kapselträger, Ostkern"} |
      | set-node-position  | {"mutation":"setNodePosition","id":"1OS4$rPqz9cOn2s4ojb1k3","position":{"x":-18600.25,"y":39583.33}} |
      | set-node-param     | {"mutation":"setNodeParam","id":"3GOXMcqS9E287ioto$RIXo","key":"ComposeConnectionParams.rotation","value":"270.0"} |
      | remove-node-param  | {"mutation":"removeNodeParam","id":"0POPlhUSnC1REPvcqnensi","key":"ComposeConnectionParams.rotation"} |
      | insert-edge        | {"mutation":"insertEdge","edge":{"id":"1RSTKANTE00000000000AA","from":{"node":"1tZkmTaMP4R8yLkBdfebfl","port":"2r0obXvqjDmQpZoWs86xTs"},"to":{"node":"0POPlhUSnC1REPvcqnensi","port":"28MKF16un8NBtKsWfORP5Y"},"kind":"Rückführung auf den Ostkern."}} |
      | remove-edge        | {"mutation":"removeEdge","id":"3NLh69tTrEpfV9iDbwoXYL"} |
      | set-edge-endpoints | {"mutation":"setEdgeEndpoints","id":"2jGlFQA9H2mvmjiNpnYG5Q","from":{"node":"0IEifuk9T5eR2vbWao4vJp","port":"0DFWl3CFjFrhgWeoJyVitG"},"to":{"node":"0POPlhUSnC1REPvcqnensi","port":"28MKF16un8NBtKsWfORP5Y"}} |
      | set-edge-kind      | {"mutation":"setEdgeKind","id":"2jGlFQA9H2mvmjiNpnYG5Q","kind":"Die Mitte des östlichen Rechteckkerns."} |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real 180-node capsule network
    Given the real capsule network local://📝️nakagin-capsule-tower.dsl.semio
    When the no-mutation mutation is applied to the flow parsed from it
      """
      {"mutation":"noMutation"}
      """
    Then the independent implementation and the subject agree on the resulting flow

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real 180-node capsule network
    Given the real capsule network local://📝️nakagin-capsule-tower.dsl.semio
    When the <id> mutation is applied to the flow parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the capsule network and agree on the mutated and the restored flow
    Examples:
      | id                 | mutation |
      | set-snapshot       | {"mutation":"setSnapshot","snapshot":{"schema":"stdio.semio.flow","nodes":[{"id":"0POPlhUSnC1REPvcqnensi","kind":"IfcElementAssembly","label":"Kernrest","params":[{"key":"ComposeConnectionParams.rotation","value":"270.0"}],"position":{"x":0.0,"y":0.0}}],"edges":[]}} |
      | insert-node        | {"mutation":"insertNode","node":{"id":"1RSTKAPSEL0000000000AA","kind":"IfcBuildingElementProxy","label":"Ersatzkapsel A1101","params":[{"key":"ComposePieceAttributes.name","value":"Ersatzkapsel A1101"},{"key":"ComposeConnectionParams.shift","value":"-12.5"}],"position":{"x":-2650.0,"y":42283.33}}} |
      | remove-node        | {"mutation":"removeNode","id":"1tZkmTaMP4R8yLkBdfebfl"} |
      | set-node-kind      | {"mutation":"setNodeKind","id":"0POPlhUSnC1REPvcqnensi","kind":"IfcElementAssembly"} |
      | set-node-label     | {"mutation":"setNodeLabel","id":"3GOXMcqS9E287ioto$RIXo","label":"Kapselträger, Ostkern"} |
      | set-node-position  | {"mutation":"setNodePosition","id":"1OS4$rPqz9cOn2s4ojb1k3","position":{"x":-18600.25,"y":39583.33}} |
      | set-node-param     | {"mutation":"setNodeParam","id":"3GOXMcqS9E287ioto$RIXo","key":"ComposeConnectionParams.rotation","value":"270.0"} |
      | remove-node-param  | {"mutation":"removeNodeParam","id":"0POPlhUSnC1REPvcqnensi","key":"ComposeConnectionParams.rotation"} |
      | insert-edge        | {"mutation":"insertEdge","edge":{"id":"1RSTKANTE00000000000AA","from":{"node":"1tZkmTaMP4R8yLkBdfebfl","port":"2r0obXvqjDmQpZoWs86xTs"},"to":{"node":"0POPlhUSnC1REPvcqnensi","port":"28MKF16un8NBtKsWfORP5Y"},"kind":"Rückführung auf den Ostkern."}} |
      | remove-edge        | {"mutation":"removeEdge","id":"3NLh69tTrEpfV9iDbwoXYL"} |
      | set-edge-endpoints | {"mutation":"setEdgeEndpoints","id":"2jGlFQA9H2mvmjiNpnYG5Q","from":{"node":"0IEifuk9T5eR2vbWao4vJp","port":"0DFWl3CFjFrhgWeoJyVitG"},"to":{"node":"0POPlhUSnC1REPvcqnensi","port":"28MKF16un8NBtKsWfORP5Y"}} |
      | set-edge-kind      | {"mutation":"setEdgeKind","id":"2jGlFQA9H2mvmjiNpnYG5Q","kind":"Die Mitte des östlichen Rechteckkerns."} |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-differential
  Scenario: Undoing no-mutation restores the real 180-node capsule network
    Given the real capsule network local://📝️nakagin-capsule-tower.dsl.semio
    When the no-mutation mutation is applied to the flow parsed from it and each side undoes it with its own computed inverse
      """
      {"mutation":"noMutation"}
      """
    Then both sides restore the capsule network and agree on the mutated and the restored flow

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply and undo <id> on its committed specification vector over the real demo pipeline
    Given the committed specification vector local://🧫️<id>/🦠️mutation/🔣️.json whose before-snapshot is the real pipeline artifact decoded
    When both implementations apply the vector's mutation to its before-snapshot and undo it again
    Then each reaches the vector's after-snapshot, each returns to its before-snapshot, and the two agree
    Examples:
      | id                 |
      | no-mutation        |
      | set-snapshot       |
      | insert-node        |
      | remove-node        |
      | set-node-kind      |
      | set-node-label     |
      | set-node-position  |
      | set-node-param     |
      | remove-node-param  |
      | insert-edge        |
      | remove-edge        |
      | set-edge-endpoints |
      | set-edge-kind      |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both encodings of the demo pipeline and of the real capsule network from the parsed documents
    Given the real committed text artifact asset://📚️examples/🌊️pipeline/🖼️assets/🗣️.dsl.semio
    And its committed binary twin asset://📚️examples/🌊️pipeline/🖼️assets/🎒️.pack.semio
    And the real capsule network local://📝️nakagin-capsule-tower.dsl.semio
    And its binary twin local://📦️nakagin-capsule-tower.pack.semio
    When each implementation parses all four files, prints the two documents back and re-encodes both packs
    Then all four files are reproduced byte for byte and the two implementations agree on the documents and on the digests of what they emitted
