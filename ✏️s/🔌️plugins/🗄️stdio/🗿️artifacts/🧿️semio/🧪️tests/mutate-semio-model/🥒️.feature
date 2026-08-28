@capability-semio-v1-model-mutate
@oracle-semio-model-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-model
Feature: Apply every typed semio MODEL mutation to the Nakagin Capsule Tower, against an independent Python implementation
  `stdio.semio.model` is a semio-NATIVE format: no third party in any ecosystem reads or writes
  `.dsl.semio` or `.pack.semio`, so the second producer a differential comparison needs is a second
  IMPLEMENTATION. `🐍️component.py` beside this file is that implementation — the envelope, the DSL
  grammar with its `spatial-kind`/`element-class`/`geometry-ref`/`pset-value`/`relation-kind`
  vocabularies, the LEB128 pack frame with its little-endian `f64` transforms, and all eleven verbs
  with their inverses, written in Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/🧬️mutations/📝️text/📖️component.grammar.semio`,
  the committed `(before, mutation, after)` vectors in this case's own `🧫️fixtures/`, and the semio
  envelope region of `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing
  from and transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-model-python-independent` in `…/✳️model/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  🏗️ **The model under test is a real building.** The richest `stdio.semio.model` document committed
  anywhere in this artifact is the two-node, one-wall demo building, which is a fixture, not a model.
  So the model this case mutates was derived ONCE — by `🐍️derive-model-fixture.py` in the ticket
  folder — from the real committed IFC 4 file `../../../🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc`,
  Kisho Kurokawa's Nakagin Capsule Tower, 2.5 MB and 24 792 entities, read with **IfcOpenShell
  0.8.4** — a genuine third-party IFC implementation, and the only reader involved in the provenance.
  Its `IfcSite`/`IfcBuilding`/`IfcBuildingStorey` became the three spatial nodes, its
  `IfcElementAssembly` and 180 `IfcBuildingElementProxy` capsules the 181 elements, their 185
  `IfcPropertySet`s and 366 `IfcPropertySingleValue`s the property sets, their `IfcAxis2Placement3D`
  frames the placements — real translations in millimetres and real orientation quaternions computed
  from the `Axis`/`RefDirection` pair — and its `IfcRelAggregates`,
  `IfcRelContainedInSpatialStructure` and `IfcRelConnectsElements` the 362 relations. The result is
  119 066 bytes of DSL and 69 388 bytes of pack, against 544 and 476 for the demo building the case
  used to rest on. IfcOpenShell reads IFC, not a semio envelope, and cannot express `set-snapshot` at
  all, which is why it is the source of the ARTIFACT and not the oracle.

  🔢️ **Three pack tags the committed example never exercised.** The protocol description stops at the
  repeated records and names only their shape, so both implementations take every enum ORDINAL from
  the order the DSL grammar declares its alternatives in. The committed demo building pins `S`, `T`,
  `WA`, the `B` brep reference, all three `pset-value` tags and `CI`. The capsule tower additionally
  carries `OT` element classes, `M` mesh references and elements with NO `spatialId` — three tags no
  committed pack had ever carried. They are derived from the grammar's declared order alone, and
  `identity-round-trip` is where a disagreement about them would surface, in red, rather than
  silently.

  ⚖️ **A property of the vocabulary the parameters have to respect.** `insert-spatial-node`,
  `insert-element` and `insert-relation` carry no index and append, so the undo of a removal can only
  put the record back at the END of its collection: removing a member that is not the last one is not
  invertible in this vocabulary at all. `remove-spatial-node`, `remove-element` and `remove-relation`
  therefore address the last member of their collection, exactly as the committed vectors do. This is
  stated rather than worked around; the same constraint binds both implementations.

  🧮 **`set-snapshot` names its spatial nodes in a DIFFERENT order from the tower's, on purpose.**
  The replacement snapshot drops all 181 elements and all 362 relations, drops the building level, and
  lists the storey BEFORE the site — two survivors whose relative order is the reverse of the base's.
  A sparse keyed diff that retains survivors where they stand and appends newcomers cannot express
  that, and this parameter is what caught it: the subject answered `[site, storey]` where the named
  snapshot says `[storey, site]`, so applying a snapshot did not make the document equal to that
  snapshot. Fixed at the cause in `…/✳️model/🧬️schema/🔺️diff/🦀️component.rs`, whose `between_named` now
  degrades to a full replacement whenever the sparse triple cannot reproduce the target's key
  sequence — the same guard `✳️flow` carries for the same defect. The parameter stays, so a
  regression is red again.

  The remaining parameters are chosen against the model's own shape, so a plausible wrong codec
  fails: `set-spatial-node` renames the storey and lifts it to a fractional elevation while leaving
  its `kind` and `parentId` untouched — the tri-state slots the committed vectors spell as `null` —
  `set-element` rewrites the assembly's class, geometry, containing storey and property sets at once,
  and `set-relation` retags one relation to `fillsVoid`, a kind neither the demo building nor any
  committed vector uses.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed
  `(before, mutation, after)` vector for each kind, whose before-snapshot is the real demo building
  artifact decoded, now applied AND undone by BOTH implementations and checked against the committed
  after- and before-snapshots by each of them in role. Nothing was removed to make room for the
  oracle.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions. `.dsl.semio`
  is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an exact re-emission is
  the CORRECT answer here and the wave's must-differ tripwire would be backwards, which is why the
  Rust side asserts `law::carrier_is_exact`. What stops that being a codec agreeing with itself is
  that the demo building's two encodings were written by the RUST codec and the Python side
  reproduces them byte for byte from the grammar alone, while the capsule tower's two encodings were
  written by the PYTHON implementation and the Rust codec has to reproduce THOSE — 1 840 `f64`
  transform components among them.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real Nakagin Capsule Tower model
    Given the real capsule tower model local://🏗️nakagin-capsule-tower.dsl.semio
    When the <id> mutation is applied to the model parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting model
    Examples:
      | id                  | mutation |
      | no-mutation         | {"mutation":"noMutation"} |
      | set-snapshot        | {"mutation":"setSnapshot","snapshot":{"schema":"stdio.semio.model","spatial":[{"id":"25h1tviqb5o97WsO89tzwZ","kind":"storey","name":"Kapselgeschoss","parentId":"3hePCnUzPDnQxT0FznTQjx","placement":{"translation":{"x":0.0,"y":0.0,"z":8616.67},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"scale":{"x":1.0,"y":1.0,"z":1.0}}},{"id":"3hePCnUzPDnQxT0FznTQjx","kind":"site","name":"Ginza 8-chome","parentId":null,"placement":{"translation":{"x":0.0,"y":0.0,"z":0.0},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"scale":{"x":1.0,"y":1.0,"z":1.0}}}],"elements":[],"relations":[]}} |
      | insert-spatial-node | {"mutation":"insertSpatialNode","node":{"id":"1RSTRAUM000000000000AA","kind":"space","name":"Kapsel A1101","parentId":"25h1tviqb5o97WsO89tzwZ","placement":{"translation":{"x":-2650.0,"y":-6350.0,"z":42283.33},"rotation":{"x":0.0,"y":0.0,"z":0.707107,"w":0.707107},"scale":{"x":1.0,"y":1.0,"z":1.0}}}} |
      | remove-spatial-node | {"mutation":"removeSpatialNode","id":"25h1tviqb5o97WsO89tzwZ"} |
      | set-spatial-node    | {"mutation":"setSpatialNode","id":"25h1tviqb5o97WsO89tzwZ","kind":null,"name":"Kapselgeschoss","placement":{"translation":{"x":0.0,"y":0.0,"z":8616.67},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"scale":{"x":1.0,"y":1.0,"z":1.0}}} |
      | insert-element      | {"mutation":"insertElement","element":{"id":"1RSTKAPSEL0000000000AA","class":{"kind":"other","name":"IfcBuildingElementProxy"},"placement":{"translation":{"x":-2650.0,"y":-6350.0,"z":42283.33},"rotation":{"x":0.0,"y":0.0,"z":0.707107,"w":0.707107},"scale":{"x":1.0,"y":1.0,"z":1.0}},"geometry":{"kind":"none"},"spatialId":null,"psets":[{"name":"ComposePieceAttributes","properties":[{"key":"name","value":{"kind":"text","value":"Ersatzkapsel A1101"}},{"key":"rotation","value":{"kind":"number","value":90.0}},{"key":"isReplacement","value":{"kind":"boolean","value":true}}]}]}} |
      | remove-element      | {"mutation":"removeElement","id":"1tZkmTaMP4R8yLkBdfebfl"} |
      | set-element         | {"mutation":"setElement","id":"1o$D5QcDP68vy1YIk$DDV$","class":{"kind":"column"},"placement":null,"geometry":{"kind":"brep","brep_id":"#25275"},"spatial_id":"2qJLCLo_vCt8SJs9zMCdFg","psets":[{"name":"Pset_ElementAssemblyCommon","properties":[{"key":"Reference","value":{"kind":"text","value":"Nakagin"}},{"key":"IsExternal","value":{"kind":"boolean","value":true}}]}]} |
      | insert-relation     | {"mutation":"insertRelation","relation":{"id":"1RSTRELATION00000000AA","kind":{"kind":"voidsElement"},"from":"1tZkmTaMP4R8yLkBdfebfl","to":"1o$D5QcDP68vy1YIk$DDV$"}} |
      | remove-relation     | {"mutation":"removeRelation","id":"2mdiribe9DXeLAu6hdMs_9"} |
      | set-relation        | {"mutation":"setRelation","id":"1RxL5nFpL4dwsNtKgSpmM_-0","kind":{"kind":"fillsVoid"},"from":null,"to":null} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real Nakagin Capsule Tower model
    Given the real capsule tower model local://🏗️nakagin-capsule-tower.dsl.semio
    When the <id> mutation is applied to the model parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the capsule tower and agree on the mutated and the restored model
    Examples:
      | id                  | mutation |
      | no-mutation         | {"mutation":"noMutation"} |
      | set-snapshot        | {"mutation":"setSnapshot","snapshot":{"schema":"stdio.semio.model","spatial":[{"id":"25h1tviqb5o97WsO89tzwZ","kind":"storey","name":"Kapselgeschoss","parentId":"3hePCnUzPDnQxT0FznTQjx","placement":{"translation":{"x":0.0,"y":0.0,"z":8616.67},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"scale":{"x":1.0,"y":1.0,"z":1.0}}},{"id":"3hePCnUzPDnQxT0FznTQjx","kind":"site","name":"Ginza 8-chome","parentId":null,"placement":{"translation":{"x":0.0,"y":0.0,"z":0.0},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"scale":{"x":1.0,"y":1.0,"z":1.0}}}],"elements":[],"relations":[]}} |
      | insert-spatial-node | {"mutation":"insertSpatialNode","node":{"id":"1RSTRAUM000000000000AA","kind":"space","name":"Kapsel A1101","parentId":"25h1tviqb5o97WsO89tzwZ","placement":{"translation":{"x":-2650.0,"y":-6350.0,"z":42283.33},"rotation":{"x":0.0,"y":0.0,"z":0.707107,"w":0.707107},"scale":{"x":1.0,"y":1.0,"z":1.0}}}} |
      | remove-spatial-node | {"mutation":"removeSpatialNode","id":"25h1tviqb5o97WsO89tzwZ"} |
      | set-spatial-node    | {"mutation":"setSpatialNode","id":"25h1tviqb5o97WsO89tzwZ","kind":null,"name":"Kapselgeschoss","placement":{"translation":{"x":0.0,"y":0.0,"z":8616.67},"rotation":{"x":0.0,"y":0.0,"z":0.0,"w":1.0},"scale":{"x":1.0,"y":1.0,"z":1.0}}} |
      | insert-element      | {"mutation":"insertElement","element":{"id":"1RSTKAPSEL0000000000AA","class":{"kind":"other","name":"IfcBuildingElementProxy"},"placement":{"translation":{"x":-2650.0,"y":-6350.0,"z":42283.33},"rotation":{"x":0.0,"y":0.0,"z":0.707107,"w":0.707107},"scale":{"x":1.0,"y":1.0,"z":1.0}},"geometry":{"kind":"none"},"spatialId":null,"psets":[{"name":"ComposePieceAttributes","properties":[{"key":"name","value":{"kind":"text","value":"Ersatzkapsel A1101"}},{"key":"rotation","value":{"kind":"number","value":90.0}},{"key":"isReplacement","value":{"kind":"boolean","value":true}}]}]}} |
      | remove-element      | {"mutation":"removeElement","id":"1tZkmTaMP4R8yLkBdfebfl"} |
      | set-element         | {"mutation":"setElement","id":"1o$D5QcDP68vy1YIk$DDV$","class":{"kind":"column"},"placement":null,"geometry":{"kind":"brep","brep_id":"#25275"},"spatial_id":"2qJLCLo_vCt8SJs9zMCdFg","psets":[{"name":"Pset_ElementAssemblyCommon","properties":[{"key":"Reference","value":{"kind":"text","value":"Nakagin"}},{"key":"IsExternal","value":{"kind":"boolean","value":true}}]}]} |
      | insert-relation     | {"mutation":"insertRelation","relation":{"id":"1RSTRELATION00000000AA","kind":{"kind":"voidsElement"},"from":"1tZkmTaMP4R8yLkBdfebfl","to":"1o$D5QcDP68vy1YIk$DDV$"}} |
      | remove-relation     | {"mutation":"removeRelation","id":"2mdiribe9DXeLAu6hdMs_9"} |
      | set-relation        | {"mutation":"setRelation","id":"1RxL5nFpL4dwsNtKgSpmM_-0","kind":{"kind":"fillsVoid"},"from":null,"to":null} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply and undo <id> on its committed specification vector over the real demo building
    Given the committed specification fixtures for the <id> kind
      | role     | fixture                     |
      | before   | local://<id>/⬅️before.json   |
      | mutation | local://<id>/🦠️mutation.json |
      | after    | local://<id>/➡️after.json    |
    When both implementations apply the committed mutation to the committed before-snapshot and undo it again
    Then each reaches the committed after-snapshot, each returns to the before-snapshot, and the two agree
    Examples:
      | id                  |
      | no-mutation         |
      | set-snapshot        |
      | insert-spatial-node |
      | remove-spatial-node |
      | set-spatial-node    |
      | insert-element      |
      | remove-element      |
      | set-element         |
      | insert-relation     |
      | remove-relation     |
      | set-relation        |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both encodings of the demo building and of the real capsule tower from the parsed documents
    Given the real committed text artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🎒️example.pack.semio
    And the real capsule tower model local://🏗️nakagin-capsule-tower.dsl.semio
    And its binary twin local://🏗️nakagin-capsule-tower.pack.semio
    When each implementation parses all four files, prints the two documents back and re-encodes both packs
    Then all four files are reproduced byte for byte and the two implementations agree on the documents and on the digests of what they emitted
