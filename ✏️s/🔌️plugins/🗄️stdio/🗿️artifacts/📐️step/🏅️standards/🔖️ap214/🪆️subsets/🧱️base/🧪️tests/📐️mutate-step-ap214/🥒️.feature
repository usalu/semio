@capability-step-ap214-base-mutate
@oracle-ruststep-step-ap214-base-mutate
@comparison-semantic-step-v1
@mutations-step-ap214-base
Feature: Apply every typed STEP AP214 mutation to a real-world exchange structure
  The input is a real ISO 10303-21 exchange structure derived once from a real committed BIM export,
  not a synthetic fixture: `shared://🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp`, whose own header
  comment records the exact derivation. Its DATA section (1396 real entities: 449 CARTESIAN_POINT,
  126 B_SPLINE_CURVE_WITH_KNOTS, 71 VERTEX_POINT, 57 each of ADVANCED_FACE/PLANE/EDGE_LOOP/
  FACE_OUTER_BOUND, one MANIFOLD_SOLID_BREP/CLOSED_SHELL, plus the real PRODUCT/PRODUCT_DEFINITION
  management chain) is byte-for-byte the real, untouched DATA section of the real committed
  `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.stp` (a real Rhino
  8.31 / ST-Developer v19.2 export of the real "hexagonal cut concrete forest" structure).

  FINDING: every real `.stp` file committed under `♻️mit-bestand/🖼️asset/🏚️abbau-aufbau` (all five of
  them, including the ticket-designated `-bim.stp` one) declares
  `FILE_SCHEMA(('AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF {...}'))`, confirmed by reading each
  header directly, not AP214 -- and no git-tracked real-world AP214 (`AUTOMOTIVE_DESIGN`) file exists
  anywhere in this repository. That entity set is drawn entirely from ISO 10303-214's own
  common-resource advanced-B-rep-shape-representation schema -- the same resources AP214's
  `AUTOMOTIVE_DESIGN` declares -- with zero AP242-only PMI/GD&T/kinematics entities present anywhere
  in the source, so this fixture's single edit is its `FILE_SCHEMA` line, changed from the source's
  real AP242 declaration to `AUTOMOTIVE_DESIGN` so it is genuinely, exactly conformant with the
  standard this subset targets. Every entity id, coordinate, curve, topology relationship and product
  record is real and untouched. Every scenario copies the fixture into the case work directory before
  touching it; the committed asset is never written to.

  ANOTHER FINDING (this wave's assigned reference, established before writing any scenario below):
  `ruststep` 0.4 (newly linked this wave) is a real ISO 10303-21 reader -- its `ast::Exchange`,
  `ast::DataSection`, `ast::Record`, `ast::Parameter` and `ast::EntityInstance` all implement
  `FromStr` over the genuine Part-21 clear-text grammar, and it parsed this real fixture's 1396
  entities with zero errors. It has NO writer at all: reading its own source finds no `Display`/
  `fmt::Formatter` impl anywhere on those AST types, and `ast::ser::to_record` only builds an
  in-memory `Record` from an already-typed Rust struct (moot here anyway, since ruststep compiles no
  generated schema module for AP214 at all -- only `ap201`/`ap203` are feature-gated in, and neither
  is enabled). Per the fleet brief's §6, this rules out `@mode-differential`: ruststep cannot be a
  second PRODUCER of mutated bytes to diff the subject against. It is registered as the INDEPENDENT
  READER instead: every mutation this subset's own oracle dispatcher performs (`ruststep` parses the
  real input; this subset's own from-scratch Part-21 writer, in `../../🏅️standards/🔖️ap214/🪆️subsets/
  🧱️base/🔮️oracle/🦀️component.rs`, re-serializes it, since ruststep has nothing to reuse for that half)
  is read back through a FRESH, independent `ruststep::ast::Exchange::from_str` call
  (`project_step_ap214_any`) before `semantic-step-v1` compares it, and the identity round trip is
  checked the same way. That is real third-party evidence about the entity graph's structure, argument
  values and declared `FILE_SCHEMA` as ruststep's own parser sees them -- but it cannot discharge a
  byte-level differential claim, which every scenario below is typed to reflect honestly rather than
  claim.


  THE FIRST DIFFERENTIAL RUN OF THIS CASE DIVERGED ON EVERY SINGLE SCENARIO, AND THE PROJECTION WAS
  WHAT WAS WRONG. Both roles passed their own laws (`executed=46 passed=46`) and yet the two producers still disagreed:
  `$.entities[824].args[3].v` read `…at asserted c<LF>onnectivities` from the oracle and `…at
  asserted c\X2\000A\X0\onnectivities` from the subject. That is ONE value spelled two ways.
  ST-Developer wrapped its output line INSIDE that string literal, so the real committed fixture
  carries a raw line break there; this repository's own writer decodes it and re-emits it as the
  conformant `\X2\000A\X0\` control directive, which ISO 10303-21 §6.4.2 defines and a raw
  0x0A inside a string literal is not. Neither writer is wrong about the VALUE. What was wrong is
  that `ruststep`'s own `string` combinator is `many0(none_of("'"))` — it hands back the raw text
  between the apostrophes and decodes no control directive at all (its own doc comment quotes the
  production it does not implement) — so the projection was comparing ENCODINGS while calling them
  argument values. The oracle now decodes every string literal to the value it denotes, through a
  from-scratch §6.4.2 reader in the shared `../../🏅️standards/🔖️ap214/🔮️oracle/🦀️component.rs`
  written independently of the production codec under test, pinned by
  `every_control_directive_decodes_to_the_value_it_denotes` and
  `a_malformed_or_unmappable_directive_is_refused`. A malformed directive is an ERROR, not a lexeme
  waved through, so a subject that emitted a broken escape still fails. No `ignoreKeys` was added,
  no tolerance loosened, no fixture touched: `semantic-step-v1` is compared over exactly the same
  axes, on decoded values instead of lexemes.

  📌️ Every Examples row below other than `no-mutation` is required to MOVE the semantic projection,
  and the adapter fails the scenario in role when it does not: a row whose parameters make the
  mutation a no-op passes whenever the reference library merely declined to error, which is not a
  test. The baseline it is measured against runs one `no-mutation` cycle first, so the comparison
  isolates the mutation rather than the writer's own normal form.
  @id-mutate
  @level-exhaustive
  @mode-property
  Scenario Outline: Apply <id> to the real exchange structure
    Given the real input document shared://🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the independently read projection shows the mutation's real effect
    Examples:
      | id                    | params                                                                                                                                                                          |
      | set-snapshot          | {"fileSchema": ["CONFIG_CONTROL_DESIGN"]}                                                                                                                                      |
      | set-file-description  | {"fileDescription": {"description": ["ticket 26/08/23 wave-7 mutation"], "implementationLevel": "2;1"}}                                                                       |
      | set-file-name         | {"fileName": {"name": "wave-7-mutated", "timestamp": "2026-08-23T00:00:00", "author": ["Ueli"], "organization": ["semio"], "preprocessorVersion": "semio-step", "originatingSystem": "semio", "authorization": ""}} |
      | set-file-schema       | {"fileSchema": {"schemas": ["CONFIG_CONTROL_DESIGN"]}}                                                                                                                         |
      | insert-entity         | {"index": 1396, "entity": {"id": 9001, "name": "CARTESIAN_POINT", "args": [{"t": "string", "v": ""}, {"t": "aggregate", "v": [{"t": "real", "v": 1.0}, {"t": "real", "v": 2.0}, {"t": "real", "v": 3.0}]}]}} |
      | remove-entity         | {"id": 1405}                                                                                                                                                                    |
      | set-entity-name       | {"id": 1394, "name": "RENAMED_POINT"}                                                                                                                                          |
      | set-entity-arg        | {"id": 1394, "argIndex": 0, "value": {"t": "string", "v": "origin-marker"}}                                                                                                    |
      | insert-entity-arg     | {"id": 1394, "argIndex": 2, "value": {"t": "enum", "v": "T"}}                                                                                                                  |
      | remove-entity-arg     | {"id": 1394, "argIndex": 1}                                                                                                                                                     |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-property
  Scenario: Apply no-mutation to the real exchange structure
    Given the real input document shared://🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the independently read projection shows the mutation's real effect

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real exchange structure
    Given the real input document shared://🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the independently read projection matches its pre-mutation projection
    Examples:
      | id                    | params                                                                                                                                                                          |
      | set-snapshot          | {"fileSchema": ["CONFIG_CONTROL_DESIGN"]}                                                                                                                                      |
      | set-file-description  | {"fileDescription": {"description": ["ticket 26/08/23 wave-7 mutation"], "implementationLevel": "2;1"}}                                                                       |
      | set-file-name         | {"fileName": {"name": "wave-7-mutated", "timestamp": "2026-08-23T00:00:00", "author": ["Ueli"], "organization": ["semio"], "preprocessorVersion": "semio-step", "originatingSystem": "semio", "authorization": ""}} |
      | set-file-schema       | {"fileSchema": {"schemas": ["CONFIG_CONTROL_DESIGN"]}}                                                                                                                         |
      | insert-entity         | {"index": 1396, "entity": {"id": 9001, "name": "CARTESIAN_POINT", "args": [{"t": "string", "v": ""}, {"t": "aggregate", "v": [{"t": "real", "v": 1.0}, {"t": "real", "v": 2.0}, {"t": "real", "v": 3.0}]}]}} |
      | remove-entity         | {"id": 1405}                                                                                                                                                                    |
      | set-entity-name       | {"id": 1394, "name": "RENAMED_POINT"}                                                                                                                                          |
      | set-entity-arg        | {"id": 1394, "argIndex": 0, "value": {"t": "string", "v": "origin-marker"}}                                                                                                    |
      | insert-entity-arg     | {"id": 1394, "argIndex": 2, "value": {"t": "enum", "v": "T"}}                                                                                                                  |
      | remove-entity-arg     | {"id": 1394, "argIndex": 1}                                                                                                                                                     |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real exchange structure
    Given the real input document shared://🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    And the mutation's own inverse is applied to the result
    Then the independently read projection matches its pre-mutation projection

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real exchange structure without passing bytes through
    Given the real input document shared://🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the independently read projections of input and output agree
