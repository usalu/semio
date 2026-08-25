@capability-semio-v1-cad-mutate
@no-oracle-semio-cad-mutation-semantics
@comparison-ordered-json-v1
@mutations-semio-v1-cad
Feature: Apply every typed semio CAD mutation to the real committed drawing artifact
  `s.stdio.semio.cad` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`/`.pack.semio`, so there is no reference implementation to register as an oracle. That
  is recorded as the `semio-cad-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧪️oracle/🔣️component.json`, which also records why the
  obvious candidate was surveyed and rejected rather than merely absent: this subset ships real DXF
  R12, STEP AP214 and DWG AC1024 serializers, and `dxf` 0.6 and `ruststep` 0.4 are both already
  linked into this owner's oracle crate — but the oracle role may never link the subject crate, so
  handing `dxf` a drawing would mean routing the snapshot through THIS repository's own exporter
  first, and DXF's `BLOCKS` section carries no per-entity handle addressing that survives a
  write-read cycle, which strands the four `*-block-entity` kinds with nothing to compare against.

  The input is not synthetic. Every one of the sixteen kinds is applied to the snapshot this
  standard's own committed real artifact decodes to: two layers (`0` continuous, `dim` dashed), one
  `door` block definition holding one handle-addressed line, and eight top-level entities covering
  all nine `CadEntity` geometry variants but the one reused inside the block — an arc, a circle, an
  ellipse, a closed polyline, a rotated text, a block insert, a solid and a dimension. Each kind's
  committed `(before, mutation, after)` specification vector lives in this case's own `🧫️fixtures/`
  and was derived by an INDEPENDENT Python implementation of both the committed DSL grammar and this
  vocabulary's specification, never by running this repository's own Rust. Both roles read the same
  committed bytes: the `oracle` role reads the vector literally (no recomputation, no
  reimplementation of mutation semantics) and the `subject` role decodes it into real
  `SemioCadSnapshot`/`SemioCadMutation` values and runs the production entry point
  `apply_semio_cad_mutation`.

  What genuinely distinguishes this vocabulary from its siblings is that it addresses TWO nested
  name-keyed collections at once: `add-block-entity`, `remove-block-entity`,
  `set-block-entity-layer` and `set-block-entity-geometry` reach an entity by handle INSIDE a named
  block definition, which is why they are exercised against the real `door` block's own `be1` line
  rather than against a top-level entity.

  The `identity-round-trip` scenario is what keeps the vectors honest, and it is the only scenario
  here that touches raw artifact bytes. It asserts that production's OWN `parse_dsl` of the same
  real artifact equals the `before` snapshot every vector starts from, so a mistake in the
  independent Python decoder surfaces as a red scenario instead of a quietly agreeable one. It also
  crosses the two committed encodings of that one drawing against each other — the text
  `🗣️example.dsl.semio` and the binary `🎒️example.pack.semio` are separate committed files produced by
  two separate codecs, so agreeing on one snapshot cannot be achieved by smuggling bytes from either
  one. Note that unlike a foreign-writer format, byte-identical re-emission IS the expected result
  here: the committed text is this codec's own output, so the wave's usual "output must not equal
  input" tripwire does not apply; its mirror law is asserted in its place (see the next paragraph).

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
  Scenario Outline: Apply <id> to the decoded real drawing snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_cad_mutation
    Then the resulting snapshot matches the vector's after-snapshot
    Examples:
      | id                        |
      | no-mutation               |
      | set-snapshot              |
      | add-layer                 |
      | remove-layer              |
      | set-layer                 |
      | add-block                 |
      | remove-block              |
      | set-block-base-point      |
      | add-entity                |
      | remove-entity             |
      | set-entity-layer          |
      | set-entity-geometry       |
      | add-block-entity          |
      | remove-block-entity       |
      | set-block-entity-layer    |
      | set-block-entity-geometry |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the decoded real drawing snapshot
    Given the committed specification vector local://🦠️<id>.json for the <id> kind
    When <id> is applied to its before-snapshot through apply_semio_cad_mutation
    And the mutation's own computed inverse is applied through apply_semio_cad_mutation
    Then the snapshot matches the vector's before-snapshot again
    Examples:
      | id                        |
      | no-mutation               |
      | set-snapshot              |
      | add-layer                 |
      | remove-layer              |
      | set-layer                 |
      | add-block                 |
      | remove-block              |
      | set-block-base-point      |
      | add-entity                |
      | remove-entity             |
      | set-entity-layer          |
      | set-entity-geometry       |
      | add-block-entity          |
      | remove-block-entity       |
      | set-block-entity-layer    |
      | set-block-entity-geometry |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real drawing artifact through both of its committed encodings
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📐️drawing/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📐️drawing/🖼️assets/🎒️example.pack.semio
    And the committed specification vector local://🦠️no-mutation.json whose before-snapshot is that artifact decoded
    When the text artifact is parsed, printed back to DSL and parsed again, and the binary twin is decoded and re-encoded
    Then every decoding agrees with the committed before-snapshot
