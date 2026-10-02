@capability-rewriting-1-mutate
@oracle-rewriting-python-independent
@comparison-ordered-json-v1
@mutations-rewriting-1-any
Feature: Apply every typed graph-rewrite-rule mutation twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.trinity.rewriting` rule document and all fourteen typed mutations,
  written in Python from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` (the
  document: three whole JSON documents carried as `contentMediaType: application/json` strings, plus
  a map of parameter bindings and a map of `{x, y}` layout points, `additionalProperties: false`),
  from `…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (the fourteen verbs and their argument
  lists) and from the fourteen committed specification vectors. It imports nothing from this
  repository's Rust.

  Why a second implementation rather than a third-party library. A rewrite rule here is not a
  rewriting ENGINE, it is the rule DOCUMENT: a before-fixture graph, a left-hand pattern, a
  right-hand side, the parameter bindings and the editor layout of the rule's variables. GrGen, AGG
  and `networkx`'s isomorphism module implement rewriting and have no model of this document, no
  reader for `.dsl.semio`, and no opinion on whether `change-parameter-binding` on an absent key
  inserts or refuses. What a reference can genuinely adjudicate is the document algebra — three
  whole-value setters and a set/remove pair over each of two maps — and that is what this one does.

  🏗️ **The artifact is real, and it is now the whole building.**
  `shared://♻️mutate-rewrite-1/🔣️.snapshot.json` carries a before-fixture of 180 real pieces, 364
  real ports and 179 real connections — 218 839 bytes of `trinity.graph` inside a 246 269-byte rule —
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

  Why the Python reference does not read the carrier. `.rewriting.dsl.semio` has no prose document and
  mixes three different value encodings in one file — a backslash-escaped quoted string for
  `before-fixture-json` and `lhs-json`, a braced block for `parameter-bindings` and `rule-layout`,
  and a fenced ```json block for `rhs-json` — with nothing stating which member gets which. Guessing
  the rule and then claiming byte-exact reproduction would be asserting a specification that does not
  exist. The reading is therefore done once, in the derivation script above, and the carrier's own
  laws stay asserted in role on the Rust side in `identity-round-trip`.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted `(before, mutation, after)` triple through both implementations. Unlike this plugin's
  `🔌️jack` sibling, all fourteen of them are ACCEPTING, so the accepting direction here already had
  committed evidence; what the real-document rows add is the same fourteen verbs against a rule whose
  before-fixture is a real graph rather than a two-node sketch.

  Both implementations additionally assert, in role, that each verb writes exactly ONE of the five
  members. That is the check a comparison against an after-snapshot cannot make on its own: an
  implementation that re-serialized a JSON string member on every edit — reordering its keys or
  changing its whitespace — would still land on the right document for the member it meant to write
  while silently rewriting the other two.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived Nakagin Capsule Tower rule
    Given the real derived rule shared://♻️mutate-rewrite-1/🔣️.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same rule, and only the member this verb writes moved
    Examples:
      | id                        | mutation                                                                                                                                                                                                             |
      | edit-before-fixture       | {"mutation":"editBeforeFixture","newBeforeFixtureJson":"{\"schema\":\"trinity.graph\",\"name\":\"Nakagin Capsule Tower — Core Only\",\"manifestId\":\"nakagin\",\"camera\":{\"x\":0.0,\"y\":0.0,\"zoom\":1.0},\"nodes\":[],\"edges\":[],\"rootNodeId\":\"\"}"} |
      | edit-lhs                  | {"mutation":"editLhs","newLhsJson":"{\"pattern\":{\"leftVar\":\"a\",\"leftKind\":\"Piece\",\"edgeVar\":\"r\",\"edgeKind\":\"Connection\",\"rightVar\":\"b\",\"rightKind\":\"Piece\"},\"whereClause\":\"a.tier = 0\"}"}  |
      | edit-rhs                  | {"mutation":"editRhs","newRhsJson":"{\"create\":[],\"delete\":[],\"set\":[{\"var\":\"a\",\"prop\":\"label\",\"value\":\"$label\"},{\"var\":\"b\",\"prop\":\"tier\",\"value\":\"$tier\"}],\"merge\":[],\"parameters\":[{\"name\":\"label\",\"kind\":\"string\",\"default\":\"nakagin-core\"},{\"name\":\"tier\",\"kind\":\"number\",\"default\":1}]}"} |
      | change-parameter-binding  | {"mutation":"changeParameterBinding","key":"label","newValue":"nakagin-core-rev-b"}                                                                                                                                  |
      | remove-parameter-binding  | {"mutation":"removeParameterBinding","key":"label"}                                                                                                                                                                  |
      | change-rule-layout-point  | {"mutation":"changeRuleLayoutPoint","key":"b","newPoint":{"x":-96.5,"y":112.25}}                                                                                                                                     |
      | remove-rule-layout-point  | {"mutation":"removeRuleLayoutPoint","key":"a"}                                                                                                                                                                       |
      | drag-working-nodes        | {"mutation":"dragWorkingNodes","targets":["7dc5b737-3b6b-4068-b315-b7bacc91c2e1","6947a41b-8c6d-4291-bdd8-96cd535c78fc"],"dx":24.0,"dy":-12.5} |
      | patch-working-nodes       | {"mutation":"patchWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc"],"field":"name","value":"ci_t_f8_b_c0_renamed"} |
      | drag-rule-nodes           | {"mutation":"dragRuleNodes","targets":["lhs-where","rhs-set-0"],"dx":30.0,"dy":-20.0} |
      | set-rule-layout-points    | {"mutation":"setRuleLayoutPoints","points":[{"key":"lhs-match","x":15.5,"y":-4.0}],"cleared":["a"]} |
      | delete-working-nodes      | {"mutation":"deleteWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc","17d5dec8-87b2-44a9-84ff-93b7e7419bdd"]} |
      | connect-working-ports     | {"mutation":"connectWorkingPorts","source":"6947a41b-8c6d-4291-bdd8-96cd535c78fc@4ba51a88-2a7f-4b78-b119-0f02eacdb702","target":"9fde3a12-8b39-42d9-850f-f0e8343caddd@4ba51a88-2a7f-4b78-b119-0f02eacdb702","kind":"Connection"} |
      | disconnect-working-edges  | {"mutation":"disconnectWorkingEdges","targets":["2jGlFQA9H2mvmjiNpnYG5Q","2aA4mq3Qj0XRbikPznzENT"]} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived rule and land back on it
    Given the real derived rule shared://♻️mutate-rewrite-1/🔣️.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated rule AND on the restored one, member for member
    Examples:
      | id                        | mutation                                                                                                                                                                                                             |
      | edit-before-fixture       | {"mutation":"editBeforeFixture","newBeforeFixtureJson":"{\"schema\":\"trinity.graph\",\"name\":\"Nakagin Capsule Tower — Core Only\",\"manifestId\":\"nakagin\",\"camera\":{\"x\":0.0,\"y\":0.0,\"zoom\":1.0},\"nodes\":[],\"edges\":[],\"rootNodeId\":\"\"}"} |
      | edit-lhs                  | {"mutation":"editLhs","newLhsJson":"{\"pattern\":{\"leftVar\":\"a\",\"leftKind\":\"Piece\",\"edgeVar\":\"r\",\"edgeKind\":\"Connection\",\"rightVar\":\"b\",\"rightKind\":\"Piece\"},\"whereClause\":\"a.tier = 0\"}"}  |
      | edit-rhs                  | {"mutation":"editRhs","newRhsJson":"{\"create\":[],\"delete\":[],\"set\":[{\"var\":\"a\",\"prop\":\"label\",\"value\":\"$label\"},{\"var\":\"b\",\"prop\":\"tier\",\"value\":\"$tier\"}],\"merge\":[],\"parameters\":[{\"name\":\"label\",\"kind\":\"string\",\"default\":\"nakagin-core\"},{\"name\":\"tier\",\"kind\":\"number\",\"default\":1}]}"} |
      | change-parameter-binding  | {"mutation":"changeParameterBinding","key":"label","newValue":"nakagin-core-rev-b"}                                                                                                                                  |
      | remove-parameter-binding  | {"mutation":"removeParameterBinding","key":"label"}                                                                                                                                                                  |
      | change-rule-layout-point  | {"mutation":"changeRuleLayoutPoint","key":"b","newPoint":{"x":-96.5,"y":112.25}}                                                                                                                                     |
      | remove-rule-layout-point  | {"mutation":"removeRuleLayoutPoint","key":"a"}                                                                                                                                                                       |
      | drag-working-nodes        | {"mutation":"dragWorkingNodes","targets":["7dc5b737-3b6b-4068-b315-b7bacc91c2e1","6947a41b-8c6d-4291-bdd8-96cd535c78fc"],"dx":24.0,"dy":-12.5} |
      | patch-working-nodes       | {"mutation":"patchWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc"],"field":"name","value":"ci_t_f8_b_c0_renamed"} |
      | drag-rule-nodes           | {"mutation":"dragRuleNodes","targets":["lhs-where","rhs-set-0"],"dx":30.0,"dy":-20.0} |
      | set-rule-layout-points    | {"mutation":"setRuleLayoutPoints","points":[{"key":"lhs-match","x":15.5,"y":-4.0}],"cleared":["a"]} |
      | delete-working-nodes      | {"mutation":"deleteWorkingNodes","targets":["6947a41b-8c6d-4291-bdd8-96cd535c78fc","17d5dec8-87b2-44a9-84ff-93b7e7419bdd"]} |
      | connect-working-ports     | {"mutation":"connectWorkingPorts","source":"6947a41b-8c6d-4291-bdd8-96cd535c78fc@4ba51a88-2a7f-4b78-b119-0f02eacdb702","target":"9fde3a12-8b39-42d9-850f-f0e8343caddd@4ba51a88-2a7f-4b78-b119-0f02eacdb702","kind":"Connection"} |
      | disconnect-working-edges  | {"mutation":"disconnectWorkingEdges","targets":["2jGlFQA9H2mvmjiNpnYG5Q","2aA4mq3Qj0XRbikPznzENT"]} |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-rule shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-rule shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
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
      | drag-working-nodes        | ✋️drag-working  | ✋️moves  |
      | patch-working-nodes       | 🩹️patch-working  | 🩹️renames  |
      | drag-rule-nodes           | 🫳️drag-rule  | 🫳️moves  |
      | set-rule-layout-points    | 📍️set-rule-layout  | 📍️places  |
      | delete-working-nodes      | ✂️delete-working  | ✂️deletes  |
      | connect-working-ports     | 🔌️connect-working | 🔌️connects |
      | disconnect-working-edges  | 🪚️disconnect-working | 🪚️cuts |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read both real derived rules in both languages, and hold the committed carrier to its own law in Rust
    Given the real derived rule shared://♻️mutate-rewrite-1/🔣️.snapshot.json
    And the two-node ground-floor rule this case used to rest on shared://♻️mutate-rewrite-1/🏢️nakagin-ground-floor/🔣️.snapshot.json
    And the artifact's own committed carrier asset://🎬️demo/🗣️.dsl.semio
    When each implementation reads both derived rules, and the Rust additionally parses the committed carrier, prints it back and parses it again
    Then both languages read the same five members of each rule, and the Rust reproduces the committed carrier byte for byte
