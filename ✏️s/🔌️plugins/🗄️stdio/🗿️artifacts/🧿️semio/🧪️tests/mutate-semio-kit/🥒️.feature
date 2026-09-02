@capability-semio-v1-kit-mutate
@oracle-semio-kit-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-kit
Feature: Apply every typed semio KIT mutation to the Nakagin Capsule Tower kit of parts, against an independent Python implementation
  `s.stdio.semio.kit` is a semio-NATIVE format: no third party reads or writes `.dsl.semio` or
  `.pack.semio`, so the second producer a differential comparison needs is a second IMPLEMENTATION.
  `🐍️component.py` beside this file is that implementation — the carrier, the DSL grammar, the pack
  frame and all fifteen verbs, written in Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/🔣️.json`,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`, the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`, and `ArtifactRef::to_uri` plus
  `LinkPin`/`BlobRef` for the dialect coordinate and the three pin shapes), importing nothing from
  and transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-kit-python-independent` in `…/✳️kit/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  🏗️ **The kit under test is a real building, read as a kit of parts.** The richest
  `s.stdio.semio.kit` document committed anywhere in this artifact is the one-type two-piece
  furniture kit — 734 bytes, which is a fixture, not a kit. So the document every mutation row below
  runs on was derived ONCE — by `🐍️derive-kit-fixture.py` in the ticket folder — from the real
  committed IFC 4 file `../../../🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc`, Kisho Kurokawa's
  Nakagin Capsule Tower, 2.5 MB and 24 792 entities, read with **IfcOpenShell 0.8.4**, a genuine
  third-party IFC implementation and the only reader involved in the provenance. Its 12 real
  `IfcBuildingElementProxyType`s became the type catalogue, its `IfcElementAssembly` became the one
  design, its 180 real capsules became that design's pieces — each carrying its real `GlobalId`, its
  real declared type and its real placement transform (translation in millimetres, orientation
  quaternion computed from the real `Axis`/`RefDirection` pair, unit scale) — its 179 real
  `IfcRelConnectsPorts` became the connections, naming the real port GUIDs on both ends, and each
  real type got a representation link addressed by its own real `GlobalId`. The result is 78 066
  bytes of DSL and 50 019 bytes of pack, against 734 and 498 for the furniture kit the case used to
  rest on. IfcOpenShell reads IFC, not a semio envelope, and cannot express a single one of the
  fifteen verbs, which is why it is the source of the ARTIFACT and never the oracle.

  🧩️ **All four composition shapes are still present.** IFC has no notion of an owned semio child, so
  the derived kit keeps the same three real handles the furniture kit carries in its `object` pool,
  its `model` pool and its single `properties` slot; the repeated child pool, the single child slot,
  the link pool and the nested design are therefore all exercised here exactly as before, and this is
  stated rather than left for a reader to discover.

  Two rows need the real document put into the state their verb is DEFINED for, and each scenario's
  doc string carries the `prepare` list that does it. `create-properties` attaches to the single
  properties SLOT, which the kit already occupies, so its list detaches it first. `remove-type` is
  aimed at a type appended by its own `prepare` list rather than at one of the tower's twelve,
  because every one of those is referenced by pieces of the design and neither the grammar nor any
  committed vector says what removing a referenced type does to them — exercising that would measure
  a gap in the specification rather than the two implementations, and it is recorded as a gap
  instead.

  The parameters are chosen against the tower kit's own shape, so a plausible wrong codec fails:
  `create-object`/`create-model` append a SECOND handle beside the existing one and must not disturb
  it, `delete-object`/`delete-model` empty one pool and must leave the other whole,
  `bind-representation` binds with a CHECKPOINT pin where all twelve committed links are pinned to
  head, so a pin codec that only knows head fails; `change-representation-pin` repins the first of
  those twelve; `unbind-representation` detaches the LAST of them, because `bind-representation`
  appends and the undo of a removal can only put the link back at the end; `rename-type` renames the
  real type `0IJK4xQ0PB2hjiZlMVI36G` and must leave its category and id alone; `remove-design` drops
  the tower's only design whole, 180 pieces and 179 connections with it, so its inverse has to
  restore every one of them in order; and `edit-design` rewrites that design to two of its own real
  pieces and one of its own real connections, with a real capsule position in millimetres and a
  non-unit scale, so a codec that keeps stale members fails.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each kind, now applied by BOTH
  implementations and checked against the committed after-snapshot by each of them in role. Nothing
  was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions, over FOUR
  files. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an
  exact re-emission is the CORRECT answer here and the wave's must-differ tripwire would be
  backwards, which is why the Rust side asserts `law::carrier_is_exact`. The furniture kit's two
  encodings were written by the RUST codec and the Python side reproduces them byte for byte from the
  grammar alone — it is kept for exactly that reason, and nothing it proved was given up — while the
  capsule tower kit's two encodings were written by the PYTHON implementation and the Rust codec has
  to reproduce THOSE, 1 800 real `f64` transform components among them.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real Nakagin Capsule Tower kit
    Given the real capsule tower kit local://🧪️nakagin-capsule-tower/🗣️.dsl.semio
    When the <id> mutation is applied to the prepared kit parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id                        | mutation |
      | create-object             | {"prepare":[],"mutation":{"CreateObject":{"child_id":"obj-02","target":{"artifactId":"nakagin-capsule-instance","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"object"}}}}} |
      | delete-object             | {"prepare":[],"mutation":{"DeleteObject":{"child_id":"obj-01"}}} |
      | create-model              | {"prepare":[],"mutation":{"CreateModel":{"child_id":"model-02","target":{"artifactId":"nakagin-capsule-tower-bim","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"model"}}}}} |
      | delete-model              | {"prepare":[],"mutation":{"DeleteModel":{"child_id":"model-01"}}} |
      | create-properties         | {"prepare":[{"DeleteProperties":{}}],"mutation":{"CreateProperties":{"child_id":"props-02","target":{"artifactId":"kit-props-metric","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"value"}}}}} |
      | delete-properties         | {"prepare":[],"mutation":{"DeleteProperties":{}}} |
      | bind-representation       | {"prepare":[],"mutation":{"BindRepresentation":{"target":{"artifactId":"nakagin-capsule-plan","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"drawing"}},"pin":{"kind":"checkpoint","id":"v7"},"role":"2xDbYUk7X3XwXfITBwE0ij"}}} |
      | unbind-representation     | {"prepare":[],"mutation":{"UnbindRepresentation":{"index":11}}} |
      | change-representation-pin | {"prepare":[],"mutation":{"ChangeRepresentationPin":{"index":0,"pin":{"kind":"checkpoint","id":"v3"}}}} |
      | add-type                  | {"prepare":[],"mutation":{"AddType":{"id":"1RSTTYP0000000000000AA","name":"Kapseltyp K","category":"IfcBuildingElementProxyType"}}} |
      | remove-type               | {"prepare":[{"AddType":{"id":"1RSTTYP0000000000000AA","name":"Kapseltyp K","category":"IfcBuildingElementProxyType"}}],"mutation":{"RemoveType":{"id":"1RSTTYP0000000000000AA"}}} |
      | rename-type               | {"prepare":[],"mutation":{"RenameType":{"id":"0IJK4xQ0PB2hjiZlMVI36G","new_name":"Schacht"}}} |
      | add-design                | {"prepare":[],"mutation":{"AddDesign":{"id":"1RSTENTWURF000000000AA","name":"Kapselgeschoss"}}} |
      | remove-design             | {"prepare":[],"mutation":{"RemoveDesign":{"id":"1o$D5QcDP68vy1YIk$DDV$"}}} |
      | edit-design               | {"prepare":[],"mutation":{"EditDesign":{"id":"1o$D5QcDP68vy1YIk$DDV$","pieces":[{"id":"0POPlhUSnC1REPvcqnensi","typeId":"0UyxhWPMj1p983Bl6$TZEY","transform":{"translation":{"x":0.0,"y":0.0,"z":0.0},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"scale":{"x":1.0,"y":1.0,"z":1.0}}},{"id":"1tZkmTaMP4R8yLkBdfebfl","typeId":"0X0Bv8cZnBrPiyH9U4$zNu","transform":{"translation":{"x":-15850.0,"y":-8100.0,"z":2735.0},"rotation":{"x":0.0,"y":0.0,"z":1.0,"w":0.0},"scale":{"x":1.0,"y":2.0,"z":1.0}}}],"connections":[{"id":"3NLh69tTrEpfV9iDbwoXYL","connectingPieceId":"1tZkmTaMP4R8yLkBdfebfl","connectingPort":"b6b3121a-252b-4ba7-ac8d-152c1d0fece6","connectedPieceId":"0POPlhUSnC1REPvcqnensi","connectedPort":"c5465220-19ba-4443-8f1d-617c832dd13c"}]}}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the prepared Nakagin Capsule Tower kit
    Given the real capsule tower kit local://🧪️nakagin-capsule-tower/🗣️.dsl.semio
    When the <id> mutation is applied to the prepared kit parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the prepared kit and agree on the mutated and the restored snapshot
    Examples:
      | id                        | mutation |
      | create-object             | {"prepare":[],"mutation":{"CreateObject":{"child_id":"obj-02","target":{"artifactId":"nakagin-capsule-instance","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"object"}}}}} |
      | delete-object             | {"prepare":[],"mutation":{"DeleteObject":{"child_id":"obj-01"}}} |
      | create-model              | {"prepare":[],"mutation":{"CreateModel":{"child_id":"model-02","target":{"artifactId":"nakagin-capsule-tower-bim","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"model"}}}}} |
      | delete-model              | {"prepare":[],"mutation":{"DeleteModel":{"child_id":"model-01"}}} |
      | create-properties         | {"prepare":[{"DeleteProperties":{}}],"mutation":{"CreateProperties":{"child_id":"props-02","target":{"artifactId":"kit-props-metric","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"value"}}}}} |
      | delete-properties         | {"prepare":[],"mutation":{"DeleteProperties":{}}} |
      | bind-representation       | {"prepare":[],"mutation":{"BindRepresentation":{"target":{"artifactId":"nakagin-capsule-plan","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"drawing"}},"pin":{"kind":"checkpoint","id":"v7"},"role":"2xDbYUk7X3XwXfITBwE0ij"}}} |
      | unbind-representation     | {"prepare":[],"mutation":{"UnbindRepresentation":{"index":11}}} |
      | change-representation-pin | {"prepare":[],"mutation":{"ChangeRepresentationPin":{"index":0,"pin":{"kind":"checkpoint","id":"v3"}}}} |
      | add-type                  | {"prepare":[],"mutation":{"AddType":{"id":"1RSTTYP0000000000000AA","name":"Kapseltyp K","category":"IfcBuildingElementProxyType"}}} |
      | remove-type               | {"prepare":[{"AddType":{"id":"1RSTTYP0000000000000AA","name":"Kapseltyp K","category":"IfcBuildingElementProxyType"}}],"mutation":{"RemoveType":{"id":"1RSTTYP0000000000000AA"}}} |
      | rename-type               | {"prepare":[],"mutation":{"RenameType":{"id":"0IJK4xQ0PB2hjiZlMVI36G","new_name":"Schacht"}}} |
      | add-design                | {"prepare":[],"mutation":{"AddDesign":{"id":"1RSTENTWURF000000000AA","name":"Kapselgeschoss"}}} |
      | remove-design             | {"prepare":[],"mutation":{"RemoveDesign":{"id":"1o$D5QcDP68vy1YIk$DDV$"}}} |
      | edit-design               | {"prepare":[],"mutation":{"EditDesign":{"id":"1o$D5QcDP68vy1YIk$DDV$","pieces":[{"id":"0POPlhUSnC1REPvcqnensi","typeId":"0UyxhWPMj1p983Bl6$TZEY","transform":{"translation":{"x":0.0,"y":0.0,"z":0.0},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"scale":{"x":1.0,"y":1.0,"z":1.0}}},{"id":"1tZkmTaMP4R8yLkBdfebfl","typeId":"0X0Bv8cZnBrPiyH9U4$zNu","transform":{"translation":{"x":-15850.0,"y":-8100.0,"z":2735.0},"rotation":{"x":0.0,"y":0.0,"z":1.0,"w":0.0},"scale":{"x":1.0,"y":2.0,"z":1.0}}}],"connections":[{"id":"3NLh69tTrEpfV9iDbwoXYL","connectingPieceId":"1tZkmTaMP4R8yLkBdfebfl","connectingPort":"b6b3121a-252b-4ba7-ac8d-152c1d0fece6","connectedPieceId":"0POPlhUSnC1REPvcqnensi","connectedPort":"c5465220-19ba-4443-8f1d-617c832dd13c"}]}}} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                        | dir                        | fixture                                                               |
      | create-object             | 🏗️create-object            | attaches-a-second-object-child                                        |
      | delete-object             | 🪓delete-object             | detaches-the-only-object-child-and-keeps-the-model-child              |
      | create-model              | 🏛️create-model             | attaches-a-second-model-child                                         |
      | delete-model              | 💣delete-model              | detaches-the-only-model-child-and-keeps-the-object-child              |
      | create-properties         | 🏷️create-properties        | attaches-a-properties-child-to-a-kit-that-has-none                    |
      | delete-properties         | 🚫delete-properties         | detaches-the-properties-child-and-leaves-every-other-collection-alone |
      | bind-representation       | 🔗bind-representation       | binds-a-second-representation-to-an-existing-type                     |
      | unbind-representation     | ✂️unbind-representation    | unbinds-the-leading-representation-and-keeps-the-trailing-one         |
      | change-representation-pin | 📌change-representation-pin | repins-the-representation-from-head-to-a-checkpoint                   |
      | add-type                  | ➕add-type                  | appends-a-slab-type-to-the-catalogue                                  |
      | remove-type               | ➖remove-type               | removes-the-column-type-and-keeps-the-beam-type                       |
      | rename-type               | ✏️rename-type              | renames-the-beam-type-without-recategorising-it                       |
      | add-design                | 🆕add-design                | adds-an-empty-roof-design                                             |
      | remove-design             | 🗑️remove-design            | removes-the-only-design-together-with-its-pieces                      |
      | edit-design               | 🖊️edit-design              | replaces-the-designs-pieces-and-connections-in-one-step               |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both encodings of the committed furniture kit and of the real capsule tower kit
    Given the real committed kit artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🖼️assets/🗣️.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️kit/📚️examples/🪑️furniture/🖼️assets/🎒️.pack.semio
    And the real capsule tower kit local://🧪️nakagin-capsule-tower/🗣️.dsl.semio
    And its binary twin local://🎒️.pack.semio
    When each implementation parses all four files, prints both documents back and re-encodes both packs
    Then all four files are reproduced byte for byte and the two implementations agree on both kits and on the digests of what they emitted
