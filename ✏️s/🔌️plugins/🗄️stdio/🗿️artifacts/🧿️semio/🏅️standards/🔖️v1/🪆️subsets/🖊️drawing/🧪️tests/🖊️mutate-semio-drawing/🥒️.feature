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
  `shared://🖊️mutate-semio-drawing/🗣️.dsl.semio` and its binary twin were derived ONCE from two real committed SVG
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

  **`inverse-unflatten-node` was RED, and it was fixed in the vocabulary rather than tuned away.**
  The payload replaces the mouse layer's real `clipPath` group with a different one. The independent
  implementation undoes it by putting the captured node back and restores the drawing exactly; the
  subject's own inverse law failed, because `Unflatten`'s computed inverse was a bare `Flatten`, and
  flattening the REPLACEMENT does not bring the replaced node back — that inverse was exact only for a
  payload whose current node is the flattening of `original`, which is how the vocabulary pairs the
  two verbs but not what the grammar admits. `Unflatten`'s inverse now captures the node it
  overwrites and restores exactly that node (`🎈unflatten-node/↩️inverse`), pinned by the leaf's
  `the_undo_restores_a_node_that_was_not_the_flattening` test. No `ignoreKeys`, no relaxed profile,
  and the payload was not swapped.

  **The census disagreement was this side's own projection, not the drawing.** Every Rust row came
  back with `nodes.path` inflated and a `nodes.group-nodes` key the independent implementation never
  emits: the census looked for a node kind spelled like the `group-nodes` VERB, while the snapshot's
  wire tag — the schema's `DrawNode` union, which the Python census reads correctly — is `group`, so
  every group was tallied as a path. The census now keys every node by the snapshot's own tag.

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
    Given the real derived drawing artifact shared://🖊️mutate-semio-drawing/🗣️.dsl.semio
    And the committed mutation payload shared://🖊️mutate-semio-drawing/<fixture>/🦠️mutation/🔣️.json
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
      | change-stroke-color | 🖌️change-stroke |
      | change-stroke-width | 📐️change-stroke-width |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real derived drawing
    Given the real derived drawing artifact shared://🖊️mutate-semio-drawing/🗣️.dsl.semio
    And the committed mutation payload shared://🖊️mutate-semio-drawing/<fixture>/🦠️mutation/🔣️.json
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
      | change-stroke-color | 🖌️change-stroke |
      | change-stroke-width | 📐️change-stroke-width |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed handcrafted specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<slug>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<slug>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<slug>/📸️snapshot/➡️after/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot and the two agree
    Examples:
      | id                  | dir                    | slug                                                        |
      | change-stroke-color | 🖌️change-stroke | 🎨️recolours    |
      | change-stroke-width | 📐change-stroke-width  | 📐️thickens                          |
      | create-layer        | 🌱create-layer         | 🪜️inserts                 |
      | create-node         | ➕create-node          | 🔤️appends               |
      | delete-layer        | 🗑️delete-layer        | 🚫️removes             |
      | delete-node         | ➖delete-node          | 🚫️removes                   |
      | drag-nodes          | 🖐️drag-nodes          | 🖐️drags |
      | flatten             | 🫓flatten-node         | 🫓️flattens           |
      | group               | 🧷group-nodes          | 🧷️groups            |
      | move-node           | 📍move-node            | 📍️moves                         |
      | reorder-nodes       | 🔀reorder-nodes        | 🔀️moves    |
      | replace-fill        | 🪣replace-fill         | 🎨️repaints           |
      | replace-path        | 🛤️replace-path        | 🔺️swaps                   |
      | rotate              | 🔄rotate-node          | 🔄️rotates                |
      | scale               | 📏scale-node           | 📏️scales                       |
      | unflatten           | 🎈unflatten-node       | 🎈️restores         |
      | ungroup             | 💫ungroup-node         | 💫️dissolves                  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit both committed encodings of the real derived drawing from the parsed document
    Given the real derived drawing artifact shared://🖊️mutate-semio-drawing/🗣️.dsl.semio
    And its committed binary twin shared://🖊️mutate-semio-drawing/🎒️.pack.semio
    When each implementation parses the text artifact, prints it back, decodes the binary twin and re-encodes it
    Then both reproduce the two committed files byte for byte and agree on the drawing, the scene-graph census and the digests of what they emitted
