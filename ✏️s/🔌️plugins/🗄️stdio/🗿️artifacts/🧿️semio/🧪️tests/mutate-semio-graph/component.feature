@capability-semio-v1-graph-mutate
@no-oracle-semio-graph-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-graph
Feature: Apply every typed semio GRAPH mutation to its committed specification fixtures
  `s.stdio.semio.graph` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, so there is no reference implementation to register as an oracle (recorded as the
  `semio-graph-mutation-semantics` no-oracle decision in `../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/
  🧪️oracle/🔣️component.json`). Every one of this subset's 11 kinds carries an independently
  handcrafted `(before, mutation, after, diff)` specification fixture under its own leaf's
  `🧪️tests/` directory, and this feature re-exercises those SAME committed bytes end-to-end through
  `apply_semio_graph_mutation` rather than calling `Mutation::diff`/`inverse` directly the way the
  in-crate fixture tests do.

  What distinguishes this subset is that identity is a NEWTYPE on the wire: a node is addressed as
  `{"value": "n1"}`, never as a bare string, and an edge's `source`/`target` carry the same wrapper.
  Ports and properties are POSITIONAL collections nested one level inside a node, so `add-node-port`
  and `add-node-property` address `(node, index)` while `create-node`/`delete-node` address the
  outer node set. The fixtures are chosen against that: `add-node-port` inserts an `in` port AHEAD
  of the existing `out` port and `add-node-property` inserts ahead of an existing property, so an
  implementation that appended instead of inserting fails; `change-node-kind` retypes a node without
  relabelling it and `change-node-label` relabels without retyping, so an edit that touched both
  fails; and `delete-node` removes the node the one committed edge points INTO, which is the single
  kind here whose effect reaches a collection it was not addressed against — the severed edge must
  come back when the deletion is undone, not merely the node.

  Because this case records a no-oracle decision, the runner executes NO oracle role — every
  assertion below therefore lives inside the subject handler, which compares the applied snapshot
  against the committed after-snapshot and the undone snapshot against the committed
  before-snapshot, and fails with both JSON documents printed. A handler that merely ran the
  mutation and returned would report a pass having checked nothing.

  The `identity-round-trip` scenario carries the BYTE half of the identity law as well as the
  semantic half. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin,
  and both committed example files were produced by these very codecs — so re-printing the parsed
  snapshot and re-encoding it must reproduce those files BYTE FOR BYTE, and the scenario asserts
  exactly that through the shared `law::carrier_is_exact`. The must-differ tripwire the wave applies
  to third-party carriers would be backwards here: a re-emission that DIFFERED would be the defect,
  not the evidence. The two encodings also cross-check each other — the binary twin has to decode to
  the same document the text does, which no single codec can arrange on its own.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_graph_mutation
    Then the resulting snapshot matches the committed after-snapshot fixture for <id>
    Examples:
      | id                    |
      | create-node           |
      | delete-node           |
      | change-node-kind      |
      | change-node-label     |
      | move-node             |
      | add-node-port         |
      | remove-node-port      |
      | add-node-property     |
      | remove-node-property  |
      | create-edge           |
      | delete-edge           |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_semio_graph_mutation
    And the mutation's own computed inverse is applied through apply_semio_graph_mutation
    Then the snapshot matches the committed before-snapshot fixture again
    Examples:
      | id                    |
      | create-node           |
      | delete-node           |
      | change-node-kind      |
      | change-node-label     |
      | move-node             |
      | add-node-port         |
      | remove-node-port      |
      | add-node-property     |
      | remove-node-property  |
      | create-edge           |
      | delete-edge           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed wires graph through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/📚️examples/🕸️wires/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/📚️examples/🕸️wires/🖼️assets/🎒️example.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees on the same two-node, one-edge graph, one node carrying an integer property at the canvas origin and one carrying a string property at a negative coordinate
