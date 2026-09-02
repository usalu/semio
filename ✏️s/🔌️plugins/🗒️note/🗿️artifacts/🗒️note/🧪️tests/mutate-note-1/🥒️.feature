@capability-note-1-mutate
@oracle-note-python-independent
@comparison-ordered-json-v1
@mutations-note-1-any
Feature: Apply every typed note document mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.note.note` document and all thirty-three typed mutations, written in
  Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from rules 1, 2,
  3, 5 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`, and
  from the thirty-three committed vectors. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library, and why the previous answer was
  wrong. This case used to say that because `s.note.note` is persisted through this subset's own
  codecs and no third party reads them, "there is no reference implementation to register as an
  oracle". `mutate-semio-drawing`, `mutate-semio-mesh` and the fifteen `📕️norm` references refuted
  that in this same wave by taking Python second implementations over this same carrier. A third-party
  library was nonetheless declined and the reason is concrete: no canvas or whiteboard format models
  a Z-ORDERED tree whose leaves are six differently-shaped typed blocks — text, stroke, table, math,
  image, group — reached by different verbs, and none of them reads this carrier.

  What distinguishes this subset is its SHAPE, and the reference is written against exactly that.
  Blocks are id-keyed but their ORDER is the z-order the canvas paints in, so every tree inverse here
  restores POSITION and not merely membership: `delete-blocks` puts its blocks back in ASCENDING index
  order so each lands where it was, and `move-block-to-container` inverts to a re-parent back to the
  original container AT the original index. `drag-blocks` translates a named block and its whole
  SUBTREE. Four kinds reach INSIDE one block's typed content — the math tex, the ink polyline, the
  table's rows and columns — rather than moving the block at all.

  📌️ A FINDING THE REFERENCE MADE WHILE IT WAS BEING WRITTEN. `duplicate-blocks` computes each copy's
  insertion index against the PRE-MUTATION list and does not re-base it as earlier copies land. Its
  committed vector duplicates `blk-ink` (root index 1) and `blk-table` (root index 2) in one mutation,
  and the committed after-snapshot orders the root
  `blk-text, blk-ink, blk-ink-copy, blk-table-COPY, blk-table, …` — the second copy lands BEFORE its
  own source, where the singular `duplicate-block` places its copy after. Both implementations
  reproduce the committed order; naming it here is what keeps it from passing as intended behaviour.

  🚧️ THREE SCENARIOS THE REFERENCE REFUSES BY CLAUSE, and reports rather than works around. First,
  `edit-block-text` in both roles. A text block does not hold its paragraphs: it holds a COMPOSED
  CHILD HANDLE `{childId, target}` into an `s.stdio.semio@v1/text` document, and the committed
  vector's whole observable effect is that handle's `childId` moving from
  `note-text-eea42a3b80b1052b` to `note-text-938222b3522927c6` — a content address of the child AFTER
  the new paragraphs are written, computed by a function no document in this repository states. Every
  OTHER verb over that same block is implemented. `mutate-program-1` reports the identical blocker
  over `knowledge`/`benchmarks`, `mutate-block-3d-1` over `catalog`, and `mutate-en1990-1`'s two red
  scenarios are the same finding again. Second, `identity-round-trip`. Unlike its `✒️writer`, `🌿️vcs`
  and `🔌️wires` siblings this subset commits a REAL grammar rather than the repository-wide
  `payload = OCTET+` placeholder — which is what makes the gap citable. Grammar and artifact disagree
  on three points: `block = text-block | image-block | shape-block` covers three of the SIX declared
  block kinds, leaving stroke, table, math and group with no production at all; `block-field` names
  `paragraphs` and `asset-id` while the committed artifact writes neither and writes
  `content=child_id=… target="…"`, a flattened nested record nothing bounds; and
  `artifact-mark = "note.note"` is contradicted by the artifact's own first line
  `semio note.note.dsl v1`.

  📌️ All 33 committed vectors move the document — this is the only artifact of the five in this wave
  for which that is true, and it is why the reference holds every applied vector to the observability
  law with no exemption at all.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome vector asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️.json
    When <id> is applied through apply_note_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                       | vector                                                                 |
      | rename-note              | ../../../✳️document/🧬️schema/🧬️mutations/🏷️rename-note/🧪️tests/retitles-the-document |
      | change-grid-visible      | ../../../✳️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🧪️tests/hides-the-grid |
      | change-grid-spacing      | ../../../✳️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🧪️tests/widens-grid-spacing |
      | change-grid-subdivisions | ../../../✳️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🧪️tests/doubles-grid-subdivisions |
      | change-grid-opacity      | ../../../✳️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🧪️tests/raises-grid-opacity |
      | change-snap-enabled      | ../../../✳️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🧪️tests/enables-snap |
      | change-snap-grid-spacing | ../../../✳️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🧪️tests/halves-snap-grid-spacing |
      | change-pencil-width      | ../../../✳️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🧪️tests/thickens-pencil |
      | change-eraser-radius     | ../../../✳️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🧪️tests/enlarges-eraser |
      | create-asset             | ../../../✳️asset/🧬️schema/🧬️mutations/🆕️create-asset/🧪️tests/adds-a-second-image-asset |
      | replace-asset-payload    | ../../../✳️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🧪️tests/swaps-logo-payload-for-svg |
      | delete-asset             | ../../../✳️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🧪️tests/removes-the-logo-asset |
      | create-block             | ../../../✳️block/🧬️schema/🧬️mutations/➕️create-block/🧪️tests/inserts-a-photo-block-at-root-index-2 |
      | delete-block             | ../../../✳️block/🧬️schema/🧬️mutations/❌️delete-block/🧪️tests/removes-the-math-block |
      | delete-blocks            | ../../../✳️block/🧬️schema/🧬️mutations/🧺️delete-blocks/🧪️tests/removes-the-ink-and-image-blocks |
      | duplicate-block          | ../../../✳️block/🧬️schema/🧬️mutations/🎯️duplicate-block/🧪️tests/copies-the-math-block-right-after-its-source |
      | duplicate-blocks         | ../../../✳️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🧪️tests/copies-ink-and-table-with-shifting-indices |
      | move-block-to-container  | ../../../✳️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🧪️tests/reparents-ink-into-the-callout-group |
      | drag-blocks              | ../../../✳️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🧪️tests/nudges-ink-and-the-whole-group-subtree |
      | rename-block             | ../../../✳️block/🧬️schema/🧬️mutations/🔖️rename-block/🧪️tests/renames-the-table-block |
      | change-block-visible     | ../../../✳️block/🧬️schema/🧬️mutations/👀️change-block-visible/🧪️tests/hides-the-image-block |
      | change-block-locked      | ../../../✳️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🧪️tests/locks-the-callout-group |
      | move-block               | ../../../✳️block/🧬️schema/🧬️mutations/📍️move-block/🧪️tests/repositions-the-math-block |
      | resize-block             | ../../../✳️block/🧬️schema/🧬️mutations/↔️resize-block/🧪️tests/enlarges-the-image-block |
      | change-block-font-size   | ../../../✳️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🧪️tests/enlarges-the-intro-font |
      | edit-block-text          | ../../../✳️text/🧬️schema/🧬️mutations/📝️edit-block-text/🧪️tests/replaces-the-intro-paragraphs |
      | edit-block-math          | ../../../✳️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🧪️tests/replaces-the-tex-with-pythagoras |
      | change-block-ink-width   | ../../../✳️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🧪️tests/thickens-the-sketch-stroke |
      | edit-block-ink-stroke    | ../../../✳️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🧪️tests/redraws-the-sketch-polyline |
      | insert-table-row         | ../../../✳️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🧪️tests/appends-a-blank-third-row |
      | remove-table-row         | ../../../✳️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🧪️tests/drops-the-trailing-blank-row |
      | insert-table-column      | ../../../✳️table/🧬️schema/🧬️mutations/➡️insert-table-column/🧪️tests/appends-the-lettered-column-c |
      | remove-table-column      | ../../../✳️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🧪️tests/drops-the-trailing-column-b |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️.json
    When <id> is applied and then its own computed inverse is applied through apply_note_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                       | vector                                                                 |
      | rename-note              | ../../../✳️document/🧬️schema/🧬️mutations/🏷️rename-note/🧪️tests/retitles-the-document |
      | change-grid-visible      | ../../../✳️canvas/🧬️schema/🧬️mutations/👁️change-grid-visible/🧪️tests/hides-the-grid |
      | change-grid-spacing      | ../../../✳️canvas/🧬️schema/🧬️mutations/📏️change-grid-spacing/🧪️tests/widens-grid-spacing |
      | change-grid-subdivisions | ../../../✳️canvas/🧬️schema/🧬️mutations/🔢️change-grid-subdivisions/🧪️tests/doubles-grid-subdivisions |
      | change-grid-opacity      | ../../../✳️canvas/🧬️schema/🧬️mutations/🌫️change-grid-opacity/🧪️tests/raises-grid-opacity |
      | change-snap-enabled      | ../../../✳️canvas/🧬️schema/🧬️mutations/🧲️change-snap-enabled/🧪️tests/enables-snap |
      | change-snap-grid-spacing | ../../../✳️canvas/🧬️schema/🧬️mutations/📐️change-snap-grid-spacing/🧪️tests/halves-snap-grid-spacing |
      | change-pencil-width      | ../../../✳️ink/🧬️schema/🧬️mutations/✏️change-pencil-width/🧪️tests/thickens-pencil |
      | change-eraser-radius     | ../../../✳️ink/🧬️schema/🧬️mutations/🧽️change-eraser-radius/🧪️tests/enlarges-eraser |
      | create-asset             | ../../../✳️asset/🧬️schema/🧬️mutations/🆕️create-asset/🧪️tests/adds-a-second-image-asset |
      | replace-asset-payload    | ../../../✳️asset/🧬️schema/🧬️mutations/🔁️replace-asset-payload/🧪️tests/swaps-logo-payload-for-svg |
      | delete-asset             | ../../../✳️asset/🧬️schema/🧬️mutations/🗑️delete-asset/🧪️tests/removes-the-logo-asset |
      | create-block             | ../../../✳️block/🧬️schema/🧬️mutations/➕️create-block/🧪️tests/inserts-a-photo-block-at-root-index-2 |
      | delete-block             | ../../../✳️block/🧬️schema/🧬️mutations/❌️delete-block/🧪️tests/removes-the-math-block |
      | delete-blocks            | ../../../✳️block/🧬️schema/🧬️mutations/🧺️delete-blocks/🧪️tests/removes-the-ink-and-image-blocks |
      | duplicate-block          | ../../../✳️block/🧬️schema/🧬️mutations/🎯️duplicate-block/🧪️tests/copies-the-math-block-right-after-its-source |
      | duplicate-blocks         | ../../../✳️block/🧬️schema/🧬️mutations/👥️duplicate-blocks/🧪️tests/copies-ink-and-table-with-shifting-indices |
      | move-block-to-container  | ../../../✳️block/🧬️schema/🧬️mutations/🚚️move-block-to-container/🧪️tests/reparents-ink-into-the-callout-group |
      | drag-blocks              | ../../../✳️block/🧬️schema/🧬️mutations/🤏️drag-blocks/🧪️tests/nudges-ink-and-the-whole-group-subtree |
      | rename-block             | ../../../✳️block/🧬️schema/🧬️mutations/🔖️rename-block/🧪️tests/renames-the-table-block |
      | change-block-visible     | ../../../✳️block/🧬️schema/🧬️mutations/👀️change-block-visible/🧪️tests/hides-the-image-block |
      | change-block-locked      | ../../../✳️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🧪️tests/locks-the-callout-group |
      | move-block               | ../../../✳️block/🧬️schema/🧬️mutations/📍️move-block/🧪️tests/repositions-the-math-block |
      | resize-block             | ../../../✳️block/🧬️schema/🧬️mutations/↔️resize-block/🧪️tests/enlarges-the-image-block |
      | change-block-font-size   | ../../../✳️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🧪️tests/enlarges-the-intro-font |
      | edit-block-text          | ../../../✳️text/🧬️schema/🧬️mutations/📝️edit-block-text/🧪️tests/replaces-the-intro-paragraphs |
      | edit-block-math          | ../../../✳️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🧪️tests/replaces-the-tex-with-pythagoras |
      | change-block-ink-width   | ../../../✳️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🧪️tests/thickens-the-sketch-stroke |
      | edit-block-ink-stroke    | ../../../✳️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🧪️tests/redraws-the-sketch-polyline |
      | insert-table-row         | ../../../✳️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🧪️tests/appends-a-blank-third-row |
      | remove-table-row         | ../../../✳️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🧪️tests/drops-the-trailing-blank-row |
      | insert-table-column      | ../../../✳️table/🧬️schema/🧬️mutations/➡️insert-table-column/🧪️tests/appends-the-lettered-column-c |
      | remove-table-column      | ../../../✳️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🧪️tests/drops-the-trailing-column-b |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed to a NoteSnapshot, printed back to `.note` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
