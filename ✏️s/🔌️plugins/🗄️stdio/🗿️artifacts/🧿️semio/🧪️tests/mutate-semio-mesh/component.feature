@capability-semio-v1-mesh-mutate
@oracle-semio-mesh-typescript-three-independent
@comparison-ordered-json-v1
@mutations-semio-v1-mesh
Feature: Apply every typed semio MESH mutation to a real architectural model, against three.js and an independent TypeScript implementation
  Two independent producers answer this case, and each answers the half it genuinely speaks.

  **three.js r185 is a real third party and it speaks the GEOMETRY.** Every scenario that produces
  primitives builds a real `THREE.BufferGeometry` from them — positions in a `Float64Array`, so
  nothing is quantised on the way in — and three.js states what it is: the attribute counts, the
  bounding box `Box3` computes from the position attribute, and the flat vertex stream
  `BufferGeometry.toNonIndexed()` produces by walking the index buffer. The Rust subject computes
  those same facts from the same primitives by hand, so a projection matches only when a 3D engine
  that has never seen this repository and this repository's own codec agree about the actual mesh —
  a permuted index buffer or a lost vertex shows up in the expanded stream immediately. What three.js
  does NOT do is read `.dsl.semio` or hold an opinion about a mutation verb, and that boundary is
  named here rather than blurred.

  **`🟦️component.ts` beside this file is the second IMPLEMENTATION, for the half no third party
  speaks.** The `stdio.semio.mesh` carrier and its seventeen verbs are semio's own, so the second
  producer is a second implementation, written in TypeScript from the committed specification
  documents alone: the DSL body from
  `../../🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`, the
  pack frame from `…/📸️snapshot/💾️binary/📡️component.protocol.semio` and its Kaitai mirror — which
  names the fields inside the opaque tail, though not their order, so the order was derived from the
  committed `🧊️cube` bytes against their readable DSL twin — the envelope from
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs`, and the verbs from
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` with the committed proto and JSON schema
  mirrors. It imports nothing from and transliterates nothing of the Rust it judges, and it was
  pinned before use: it reproduces the committed `🧊️cube` artifact byte for byte in BOTH encodings
  and reaches all seventeen committed after-snapshots. It is registered as the oracle
  `semio-mesh-typescript-three-independent`; the recorded no-oracle decision it replaces is gone.

  **The model under test is a real one, and its provenance is written down.**
  `local://🗣️artifact.dsl.semio` and its binary twin were derived ONCE from the real committed glTF
  binary `🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism/🖼️assets/🧊️base.glb`
  — a real architectural model from the Metabolism study: 271 MESHES, 459 PRIMITIVES, 1 544 vertices,
  2 184 indices, two draw modes (`LINE_STRIP` and `TRIANGLES`) and two PBR materials. The GLB was
  walked by hand — header, JSON chunk, BIN chunk, accessors read straight out of their buffer views —
  by neither a glTF library nor this repository's own gltf bridge. Every coordinate, normal, texture
  coordinate, index, draw mode and PBR factor is the file's own; ids are the one thing the source does
  not carry, because every mesh, primitive and material in that GLB is anonymous, so they are derived
  from the source's own indices and that is said rather than dressed up as data. One addition is
  stated as one: the GLB embeds no image at all, so the four texture verbs would have had nothing to
  address, and a single texture carries the real committed `🖼️marker-left.png`'s own 666 bytes. That
  is 188 746 bytes of DSL against the committed `🧊️cube`'s 340, and it is what puts the pack frame's
  varint counts past one LEB128 byte. `asset://` cannot leave this artifact's root, which is why the
  derived model is committed here as a case fixture rather than borrowed in place; the derivation
  script and its provenance note live in the ticket folder.

  The parameters are chosen against the model's own shape, so a plausible wrong codec fails:
  `delete-mesh` removes a MIDDLE mesh out of 271, so restoring it means restoring its position and
  not merely its presence in an append-only pool; `delete-material` removes the LEADING of two, the
  same shape the committed vector uses; `set-primitive-topology` retags a real `LINE_STRIP` primitive
  as `lines`, which changes nothing but the tag, so a codec that re-derived topology from the index
  count fails; `replace-primitive-geometry` swaps a real triangle's buffers for a longer set built
  from two real primitives, so every parallel array has to change length together and the index
  buffer with them; `move-vertex` lifts the LAST vertex of a real primitive, leaving the parallel
  normal and uv arrays it does not address untouched; and `change-texture-mime` retags without
  touching the bytes while `replace-texture-bytes` swaps the payload without retagging, so an
  implementation that conflated the two fails.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each of the seventeen kinds,
  applied now by BOTH implementations and checked against the committed after-snapshot by each of
  them in role. Nothing was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law. `.dsl.semio` is a fixed-layout
  record grammar and `.pack.semio` is its binary twin, so reproducing both committed files byte for
  byte is the CORRECT answer here and the wave's must-differ tripwire would be backwards — which is
  why the Rust side asserts `law::carrier_is_exact`. What stops that being a codec agreeing with
  itself is that the two files were WRITTEN by the TypeScript implementation from the grammar alone,
  in another language, and the two sides' digests of the re-emitted bytes are compared.

  🔴 **`identity-round-trip` is RED, and it is left red: the DSL text carrier is not byte-stable
  across implementations.** All 51 other scenarios agree — every `mutate-`, every `inverse-` and every
  `spec-vector-`, so the two implementations agree completely about the MODEL and about what each verb
  does to it. What they do not agree on is one character of one coordinate. The real model carries the
  vertex `20.170669555664062`; the Rust printer emits `20.170669555664063` for the same double. Both
  are seventeen significant digits, no shorter decimal round-trips (fifteen and sixteen both fail),
  and both parse back to the IDENTICAL bit pattern `00000000b12b3440` — verified, not assumed. Python
  and JavaScript both spell it `…062`; Rust's `f64` `Display` is the outlier on the last digit's
  tie-break.

  That is a finding about the FORMAT, not about either codec's semantics: the `number` production is
  `INT | FLOAT` and names no canonical spelling, so "re-printing the document reproduces the file byte
  for byte" is a property of one language's float printer rather than of `.dsl.semio`. The pack twin
  has no such problem and passes — it moves the `f64` bit pattern, not a decimal. Making the
  TypeScript writer imitate Rust's tie-break would hide exactly the ambiguity this case just found, so
  it is not done; the fix belongs in the grammar, which needs to name a canonical `FLOAT` spelling.

  ⚠️ One honest limit. `SemioRgba`'s four channels and a material's `metallic`/`roughness` are
  SINGLE precision, and the reference's JSON wire form spells such a leaf with the shortest decimal
  that round-trips as an `f32` while a JavaScript number would print the widened double. The
  TypeScript side therefore routes every single-precision leaf through the same shortest-`f32`
  printer its DSL writer uses before projecting it — that is emulating the format's own wire form,
  not a tolerance — and the colour parameters below are dyadic so the two spellings coincide anyway.
  The DSL and pack carriers are unaffected: both move the exact `f32` bit pattern.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived model
    Given the real derived mesh artifact local://🗣️artifact.dsl.semio
    And the committed mutation payload local://🦠️<id>.json
    When the <id> mutation is applied to the model parsed from it
    Then the independent implementation and the subject agree on the resulting snapshot and on what three.js says its primitives are
    Examples:
      | id                         |
      | create-mesh                |
      | delete-mesh                |
      | create-primitive           |
      | delete-primitive           |
      | set-primitive-topology     |
      | replace-primitive-geometry |
      | set-primitive-material     |
      | create-material            |
      | delete-material            |
      | change-material-base-color |
      | change-material-metallic   |
      | change-material-roughness  |
      | create-texture             |
      | delete-texture             |
      | change-texture-mime        |
      | replace-texture-bytes      |
      | move-vertex                |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real derived model
    Given the real derived mesh artifact local://🗣️artifact.dsl.semio
    And the committed mutation payload local://🦠️<id>.json
    When the <id> mutation is applied to the model parsed from it and each side undoes it with its own computed inverse
    Then both sides restore the model and agree on the mutated and the restored snapshot
    Examples:
      | id                         |
      | create-mesh                |
      | delete-mesh                |
      | create-primitive           |
      | delete-primitive           |
      | set-primitive-topology     |
      | replace-primitive-geometry |
      | set-primitive-material     |
      | create-material            |
      | delete-material            |
      | change-material-base-color |
      | change-material-metallic   |
      | change-material-roughness  |
      | create-texture             |
      | delete-texture             |
      | change-texture-mime        |
      | replace-texture-bytes      |
      | move-vertex                |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/➡️after/🔣️component.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                         | dir                         | slug                                                  |
      | create-mesh                | 🕸️create-mesh                | adds-an-empty-second-mesh-at-the-end                  |
      | delete-mesh                | 🗑️delete-mesh               | removes-the-leading-mesh-and-keeps-the-trailing-one   |
      | create-primitive           | 🔺create-primitive           | adds-a-second-primitive-inside-the-existing-mesh      |
      | delete-primitive           | ✂️delete-primitive          | removes-the-leading-primitive-and-keeps-the-trailing-one |
      | set-primitive-topology     | 🔀set-primitive-topology     | switches-the-primitive-to-a-triangle-strip            |
      | replace-primitive-geometry | 📐replace-primitive-geometry | swaps-the-triangle-for-a-textured-quad                |
      | set-primitive-material     | 🔗set-primitive-material     | binds-the-primitive-to-the-existing-material          |
      | create-material            | 🎨create-material            | adds-a-second-material-at-the-end                     |
      | delete-material            | 🚮delete-material            | removes-the-leading-material-and-keeps-the-trailing-one |
      | change-material-base-color | 🌈change-material-base-color | repaints-the-material-from-red-to-blue                |
      | change-material-metallic   | ⚙️change-material-metallic  | raises-the-metallic-factor-to-fully-metallic          |
      | change-material-roughness  | 🧱change-material-roughness  | lowers-the-roughness-factor-to-a-quarter              |
      | create-texture             | 🖼️create-texture            | adds-a-second-texture-at-the-end                      |
      | delete-texture             | 🕳️delete-texture            | removes-the-leading-texture-and-keeps-the-trailing-one |
      | change-texture-mime        | 🏷️change-texture-mime       | retags-the-texture-as-jpeg-without-touching-its-bytes  |
      | replace-texture-bytes      | 📀replace-texture-bytes      | swaps-the-texture-payload-without-retagging-its-mime   |
      | move-vertex                | 📍move-vertex                | lifts-the-third-vertex-of-the-triangle                |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both committed encodings of the real derived model from the parsed document
    Given the real derived mesh artifact local://🗣️artifact.dsl.semio
    And its committed binary twin local://🎒️artifact.pack.semio
    When each implementation parses the text artifact, prints it back, decodes the binary twin and re-encodes it
    Then both reproduce the two committed files byte for byte, agree on the model and on the digests of what they emitted, and agree with three.js about the primitives
