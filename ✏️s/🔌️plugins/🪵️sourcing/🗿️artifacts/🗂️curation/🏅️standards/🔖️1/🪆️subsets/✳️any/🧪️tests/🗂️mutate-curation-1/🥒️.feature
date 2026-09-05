@capability-curation-1-mutate
@oracle-curation-python-independent
@comparison-ordered-json-v1
@mutations-curation-1-any
Feature: Apply every typed curation mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `sourcing.curation` document and all three typed mutations, written in
  Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` (the document:
  a composed catalogue handle, a stock table and an ordered id-keyed curation of `(objectId, count)`
  pairs, with `GeometryRecipe` a closed four-variant union), from
  `…/🧬️schema/🧬️mutations/🔣️.json` (the three variants and their internally tagged wire
  form) and `…/🧬️schema/🧬️mutations/📖️component.grammar.semio` (the three verbs), and from the three
  committed specification vectors. It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library. What distinguishes this vocabulary
  is how SMALL it is and why. Three kinds over exactly one collection. `stock` is not in the
  vocabulary at all: it is a bulk-populated reference catalogue seeded from hot-installed
  `sourcing.module` contributions and replaced wholesale through `ArtifactStore::reset`, the same
  non-history path whole-document replace uses — so there is no `create-stock-item` and no
  `set-snapshot` here. Within `curated` the schema closes the vocabulary just as tightly: a
  `CuratedItem` is an `objectId` and a `count` and nothing else, so there is no rename and no nested
  add/remove pair. No inventory or bill-of-materials library speaks `.curation.dsl.semio`, and none is
  authoritative over that closed surface; what a reference genuinely can adjudicate is membership,
  POSITION and count, and that is what this one does.

  The artifact is real. `local://🔣️.snapshot.json` was derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-curate-selection.py`
  from the artifact's own committed demo document
  (`🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`): the
  `s.stdio.semio@v1/kit` catalogue handle and all ten real stock entries — beams, windows and slabs
  with their real availabilities, typology paths and dimensioned geometry — are carried across
  unchanged. The carrier does not hold a `name` or a `moduleId` for a stock entry at all, so both are
  the empty string, which is stated rather than invented. The curation itself is derived, because the
  committed example leaves it EMPTY and `delete-curated-item` and `change-curated-item-count` would
  then address nothing: three of the ten real ids are curated, one per typology family, each at half
  its own committed availability. Every id and every number traces to the committed file.

  A finding recorded while deriving it: this repository's own `parse_curation_dsl` REJECTS that
  committed example — `TextError { message: "expected Text, found Absent", line 1, column 1 }` — which
  is why the fixture was derived by reading the carrier rather than by running the subset's codec,
  and why `identity-round-trip` below still asserts the carrier law in role on the Rust side and
  still fails. The failure predates this conversion and is kept, not routed around.

  Why the Python reference does not read the carrier. `.curation.dsl.semio` has no prose document, and
  its committed example is a structured document — a catalogue handle, a flat stock member list whose
  per-entry geometry recipe starts the line AFTER the entry it belongs to, and a declared-column
  `curated` table — whose rules for strings, absent members and numbers cannot be read off one
  example. Guessing them and calling the guess a specification is what this exercise forbids.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations, and the Rust side
  additionally holds the reported diagnostics to the committed `🎯️outcome`.

  A LIMIT OF THIS VOCABULARY, found by the second implementation and recorded rather than hidden.
  `create-curated-item` carries an `objectId` and a `count` and NOTHING ELSE — the grammar writes
  `create-curated-item "object-id" "=" IDENT "count" "=" INT`, with no index — so a created item can
  only ever land at the end. The inverse of `delete-curated-item` is therefore exact only for a
  TRAILING item, and undoing the deletion of any other one puts it back in the wrong place. That is a
  property of the closed schema, not a defect in an implementation, and both implementations share
  it, so a differential alone would report a comfortable green over a violated law. It is caught here
  because both sides assert the restoring law IN ROLE, position for position.
  The two tables below therefore differ on purpose, and neither is softened: `mutate-` deletes the
  LEADING entry, where the positional `detach` claim is sharpest and no inverse is asserted, and
  `inverse-` deletes the TRAILING entry, the only place this vocabulary's inverse is defined at all.

  The `effect` column states each kind's POSITIONAL claim and both implementations assert it in role.
  `append` requires the new entry to land AFTER both members already present, `detach` requires the
  survivors to keep their order, and `retune` requires the list length and every position to be
  untouched while one count moves. An implementation that rebuilt or re-sorted the curation would
  satisfy a membership comparison and fail all three.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived timber-kit curation
    Given the real derived curation local://🔣️.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      {"effect": "<effect>", "mutation": <mutation>}
      """
    Then both implementations produce the same catalogue, stock table and curation, and the curation moved the way <effect> claims
    Examples:
      | id                        | effect | mutation                                                                                             |
      | create-curated-item       | append | {"mutation":"createCuratedItem","item":{"objectId":"beam-steel-ipe200","count":6}}                    |
      | delete-curated-item       | detach | {"mutation":"deleteCuratedItem","objectId":"beam-glulam-gl24h"}                                       |
      | change-curated-item-count | retune | {"mutation":"changeCuratedItemCount","objectId":"window-tilt-turn-120x140","newCount":14}             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived curation and land back on it
    Given the real derived curation local://🔣️.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      {"effect": "<effect>", "mutation": <mutation>}
      """
    Then both implementations agree on the mutated document AND on the restored one, entry for entry and position for position
    Examples:
      | id                        | effect | mutation                                                                                             |
      | create-curated-item       | append | {"mutation":"createCuratedItem","item":{"objectId":"beam-steel-ipe200","count":6}}                    |
      | delete-curated-item       | detach | {"mutation":"deleteCuratedItem","objectId":"slab-clt-160"}                                            |
      | change-curated-item-count | retune | {"mutation":"changeCuratedItemCount","objectId":"window-tilt-turn-120x140","newCount":14}             |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-snapshot
      """
      {"effect": "<effect>"}
      """
    Then each implementation lands on the committed after-snapshot in role, the curation shows the <effect> claim, and the two agree
    Examples:
      | id                        | effect | dir                       | fixture                                    |
      | create-curated-item       | append | 🌱create-curated-item      | 🧲️appends-a-steel-plate-to-the-curation      |
      | delete-curated-item       | detach | 🗑️delete-curated-item      | 🚫️removes-the-clt-panel-from-the-curation    |
      | change-curated-item-count | retune | 🔢change-curated-item-count | 🔢️raises-the-glulam-beam-count-to-20         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real derived curation in both languages, and hold the committed carrier to its own law in Rust
    Given the real derived curation local://🔣️.snapshot.json
    And the artifact's own committed carrier asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation reads the derived curation, and the Rust additionally parses the committed carrier and prints it back
    Then both languages read the same catalogue, stock table and curation, and the Rust reproduces the committed carrier byte for byte
