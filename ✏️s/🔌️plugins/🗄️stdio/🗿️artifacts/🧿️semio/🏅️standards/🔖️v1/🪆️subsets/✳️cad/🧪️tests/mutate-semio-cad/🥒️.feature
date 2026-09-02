@capability-semio-v1-cad-mutate
@oracle-semio-cad-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-cad
Feature: Apply every typed semio CAD mutation to the real committed drawing, against an independent Python implementation
  `s.stdio.semio.cad` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio`/`.pack.semio`, so the second producer a differential comparison needs is a second
  IMPLEMENTATION. `🐍️component.py` beside this file is that implementation — the envelope, the DSL
  grammar, the binary pack frame and all sixteen verbs together with their inverses, written in
  Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` and the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing from and
  transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-cad-python-independent` in `…/✳️cad/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  The DXF route the replaced decision surveyed stays rejected and nothing here revives it: the
  oracle role may never link the subject crate, so handing `dxf` 0.6 a drawing would mean routing
  the snapshot through THIS repository's own DXF exporter first, and DXF's `BLOCKS` section carries
  no per-entity handle addressing that survives a write-read cycle, which strands the four
  `*-block-entity` kinds. A from-specification second implementation judges all sixteen.

  The document under test is the REAL committed drawing, read where the domain keeps it through
  `asset://` and never written to: two layers (`0` continuous, `dim` dashed), one `door` block
  definition holding one handle-addressed line, and eight top-level entities covering all nine
  `CadEntity` geometry variants but the one reused inside the block — an arc, a circle, an ellipse,
  a closed polyline, a rotated text, a block insert, a solid and a dimension. It is the richest
  `s.stdio.semio.cad` document committed anywhere in this artifact; `asset://` resolves against the
  artifact root, so no other plugin's larger `.dsl.semio` is reachable from here, and that limit is
  stated rather than papered over.

  The `mutate-` parameters are chosen against the drawing's own shape, so a plausible wrong codec
  fails: `remove-layer` deletes the LAST layer while `remove-entity` deletes the MIDDLE circle `h2`
  so an implementation that pops the tail fails, `set-layer` writes all three of its optional
  arguments at once so an implementation that honours only one fails, `set-entity-geometry`
  replaces a TEXT entity with a DIMENSION so a variant-preserving shortcut fails, and the four
  `*-block-entity` verbs reach `be1` INSIDE the `door` block rather than a top-level entity.

  One honest boundary, exercised at its edge rather than hidden: the vocabulary's own inverse of a
  removal is the matching `add-…`, and `add-…` APPENDS to its name-keyed collection. Undoing a
  removal therefore restores the value but not the position unless the removed entry was the last
  one. `inverse-remove-entity` consequently removes the FINAL entity `h8` where `mutate-remove-entity`
  removes the middle `h2`; both implementations agree on this and both fail identically at `h2`, so
  it is a property of the vocabulary rather than a disagreement between codecs, and it is recorded
  here and in the oracle manifest instead of being papered over by a tail-only forward parameter.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each kind in this case's own `🧫️fixtures/`, now applied by
  BOTH implementations and checked against the committed after-snapshot by each of them in role.
  Nothing was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law on BOTH committed encodings.
  `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, and both files
  were produced by the Rust codecs, so an exact re-emission is the CORRECT answer here and the
  wave's must-differ tripwire would be backwards, which is why the Rust side asserts
  `law::carrier_is_exact` twice. What stops that being a codec agreeing with itself is that the
  Python side reproduces the same two files byte for byte — the text from the grammar, the binary
  from the committed protocol plus a record layout derived from the committed bytes because the
  protocol document declares the three collections one opaque `payload` chain by its own admission —
  and the two sides' digests of what each emitted are compared. The two encodings also cross-check
  each other: the binary twin has to decode to the same drawing the text does.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real committed drawing
    Given the real committed drawing asset://📚️examples/📐️drawing/🖼️assets/🗣️.dsl.semio
    When the <id> mutation is applied to the drawing parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id                        | mutation                                                                                                                                                                                       |
      | no-mutation               | {"mutation":"noMutation"}                                                                                                                                                                      |
      | set-snapshot              | {"mutation":"setSnapshot","snapshot":{"schema":"stdio.semio.cad","layers":[{"name":"0","colorIndex":7,"lineType":"CONTINUOUS","visible":true}],"blocks":[],"entities":[]}}                     |
      | add-layer                 | {"mutation":"addLayer","layer":{"name":"hidden","colorIndex":8,"lineType":"HIDDEN","visible":false}}                                                                                           |
      | remove-layer              | {"mutation":"removeLayer","name":"dim"}                                                                                                                                                        |
      | set-layer                 | {"mutation":"setLayer","name":"0","color_index":5,"line_type":"DASHED","visible":false}                                                                                                        |
      | add-block                 | {"mutation":"addBlock","block":{"name":"window","basePoint":{"x":0.0,"y":0.0},"entities":[{"handle":"we1","layer":"0","entity":{"kind":"line","a":{"x":0.0,"y":0.0},"b":{"x":0.0,"y":1.0}}}]}} |
      | remove-block              | {"mutation":"removeBlock","name":"door"}                                                                                                                                                       |
      | set-block-base-point      | {"mutation":"setBlockBasePoint","name":"door","base_point":{"x":2.5,"y":-1.0}}                                                                                                                 |
      | add-entity                | {"mutation":"addEntity","entity":{"handle":"h9","layer":"dim","entity":{"kind":"circle","center":{"x":9.0,"y":9.0},"radius":0.5}}}                                                             |
      | remove-entity             | {"mutation":"removeEntity","handle":"h2"}                                                                                                                                                      |
      | set-entity-layer          | {"mutation":"setEntityLayer","handle":"h1","layer":"dim"}                                                                                                                                      |
      | set-entity-geometry       | {"mutation":"setEntityGeometry","handle":"h5","entity":{"kind":"dimension","def_point":{"x":0.0,"y":0.0},"text_position":{"x":1.0,"y":1.0},"measurement":4.2,"text":"4.20m"}}                  |
      | add-block-entity          | {"mutation":"addBlockEntity","block_name":"door","entity":{"handle":"be2","layer":"dim","entity":{"kind":"line","a":{"x":1.0,"y":0.0},"b":{"x":1.0,"y":1.0}}}}                                 |
      | remove-block-entity       | {"mutation":"removeBlockEntity","block_name":"door","handle":"be1"}                                                                                                                            |
      | set-block-entity-layer    | {"mutation":"setBlockEntityLayer","block_name":"door","handle":"be1","layer":"dim"}                                                                                                            |
      | set-block-entity-geometry | {"mutation":"setBlockEntityGeometry","block_name":"door","handle":"be1","entity":{"kind":"arc","center":{"x":0.0,"y":0.0},"radius":1.0,"start_angle":0.0,"end_angle":90.0}}                    |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real committed drawing
    Given the real committed drawing asset://📚️examples/📐️drawing/🖼️assets/🗣️.dsl.semio
    When the <id> mutation is applied to the drawing parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the drawing and agree on the mutated and the restored snapshot
    Examples:
      | id                        | mutation                                                                                                                                                                                       |
      | no-mutation               | {"mutation":"noMutation"}                                                                                                                                                                      |
      | set-snapshot              | {"mutation":"setSnapshot","snapshot":{"schema":"stdio.semio.cad","layers":[{"name":"0","colorIndex":7,"lineType":"CONTINUOUS","visible":true}],"blocks":[],"entities":[]}}                     |
      | add-layer                 | {"mutation":"addLayer","layer":{"name":"hidden","colorIndex":8,"lineType":"HIDDEN","visible":false}}                                                                                           |
      | remove-layer              | {"mutation":"removeLayer","name":"dim"}                                                                                                                                                        |
      | set-layer                 | {"mutation":"setLayer","name":"0","color_index":5,"line_type":"DASHED","visible":false}                                                                                                        |
      | add-block                 | {"mutation":"addBlock","block":{"name":"window","basePoint":{"x":0.0,"y":0.0},"entities":[{"handle":"we1","layer":"0","entity":{"kind":"line","a":{"x":0.0,"y":0.0},"b":{"x":0.0,"y":1.0}}}]}} |
      | remove-block              | {"mutation":"removeBlock","name":"door"}                                                                                                                                                       |
      | set-block-base-point      | {"mutation":"setBlockBasePoint","name":"door","base_point":{"x":2.5,"y":-1.0}}                                                                                                                 |
      | add-entity                | {"mutation":"addEntity","entity":{"handle":"h9","layer":"dim","entity":{"kind":"circle","center":{"x":9.0,"y":9.0},"radius":0.5}}}                                                             |
      | remove-entity             | {"mutation":"removeEntity","handle":"h8"}                                                                                                                                                      |
      | set-entity-layer          | {"mutation":"setEntityLayer","handle":"h1","layer":"dim"}                                                                                                                                      |
      | set-entity-geometry       | {"mutation":"setEntityGeometry","handle":"h5","entity":{"kind":"dimension","def_point":{"x":0.0,"y":0.0},"text_position":{"x":1.0,"y":1.0},"measurement":4.2,"text":"4.20m"}}                  |
      | add-block-entity          | {"mutation":"addBlockEntity","block_name":"door","entity":{"handle":"be2","layer":"dim","entity":{"kind":"line","a":{"x":1.0,"y":0.0},"b":{"x":1.0,"y":1.0}}}}                                 |
      | remove-block-entity       | {"mutation":"removeBlockEntity","block_name":"door","handle":"be1"}                                                                                                                            |
      | set-block-entity-layer    | {"mutation":"setBlockEntityLayer","block_name":"door","handle":"be1","layer":"dim"}                                                                                                            |
      | set-block-entity-geometry | {"mutation":"setBlockEntityGeometry","block_name":"door","handle":"be1","entity":{"kind":"arc","center":{"x":0.0,"y":0.0},"radius":1.0,"start_angle":0.0,"end_angle":90.0}}                    |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed specification vector local://🧫️<id>/🦠️mutation/🔣️.json for the <id> kind
    When both implementations apply the vector's mutation to its before-snapshot
    Then each reaches the committed after-snapshot and the two agree
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
  Scenario: Re-emit both committed encodings of the real drawing from the parsed snapshot
    Given the real committed drawing asset://📚️examples/📐️drawing/🖼️assets/🗣️.dsl.semio
    And its committed binary twin asset://📚️examples/📐️drawing/🖼️assets/🎒️.pack.semio
    And the committed specification vector local://🧫️no-mutation/🦠️mutation/🔣️.json whose before-snapshot is that artifact decoded
    When each implementation parses the text artifact, prints it back, decodes the binary twin and re-encodes it
    Then both reproduce the two committed files byte for byte and agree on the drawing and on the digests of what they emitted
