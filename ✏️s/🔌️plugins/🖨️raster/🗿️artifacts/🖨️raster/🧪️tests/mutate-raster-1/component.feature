@capability-raster-1-mutate
@oracle-raster-python-independent
@comparison-ordered-json-v1
@mutations-raster-1-any
Feature: Apply every typed raster-document mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.raster.raster` layered document and all twelve typed mutations,
  written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️component.json` (all
  twelve variants and their internally tagged wire form), from
  `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (the twelve verbs and their argument
  lists) and from the twelve committed specification vectors. It imports nothing from this
  repository's Rust.

  Why a second implementation rather than a third-party library. `s.raster.raster` is a LAYERED
  DOCUMENT, not an image file. Its pixels live behind ids in a root `assets` pool and the vocabulary
  edits the layer TREE around them. `png`, `image` and `tiff` read a different artifact entirely, and
  Pillow — which really does read the pixel payloads — has nothing to say about a group's `children`,
  a layer's blend mode, or a reorder that lifts a node out of a group. What a reference can genuinely
  adjudicate is the tree algebra: insert-under-parent-at-index, delete-with-subtree,
  move-between-parents, the per-node field edits, and the root asset pool.

  A DEFECT IN THE SPECIFICATION, found while writing the reference and reported rather than worked
  around. `…/🧬️schema/📸️snapshot/🔣️component.json` does not describe this artifact at all: it is a
  verbatim copy of `s.stdio.json`'s `JsonSnapshot` schema — `{schema, value}` — carrying the wrong
  `$id`. The mutation schema's `RasterLayerNode` points at it and therefore points at nothing. The
  document shape the reference implements was read off the twelve committed vectors instead, which
  agree with one another on every field: a document is `{schema, id, title, layers, assets}`, and a
  layer node is one of three kinds over a shared base of `id`, `name`, `visible`, `opacity`,
  `blendMode` and `transform` — `group` adds `mask` and `children`, `pixel` adds `mask`, `width`,
  `height` and `imageKey`, and `adjustment` adds `adjustmentKind` and `params` and carries no mask.

  The artifact is real. `local://🖨️semio-demo-board.snapshot.json` was derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-raster-board.py`,
  and every node in it is copied from a committed file. The document's `schema`, `id` and `title`,
  its 1024×1024 `backdrop` pixel layer bound to the `semio-emblem` image key, its `brighten`
  `brightnessContrast` adjustment layer with its committed 0.12/0.08 parameters, and the real
  `semio-emblem` asset handle all come from the artifact's own committed demo carrier
  (`🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, whose members are
  hex-of-UTF-8 in a compact positional encoding). The `artwork` GROUP with its `sketch` child is
  taken verbatim from the committed `create-layer` vector's before-document. The derivation is that
  composition and nothing else; no field is edited.

  It exists because the committed carrier is FLAT and three of the twelve kinds address a group:
  `create-layer` takes a `parentId`, `delete-layer`'s committed vector removes a group with nested
  children, and `reorder-layers` can lift a node OUT of a group. It also closes a gap this case's own
  feature already named: `add-layer-asset` and `remove-layer-asset` had NO accepting committed
  vector — the two that exist are a declared warned no-op and a declared rejection — and the
  real-document rows below exercise both in their accepting direction for the first time.

  The committed specification vectors were KEPT, not replaced. `spec-vector-<kind>` replays each
  handcrafted triple through both implementations, and its `verdict` column states which of the three
  answers that vector commits to: `applied` must reach the committed after-document and move it,
  `noop` must reach it without moving it (`add-layer-asset` re-attaching an asset already on the
  document, whose committed outcome is `status: "applied"` with a `mutation.no-op` warning), and
  `refused` must be refused with the document left alone (`remove-layer-asset` naming an asset the
  document never attached, whose committed outcome is `status: "rejected"`,
  `code: "mutation.target-missing"`).

  Why the Python reference does not read the carrier. The committed `.dsl.semio` example is a compact
  positional tuple encoding with no prose document, and only ONE example of it exists — one pixel
  layer and one adjustment layer, with no group node at all, so the encoding of a group's `children`
  cannot be read off it. It is read once by the derivation script above, whose reading is checked by
  the fields it produces, and the carrier's own laws stay asserted in role on the Rust side.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived Semio demo board
    Given the real derived board local://🖨️semio-demo-board.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same document
    Examples:
      | id                           | mutation                                                                                                                                                                                                                                                                                    |
      | create-layer                 | {"mutation":"createLayer","parentId":"artwork","index":0,"layer":{"kind":"pixel","id":"ink","name":"Ink","visible":true,"opacity":1.0,"blendMode":"normal","transform":{"x":0.0,"y":0.0,"scaleX":1.0,"scaleY":1.0,"rotation":0.0},"mask":null,"width":1024,"height":1024,"imageKey":null}}    |
      | delete-layer                 | {"mutation":"deleteLayer","layerId":"artwork"}                                                                                                                                                                                                                                              |
      | reorder-layers               | {"mutation":"reorderLayers","layerId":"sketch","parentId":null,"index":0}                                                                                                                                                                                                                   |
      | rename-layer                 | {"mutation":"renameLayer","layerId":"backdrop","newName":"Emblem Backdrop"}                                                                                                                                                                                                                 |
      | change-layer-visible         | {"mutation":"changeLayerVisible","layerId":"brighten","newVisible":false}                                                                                                                                                                                                                   |
      | change-layer-opacity         | {"mutation":"changeLayerOpacity","layerId":"backdrop","newOpacity":0.375}                                                                                                                                                                                                                   |
      | change-layer-blend-mode      | {"mutation":"changeLayerBlendMode","layerId":"brighten","newBlendMode":"screen"}                                                                                                                                                                                                            |
      | move-layer                   | {"mutation":"moveLayer","layerId":"backdrop","newX":-128.5,"newY":64.25}                                                                                                                                                                                                                    |
      | resize-layer                 | {"mutation":"resizeLayer","layerId":"backdrop","newWidth":2048,"newHeight":1024}                                                                                                                                                                                                            |
      | change-layer-adjustment-kind | {"mutation":"changeLayerAdjustmentKind","layerId":"brighten","newAdjustmentKind":"curves"}                                                                                                                                                                                                  |
      | add-layer-asset              | {"mutation":"addLayerAsset","assetId":"semio-wordmark","asset":{"childId":"raster-asset-9f2c1d7b40ae5533","target":{"artifactId":"semio-wordmark-image","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"image"}}}}                                                        |
      | remove-layer-asset           | {"mutation":"removeLayerAsset","assetId":"semio-emblem"}                                                                                                                                                                                                                                    |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived board and land back on it
    Given the real derived board local://🖨️semio-demo-board.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated document AND on the restored one, node for node and index for index
    Examples:
      | id                           | mutation                                                                                                                                                                                                                                                                                    |
      | create-layer                 | {"mutation":"createLayer","parentId":"artwork","index":0,"layer":{"kind":"pixel","id":"ink","name":"Ink","visible":true,"opacity":1.0,"blendMode":"normal","transform":{"x":0.0,"y":0.0,"scaleX":1.0,"scaleY":1.0,"rotation":0.0},"mask":null,"width":1024,"height":1024,"imageKey":null}}    |
      | delete-layer                 | {"mutation":"deleteLayer","layerId":"artwork"}                                                                                                                                                                                                                                              |
      | reorder-layers               | {"mutation":"reorderLayers","layerId":"sketch","parentId":null,"index":0}                                                                                                                                                                                                                   |
      | rename-layer                 | {"mutation":"renameLayer","layerId":"backdrop","newName":"Emblem Backdrop"}                                                                                                                                                                                                                 |
      | change-layer-visible         | {"mutation":"changeLayerVisible","layerId":"brighten","newVisible":false}                                                                                                                                                                                                                   |
      | change-layer-opacity         | {"mutation":"changeLayerOpacity","layerId":"backdrop","newOpacity":0.375}                                                                                                                                                                                                                   |
      | change-layer-blend-mode      | {"mutation":"changeLayerBlendMode","layerId":"brighten","newBlendMode":"screen"}                                                                                                                                                                                                            |
      | move-layer                   | {"mutation":"moveLayer","layerId":"backdrop","newX":-128.5,"newY":64.25}                                                                                                                                                                                                                    |
      | resize-layer                 | {"mutation":"resizeLayer","layerId":"backdrop","newWidth":2048,"newHeight":1024}                                                                                                                                                                                                            |
      | change-layer-adjustment-kind | {"mutation":"changeLayerAdjustmentKind","layerId":"brighten","newAdjustmentKind":"curves"}                                                                                                                                                                                                  |
      | add-layer-asset              | {"mutation":"addLayerAsset","assetId":"semio-wordmark","asset":{"childId":"raster-asset-9f2c1d7b40ae5533","target":{"artifactId":"semio-wordmark-image","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"image"}}}}                                                        |
      | remove-layer-asset           | {"mutation":"removeLayerAsset","assetId":"semio-emblem"}                                                                                                                                                                                                                                    |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json
    And the committed after-document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️component.json
    When the committed mutation is applied to the committed before-document
      """
      {"verdict": "<verdict>"}
      """
    Then each implementation gives the committed <verdict> answer in role, and the two agree
    Examples:
      | id                           | verdict | dir                             | fixture                                               |
      | create-layer                 | applied | 🌱create-layer                   | creates-an-ink-layer-inside-the-artwork-group         |
      | delete-layer                 | applied | 🗑️delete-layer                   | deletes-the-frame-group-and-its-nested-children       |
      | reorder-layers               | applied | 🔀reorder-layers                 | lifts-the-caption-layer-out-of-the-frame-group        |
      | rename-layer                 | applied | ✏️rename-layer                   | renames-the-sketch-layer-to-final-linework            |
      | change-layer-visible         | applied | 👁️change-layer-visible           | hides-the-overlay-layer                               |
      | change-layer-opacity         | applied | 🌫️change-layer-opacity           | fades-the-highlight-layer-to-a-quarter                |
      | change-layer-blend-mode      | applied | 🎨change-layer-blend-mode        | switches-the-glow-layer-to-screen                     |
      | move-layer                   | applied | ↔️move-layer                     | slides-the-stamp-layer-off-the-origin                 |
      | resize-layer                 | applied | 📐resize-layer                   | resizes-the-canvas-layer-to-256-by-128                |
      | change-layer-adjustment-kind | applied | 🎚️change-layer-adjustment-kind   | switches-the-tone-layer-from-levels-to-curves         |
      | add-layer-asset              | noop    | 🖇️add-layer-asset                | declines-to-reattach-an-asset-already-on-the-document |
      | remove-layer-asset           | refused | 🗂️remove-layer-asset             | rejects-removing-an-asset-the-document-never-attached |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real derived board in both languages, and hold the committed carrier to its own laws in Rust
    Given the real derived board local://🖨️semio-demo-board.snapshot.json
    And the artifact's own committed carrier asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When each implementation reads the derived board, and the Rust additionally parses the committed carrier, prints it back and parses it again
    Then both languages read the same document, and the Rust printing is an ArtifactDsl fixpoint
