@capability-note-1-mutate
@no-oracle-note-document-mutation-semantics
@comparison-ordered-json-v1
@mutations-note-1-any
Feature: Apply every typed note document mutation to its committed specification vectors
  `s.note.note` is a semio-NATIVE artifact: it is persisted as `.dsl.semio` text and `.pack.semio`
  binary through this subset's own codecs, and no third party reads or writes either. There is
  therefore no reference implementation to register as an oracle — recorded as the
  `note-document-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, whose substitutes are the
  committed specification vectors and the inverse law. Because that decision is recorded, the runner
  dispatches NO oracle role for this case: every assertion below lives inside the subject handler,
  and a handler that merely ran the mutation and returned would report a pass having checked nothing.

  What distinguishes this subset is its SHAPE. Of the five artifacts covered in this wave, note is
  the only one whose document is a nested, positional tree: blocks are id-keyed, their order is the
  z-order the canvas paints in, and a `group` block contains other blocks. The 33 kinds fall into
  three tiers that behave nothing like each other — 9 document-root scalars (the title plus the
  eight grid, snap and tool settings), 3 id-keyed image-asset kinds over a flat pool, and 21 kinds
  over that tree. The tree tier is what an implementation gets wrong: `move-block-to-container`
  REPARENTS across the hierarchy, `drag-blocks` translates a whole subtree, `duplicate-blocks`
  copies several blocks at once with indices that shift as each copy lands, `delete-blocks` removes
  several at once, and four kinds — `edit-block-text`, `edit-block-math`, `edit-block-ink-stroke`
  and the four table row/column kinds — reach INSIDE one block's typed content rather than moving
  the block at all. An inverse that restores membership but not position passes nothing here.

  The vectors are chosen against exactly that. `duplicate-blocks` copies ink and table together so
  the shifting-index case is real rather than incidental; `drag-blocks` nudges an ink block AND a
  whole group subtree in the same mutation; `move-block-to-container` reparents ink into a callout
  group; the table kinds each act on the TRAILING row or column, so an implementation that removed
  by position-from-the-start passes nothing.

  📌️ All 33 committed vectors move the document — this is the only artifact of the five in this
  wave for which that is true, and it is why `GUARD_VECTORS` in the adapter is EMPTY: every one of
  the 33 `mutate-<kind>` scenarios is held to the observability law with no exemption at all.

  Every scenario reads the committed vectors where the domain already keeps them, through
  `asset://`, and never writes to them.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Applying <id> to its committed before-snapshot yields the committed after-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️component.json
    And the committed outcome vector asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🎯️outcome/🔣️component.json
    When <id> is applied through apply_note_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the resulting snapshot is the committed after-snapshot and the raised diagnostics are the committed outcome's
    Examples:
      | id                       | vector                                                                 |
      | rename-note              | 🏷️rename-note/🧪️tests/retitles-the-document                            |
      | change-grid-visible      | 👁️change-grid-visible/🧪️tests/hides-the-grid                           |
      | change-grid-spacing      | 📏️change-grid-spacing/🧪️tests/widens-grid-spacing                      |
      | change-grid-subdivisions | 🔢️change-grid-subdivisions/🧪️tests/doubles-grid-subdivisions           |
      | change-grid-opacity      | 🌫️change-grid-opacity/🧪️tests/raises-grid-opacity                      |
      | change-snap-enabled      | 🧲️change-snap-enabled/🧪️tests/enables-snap                             |
      | change-snap-grid-spacing | 📐️change-snap-grid-spacing/🧪️tests/halves-snap-grid-spacing            |
      | change-pencil-width      | ✏️change-pencil-width/🧪️tests/thickens-pencil                          |
      | change-eraser-radius     | 🧽️change-eraser-radius/🧪️tests/enlarges-eraser                         |
      | create-asset             | 🆕️create-asset/🧪️tests/adds-a-second-image-asset                       |
      | replace-asset-payload    | 🔁️replace-asset-payload/🧪️tests/swaps-logo-payload-for-svg             |
      | delete-asset             | 🗑️delete-asset/🧪️tests/removes-the-logo-asset                          |
      | create-block             | ➕️create-block/🧪️tests/inserts-a-photo-block-at-root-index-2           |
      | delete-block             | ❌️delete-block/🧪️tests/removes-the-math-block                          |
      | delete-blocks            | 🧺️delete-blocks/🧪️tests/removes-the-ink-and-image-blocks               |
      | duplicate-block          | 🎯️duplicate-block/🧪️tests/copies-the-math-block-right-after-its-source |
      | duplicate-blocks         | 👥️duplicate-blocks/🧪️tests/copies-ink-and-table-with-shifting-indices  |
      | move-block-to-container  | 🚚️move-block-to-container/🧪️tests/reparents-ink-into-the-callout-group |
      | drag-blocks              | 🤏️drag-blocks/🧪️tests/nudges-ink-and-the-whole-group-subtree           |
      | rename-block             | 🔖️rename-block/🧪️tests/renames-the-table-block                         |
      | change-block-visible     | 👀️change-block-visible/🧪️tests/hides-the-image-block                   |
      | change-block-locked      | 🔒️change-block-locked/🧪️tests/locks-the-callout-group                  |
      | move-block               | 📍️move-block/🧪️tests/repositions-the-math-block                        |
      | resize-block             | ↔️resize-block/🧪️tests/enlarges-the-image-block                        |
      | change-block-font-size   | 🔤️change-block-font-size/🧪️tests/enlarges-the-intro-font               |
      | edit-block-text          | 📝️edit-block-text/🧪️tests/replaces-the-intro-paragraphs                |
      | edit-block-math          | 🧮️edit-block-math/🧪️tests/replaces-the-tex-with-pythagoras             |
      | change-block-ink-width   | 🖊️change-block-ink-width/🧪️tests/thickens-the-sketch-stroke            |
      | edit-block-ink-stroke    | 🎨️edit-block-ink-stroke/🧪️tests/redraws-the-sketch-polyline            |
      | insert-table-row         | ⬇️insert-table-row/🧪️tests/appends-a-blank-third-row                   |
      | remove-table-row         | ⬆️remove-table-row/🧪️tests/drops-the-trailing-blank-row                |
      | insert-table-column      | ➡️insert-table-column/🧪️tests/appends-the-lettered-column-c            |
      | remove-table-column      | ⬅️remove-table-column/🧪️tests/drops-the-trailing-column-b              |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation payload asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<vector>/🦠️mutation/🔣️component.json
    When <id> is applied and then its own computed inverse is applied through apply_note_mutation_outcome
      """
      {"kind": "<id>", "vector": "<vector>"}
      """
    Then the projection is the committed before-snapshot's again, field for field
    Examples:
      | id                       | vector                                                                 |
      | rename-note              | 🏷️rename-note/🧪️tests/retitles-the-document                            |
      | change-grid-visible      | 👁️change-grid-visible/🧪️tests/hides-the-grid                           |
      | change-grid-spacing      | 📏️change-grid-spacing/🧪️tests/widens-grid-spacing                      |
      | change-grid-subdivisions | 🔢️change-grid-subdivisions/🧪️tests/doubles-grid-subdivisions           |
      | change-grid-opacity      | 🌫️change-grid-opacity/🧪️tests/raises-grid-opacity                      |
      | change-snap-enabled      | 🧲️change-snap-enabled/🧪️tests/enables-snap                             |
      | change-snap-grid-spacing | 📐️change-snap-grid-spacing/🧪️tests/halves-snap-grid-spacing            |
      | change-pencil-width      | ✏️change-pencil-width/🧪️tests/thickens-pencil                          |
      | change-eraser-radius     | 🧽️change-eraser-radius/🧪️tests/enlarges-eraser                         |
      | create-asset             | 🆕️create-asset/🧪️tests/adds-a-second-image-asset                       |
      | replace-asset-payload    | 🔁️replace-asset-payload/🧪️tests/swaps-logo-payload-for-svg             |
      | delete-asset             | 🗑️delete-asset/🧪️tests/removes-the-logo-asset                          |
      | create-block             | ➕️create-block/🧪️tests/inserts-a-photo-block-at-root-index-2           |
      | delete-block             | ❌️delete-block/🧪️tests/removes-the-math-block                          |
      | delete-blocks            | 🧺️delete-blocks/🧪️tests/removes-the-ink-and-image-blocks               |
      | duplicate-block          | 🎯️duplicate-block/🧪️tests/copies-the-math-block-right-after-its-source |
      | duplicate-blocks         | 👥️duplicate-blocks/🧪️tests/copies-ink-and-table-with-shifting-indices  |
      | move-block-to-container  | 🚚️move-block-to-container/🧪️tests/reparents-ink-into-the-callout-group |
      | drag-blocks              | 🤏️drag-blocks/🧪️tests/nudges-ink-and-the-whole-group-subtree           |
      | rename-block             | 🔖️rename-block/🧪️tests/renames-the-table-block                         |
      | change-block-visible     | 👀️change-block-visible/🧪️tests/hides-the-image-block                   |
      | change-block-locked      | 🔒️change-block-locked/🧪️tests/locks-the-callout-group                  |
      | move-block               | 📍️move-block/🧪️tests/repositions-the-math-block                        |
      | resize-block             | ↔️resize-block/🧪️tests/enlarges-the-image-block                        |
      | change-block-font-size   | 🔤️change-block-font-size/🧪️tests/enlarges-the-intro-font               |
      | edit-block-text          | 📝️edit-block-text/🧪️tests/replaces-the-intro-paragraphs                |
      | edit-block-math          | 🧮️edit-block-math/🧪️tests/replaces-the-tex-with-pythagoras             |
      | change-block-ink-width   | 🖊️change-block-ink-width/🧪️tests/thickens-the-sketch-stroke            |
      | edit-block-ink-stroke    | 🎨️edit-block-ink-stroke/🧪️tests/redraws-the-sketch-polyline            |
      | insert-table-row         | ⬇️insert-table-row/🧪️tests/appends-a-blank-third-row                   |
      | remove-table-row         | ⬆️remove-table-row/🧪️tests/drops-the-trailing-blank-row                |
      | insert-table-column      | ➡️insert-table-column/🧪️tests/appends-the-lettered-column-c            |
      | remove-table-column      | ⬅️remove-table-column/🧪️tests/drops-the-trailing-column-b              |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed example document and print it back without losing or copying anything
    Given the real committed artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the artifact is parsed to a NoteSnapshot, printed back to `.note` DSL and parsed again
    Then both parses agree on the same document and the printed text reproduces the committed bytes exactly
