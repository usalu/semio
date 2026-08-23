@capability-dxf-r12-mutate
@oracle-dxf-crate-r12-mutate
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

  `dxf` 0.6 reads AND writes DXF, so it is a genuine differential second producer for every
  `@mode-differential` scenario below, not merely an independent reader. Both the oracle's and the
  subject's results are read back by the SAME independent `dxf`-backed projection
  (`project_dxf_r12`) before the `semantic-dxf-r12-v1` profile compares them. `InsertEntity`/
  `RemoveEntity`/`InsertBlock`/`RemoveBlock` are this subset's structural analogue of the page
  operations the wave asked for: `insert-entity`/`remove-entity` add and drop a real `CIRCLE` fixing
  marker from the shelter's own `ENTITIES` list, and `insert-block`/`remove-block` add and drop a
  whole nested `BLOCK` definition, against the real drawing derived above.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️r12/🪆️subsets/✳️any/📚️examples/🚏️bus-shelter/🖼️assets/🖊️bus-shelter-r12.dxf
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
      | insert-layer       | {"index": 1, "name": "MARKERS", "color": 6, "linetype": "CONTINUOUS"}                                                                           |
      | remove-layer       | {"name": "DIMS"}                                                                                                                                 |
      | set-layer          | {"name": "DIMS", "color": 4, "linetype": "DASHED"}                                                                                              |
      | insert-style       | {"index": 1, "name": "LABELS", "font": "arial.ttf"}                                                                                             |
      | remove-style       | {"name": "NOTES"}                                                                                                                                |
      | set-style          | {"name": "NOTES", "font": "romans.shx"}                                                                                                         |
      | insert-linetype    | {"index": 1, "name": "CENTER", "description": "Center line"}                                                                                    |
      | remove-linetype    | {"name": "DASHED"}                                                                                                                               |
      | set-linetype       | {"name": "DASHED", "description": "Dash pattern"}                                                                                               |
      | insert-entity      | {"index": 2, "entityKind": "circle", "layer": "0", "center": [1200, 100, 0], "radius": 30}                                                      |
      | remove-entity      | {"index": 3}                                                                                                                                     |
      | set-entity         | {"index": 5, "entityKind": "text", "layer": "DIMS", "position": [200, 260, 0], "height": 80, "value": "WAVE 7 SHELTER"}                          |
      | insert-block       | {"index": 1, "name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"entityKind": "line", "layer": "0", "start": [0, 0, 0], "end": [100, 0, 0]}]} |
      | remove-block       | {"index": 1}                                                                                                                                     |
      | set-block          | {"index": 0, "name": "SHELTER_POST", "basePoint": [0, 0, 0], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 20}]} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🏅️standards/🔖️r12/🪆️subsets/✳️any/📚️examples/🚏️bus-shelter/🖼️assets/🖊️bus-shelter-r12.dxf
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
      | insert-layer       | {"index": 1, "name": "MARKERS", "color": 6, "linetype": "CONTINUOUS"}                                                                           |
      | remove-layer       | {"name": "DIMS"}                                                                                                                                 |
      | set-layer          | {"name": "DIMS", "color": 4, "linetype": "DASHED"}                                                                                              |
      | insert-style       | {"index": 1, "name": "LABELS", "font": "arial.ttf"}                                                                                             |
      | remove-style       | {"name": "NOTES"}                                                                                                                                |
      | set-style          | {"name": "NOTES", "font": "romans.shx"}                                                                                                         |
      | insert-linetype    | {"index": 1, "name": "CENTER", "description": "Center line"}                                                                                    |
      | remove-linetype    | {"name": "DASHED"}                                                                                                                               |
      | set-linetype       | {"name": "DASHED", "description": "Dash pattern"}                                                                                               |
      | insert-entity      | {"index": 2, "entityKind": "circle", "layer": "0", "center": [1200, 100, 0], "radius": 30}                                                      |
      | remove-entity      | {"index": 3}                                                                                                                                     |
      | set-entity         | {"index": 5, "entityKind": "text", "layer": "DIMS", "position": [200, 260, 0], "height": 80, "value": "WAVE 7 SHELTER"}                          |
      | insert-block       | {"index": 1, "name": "BENCH_MARK", "basePoint": [0, 0, 0], "entities": [{"entityKind": "line", "layer": "0", "start": [0, 0, 0], "end": [100, 0, 0]}]} |
      | remove-block       | {"index": 1}                                                                                                                                     |
      | set-block          | {"index": 0, "name": "SHELTER_POST", "basePoint": [0, 0, 0], "entities": [{"entityKind": "circle", "layer": "0", "center": [0, 0, 0], "radius": 20}]} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🏅️standards/🔖️r12/🪆️subsets/✳️any/📚️examples/🚏️bus-shelter/🖼️assets/🖊️bus-shelter-r12.dxf
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
