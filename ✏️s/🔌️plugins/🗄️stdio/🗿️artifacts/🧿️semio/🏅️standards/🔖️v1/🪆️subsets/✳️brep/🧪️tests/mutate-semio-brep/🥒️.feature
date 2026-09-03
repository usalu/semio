@capability-semio-v1-brep-mutate
@oracle-semio-brep-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-brep
Feature: Apply every typed semio BREP mutation to the real concrete-forest structure, against an independent Python implementation
  `s.stdio.semio.brep` is a semio-NATIVE format: no third party reads or writes `.dsl.semio` or
  `.pack.semio`, so the second producer a differential comparison needs is a second IMPLEMENTATION.
  `🐍️component.py` beside this file is that implementation — the carrier, the DSL grammar, the pack
  frame, both tagged geometry unions and all thirteen verbs, written in Python from the committed
  specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio`, `…/📸️snapshot/🔣️.json`,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` and the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`), importing nothing from and
  transliterating nothing of the Rust it judges. It is registered as the oracle
  `semio-brep-python-independent` in `…/✳️brep/🧪️oracle/🔣️.json`; the recorded no-oracle
  decision it replaces is gone, because there is now a reference to compare against.

  🌲️ **The solid under test is a real building structure.** The richest `stdio.semio.brep` document
  committed anywhere in this artifact is the three-vertex demo solid under `✳️base`'s example set —
  537 bytes, which is a fixture, not a solid. So the document every mutation row below runs on was
  derived ONCE — by `🐍️derive-brep-fixture.py` in the ticket folder — from the real committed
  `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/📐️hexagonal-cut-concrete-forest-left-bim.stp`, the real
  Rhino 8.31 / ST-Developer v19.2 BIM export of the "hexagonal cut concrete forest" structure and
  the richest B-rep committed anywhere in this repository. Its 167 `VERTEX_POINT` became the 167
  vertices, its 270 `EDGE_CURVE` — every one of them carrying a real `B_SPLINE_CURVE_WITH_KNOTS` —
  the 270 NURBS edges with their real control points and their real knot vectors expanded through
  the source's own multiplicities, its 127 `EDGE_LOOP` the 127 loops with the real orientation flag
  of every `ORIENTED_EDGE`, its 127 `ADVANCED_FACE` the 127 faces with their real `PLANE` normals,
  and its 12 `CLOSED_SHELL`/`MANIFOLD_SOLID_BREP` pairs the 12 shells and 12 solids. Every semio id
  carries the STEP entity number it came from (`v1666` is `#1666=VERTEX_POINT`), so any record here
  can be looked up in the source file. The result is 90 063 bytes of DSL and 56 627 bytes of pack.
  The reader in that script is a purpose-written ISO 10303-21 Part 21 reader, not a semio codec: it
  cannot express a single one of the thirteen verbs, which is why it is the source of the ARTIFACT
  and never the oracle.

  🔢️ **A lexeme class no committed artifact had ever carried.** `number = INT | FLOAT` has no
  exponent part in either alternative, and the real export carries 98 magnitudes — real Rhino
  residues down to 1e-18 in plane normals and B-spline control points — whose shortest
  round-tripping digit string needs one. Both implementations therefore have to write those
  positionally, placing the shortest identifying digits against the decimal point. That is derived
  from the grammar's own lexeme set alone, and `identity-round-trip` is where a disagreement about
  it would surface, in red, rather than silently.

  ⚖️ **A property of the vocabulary the parameters have to respect.** Every `create-` verb APPENDS —
  none of them carries an index — so the undo of a removal can only put the record back at the END
  of its collection: removing a member that is not the last one is not invertible in this vocabulary
  at all. `delete-edge`, `delete-face`, `delete-shell` and `delete-solid` therefore address `e1229`,
  `f165`, `s38` and `so25`, the last member of each collection, exactly as the committed vectors do.

  ↩️ **Why the two `delete-vertex` rows differ, stated rather than hidden.** `delete-vertex` is the
  only cascading verb: it severs every edge incident on the vertex it removes, and its inverse has
  to restore the vertex AND those edges in their original positions. The real structure's last
  vertex `v1666` is incident on `e1192`, `e1193` and `e1195` — edges 232, 233 and 235 of 270, not a
  suffix — so a real three-edge cascade is exactly what `mutate-delete-vertex` runs, and it is not
  order-restorable. `inverse-delete-vertex` therefore prepares its own trailing vertex and its own
  two trailing edges from real coordinates of this same structure (`v1500`, `v1501` and face `f165`'s
  real plane origin) and cascades over those, so the undo is measured where the vocabulary can
  express it. Neither row was weakened to agree with the other.

  🎯️ **Reaching the arms the structure does not carry.** The real export is B-spline edges on planar
  faces throughout — two of the ten tagged geometry arms. The parameters cover the rest and are
  chosen against this structure's own coordinates: `create-edge` builds an ELLIPSE about `f165`'s
  real plane origin, `create-face` a TORUS over the real loop `l419`, `replace-curve` swaps the real
  B-spline of `e960` for a CIRCLE through the real vertex it starts at, and `replace-surface` is
  prepared with a CONE on `f39` and a bilinear NURBS patch over four real vertices on `f40` before
  replacing `f39`'s cone with a SPHERE — three arms in one row. Together with the committed
  specification vectors, whose `replace-curve` carries a LINE and whose `replace-surface` carries a
  CYLINDER, every one of the four curve arms and all six surface arms is exercised by both
  implementations.

  A recorded gap: the committed `delete-vertex` vector runs against a brep with NO loops, so neither
  the grammar nor any vector says whether the cascade also purges loop entries naming a severed edge.
  The real structure here carries 127 such loops, so this feature measures the two implementations
  against each other in a place the specification is silent; a divergence would be a finding about
  the specification, not about either codec.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each kind, now applied by BOTH
  implementations and checked against the committed after-snapshot by each of them in role. Nothing
  was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law, in both directions, over FOUR
  files. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin, so an
  exact re-emission is the CORRECT answer here and the wave's must-differ tripwire would be
  backwards, which is why the Rust side asserts `law::carrier_is_exact`. The tiny committed solid's
  two encodings were written by the RUST codec and the Python side reproduces them byte for byte
  from the grammar alone — it is kept for exactly that reason, and nothing it proved was given up —
  while the concrete forest's two encodings were written by the PYTHON implementation and the Rust
  codec has to reproduce THOSE, 2 466 real `f64` among them.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real concrete-forest structure
    Given the real concrete-forest solid local://🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio
    When the <id> mutation is applied to the prepared solid parsed from it
      """
      <mutation>
      """
    Then the independent implementation and the subject agree on the resulting snapshot
    Examples:
      | id              | mutation |
      | create-vertex   | {"prepare":[],"mutation":{"CreateVertex":{"id":"vF01","point":{"x":10.8000102824824,"y":4.67653718043597,"z":2.735}}}} |
      | delete-vertex   | {"prepare":[],"mutation":{"DeleteVertex":{"id":"v1666"}}} |
      | create-edge     | {"prepare":[],"mutation":{"CreateEdge":{"id":"eF01","start_vertex":"v1500","end_vertex":"v1666","curve":{"kind":"ellipse","center":{"x":10.8000102824824,"y":4.67653718043597,"z":2.735},"axis":{"x":0,"y":0,"z":1},"radiusMajor":5.4,"radiusMinor":2.7}}}} |
      | delete-edge     | {"prepare":[],"mutation":{"DeleteEdge":{"id":"e1229"}}} |
      | create-face     | {"prepare":[],"mutation":{"CreateFace":{"id":"fF01","outer_loop":"l419","inner_loops":[],"surface":{"kind":"torus","center":{"x":10.8000102824824,"y":4.67653718043597,"z":2.735},"axis":{"x":0,"y":0,"z":1},"majorRadius":5.4,"minorRadius":1.35},"orientation":true}}} |
      | delete-face     | {"prepare":[],"mutation":{"DeleteFace":{"id":"f165"}}} |
      | create-shell    | {"prepare":[],"mutation":{"CreateShell":{"id":"sF01","faces":[{"face":"f165","orientation":false}]}}} |
      | delete-shell    | {"prepare":[],"mutation":{"DeleteShell":{"id":"s38"}}} |
      | create-solid    | {"prepare":[],"mutation":{"CreateSolid":{"id":"soF01","shells":[{"shell":"s38","isVoid":true}]}}} |
      | delete-solid    | {"prepare":[],"mutation":{"DeleteSolid":{"id":"so25"}}} |
      | replace-curve   | {"prepare":[],"mutation":{"ReplaceCurve":{"edge_id":"e960","new_curve":{"kind":"circle","center":{"x":2.7,"y":4.67653718043597,"z":2.735},"axis":{"x":0,"y":0,"z":1},"radius":2.7}}}} |
      | replace-surface | {"prepare":[{"ReplaceSurface":{"face_id":"f39","new_surface":{"kind":"cone","origin":{"x":18.9000205649649,"y":4.67653718043597,"z":2.735},"axis":{"x":0,"y":0,"z":1},"radius":5.4,"halfAngle":0.5}}},{"ReplaceSurface":{"face_id":"f40","new_surface":{"kind":"nurbs","controlPoints":[{"x":0.0,"y":8.88178419700125e-16,"z":2.735},{"x":8.10001028248245,"y":4.44089209850063e-16,"z":2.735},{"x":2.7,"y":4.67653718043597,"z":2.735},{"x":10.8000102824825,"y":4.67653718043597,"z":2.735}],"weights":[1,1,1,1],"uCount":2,"vCount":2,"degreeU":1,"degreeV":1,"knotsU":[0,0,1,1],"knotsV":[0,0,1,1]}}}],"mutation":{"ReplaceSurface":{"face_id":"f39","new_surface":{"kind":"sphere","center":{"x":18.9000205649649,"y":4.67653718043597,"z":2.735},"radius":5.4}}}} |
      | move-vertex     | {"prepare":[],"mutation":{"MoveVertex":{"vertex_id":"v1500","new_point":{"x":8.35980762113533,"y":2.18826859021799,"z":2.735}}}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the prepared concrete-forest structure
    Given the real concrete-forest solid local://🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio
    When the <id> mutation is applied to the prepared solid parsed from it and each side undoes it with its own computed inverse
      """
      <mutation>
      """
    Then both sides restore the prepared solid and agree on the mutated and the restored snapshot
    Examples:
      | id              | mutation |
      | create-vertex   | {"prepare":[],"mutation":{"CreateVertex":{"id":"vF01","point":{"x":10.8000102824824,"y":4.67653718043597,"z":2.735}}}} |
      | delete-vertex   | {"prepare":[{"CreateVertex":{"id":"vF02","point":{"x":10.8000102824824,"y":4.67653718043597,"z":2.735}}},{"CreateEdge":{"id":"eF02","start_vertex":"vF02","end_vertex":"v1500","curve":{"kind":"nurbs","controlPoints":[{"x":10.8000102824824,"y":4.67653718043597,"z":2.735},{"x":2.7,"y":4.67653718043597,"z":2.735}],"weights":[1,1],"degree":1,"knots":[0,0,8.1,8.1]}}},{"CreateEdge":{"id":"eF03","start_vertex":"v1501","end_vertex":"vF02","curve":{"kind":"nurbs","controlPoints":[{"x":0.0,"y":8.88178419700125e-16,"z":2.735},{"x":10.8000102824824,"y":4.67653718043597,"z":2.735}],"weights":[1,1],"degree":1,"knots":[0,0,11.77,11.77]}}}],"mutation":{"DeleteVertex":{"id":"vF02"}}} |
      | create-edge     | {"prepare":[],"mutation":{"CreateEdge":{"id":"eF01","start_vertex":"v1500","end_vertex":"v1666","curve":{"kind":"ellipse","center":{"x":10.8000102824824,"y":4.67653718043597,"z":2.735},"axis":{"x":0,"y":0,"z":1},"radiusMajor":5.4,"radiusMinor":2.7}}}} |
      | delete-edge     | {"prepare":[],"mutation":{"DeleteEdge":{"id":"e1229"}}} |
      | create-face     | {"prepare":[],"mutation":{"CreateFace":{"id":"fF01","outer_loop":"l419","inner_loops":[],"surface":{"kind":"torus","center":{"x":10.8000102824824,"y":4.67653718043597,"z":2.735},"axis":{"x":0,"y":0,"z":1},"majorRadius":5.4,"minorRadius":1.35},"orientation":true}}} |
      | delete-face     | {"prepare":[],"mutation":{"DeleteFace":{"id":"f165"}}} |
      | create-shell    | {"prepare":[],"mutation":{"CreateShell":{"id":"sF01","faces":[{"face":"f165","orientation":false}]}}} |
      | delete-shell    | {"prepare":[],"mutation":{"DeleteShell":{"id":"s38"}}} |
      | create-solid    | {"prepare":[],"mutation":{"CreateSolid":{"id":"soF01","shells":[{"shell":"s38","isVoid":true}]}}} |
      | delete-solid    | {"prepare":[],"mutation":{"DeleteSolid":{"id":"so25"}}} |
      | replace-curve   | {"prepare":[],"mutation":{"ReplaceCurve":{"edge_id":"e960","new_curve":{"kind":"circle","center":{"x":2.7,"y":4.67653718043597,"z":2.735},"axis":{"x":0,"y":0,"z":1},"radius":2.7}}}} |
      | replace-surface | {"prepare":[{"ReplaceSurface":{"face_id":"f39","new_surface":{"kind":"cone","origin":{"x":18.9000205649649,"y":4.67653718043597,"z":2.735},"axis":{"x":0,"y":0,"z":1},"radius":5.4,"halfAngle":0.5}}},{"ReplaceSurface":{"face_id":"f40","new_surface":{"kind":"nurbs","controlPoints":[{"x":0.0,"y":8.88178419700125e-16,"z":2.735},{"x":8.10001028248245,"y":4.44089209850063e-16,"z":2.735},{"x":2.7,"y":4.67653718043597,"z":2.735},{"x":10.8000102824825,"y":4.67653718043597,"z":2.735}],"weights":[1,1,1,1],"uCount":2,"vCount":2,"degreeU":1,"degreeV":1,"knotsU":[0,0,1,1],"knotsV":[0,0,1,1]}}}],"mutation":{"ReplaceSurface":{"face_id":"f39","new_surface":{"kind":"sphere","center":{"x":18.9000205649649,"y":4.67653718043597,"z":2.735},"radius":5.4}}}} |
      | move-vertex     | {"prepare":[],"mutation":{"MoveVertex":{"vertex_id":"v1500","new_point":{"x":8.35980762113533,"y":2.18826859021799,"z":2.735}}}} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id              | dir               | fixture                                                          |
      | create-vertex   | 🏗️create-vertex   | adds-an-apex-vertex-above-the-square                             |
      | delete-vertex   | 🗑️delete-vertex   | removes-a-corner-vertex-and-cascades-into-its-two-incident-edges |
      | create-edge     | 🔗create-edge      | adds-a-diagonal-edge-across-the-square                           |
      | delete-edge     | ✂️delete-edge     | removes-the-closing-edge-and-keeps-its-two-vertices              |
      | create-face     | 🔷create-face      | adds-an-opposing-face-over-the-same-loop                         |
      | delete-face     | 🚮delete-face      | removes-the-only-face-and-leaves-its-loop-behind                 |
      | create-shell    | 🐚create-shell     | adds-a-second-shell-that-reuses-the-face-with-flipped-sense      |
      | delete-shell    | 💥delete-shell     | removes-the-only-shell-and-leaves-its-faces-behind               |
      | create-solid    | 🧊create-solid     | adds-a-second-solid-that-treats-the-shell-as-a-void              |
      | delete-solid    | 🕳️delete-solid    | removes-the-only-solid-and-leaves-its-shell-behind               |
      | replace-curve   | ➰replace-curve    | swaps-the-first-edges-line-for-a-circular-arc                    |
      | replace-surface | 🗺️replace-surface | swaps-the-faces-plane-for-a-cylinder                             |
      | move-vertex     | 📍move-vertex      | lifts-the-third-corner-off-the-base-plane                        |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both encodings of the committed solid and of the real concrete-forest structure
    Given the real committed brep artifact asset://📚️examples/🧊️solid/🖼️assets/🗣️.dsl.semio
    And its committed binary twin asset://📚️examples/🧊️solid/🖼️assets/🎒️.pack.semio
    And the real concrete-forest solid local://🧪️hexagonal-cut-concrete-forest-left/🗣️.dsl.semio
    And its binary twin local://🎒️.pack.semio
    When each implementation parses all four files, prints both documents back and re-encodes both packs
    Then all four files are reproduced byte for byte and the two implementations agree on both documents and on the digests of what they emitted
