@capability-semio-v1-object-mutate
@oracle-semio-object-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-object
Feature: Apply every typed semio OBJECT mutation to the real committed crate object, against an independent Python implementation
  `s.stdio.semio.object` is a semio-NATIVE format: no third party reads or writes `.dsl.semio` or
  `.pack.semio`, so the second producer a differential comparison needs is a second IMPLEMENTATION.
  `🐍️component.py` beside this file is that implementation — the carrier, the DSL grammar, the pack
  frame and all nine verbs, written in Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/🔣️component.json`,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`, the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs` and the one dialect-coordinate codec
  `ArtifactRef::to_uri` in `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️component.rs`), importing nothing
  from and transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-object-python-independent` in `…/✳️object/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  The document under test is the REAL committed crate object, read where the domain keeps it through
  `asset://` and never written to: a non-identity translation with ALL THREE optional child slots
  occupied by real `brep`, `mesh` and `value` handles. It is the richest `s.stdio.semio.object`
  document committed anywhere in this artifact and the only one that exercises the child-handle codec
  three slots at a time; `asset://` resolves against the artifact root, so no other plugin's larger
  `.dsl.semio` is reachable from here, and that limit is stated rather than papered over.

  Because every slot of the real object is OCCUPIED, `create-<slot>` — which the grammar and the
  committed vectors define as ATTACHING to an empty slot — has no meaning against it as it stands.
  Each scenario's doc string therefore carries a `prepare` list applied to the real document before
  the verb under test, and for the three `create-` rows that list is the matching `delete-` verb.
  The prepared document, not the raw file, is what the inverse law is asserted against. Both
  implementations read the same list from the plan, so neither can drift from what the other read.

  The parameters are chosen against the crate object's own shape, so a plausible wrong codec fails:
  `move-object` writes a negative, non-integral translation and must leave rotation and scale
  untouched, `rotate-object` writes a quaternion with all four components non-zero, `scale-object`
  writes a non-uniform scale, each `create-<slot>` lands a DIFFERENT artifact id and a different
  child id from the one it displaced, and each `delete-<slot>` must leave the other two handles
  whole — a slot-confusing implementation cannot pass.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each kind, now applied by BOTH
  implementations and checked against the committed after-snapshot by each of them in role. Nothing
  was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law. `.dsl.semio` is a fixed-layout
  record grammar and `.pack.semio` is its binary twin, and both committed files were produced by the
  Rust codec — so an exact re-emission is the CORRECT answer here and the wave's must-differ tripwire
  would be backwards, which is why the Rust side asserts `law::carrier_is_exact`. What stops that
  being a codec agreeing with itself is that the Python side reproduces the same two files byte for
  byte from the grammar and the protocol alone, and the two sides' digests of the re-emitted bytes
  are compared.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real committed crate object
    Given the real committed object artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🖼️assets/🗣️example.dsl.semio
    When the <id> mutation is applied to the prepared object parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id                | mutation                                                                                                                                                                                                              |
      | move-object       | {"prepare":[],"mutation":{"MoveObject":{"translation":{"x":-4.25,"y":0.0,"z":7.5}}}}                                                                                                                                  |
      | rotate-object     | {"prepare":[],"mutation":{"RotateObject":{"rotation":{"x":0.5,"y":-0.5,"z":0.5,"w":0.5}}}}                                                                                                                            |
      | scale-object      | {"prepare":[],"mutation":{"ScaleObject":{"scale":{"x":2.0,"y":0.5,"z":4.0}}}}                                                                                                                                         |
      | create-brep       | {"prepare":[{"DeleteBrep":{}}],"mutation":{"CreateBrep":{"child_id":"brep-02","target":{"artifactId":"crate-brep-lod2","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"brep"}}}}}                  |
      | delete-brep       | {"prepare":[],"mutation":{"DeleteBrep":{}}}                                                                                                                                                                           |
      | create-mesh       | {"prepare":[{"DeleteMesh":{}}],"mutation":{"CreateMesh":{"child_id":"mesh-02","target":{"artifactId":"crate-mesh-lod2","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"mesh"}}}}}                  |
      | delete-mesh       | {"prepare":[],"mutation":{"DeleteMesh":{}}}                                                                                                                                                                           |
      | create-properties | {"prepare":[{"DeleteProperties":{}}],"mutation":{"CreateProperties":{"child_id":"props-02","target":{"artifactId":"crate-props-metric","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"value"}}}}} |
      | delete-properties | {"prepare":[],"mutation":{"DeleteProperties":{}}}                                                                                                                                                                     |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the prepared crate object
    Given the real committed object artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🖼️assets/🗣️example.dsl.semio
    When the <id> mutation is applied to the prepared object parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the prepared object and agree on the mutated and the restored snapshot
    Examples:
      | id                | mutation                                                                                                                                                                                                              |
      | move-object       | {"prepare":[],"mutation":{"MoveObject":{"translation":{"x":-4.25,"y":0.0,"z":7.5}}}}                                                                                                                                  |
      | rotate-object     | {"prepare":[],"mutation":{"RotateObject":{"rotation":{"x":0.5,"y":-0.5,"z":0.5,"w":0.5}}}}                                                                                                                            |
      | scale-object      | {"prepare":[],"mutation":{"ScaleObject":{"scale":{"x":2.0,"y":0.5,"z":4.0}}}}                                                                                                                                         |
      | create-brep       | {"prepare":[{"DeleteBrep":{}}],"mutation":{"CreateBrep":{"child_id":"brep-02","target":{"artifactId":"crate-brep-lod2","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"brep"}}}}}                  |
      | delete-brep       | {"prepare":[],"mutation":{"DeleteBrep":{}}}                                                                                                                                                                           |
      | create-mesh       | {"prepare":[{"DeleteMesh":{}}],"mutation":{"CreateMesh":{"child_id":"mesh-02","target":{"artifactId":"crate-mesh-lod2","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"mesh"}}}}}                  |
      | delete-mesh       | {"prepare":[],"mutation":{"DeleteMesh":{}}}                                                                                                                                                                           |
      | create-properties | {"prepare":[{"DeleteProperties":{}}],"mutation":{"CreateProperties":{"child_id":"props-02","target":{"artifactId":"crate-props-metric","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"value"}}}}} |
      | delete-properties | {"prepare":[],"mutation":{"DeleteProperties":{}}}                                                                                                                                                                     |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️component.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                | dir                  | fixture                                                       |
      | move-object       | 🚚move-object        | moves-the-object-to-a-new-translation                         |
      | rotate-object     | 🔄rotate-object      | rotates-the-object-a-half-turn-about-z                        |
      | scale-object      | 📏scale-object       | scales-the-object-non-uniformly                               |
      | create-brep       | 🧱create-brep        | attaches-a-brep-child-to-an-object-that-has-none              |
      | delete-brep       | 💥delete-brep        | detaches-the-brep-child-and-leaves-the-mesh-child-alone       |
      | create-mesh       | 🕸️create-mesh       | attaches-a-mesh-child-to-an-object-that-has-none              |
      | delete-mesh       | 🧨delete-mesh        | detaches-the-mesh-child-and-leaves-the-brep-child-alone       |
      | create-properties | 🏷️create-properties | attaches-a-properties-child-to-an-object-that-has-none        |
      | delete-properties | 🚫delete-properties  | detaches-the-properties-child-and-leaves-the-mesh-child-alone |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both committed encodings of the real crate object from the parsed document
    Given the real committed object artifact asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🖼️assets/🗣️example.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️v1/🪆️subsets/✳️object/📚️examples/📦️crate/🖼️assets/🎒️.pack.semio
    When each implementation parses the text artifact, prints it back, decodes the binary twin and re-encodes it
    Then both reproduce the two committed files byte for byte and agree on the object and on the digests of what they emitted
