@capability-block-2d-1-mutate
@oracle-block-2d-python-independent
@comparison-ordered-json-v1
@mutations-block-2d-1-any
Feature: Apply every typed block2d node-kind mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.block.block2d` node-kind document and all twenty-six typed
  mutations, written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`,
  from `…/🧬️schema/🧬️mutations/🔣️component.json` and `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`,
  and from the twenty-six committed specification vectors. It imports nothing from this repository's
  Rust.

  Why a second implementation rather than a third-party library. A `block2d` document is a KIND
  DEFINITION, not an instance: one node kind's identity and presentation, the handle kinds it
  declares, the handles placed on its rim by polar coordinate, the compatibility relation between
  handle kinds, its attribute table, its authors, its editor camera and its metadata. A symbol- or
  component-library format (KiCad, Modelica, IFC property sets) carries pins or ports but has no
  notion of a kind-level compatibility relation, of a rim angle in radians, or of the presentation
  VARIANT this vocabulary switches with a single verb — and none of them reads `.dsl.semio`.

  The artifact is real. `local://🧱️hexagonal-cut-concrete-forest-left.snapshot.json` was derived ONCE
  by `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-block2d-kind.py`
  from the artifact's own committed example
  (`🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️hexagonal-cut-concrete-forest-left/🖼️assets/🗣️hexagonal-cut-concrete-forest-left.dsl.semio`):
  the *Hexagonal Cut Concrete Forest Left* node kind with its real camera at 230.7/93.5 zoom 2, its
  six real handle kinds with their real HSL colours, its ELEVEN real handles at their real radian
  angles around a 0.36 rim, and its one real compatibility rule — all carried across unchanged.

  What the carrier does NOT hold, and where those values come from — stated rather than invented. The
  carrier's `node-kind` block writes only `id`, `name`, `label` and `description`, so `variant`,
  `icon` and `unit` are the empty string the schema admits. Its `presentation` block is EMPTY while
  the snapshot requires a `shape`, so the presentation is taken VERBATIM from the committed
  `✏️rename-node-kind` vector's before-snapshot. Its `attributes` and `authors` tables are empty, so
  `remove-attribute` and `remove-author` would address nothing — the committed vectors' own
  `material`/`brass` attribute and `author-ada` are taken verbatim. And every one of the six committed
  handle kinds is REFERENCED by a handle, so one spare, `hk-ground` from `🌱️create-handle-kind`, is
  appended LAST to give `delete-handle-kind` an unreferenced trailing target.

  Why trailing matters: no `create-`/`add-` verb in this vocabulary carries an index, so the inverse
  of a delete is exact only for a TRAILING record. That is a property of the closed schema, not of an
  implementation, and both implementations share it — it is caught here only because both sides
  assert the restoring law IN ROLE, index for index.

  TWO FINDINGS, reported rather than worked around. First, this subset shipped NO test bridge, so its
  Rust adapter did not link the plugin crate at all: the subject phase read the committed vectors and
  asserted laws over them, and never ran this subset's own implementation on anything. Every other
  converted subset (`🗺️gismap`, `🏗️fem`, `🏔️gisterrain`) ships one, so `block2d_mutation_report_json`
  was added to `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` in the same shape
  and the adapter now drives it. Second, the `.dsl.semio` carrier law is NOT asserted anywhere in this
  case: this subset's `store::ArtifactDsl` impl is handwritten `async` while the generated test host
  is synchronous, so `parse_dsl`/`print_dsl` are unreachable from a case adapter. `identity-round-trip`
  therefore exercises the JSON codec on the real derived document, and the carrier gap is stated here
  rather than hidden behind a scenario that claims more than it checks.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations.

  Both implementations additionally assert, in role, that each verb writes exactly ONE of the ten
  members — the check an after-snapshot comparison cannot make on its own.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived node kind
    Given the real derived node kind local://🧱️hexagonal-cut-concrete-forest-left.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same document, and only the member this verb writes moved
    Examples:
      | id                                   | mutation                                                                                                                                                    |
      | rename-node-kind                     | {"mutation":"renameNodeKind","newName":"Hexagonal Cut Concrete Forest Left Rev B"}                                                                          |
      | change-node-kind-label               | {"mutation":"changeNodeKindLabel","newLabel":"Hexagonal Cut, Left"}                                                                                         |
      | change-node-kind-variant             | {"mutation":"changeNodeKindVariant","newVariant":"mirrored"}                                                                                                |
      | change-node-kind-description         | {"mutation":"changeNodeKindDescription","newDescription":"Left half of the hexagonal cut in the concrete forest."}                                          |
      | change-node-kind-icon                | {"mutation":"changeNodeKindIcon","newIcon":"icon://hexagonal-cut-left"}                                                                                     |
      | change-node-kind-unit                | {"mutation":"changeNodeKindUnit","newUnit":"m"}                                                                                                             |
      | update-presentation                  | {"mutation":"updatePresentation","newShape":"rectangle","newRadius":null,"newWidth":0.72,"newHeight":0.72,"newColor":"hsl(206 52% 48%)","newIconKind":null} |
      | create-handle-kind                   | {"mutation":"createHandleKind","handleKind":{"id":"c-m","name":"c-m","label":"c-m","color":"hsl(0 52% 48%)","defaultWireKind":"cable.link"}}                |
      | delete-handle-kind                   | {"mutation":"deleteHandleKind","id":"hk-ground"}                                                                                                            |
      | rename-handle-kind                   | {"mutation":"renameHandleKind","id":"b-l","newName":"b-long"}                                                                                               |
      | change-handle-kind-label             | {"mutation":"changeHandleKindLabel","id":"b-l","newLabel":"Bottom Long"}                                                                                    |
      | change-handle-kind-color             | {"mutation":"changeHandleKindColor","id":"b-l","newColor":"hsl(206 52% 62%)"}                                                                               |
      | change-handle-kind-default-wire-kind | {"mutation":"changeHandleKindDefaultWireKind","id":"b-l","newDefaultWireKind":"cable.heavy"}                                                                |
      | create-handle                        | {"mutation":"createHandle","handle":{"id":"h11","handleKind":"c-t","angle":4.71238898038469,"radius":0.36}}                                                 |
      | delete-handle                        | {"mutation":"deleteHandle","id":"h10"}                                                                                                                      |
      | move-handle                          | {"mutation":"moveHandle","id":"h0","newAngle":-1.0471975511965976,"newRadius":0.42}                                                                         |
      | change-handle-handle-kind            | {"mutation":"changeHandleHandleKind","id":"h0","newHandleKind":"c-t"}                                                                                       |
      | add-compatibility-rule               | {"mutation":"addCompatibilityRule","rule":{"id":"compat1","source":"c-b","target":"c-t","bidirectional":false}}                                             |
      | remove-compatibility-rule            | {"mutation":"removeCompatibilityRule","id":"compat0"}                                                                                                       |
      | add-attribute                        | {"mutation":"addAttribute","attribute":{"key":"cut","value":"hexagonal"}}                                                                                   |
      | remove-attribute                     | {"mutation":"removeAttribute","key":"material"}                                                                                                             |
      | add-author                           | {"mutation":"addAuthor","author":{"id":"author-bo","name":"Bo"}}                                                                                            |
      | remove-author                        | {"mutation":"removeAuthor","id":"author-ada"}                                                                                                               |
      | move-camera2d                        | {"mutation":"moveCamera2d","newX":118.25,"newY":-44.5}                                                                                                      |
      | scale-camera2d                       | {"mutation":"scaleCamera2d","newZoom":3.5}                                                                                                                  |
      | change-meta-description              | {"mutation":"changeMetaDescription","newDescription":"Reviewed during the cross-language conversion."}                                                      |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived node kind and land back on it
    Given the real derived node kind local://🧱️hexagonal-cut-concrete-forest-left.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated document AND on the restored one, member for member and index for index
    Examples:
      | id                                   | mutation                                                                                                                                                    |
      | rename-node-kind                     | {"mutation":"renameNodeKind","newName":"Hexagonal Cut Concrete Forest Left Rev B"}                                                                          |
      | change-node-kind-label               | {"mutation":"changeNodeKindLabel","newLabel":"Hexagonal Cut, Left"}                                                                                         |
      | change-node-kind-variant             | {"mutation":"changeNodeKindVariant","newVariant":"mirrored"}                                                                                                |
      | change-node-kind-description         | {"mutation":"changeNodeKindDescription","newDescription":"Left half of the hexagonal cut in the concrete forest."}                                          |
      | change-node-kind-icon                | {"mutation":"changeNodeKindIcon","newIcon":"icon://hexagonal-cut-left"}                                                                                     |
      | change-node-kind-unit                | {"mutation":"changeNodeKindUnit","newUnit":"m"}                                                                                                             |
      | update-presentation                  | {"mutation":"updatePresentation","newShape":"rectangle","newRadius":null,"newWidth":0.72,"newHeight":0.72,"newColor":"hsl(206 52% 48%)","newIconKind":null} |
      | create-handle-kind                   | {"mutation":"createHandleKind","handleKind":{"id":"c-m","name":"c-m","label":"c-m","color":"hsl(0 52% 48%)","defaultWireKind":"cable.link"}}                |
      | delete-handle-kind                   | {"mutation":"deleteHandleKind","id":"hk-ground"}                                                                                                            |
      | rename-handle-kind                   | {"mutation":"renameHandleKind","id":"b-l","newName":"b-long"}                                                                                               |
      | change-handle-kind-label             | {"mutation":"changeHandleKindLabel","id":"b-l","newLabel":"Bottom Long"}                                                                                    |
      | change-handle-kind-color             | {"mutation":"changeHandleKindColor","id":"b-l","newColor":"hsl(206 52% 62%)"}                                                                               |
      | change-handle-kind-default-wire-kind | {"mutation":"changeHandleKindDefaultWireKind","id":"b-l","newDefaultWireKind":"cable.heavy"}                                                                |
      | create-handle                        | {"mutation":"createHandle","handle":{"id":"h11","handleKind":"c-t","angle":4.71238898038469,"radius":0.36}}                                                 |
      | delete-handle                        | {"mutation":"deleteHandle","id":"h10"}                                                                                                                      |
      | move-handle                          | {"mutation":"moveHandle","id":"h0","newAngle":-1.0471975511965976,"newRadius":0.42}                                                                         |
      | change-handle-handle-kind            | {"mutation":"changeHandleHandleKind","id":"h0","newHandleKind":"c-t"}                                                                                       |
      | add-compatibility-rule               | {"mutation":"addCompatibilityRule","rule":{"id":"compat1","source":"c-b","target":"c-t","bidirectional":false}}                                             |
      | remove-compatibility-rule            | {"mutation":"removeCompatibilityRule","id":"compat0"}                                                                                                       |
      | add-attribute                        | {"mutation":"addAttribute","attribute":{"key":"cut","value":"hexagonal"}}                                                                                   |
      | remove-attribute                     | {"mutation":"removeAttribute","key":"material"}                                                                                                             |
      | add-author                           | {"mutation":"addAuthor","author":{"id":"author-bo","name":"Bo"}}                                                                                            |
      | remove-author                        | {"mutation":"removeAuthor","id":"author-ada"}                                                                                                               |
      | move-camera2d                        | {"mutation":"moveCamera2d","newX":118.25,"newY":-44.5}                                                                                                      |
      | scale-camera2d                       | {"mutation":"scaleCamera2d","newZoom":3.5}                                                                                                                  |
      | change-meta-description              | {"mutation":"changeMetaDescription","newDescription":"Reviewed during the cross-language conversion."}                                                      |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️component.json
    And the committed mutation asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json
    And the committed after-document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️component.json
    When the committed mutation is applied to the committed before-document
    Then each implementation lands on the committed after-document in role, only the member this verb writes moved, and the two agree
    Examples:
      | id                                   | dir                                    | fixture                        |
      | rename-node-kind                     | ✏️rename-node-kind                     | renames-node-kind-to-gate      |
      | change-node-kind-label               | 🏷️change-node-kind-label               | relabels-node-kind             |
      | change-node-kind-variant             | 🔀️change-node-kind-variant             | switches-variant-to-b          |
      | change-node-kind-description         | 📃️change-node-kind-description         | rewrites-node-kind-description |
      | change-node-kind-icon                | 🖼️change-node-kind-icon                | repoints-node-kind-icon        |
      | change-node-kind-unit                | 📐️change-node-kind-unit                | switches-unit-to-metre         |
      | update-presentation                  | 🖌️update-presentation                  | circle-to-rectangle            |
      | create-handle-kind                   | 🌱️create-handle-kind                   | appends-ground-handle-kind     |
      | delete-handle-kind                   | 🗑️delete-handle-kind                   | removes-power-handle-kind      |
      | rename-handle-kind                   | ✒️rename-handle-kind                   | renames-power-to-mains         |
      | change-handle-kind-label             | 🔖️change-handle-kind-label             | relabels-power-handle-kind     |
      | change-handle-kind-color             | 🎨️change-handle-kind-color             | recolors-power-handle-kind     |
      | change-handle-kind-default-wire-kind | 🔌️change-handle-kind-default-wire-kind | swaps-power-default-wire-kind  |
      | create-handle                        | 🌿️create-handle                        | appends-out-handle             |
      | delete-handle                        | ❌️delete-handle                        | removes-in-handle              |
      | move-handle                          | 📍️move-handle                          | swings-in-handle-along-the-rim |
      | change-handle-handle-kind            | 🧷️change-handle-handle-kind            | rekinds-in-handle-as-power     |
      | add-compatibility-rule               | ➕️add-compatibility-rule               | allows-signal-to-power         |
      | remove-compatibility-rule            | ➖️remove-compatibility-rule            | revokes-signal-to-signal       |
      | add-attribute                        | 🧩️add-attribute                        | adds-pressure-attribute        |
      | remove-attribute                     | 🚫️remove-attribute                     | drops-material-attribute       |
      | add-author                           | 👤️add-author                           | credits-bo                     |
      | remove-author                        | 🚷️remove-author                        | uncredits-ada                  |
      | move-camera2d                        | 🎥️move-camera2d                        | pans-camera                    |
      | scale-camera2d                       | 🔍️scale-camera2d                       | zooms-camera-in                |
      | change-meta-description              | 💬️change-meta-description              | rewrites-session-notes         |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real derived node kind in both languages and agree on it
    Given the real derived node kind local://🧱️hexagonal-cut-concrete-forest-left.snapshot.json
    When each implementation reads it through its own JSON codec, the Rust through the bridge with a payload naming the value the document already holds
      """
      {"mutation":"changeMetaDescription","newDescription":""}
      """
    Then both languages read the same ten members, and neither moved the document
