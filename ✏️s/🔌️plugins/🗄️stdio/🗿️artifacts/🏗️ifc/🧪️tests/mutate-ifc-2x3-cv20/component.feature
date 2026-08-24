@capability-ifc-2x3-cv20-mutate
@oracle-ruststep-ifc-2x3-cv20-mutate
@comparison-semantic-ifc-v1
@mutations-ifc-2x3-cv20
Feature: Apply every typed Coordination View 2.0 mutation to a real IFC2X3 building model
  The input is `shared://🏗️wellness-center-sama-street-level.ifc`, a real self-consistent 3464-entity
  slice of a genuine 21 MB IFC2X3 building model (an EDM StepFileFactory export, 2021). It is the
  only real IFC2X3 file in this repository and it is genuinely a document of THIS subset's own model
  view definition: its `FILE_DESCRIPTION` declares `ViewDefinition [CoordinationView_V2.0]` and its
  `FILE_SCHEMA` declares `IFC2X3`, both read directly from the committed bytes. Every scenario copies
  it into the case work directory before touching it; the committed asset is never written to.

  This subset's vocabulary is NOT the sibling `✳️any` subset's. `✳️any` declares generic ISO 10303-21
  graph editing — `upsert-instance`, `remove-instance`, `set-header` — and knows nothing about model
  view definitions. `Ifc2x3Cv20Mutation`
  (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🧬️schema/🧬️mutations/🦀️component.rs`) declares one kind
  per rule of the Coordination View 2.0 conformance gate this repository already implements in
  production (`check_cv20_conformance`): `set-snapshot` addresses `CODE_FILE_SCHEMA`,
  `set-view-definition` addresses `CODE_VIEW_DEFINITION`, `set-structural-entity` addresses
  `CODE_STRUCTURAL_ENTITY` (CV2.0's architectural scope excludes structural-analysis entities),
  `set-project-units` addresses `CODE_PROJECT_UNITS` and `set-product-placement` addresses
  `CODE_PRODUCT_PLACEMENT`. Each of those kinds carries an OPTIONAL payload — a value sets the
  concept, `null` clears it — so every kind is total in both directions and every inverse below is a
  real inverse rather than a whole-document restore.

  The ids in the tables are real. `#120` is the real `IFCPROJECT` whose `UnitsInContext` resolves to
  the real `IFCUNITASSIGNMENT` `#107`; `#270549` is the real `IFCWALLSTANDARDCASE` whose
  `ObjectPlacement` resolves to the real `IFCLOCALPLACEMENT` `#270529`. The real document carries
  ZERO structural-analysis entities, which is exactly what CV2.0 requires, so `set-structural-entity`
  inserts one at the unused id `#9000001` — a real violation of the MVD's own exclusion — and its
  inverse removes it again.

  `ruststep` 0.4 parses IFC2X3 exactly as it parses STEP AP214 (IFC2X3 is that same ISO 10303-21
  syntax under a different EXPRESS schema) but has NO writer at all. Per the fleet brief's §6 that
  rules out `@mode-differential`: ruststep cannot be a second PRODUCER of mutated bytes to diff the
  subject against. It is the INDEPENDENT READER instead — every mutation the oracle dispatcher
  performs (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️cv20/🧪️oracle/🦀️component.rs`, re-serializing through
  the standard-level from-scratch Part-21 writer at `../../🏅️standards/🔖️2x3/🧪️oracle/🦀️component.rs`
  rather than this repository's own production `step::engine::part21` writer, which would compare the
  implementation against itself) is read back through a FRESH `ruststep::ast::Exchange::from_str`
  call before `semantic-ifc-v1` compares it. That is real third-party evidence about the entity
  graph, the declared view definition and the three CV2.0 concepts the projection reports — but it
  cannot discharge a byte-level differential claim, which every scenario below is typed to reflect.


  📌️ Every Examples row below other than `no-mutation` is required to MOVE the semantic projection,
  and the adapter fails the scenario in role when it does not: a row whose parameters make the
  mutation a no-op passes whenever the reference library merely declined to error, which is not a
  test. The baseline it is measured against runs one `no-mutation` cycle first, so the comparison
  isolates the mutation rather than the writer's own normal form.
  @id-mutate
  @level-exhaustive
  @mode-property
  Scenario Outline: Apply <id> to the real Coordination View 2.0 model
    Given the real input document shared://🏗️wellness-center-sama-street-level.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the independently read projection shows the mutation's real effect
    Examples:
      | id                     | params                                                                                                                       |
      | no-mutation            | {}                                                                                                                           |
      | set-snapshot           | {"fileSchema": ["IFC2X3", "IFC2X3-CV20-MARKER"]}                                                                             |
      | set-view-definition    | {"view": "StructuralAnalysisView"}                                                                                           |
      | set-structural-entity  | {"id": 9000001, "entity": {"typeName": "IFCSTRUCTURALANALYSISMODEL", "globalId": "2Cv20StructuralProbe0001", "name": "CV20 exclusion probe"}} |
      | set-project-units      | {"project": 120, "units": null}                                                                                              |
      | set-product-placement  | {"product": 270549, "placement": null}                                                                                       |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real Coordination View 2.0 model
    Given the real input document shared://🏗️wellness-center-sama-street-level.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the independently read projection matches its pre-mutation projection
    Examples:
      | id                     | params                                                                                                                       |
      | no-mutation            | {}                                                                                                                           |
      | set-snapshot           | {"fileSchema": ["IFC2X3", "IFC2X3-CV20-MARKER"]}                                                                             |
      | set-view-definition    | {"view": "StructuralAnalysisView"}                                                                                           |
      | set-structural-entity  | {"id": 9000001, "entity": {"typeName": "IFCSTRUCTURALANALYSISMODEL", "globalId": "2Cv20StructuralProbe0001", "name": "CV20 exclusion probe"}} |
      | set-project-units      | {"project": 120, "units": null}                                                                                              |
      | set-product-placement  | {"product": 270549, "placement": null}                                                                                       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real model without passing bytes through
    Given the real input document shared://🏗️wellness-center-sama-street-level.ifc
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the independently read projections of input and output agree
