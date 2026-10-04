@capability-rewriting-1-mutate
@oracle-rewriting-python-independent
@comparison-ordered-json-v1
@mutations-rewriting-1-any
Feature: Apply every typed graph-rewrite-rule mutation twice — once in Rust, once in Python — and require the same answer

  This case is a cross-language differential. The independent `🐍️.py` oracle reads the
  declared five-member typed rule: workingGraph, lhs, rhs, parameterBindings and ruleLayout.
  Its workingGraph retains the complete Jack parent and an explicitly declared Semio graph child;
  the child is a separate input with the exact s.stdio.semio/v1/graph dialect. The LHS and RHS are
  typed pattern, assignment and parameter records. Geometry keeps binary64 words, and intrinsic
  child properties keep all nine variants, ordered member occurrences and numeric lexemes.

  The document and all nine mutation payloads use their closed schemas. The Python oracle
  imports no Rust implementation. Strict third-party Ajv schema checks and independent BunSQLite
  child round trips run in the registered Source laws. These scenarios additionally compare actual
  Rust and Python mutation results, declared before/after owners and exact inverse restoration.

  🏗️ **The artifact is real, and it is now the whole building.**
  `shared://♻️mutate-rewrite-1/🔣️.snapshot.json` carries a before-fixture of 180 real pieces, 364
  real ports and 179 real connections in its explicitly declared full Semio child —
  derived ONCE by `🐍️derive-rewriting-fixture.py` in the ticket folder from the real committed IFC 4
  file `../../../🗄️stdio/🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc`, read with
  **IfcOpenShell 0.8.4**. That is the SAME real data the two-piece rule already carried, continued
  rather than replaced: the committed ground-floor document's node ids ARE that file's real
  `ComposePieceAttributes.composeGuid` values, its port ids ARE its real
  `ComposeConnector.composeConnectorId` values, and the derived graph's FIRST edge reproduces the
  committed one address for address. Each node carries its real `composeGuid`, its real name, its
  real placement translation in metres as `properties.position` and that translation's real `z` as
  `properties.tier`; each port carries its real connector id and a direction read off the real
  connection graph (`out` where the file uses it as an `IfcRelConnectsPorts` `RelatingPort`, `in`
  where it uses it as the `RelatedPort`, `inOut` for the six the file connects in neither
  direction); each edge carries the real `ComposeConnectionParams` `rotation`/`shift` the file
  records for the connected capsule.

  TWO values are not in the IFC and are carried from the committed document itself rather than
  invented, which is stated here rather than left to be discovered: a node's editor box
  `width`/`height` (96×48 for the root piece, 88×40 for a capsule — the committed document's own two
  values for exactly those two roles) and the `camera`. Everything else is real IFC data.

  `shared://♻️mutate-rewrite-1/🏢️nakagin-ground-floor/🔣️.snapshot.json` — the two-node rule this case used to rest on,
  derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-rewrite-rule.py`
  from the artifact's own committed demo document — is NOT gone: `identity-round-trip` still reads it
  beside the whole building and still holds it to its own shape, and additionally requires the two to
  name the same root piece, so a derivation that had drifted off the real model would be red.

  Every scenario declares both parent and child inputs. Whole working-graph replacement also
  declares its replacement child. No handler discovers a sibling input, reconstructs a reduced
  node/edge carrier or parses structured rule meaning from JSON strings. Relative working-graph
  edits are child-lane leaves of the shared Semio graph vocabulary (design §20.15), owned by its
  own cases, never a rewriting parent leaf.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations. Unlike this plugin's
  `🔌️jack` sibling, all nine of them are ACCEPTING, so the accepting direction here already had
  committed evidence; what the real-document rows add is the same nine verbs against a rule whose
  before-fixture is a real graph rather than a two-node sketch.

  Both implementations assert that each verb writes exactly one of the five semantic members.
  The working-graph replacement includes the retained full child in that comparison. Inverse application
  must restore the complete original parent and child, including identifiers, manifests, camera,
  dimensions, endpoint components and intrinsic occurrences.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived Nakagin Capsule Tower rule
    Given the real derived rule shared://♻️mutate-rewrite-1/🔣️.snapshot.json
    And the real derived child shared://♻️mutate-rewrite-1/🪆️content/🔣️.snapshot.json
    And the replacement working child shared://♻️mutate-rewrite-1/🔁️core-only/🪆️content/🔣️.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same rule, and only the member this verb writes moved
    Examples:
      | id                        | mutation                                                                                                                                                                                                             |
      | edit-before-fixture       | {"mutation":"editBeforeFixture","newWorkingGraph":{"schema":"trinity.graph","name":"Nakagin Capsule Tower — Core Only","manifestId":"nakagin","manifest":{"nodeKinds":[{"name":"Balcony","properties":[],"portKinds":[]},{"name":"Base","properties":[],"portKinds":["core rectangular bottom"]},{"name":"Base Blob","properties":[],"portKinds":["core circular bottom"]},{"name":"Bridge","properties":[],"portKinds":["platform right","platform left"]},{"name":"Capital","properties":[],"portKinds":["roof rectangular top"]},{"name":"Capsule","properties":[],"portKinds":[]},{"name":"Capsule Backslash","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule J","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule L","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule P","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule q","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule S","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule Slash","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony Backslash","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony J","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony L","properties":[],"portKinds":["door capsule left"]},{"name":"Capsule With Balcony P","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony Q","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony S","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony Slash","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony Z","properties":[],"portKinds":["door capsule left"]},{"name":"Capsule Z","properties":[],"portKinds":["door capsule left"]},{"name":"Cylindric Capital","properties":[],"portKinds":["roof circular top"]},{"name":"Cylindric First Storey Tambour","properties":[],"portKinds":["core circular top","tambour circular top","door tambour right","door tambour left"]},{"name":"Cylindric Last Storey Tambour","properties":[],"portKinds":["tambour circular bottom","roof circular bottom","door tambour right","door tambour left"]},{"name":"Cylindric Single Storey Tambour","properties":[],"portKinds":["core circular top","roof circular bottom","door tambour right","door tambour left"]},{"name":"Cylindric Tambour","properties":[],"portKinds":["tambour circular bottom","tambour circular top","door tambour right","door tambour left"]},{"name":"Ellipsoid","properties":[],"portKinds":[]},{"name":"First Storey Tambour","properties":[],"portKinds":["core rectangular top","tambour rectangular top","door tambour right","door tambour left"]},{"name":"Last Storey Tambour","properties":[],"portKinds":["tambour rectangular bottom","roof rectangular bottom","door tambour right","door tambour left"]},{"name":"Single Storey Tambour","properties":[],"portKinds":["core circular top","roof rectangular bottom","door tambour right","door tambour left"]},{"name":"Tambour","properties":[],"portKinds":["tambour rectangular bottom","tambour rectangular top","door tambour right","door tambour left"]},{"name":"Trapezoid","properties":[],"portKinds":[]},{"name":"Trapezoid Capsule Backslash","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule J","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule L","properties":[],"portKinds":["door capsule left"]},{"name":"Trapezoid Capsule P","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule Q","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule S","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule Slash","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule Z","properties":[],"portKinds":["door capsule left"]},{"name":"Piece","properties":[{"name":"position","kind":"data","valueType":{"kind":"any"}},{"name":"label","kind":"data","valueType":{"kind":"text"}},{"name":"tier","kind":"data","valueType":{"kind":"decimal"}}],"portKinds":["Connector"]}],"edgeKinds":[{"name":"Connection","properties":[{"name":"gap","kind":"data","valueType":{"kind":"decimal"}},{"name":"rotation","kind":"data","valueType":{"kind":"decimal"}},{"name":"tilt","kind":"data","valueType":{"kind":"decimal"}},{"name":"rise","kind":"data","valueType":{"kind":"decimal"}},{"name":"turn","kind":"data","valueType":{"kind":"decimal"}},{"name":"shift","kind":"data","valueType":{"kind":"decimal"}},{"name":"u","kind":"data","valueType":{"kind":"decimal"}},{"name":"v","kind":"data","valueType":{"kind":"decimal"}}]},{"name":"edge.link","properties":[]}],"portKinds":[{"name":"Connector","direction":"out","properties":[]}]},"camera":{"x":{"bits":"0000000000000000"},"y":{"bits":"0000000000000000"},"zoom":{"bits":"3ff0000000000000"}},"content":{"childId":"nakagin-core-only-content","target":{"artifactId":"nakagin-core-only-content","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"graph"}}},"rootNodeId":"","query":""}} |
      | edit-lhs                  | {"mutation":"editLhs","newLhs":{"pattern":{"leftVar":"a","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"b","rightKind":"Piece"},"whereClause":"a.tier = 0"}} |
      | edit-rhs                  | {"mutation":"editRhs","newRhs":{"create":[],"delete":[],"set":[{"var":"a","prop":"label","value":{"kind":"string","value":"$label"}},{"var":"b","prop":"tier","value":{"kind":"string","value":"$tier"}}],"merge":[],"parameters":[{"name":"label","kind":"string","default":{"kind":"string","value":"nakagin-core"}},{"name":"tier","kind":"number","default":{"kind":"number","value":{"bits":"3ff0000000000000"}}}]}} |
      | change-parameter-binding  | {"mutation":"changeParameterBinding","key":"label","newValue":{"kind":"string","value":"nakagin-core-rev-b"}} |
      | remove-parameter-binding  | {"mutation":"removeParameterBinding","key":"label"}                                                                                                                                                                  |
      | change-rule-layout-point  | {"mutation":"changeRuleLayoutPoint","key":"b","newPoint":{"x":-96.5,"y":112.25}}                                                                                                                                     |
      | remove-rule-layout-point  | {"mutation":"removeRuleLayoutPoint","key":"a"}                                                                                                                                                                       |
      | drag-rule-nodes           | {"mutation":"dragRuleNodes","targets":["lhs-where","rhs-set-0"],"dx":30.0,"dy":-20.0} |
      | set-rule-layout-points    | {"mutation":"setRuleLayoutPoints","points":[{"key":"lhs-match","x":15.5,"y":-4.0}],"cleared":["a"]} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived rule and land back on it
    Given the real derived rule shared://♻️mutate-rewrite-1/🔣️.snapshot.json
    And the real derived child shared://♻️mutate-rewrite-1/🪆️content/🔣️.snapshot.json
    And the replacement working child shared://♻️mutate-rewrite-1/🔁️core-only/🪆️content/🔣️.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated rule AND on the restored one, member for member
    Examples:
      | id                        | mutation                                                                                                                                                                                                             |
      | edit-before-fixture       | {"mutation":"editBeforeFixture","newWorkingGraph":{"schema":"trinity.graph","name":"Nakagin Capsule Tower — Core Only","manifestId":"nakagin","manifest":{"nodeKinds":[{"name":"Balcony","properties":[],"portKinds":[]},{"name":"Base","properties":[],"portKinds":["core rectangular bottom"]},{"name":"Base Blob","properties":[],"portKinds":["core circular bottom"]},{"name":"Bridge","properties":[],"portKinds":["platform right","platform left"]},{"name":"Capital","properties":[],"portKinds":["roof rectangular top"]},{"name":"Capsule","properties":[],"portKinds":[]},{"name":"Capsule Backslash","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule J","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule L","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule P","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule q","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule S","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule Slash","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony Backslash","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony J","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony L","properties":[],"portKinds":["door capsule left"]},{"name":"Capsule With Balcony P","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony Q","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony S","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony Slash","properties":[],"portKinds":["door capsule right"]},{"name":"Capsule With Balcony Z","properties":[],"portKinds":["door capsule left"]},{"name":"Capsule Z","properties":[],"portKinds":["door capsule left"]},{"name":"Cylindric Capital","properties":[],"portKinds":["roof circular top"]},{"name":"Cylindric First Storey Tambour","properties":[],"portKinds":["core circular top","tambour circular top","door tambour right","door tambour left"]},{"name":"Cylindric Last Storey Tambour","properties":[],"portKinds":["tambour circular bottom","roof circular bottom","door tambour right","door tambour left"]},{"name":"Cylindric Single Storey Tambour","properties":[],"portKinds":["core circular top","roof circular bottom","door tambour right","door tambour left"]},{"name":"Cylindric Tambour","properties":[],"portKinds":["tambour circular bottom","tambour circular top","door tambour right","door tambour left"]},{"name":"Ellipsoid","properties":[],"portKinds":[]},{"name":"First Storey Tambour","properties":[],"portKinds":["core rectangular top","tambour rectangular top","door tambour right","door tambour left"]},{"name":"Last Storey Tambour","properties":[],"portKinds":["tambour rectangular bottom","roof rectangular bottom","door tambour right","door tambour left"]},{"name":"Single Storey Tambour","properties":[],"portKinds":["core circular top","roof rectangular bottom","door tambour right","door tambour left"]},{"name":"Tambour","properties":[],"portKinds":["tambour rectangular bottom","tambour rectangular top","door tambour right","door tambour left"]},{"name":"Trapezoid","properties":[],"portKinds":[]},{"name":"Trapezoid Capsule Backslash","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule J","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule L","properties":[],"portKinds":["door capsule left"]},{"name":"Trapezoid Capsule P","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule Q","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule S","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule Slash","properties":[],"portKinds":["door capsule right"]},{"name":"Trapezoid Capsule Z","properties":[],"portKinds":["door capsule left"]},{"name":"Piece","properties":[{"name":"position","kind":"data","valueType":{"kind":"any"}},{"name":"label","kind":"data","valueType":{"kind":"text"}},{"name":"tier","kind":"data","valueType":{"kind":"decimal"}}],"portKinds":["Connector"]}],"edgeKinds":[{"name":"Connection","properties":[{"name":"gap","kind":"data","valueType":{"kind":"decimal"}},{"name":"rotation","kind":"data","valueType":{"kind":"decimal"}},{"name":"tilt","kind":"data","valueType":{"kind":"decimal"}},{"name":"rise","kind":"data","valueType":{"kind":"decimal"}},{"name":"turn","kind":"data","valueType":{"kind":"decimal"}},{"name":"shift","kind":"data","valueType":{"kind":"decimal"}},{"name":"u","kind":"data","valueType":{"kind":"decimal"}},{"name":"v","kind":"data","valueType":{"kind":"decimal"}}]},{"name":"edge.link","properties":[]}],"portKinds":[{"name":"Connector","direction":"out","properties":[]}]},"camera":{"x":{"bits":"0000000000000000"},"y":{"bits":"0000000000000000"},"zoom":{"bits":"3ff0000000000000"}},"content":{"childId":"nakagin-core-only-content","target":{"artifactId":"nakagin-core-only-content","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"graph"}}},"rootNodeId":"","query":""}} |
      | edit-lhs                  | {"mutation":"editLhs","newLhs":{"pattern":{"leftVar":"a","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"b","rightKind":"Piece"},"whereClause":"a.tier = 0"}} |
      | edit-rhs                  | {"mutation":"editRhs","newRhs":{"create":[],"delete":[],"set":[{"var":"a","prop":"label","value":{"kind":"string","value":"$label"}},{"var":"b","prop":"tier","value":{"kind":"string","value":"$tier"}}],"merge":[],"parameters":[{"name":"label","kind":"string","default":{"kind":"string","value":"nakagin-core"}},{"name":"tier","kind":"number","default":{"kind":"number","value":{"bits":"3ff0000000000000"}}}]}} |
      | change-parameter-binding  | {"mutation":"changeParameterBinding","key":"label","newValue":{"kind":"string","value":"nakagin-core-rev-b"}} |
      | remove-parameter-binding  | {"mutation":"removeParameterBinding","key":"label"}                                                                                                                                                                  |
      | change-rule-layout-point  | {"mutation":"changeRuleLayoutPoint","key":"b","newPoint":{"x":-96.5,"y":112.25}}                                                                                                                                     |
      | remove-rule-layout-point  | {"mutation":"removeRuleLayoutPoint","key":"a"}                                                                                                                                                                       |
      | drag-rule-nodes           | {"mutation":"dragRuleNodes","targets":["lhs-where","rhs-set-0"],"dx":30.0,"dy":-20.0} |
      | set-rule-layout-points    | {"mutation":"setRuleLayoutPoints","points":[{"key":"lhs-match","x":15.5,"y":-4.0}],"cleared":["a"]} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-rule shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed before-child shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🪆️child/🔣️.json
    And the committed mutation shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-rule shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed after-child shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🪆️child/🔣️.json
    When the committed mutation is applied to the committed before-rule
    Then each implementation lands on the committed after-rule in role, only the member this verb writes moved, and the two agree
    Examples:
      | id                        | dir                         | fixture                                       |
      | edit-before-fixture       | 🖼️edit-before-fixture        | 🕸️swaps              |
      | edit-lhs                  | 👈️edit-lhs                  | 👈️narrows  |
      | edit-rhs                  | 👉️edit-rhs                  | 👉️rewrites     |
      | change-parameter-binding  | 🔧️change-parameter  | 🏷️retitles                  |
      | remove-parameter-binding  | 🧹️remove-parameter-binding  | ✂️drops                      |
      | change-rule-layout-point  | 📐️change-rule-layout  | 📍️nudges          |
      | remove-rule-layout-point  | 🗑️remove-rule-layout  | 📐️clears                 |
      | drag-rule-nodes           | 🫳️drag-rule  | 🫳️moves  |
      | set-rule-layout-points    | 📍️set-rule-layout  | 📍️places  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read both real derived rules in both languages, and hold the committed carrier to its own law in Rust
    Given the real derived rule shared://♻️mutate-rewrite-1/🔣️.snapshot.json
    And the real derived child shared://♻️mutate-rewrite-1/🪆️content/🔣️.snapshot.json
    And the two-node ground-floor rule this case used to rest on shared://♻️mutate-rewrite-1/🏢️nakagin-ground-floor/🔣️.snapshot.json
    And the two-node ground-floor child shared://♻️mutate-rewrite-1/🏢️nakagin-ground-floor/🪆️content/🔣️.snapshot.json
    And the artifact's own committed carrier asset://🎬️demo/🗣️.dsl.semio
    When each implementation reads both derived rules, and the Rust additionally parses the committed carrier, prints it back and parses it again
    Then both languages read the same five members of each rule, and the Rust reproduces the committed carrier byte for byte
