@capability-semio-v1-drawing-mutate
@oracle-semio-drawing-python-independent
@comparison-ordered-json-v1
@mutations-semio-v1-drawing
Feature: Apply every typed semio DRAWING mutation to a real vector document, against an independent Python implementation
  `stdio.semio.drawing` is a semio-NATIVE format: no third party reads or writes `.dsl.semio`/
  `.pack.semio`, and the earlier survey of the vector-graphics libraries still stands on its merits.
  `usvg`/`resvg` model SVG, which has no counterpart for this subset's ANONYMOUS recursive `DrawNode`
  tree addressed by a structural `NodePath`, and no counterpart at all for the four hierarchy verbs;
  `lyon`/`kurbo` model path geometry alone and could adjudicate at most `replace-path`. Calling
  either a reference would overstate the evidence. The second producer THE STANDARD requires is
  therefore a second IMPLEMENTATION. `🐍️component.py` beside this file is that implementation — the
  carrier, the DSL grammar, the pack frame, the recursive node tree and all seventeen verbs — written
  in Python from the committed specification documents alone
  (`../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio`,
  `…/📸️snapshot/💾️binary/📡️component.protocol.semio` and its Kaitai mirror,
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`, and the semio envelope in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`). It imports nothing from and
  transliterates nothing of the Rust it judges, and it was pinned before use: it reproduces the
  committed `🖍️sketch` example artifact byte for byte in BOTH encodings — that one document exhibits
  every node tag and every segment tag, the arc included — and it reaches all seventeen committed
  after-snapshots. It is registered as the oracle `semio-drawing-python-independent`; the recorded
  no-oracle decision it replaces is gone, because a reference now exists.

  **The drawing under test is a real one, and its provenance is written down.**
  `local://🗣️.dsl.semio` and its binary twin were derived ONCE from two real committed SVG
  documents — `🗿️artifacts/🎨️svg/🧫️fixtures/mouse.svg`, the introduction demonstration mouse with its
  eight real cubic-and-line paths, its `clipPath` group and its real stroke widths and opacities, and
  `…/🎨️svg/🧫️fixtures/qr-code.svg`, a real 1015×1015 Inkscape QR document whose 329 rectangles each
  sit inside their own `matrix(0.35,0,0,0.35,tx,ty)` group inside a fill group inside a layer group,
  with a hidden background layer carrying a real 5 476-byte embedded image. The reader that produced
  it is an independent Python SVG reader built on `xml.etree` plus a path-data scanner written from
  the SVG 1.1 §8.3 command grammar — never this repository's own svg bridge. Relative commands are
  resolved to absolute, `H`/`V` to `lineTo` and `S`/`T` against the previous control point exactly as
  §8.3.6 defines; the one resolution that is not data is `currentColor`, which becomes black, CSS's
  initial value for `color`, and that is said rather than hidden. The result is THREE layers, 1 006
  nodes nested FIVE deep, 1 728 path segments, four styles — one carrying the QR background layer's
  real `opacity:0.5` — and one layer whose `display:none` is its real `visible: false`. That is
  56 205 bytes of DSL and 85 791 of pack, against the committed `🖍️sketch`'s 394 and 533; `asset://`
  cannot leave this artifact's root, which is why the derived drawing is committed here as a case
  fixture rather than borrowed in place. The derivation script and its provenance note live in the
  ticket folder.

  The parameters are chosen against the drawing's own shape, so a plausible wrong codec fails:
  `reorder-nodes` moves the FIRST of the QR foreground's 329 groups to index 40, deep inside the run,
  so an implementation that reordered by identity rather than position fails; `group` collects three
  NON-leading siblings and must leave the new group at the first of their indices; `ungroup` splices a
  real transform-carrying group's children back into a 329-child parent in place; `flatten` dissolves the mouse
  layer's real `clipPath` group into its three paths and its inverse has to put the hierarchy back —
  the mouse root is the one branch of this drawing `flatten` can touch at all, because every one of
  the QR foreground's 329 descendant groups carries a `matrix(0.35,…)` transform and `flatten` refuses
  the whole mutation when any descendant group is transformed, which is what the production test
  `flatten_refuses_a_non_identity_descendant_group` states and what the first parity run measured; `drag-nodes` moves TWO nodes by one offset; `create-node` and `replace-path` both carry
  an `arcTo` with a real `large_arc`/`sweep` pair, so the sixth segment variant is exercised through
  the mutation wire form and both implementations' node algebra — neither source SVG uses an arc, so
  the `A[…]` CARRIER production is pinned instead by the committed `🖍️sketch`, whose one path exhibits
  all six segment tags and which both implementations reproduce byte for byte; and the style verbs address the
  mouse's own `introduction-demo-mouse-button` style, which carries a fill and no stroke, so
  `change-stroke-color` has to CREATE the optional leaf where `replace-fill` replaces one.

  🔴 **`inverse-unflatten-node` is RED, and it is left red: `Unflatten`'s computed inverse cannot restore
  an arbitrary replaced node.** The payload replaces the mouse layer's real `clipPath` group with a
  different one. The independent implementation undoes it by putting the captured node back and
  restores the drawing exactly. The subject's own inverse law fails — `inverse-unflatten-node: undoing the
  mutation did not restore the drawing`, with two different digests over the same layer and style
  lists — because `Unflatten`'s inverse is `Flatten`, and flattening the REPLACEMENT does not bring
  the replaced node back. The production vocabulary knows this: the demo-variant list in
  `../../🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🧬️schema/🧬️mutations/🦀️.rs` carries the comment
  *"`original` is a genuine no-op restore (identical to the fixture's own node at this path, which has
  no nested groups) — `flatten(original) == original` here, so the `unflatten` ↔ `flatten` inverse
  pair's own law holds against the shared fixture"*. That caveat is now a measured failure rather than
  a code comment: for any `Unflatten` payload the grammar admits but that arrangement does not cover,
  the verb neither refuses the input nor captures the node it overwrites. Not tuned away — no
  `ignoreKeys`, no relaxed profile, and the payload was not swapped for one the caveat happens to
  cover.

  `spec-vector-` keeps the evidence this case rested on before the oracle existed: the committed,
  independently handcrafted `(before, mutation, after)` vector for each of the seventeen kinds,
  applied now by BOTH implementations and checked against the committed after-snapshot by each of
  them in role. Nothing was removed to make room for the oracle.

  `identity-round-trip` carries the BYTE half of the identity law. `.dsl.semio` is a fixed-layout
  record grammar and `.pack.semio` is its binary twin, so reproducing both committed files byte for
  byte is the CORRECT answer here and the wave's must-differ tripwire would be backwards — which is
  why the Rust side asserts `law::carrier_is_exact`. What stops that being a codec agreeing with
  itself is that the two files were WRITTEN by the Python implementation from the grammar alone, in
  another language, and the two sides' digests of the re-emitted bytes are compared.

  ⚠️ One honest limit. `SemioRgba`'s four channels and a style's `opacity` are SINGLE precision, and
  the reference's JSON wire form spells such a leaf with the shortest decimal that round-trips as an
  `f32` while a Python float would print the widened double. The Python side therefore routes every
  single-precision leaf through the same shortest-`f32` printer its DSL writer uses before projecting
  it — that is emulating the format's own wire form, not a tolerance. The DSL and pack carriers are
  unaffected: both move the exact `f32` bit pattern, which is why the QR document's `opacity:0.5` and
  the mouse's `stroke-opacity:0.35` survive the round trip byte for byte.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived drawing
    Given the real derived drawing artifact local://🗣️.dsl.semio
    And the committed mutation payload local://<fixture>/🦠️mutation/🔣️.json
    When the <id> mutation is applied to the drawing parsed from it
    Then the independent implementation and the subject agree on the resulting snapshot and on the scene-graph census
    Examples:
      | id | fixture |
      | create-layer | 🌱️create-layer |
      | delete-layer | 🗑️delete-layer |
      | create-node | ➕️create-node |
      | delete-node | ➖️delete-node |
      | move-node | 📍️move-node |
      | drag-nodes | 🖐️drag-nodes |
      | rotate-node | 🔄️rotate-node |
      | scale-node | 📏️scale-node |
      | reorder-nodes | 🔀️reorder-nodes |
      | group-nodes | 🧷️group-nodes |
      | ungroup-node | 💫️ungroup-node |
      | flatten-node | 🫓️flatten-node |
      | unflatten-node | 🎈️unflatten-node |
      | replace-path | 🛤️replace-path |
      | replace-fill | 🪣️replace-fill |
      | change-stroke-color | 🖌️change-stroke-color |
      | change-stroke-width | 📐️change-stroke-width |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real derived drawing
    Given the real derived drawing artifact local://🗣️.dsl.semio
    And the committed mutation payload local://<fixture>/🦠️mutation/🔣️.json
    When the <id> mutation is applied to the drawing parsed from it and each side undoes it with its own computed inverse
    Then both sides restore the drawing and agree on the mutated and the restored snapshot, scene-graph order and nesting included
    Examples:
      | id | fixture |
      | create-layer | 🌱️create-layer |
      | delete-layer | 🗑️delete-layer |
      | create-node | ➕️create-node |
      | delete-node | ➖️delete-node |
      | move-node | 📍️move-node |
      | drag-nodes | 🖐️drag-nodes |
      | rotate-node | 🔄️rotate-node |
      | scale-node | 📏️scale-node |
      | reorder-nodes | 🔀️reorder-nodes |
      | group-nodes | 🧷️group-nodes |
      | ungroup-node | 💫️ungroup-node |
      | flatten-node | 🫓️flatten-node |
      | unflatten-node | 🎈️unflatten-node |
      | replace-path | 🛤️replace-path |
      | replace-fill | 🪣️replace-fill |
      | change-stroke-color | 🖌️change-stroke-color |
      | change-stroke-width | 📐️change-stroke-width |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<slug>/📸️snapshot/➡️after/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                  | dir                    | slug                                                        |
      | change-stroke-color | 🖌️change-stroke-color | 🎨️recolours-the-primary-styles-stroke-to-translucent-white    |
      | change-stroke-width | 📐change-stroke-width  | 📐️thickens-the-primary-styles-stroke                          |
      | create-layer        | 🌱create-layer         | 🪜️inserts-a-second-layer-above-the-base-layer                 |
      | create-node         | ➕create-node          | 🔤️appends-a-caption-text-node-to-the-layer-root               |
      | delete-layer        | 🗑️delete-layer        | 🚫️removes-the-leading-layer-and-keeps-the-overlay             |
      | delete-node         | ➖delete-node          | 🚫️removes-the-text-node-from-the-layer-root                   |
      | drag-nodes          | 🖐️drag-nodes          | 🖐️drags-the-text-node-and-the-nested-group-by-the-same-offset |
      | flatten             | 🫓flatten-node         | 🫓️flattens-an-identity-nested-group-into-its-leaves           |
      | group               | 🧷group-nodes          | 🧷️groups-the-two-leading-children-into-a-new-group            |
      | move-node           | 📍move-node            | 📍️moves-the-text-node-to-a-new-origin                         |
      | reorder-nodes       | 🔀reorder-nodes        | 🔀️moves-the-leading-path-node-to-the-end-of-the-layer-root    |
      | replace-fill        | 🪣replace-fill         | 🎨️repaints-the-primary-styles-fill-from-red-to-blue           |
      | replace-path        | 🛤️replace-path        | 🔺️swaps-the-open-path-for-a-closed-triangle                   |
      | rotate              | 🔄rotate-node          | 🔄️rotates-the-nested-group-a-half-turn-about-z                |
      | scale               | 📏scale-node           | 📏️scales-the-nested-group-non-uniformly                       |
      | unflatten           | 🎈unflatten-node       | 🎈️restores-the-captured-hierarchy-over-the-flat-group         |
      | ungroup             | 💫ungroup-node         | 💫️dissolves-the-nested-group-into-its-parent                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both committed encodings of the real derived drawing from the parsed document
    Given the real derived drawing artifact local://🗣️.dsl.semio
    And its committed binary twin local://🎒️.pack.semio
    When each implementation parses the text artifact, prints it back, decodes the binary twin and re-encodes it
    Then both reproduce the two committed files byte for byte and agree on the drawing, the scene-graph census and the digests of what they emitted
