@capability-dxf-r12-mutate
@oracle-dxf-crate-r12-mutate-reader
@comparison-semantic-dxf-r12-v1
@mutations-dxf-r12-any
Feature: Apply every typed DXF R12 mutation to a real-world drawing
  The real-world input named by the fleet brief, `temp/simple_bus_shelter-gray_3D.dxf` (445 KB, a
  real exported bus-shelter model), is `$ACADVER` `AC1015` (AutoCAD 2000/R2000) -- confirmed here by
  reading the header directly with the `dxf` 0.6 reference crate, not assumed. Its `ENTITIES` section
  holds exactly ONE record: a `3DSOLID`, whose body is AutoCAD's own proprietary text-obfuscated ACIS
  data (`dxf`'s typed model exposes it only as opaque `custom_data`/`custom_data2: Vec<String>` chunks
  -- no geometric decode by any tool available in this repository) and ACIS solids do not exist in the
  DXF R12 spec at all (`3DSOLID` was introduced with AutoCAD 2000). So there is no real
  R12-representable vector geometry anywhere in this file: down-converting it, as the brief's first
  option asks, is not possible, because there is nothing this subset's LINE/CIRCLE/ARC/TEXT/SOLID/
  INSERT vocabulary could down-convert FROM.

  Per the brief's explicit fallback, a genuine R12 fixture was derived instead. `dxf` 0.6 read the
  real file and enumerated what IS real and representable: its `LAYER "0"` (color 7, linetype
  `CONTINUOUS`), `STYLE "STANDARD"` (font `txt`), `LTYPE ByBlock`/`ByLayer`/`CONTINUOUS` rows and its
  default empty `*Model_Space`/`*Paper_Space`/`*Paper_Space0` blocks -- every one of those carried
  verbatim into the derived fixture, plus one additional real, uncontroversial AutoCAD default
  (`STYLE "NOTES"`, `LAYER "DIMS"`, `LTYPE "DASHED"`, a second empty `BLOCK "SPARE"`) added so every
  Insert/Remove/Set mutation below has a safe, unambiguous target that never collides with an entity
  actually referencing it. Representative 2D vector geometry (two `INSERT`s of a `BLOCK "SHELTER_POST"`
  standing at each end of the shelter, an `ARC` roof, a `SOLID` glazing panel, a `LINE` ridge and a
  `TEXT` label) stands in for the real model's presence, since its actual coordinate data is
  cryptographically inaccessible -- this module says so plainly rather than pretending otherwise. The
  derivation script (`dxf-r12-derive`, a standalone scratch crate) and its own smoke-test harness
  (exercising all 19 kinds' mutate AND inverse against the derived fixture before this feature file was
  written) live in this ticket's folder. The real AC1015 source bytes are committed alongside the
  derived fixture, under `📷️source-ac1015.dxf`, for provenance; both files were confirmed NOT
  gitignored with `git check-ignore -v` (the taxonomy's `!**/🔖️*/**` negation rule at `.gitignore:187`
  un-ignores every `🔖️<standard>/**` subtree, this one included).

  THE FIRST DIFFERENTIAL RUN OF THIS CASE FOUND THREE REAL DEFECTS THAT
  THE SUBJECT PHASE ALONE HAD REPORTED AS 39 GREENS.

  1. THE WRITER PUT `LAYER` BEFORE `LTYPE`. A `LAYER` record names its linetype by string (group
     code 6), so the AutoCAD DXF reference requires the `LTYPE` table to precede the `LAYER` table;
     this subset's writer emitted LAYER, STYLE, LTYPE. Fed that order, the registered `dxf` 0.6
     reader invented the linetype it could not yet resolve and reported EIGHT linetypes where the
     drawing has seven — every one of the 39 comparisons diverged on `$.linetypes` alone.
     Confirmed by experiment, not inferred: reordering nothing but those two blocks in the very
     bytes the writer had already produced took the same reader from eight back to seven.
  2. THE SUBJECT HALF DISCARDED `MutationOutcome`. `apply_dxf_mutation` returns the refusal and
     leaves the snapshot untouched, so a rejected mutation re-encoded the input unchanged and the
     scenario reported green. Ten kinds did exactly that. The adapter now fails the scenario with
     the refusal's own code and target.
  3. THE FIXTURE'S SYMBOL TABLES CARRIED DUPLICATE NAMES, and a symbol table's names are its keys.
     Wave 7 derived this file by WRITING it with `dxf` 0.6, whose `normalize()` inserts its own
     `LAYER "0"`, `STYLE "STANDARD"`/`"ANNOTATIVE"` and `LTYPE "BYBLOCK"`/`"BYLAYER"`/`"CONTINUOUS"`
     on top of the source drawing's own — so the committed file carried `LAYER ["0","0","DIMS"]`,
     two `STANDARD` styles and two `CONTINUOUS` linetypes, which the paragraph above never claimed
     and which no name-keyed edit can address unambiguously. Each collision was resolved by
     dropping the `normalize()`-added record and keeping the derivation's own, which is the one
     carrying the real content (`CONTINUOUS` description "Solid line", `STANDARD` text height 2.5);
     nothing else in the file changed. Related, and fixed at the same cause: this subset's own
     name-keyed precondition demanded that the WHOLE table be duplicate-free, so it refused even
     `remove-layer {"name":"DIMS"}`, whose target is unique. It now constrains the names an edit
     actually TARGETS — still refusing an ambiguous one, no longer refusing an unambiguous one
     because of an unrelated collision elsewhere.

  No comparison profile was touched, no `ignoreKeys` added, no tolerance moved and no Examples row
  changed.

  `dxf` 0.6 reads AND writes DXF, so it is a genuine differential second producer for every
  `@mode-differential` scenario below, not merely an independent reader. Both the oracle's and the
  subject's results are read back by the SAME independent `dxf`-backed projection
  (`project_dxf_r12`) before the `semantic-dxf-r12-v1` profile compares them. `InsertEntity`/
  `RemoveEntity`/`InsertBlock`/`RemoveBlock` are this subset's structural analogue of the page
  operations the wave asked for: `insert-entity`/`remove-entity` add and drop a real `CIRCLE` fixing
  marker from the shelter's own `ENTITIES` list, and `insert-block`/`remove-block` add and drop a
  whole nested `BLOCK` definition, against the real drawing derived above.


  📌️ Every Examples row below other than `no-mutation` is required to MOVE the semantic projection,
  and the adapter fails the scenario in role when it does not: a row whose parameters make the
  mutation a no-op passes whenever the reference library merely declined to error, which is not a
  test. The baseline it is measured against runs one `no-mutation` cycle first, so the comparison
  isolates the mutation rather than the writer's own normal form.
  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                 | params                                                                                                                                          |
      | no-mutation        | {}                                                                                                                                               |
      | set-snapshot       | {"insertionBase": [5, 5, 0], "layers": [{"name": "0", "color": 7, "linetype": "CONTINUOUS"}], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 42}]} |
      | set-header-var     | {"name": "$INSBASE", "value": [15, 25, 0]}                                                                                                      |
      | remove-header-var  | {"name": "$INSBASE"}                                                                                                                             |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                 | params                                                                                                                                          |
      | no-mutation        | {}                                                                                                                                               |
      | set-snapshot       | {"insertionBase": [5, 5, 0], "layers": [{"name": "0", "color": 7, "linetype": "CONTINUOUS"}], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 42}]} |
      | set-header-var     | {"name": "$INSBASE", "value": [15, 25, 0]}                                                                                                      |
      | remove-header-var  | {"name": "$INSBASE"}                                                                                                                             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://📚️examples/🚏️bus-shelter/🖼️assets/🖊️.dxf
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
