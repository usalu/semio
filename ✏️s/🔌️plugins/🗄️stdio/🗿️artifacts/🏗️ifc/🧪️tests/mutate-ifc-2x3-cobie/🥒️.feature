@capability-ifc-2x3-cobie-mutate
@oracle-ruststep-ifc-2x3-cobie-mutate
@comparison-semantic-ifc-v1
@mutations-ifc-2x3-cobie
Feature: Apply every typed Basic FM Handover mutation to a real IFC2X3 building model
  The input is `shared://🏗️wellness-center-sama-street-level.ifc`, a real self-consistent 3464-entity
  slice of a genuine 21 MB IFC2X3 building model (an EDM StepFileFactory export, 2021) — the only
  real IFC2X3 file in this repository. Every scenario copies it into the case work directory before
  touching it; the committed asset is never written to.

  Honest limit stated up front: this is a real `ViewDefinition [CoordinationView_V2.0]` coordination
  export, NOT a native FM handover file. No real `FMHandOverView` document exists in this repository.
  That is why `set-view-definition` below performs the real stamping step an FM handover extraction
  begins with, and it is why the COBie Space sheet starts empty: the export carries zero `IFCSPACE`
  instances (verified against the FULL 21 MB source, not only the committed slice). Every other
  handover concept the vocabulary edits IS real in this file — the real `IFCBUILDING` `#130` whose
  `Name` is genuinely blank, five real `IFCBUILDINGSTOREY` instances including `#139` "Street level"
  at elevation `0.`, six real `IFC*TYPE` instances and six real `IFCRELDEFINESBYTYPE` relationships
  including `#712708`, which relates the two real `IFCWALLSTANDARDCASE` instances `#270549` and
  `#523123` to the real `IFCWALLTYPE` `#270567`.

  This subset's vocabulary is NOT the sibling `✳️any` subset's. `✳️any` declares generic ISO 10303-21
  graph editing — `upsert-instance`, `remove-instance`, `set-header` — and knows nothing about model
  view definitions. `Ifc2x3CobieMutation`
  (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🧬️schema/🧬️mutations/🦀️component.rs`) declares one kind
  per COBie handover sheet, each taken from the conformance gate this repository already implements
  in production (`check_cobie_conformance`): `set-snapshot` addresses `CODE_FILE_SCHEMA`,
  `set-view-definition` addresses `CODE_VIEW_DEFINITION`, `set-facility-name` and
  `set-floor-elevation` address `CODE_BUILDING_STOREY` (the Facility and Floor sheets), `set-space`
  addresses `CODE_SPACE_NAME` (the Space sheet is keyed by a non-empty name) and
  `set-type-assignment` addresses `CODE_TYPE_ASSIGNMENT` (the Type sheet needs maintainable products
  related to a real type). Each kind carries an OPTIONAL payload — a value sets the sheet row, `null`
  clears it — so every kind is total in both directions and every inverse below is a real inverse
  rather than a whole-document restore.

  `ruststep` 0.4 parses IFC2X3 exactly as it parses STEP AP214 but has NO writer at all. Per the
  fleet brief's §6 that rules out `@mode-differential`: ruststep cannot be a second PRODUCER of
  mutated bytes to diff the subject against. It is the INDEPENDENT READER instead — every mutation
  the oracle dispatcher performs (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️cobie/🧪️oracle/🦀️component.rs`,
  re-serializing through the standard-level from-scratch Part-21 writer at
  `../../🏅️standards/🔖️2x3/🧪️oracle/🦀️component.rs` rather than this repository's own production
  `step::engine::part21` writer, which would compare the implementation against itself) is read back
  through a FRESH `ruststep::ast::Exchange::from_str` call before `semantic-ifc-v1` compares it.


  📌️ Every Examples row below other than `no-mutation` is required to MOVE the semantic projection,
  and the adapter fails the scenario in role when it does not: a row whose parameters make the
  mutation a no-op passes whenever the reference library merely declined to error, which is not a
  test. The baseline it is measured against runs one `no-mutation` cycle first, so the comparison
  isolates the mutation rather than the writer's own normal form.
  @id-mutate
  @level-exhaustive
  @mode-property
  Scenario Outline: Apply <id> to the real building model
    Given the real input document shared://🏗️wellness-center-sama-street-level.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the independently read projection shows the mutation's real effect
    Examples:
      | id                  | params                                                                                                                                                      |
      | no-mutation         | {}                                                                                                                                                          |
      | set-snapshot        | {"fileSchema": ["IFC2X3", "IFC2X3-COBIE-MARKER"]}                                                                                                           |
      | set-view-definition | {"view": "FMHandOverView"}                                                                                                                                  |
      | set-facility-name   | {"building": 130, "name": "Wellness Center Sama"}                                                                                                           |
      | set-floor-elevation | {"storey": 139, "elevation": 150.0}                                                                                                                         |
      | set-space           | {"id": 9100001, "space": {"globalId": "2CobieHandoverSpace0001", "name": "Street level lobby", "placement": 137}}                                            |
      | set-type-assignment | {"id": 712708, "assignment": null}                                                                                                                          |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real building model
    Given the real input document shared://🏗️wellness-center-sama-street-level.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the independently read projection matches its pre-mutation projection
    Examples:
      | id                  | params                                                                                                                                                      |
      | no-mutation         | {}                                                                                                                                                          |
      | set-snapshot        | {"fileSchema": ["IFC2X3", "IFC2X3-COBIE-MARKER"]}                                                                                                           |
      | set-view-definition | {"view": "FMHandOverView"}                                                                                                                                  |
      | set-facility-name   | {"building": 130, "name": "Wellness Center Sama"}                                                                                                           |
      | set-floor-elevation | {"storey": 139, "elevation": 150.0}                                                                                                                         |
      | set-space           | {"id": 9100001, "space": {"globalId": "2CobieHandoverSpace0001", "name": "Street level lobby", "placement": 137}}                                            |
      | set-type-assignment | {"id": 712708, "assignment": null}                                                                                                                          |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real model without passing bytes through
    Given the real input document shared://🏗️wellness-center-sama-street-level.ifc
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the independently read projections of input and output agree
