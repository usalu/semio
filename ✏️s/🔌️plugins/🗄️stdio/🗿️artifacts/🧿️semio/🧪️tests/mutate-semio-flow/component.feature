@capability-semio-v1-flow-mutate
@no-oracle-semio-flow-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-flow
Feature: Apply every typed semio FLOW mutation to the real committed pipeline artifact
  `s.stdio.semio.flow` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an oracle. That
  is recorded as the `semio-flow-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧪️oracle/🔣️component.json`, which also records why
  json-rust — already linked into this owner's oracle crate, and reachable through this subset's own
  JSON RFC8259 export serializer — was surveyed and DELIBERATELY declined rather than merely absent:
  it supplies a generic JSON DOM and no knowledge of nodes, edges, ports or params, so all thirteen
  mutation semantics would still be hand-written on top of it and calling that a third-party
  reference would overstate the evidence.

  The input is not synthetic. Every one of the thirteen kinds is applied to the snapshot this
  standard's own committed real artifact decodes to — 2 nodes (one carrying two params and the
  origin, one carrying none and a negative coordinate) joined by one `out`→`in` edge. Each kind's
  committed `(before, mutation, after)` specification vector lives in this case's own `🧫️fixtures/`
  and was derived by an INDEPENDENT Python implementation of both the committed DSL grammar and this
  vocabulary's specification, never by running this repository's own Rust. Both roles read the same
  committed bytes: the `oracle` role reads the vector literally (no recomputation, no
  reimplementation of mutation semantics) and the `subject` role decodes it into real
  `SemioFlowSnapshot`/`SemioFlowMutation` values and runs the production entry point
  `apply_semio_flow_mutation`.

  The `identity-round-trip` scenario is what keeps the vectors honest, and it is the only scenario
  here that touches raw artifact bytes. It asserts that production's OWN `parse_dsl` of the same
  real artifact equals the `before` snapshot every vector starts from, so a mistake in the
  independent Python decoder surfaces as a red scenario instead of a quietly agreeable one. It also
  crosses the two committed encodings of that one document against each other — the text
  `🗣️example.dsl.semio` and the binary `🎒️example.pack.semio` are separate committed files produced by
  two separate codecs, so agreeing on one snapshot cannot be achieved by smuggling bytes from either
  one. Note that unlike a foreign-writer format, byte-identical re-emission IS the expected result
  here: the committed text is this codec's own output, so the wave's usual "output must not equal
  input" tripwire does not apply and the pack/DSL cross-check carries that evidence instead.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to the decoded real pipeline snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_flow_mutation
    Then the resulting snapshot matches the vector's after-snapshot
    Examples:
      | id                |
      | no-mutation       |
      | set-snapshot      |
      | insert-node       |
      | remove-node       |
      | set-node-kind     |
      | set-node-label    |
      | set-node-position |
      | set-node-param    |
      | remove-node-param |
      | insert-edge       |
      | remove-edge       |
      | set-edge-endpoints |
      | set-edge-kind     |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the decoded real pipeline snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_flow_mutation
    And the mutation's own computed inverse is applied through apply_semio_flow_mutation
    Then the snapshot matches the vector's before-snapshot again
    Examples:
      | id                |
      | no-mutation       |
      | set-snapshot      |
      | insert-node       |
      | remove-node       |
      | set-node-kind     |
      | set-node-label    |
      | set-node-position |
      | set-node-param    |
      | remove-node-param |
      | insert-edge       |
      | remove-edge       |
      | set-edge-endpoints |
      | set-edge-kind     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real pipeline artifact through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets/🎒️example.pack.semio
    And the committed specification vector local://🦠️no-mutation.json whose before-snapshot is that artifact decoded
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees with the committed before-snapshot
