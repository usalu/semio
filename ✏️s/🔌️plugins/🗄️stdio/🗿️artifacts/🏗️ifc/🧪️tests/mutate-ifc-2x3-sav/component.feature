@capability-ifc-2x3-sav-mutate
@oracle-ruststep-ifc-2x3-sav-mutate
@comparison-semantic-ifc-v1
@mutations-ifc-2x3-sav
Feature: Apply every typed Structural Analysis View mutation to an IFC2X3 building model
  The input is `shared://🏗️wellness-center-sama-structural-seed.ifc`.

  Honest limit stated up front, because it is the weakest real-world claim in this subset: **this
  repository contains no real IFC2X3 Structural Analysis View document.** Grepping the FULL 21 MB
  real source (`temp/wellness-center-sama.ifc`, the EDM StepFileFactory export the committed IFC2X3
  fixtures are derived from) for `IFCSTRUCTURAL*` returns **zero** matches — it is an architectural
  coordination model, and nothing else in the repository is IFC2X3 at all. The committed input this
  case uses is therefore the real 3464-entity export with its `FILE_DESCRIPTION` re-stamped to
  `ViewDefinition [StructuralAnalysisView]` and exactly THREE seeded structural entities appended:
  `#9200001` an `IFCSTRUCTURALANALYSISMODEL`, `#9200002` an `IFCSTRUCTURALLOADGROUP`, and `#9200003`
  an `IFCRELASSIGNSTOGROUP` relating the two REAL `IFCWALLSTANDARDCASE` instances `#270549` and
  `#523123` to that model through the real `IFCOWNERHISTORY` `#41`. 3464 of the 3467 entities are
  real and untouched; the structural half is seeded. Both the real base and this derived file are
  committed, and the derivation script is
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/ifc-2x3-mvd-subsets/🐍️derive-structural-seed.py`,
  so the substitution is auditable. Read this case as exercising the vocabulary against a real
  building model, not against a real structural analysis. A genuine IFC2X3 `StructuralAnalysisView`
  export would fix it. Every scenario copies the fixture into the case work directory before touching
  it; the committed asset is never written to.

  This subset's vocabulary is NOT the sibling `✳️any` subset's. `✳️any` declares generic ISO 10303-21
  graph editing — `upsert-instance`, `remove-instance`, `set-header` — and knows nothing about model
  view definitions. `Ifc2x3SavMutation`
  (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧬️schema/🧬️mutations/🦀️component.rs`) declares one kind per
  rule of the Structural Analysis View conformance gate this repository already implements in
  production (`check_sav_conformance`): `set-snapshot` addresses `CODE_FILE_SCHEMA`,
  `set-view-definition` addresses `CODE_VIEW_DEFINITION`, `set-analysis-model` addresses
  `CODE_NO_ANALYSIS_MODEL` (the view's one HARD entity requirement), `set-load-group` addresses
  `CODE_NO_LOADS` and `set-group-assignment` addresses `CODE_NO_GROUP_ASSIGNMENT`. Each kind carries
  an OPTIONAL payload — a value sets the concept, `null` clears it — so every kind is total in both
  directions and every inverse below is a real inverse rather than a whole-document restore. The
  forward direction of each structural kind deliberately CLEARS the concept, which is the real
  violation the production gate exists to catch; the inverse restores it exactly.

  `set-analysis-model`'s forward direction leaves `#9200003`'s `RelatingGroup` pointing at a removed
  `#9200001`. That dangling reference is deliberate and matches production's own no-cascade policy
  (`Ifc2x3Mutation::RemoveInstance` is a bare `retain`), and the inverse heals it by restoring the
  same id — recorded here rather than hidden by removing an unreferenced entity instead.

  `ruststep` 0.4 parses IFC2X3 exactly as it parses STEP AP214 but has NO writer at all. Per the
  fleet brief's §6 that rules out `@mode-differential`: ruststep cannot be a second PRODUCER of
  mutated bytes to diff the subject against. It is the INDEPENDENT READER instead — every mutation
  the oracle dispatcher performs (`../../🏅️standards/🔖️2x3/🪆️subsets/✳️sav/🧪️oracle/🦀️component.rs`,
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
  Scenario Outline: Apply <id> to the structural analysis document
    Given the real input document shared://🏗️wellness-center-sama-structural-seed.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the independently read projection shows the mutation's real effect
    Examples:
      | id                    | params                                             |
      | no-mutation           | {}                                                 |
      | set-snapshot          | {"fileSchema": ["IFC2X3", "IFC2X3-SAV-MARKER"]}    |
      | set-view-definition   | {"view": "CoordinationView_V2.0"}                  |
      | set-analysis-model    | {"id": 9200001, "model": null}                     |
      | set-load-group        | {"id": 9200002, "group": null}                     |
      | set-group-assignment  | {"id": 9200003, "assignment": null}                |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the structural analysis document
    Given the real input document shared://🏗️wellness-center-sama-structural-seed.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the independently read projection matches its pre-mutation projection
    Examples:
      | id                    | params                                             |
      | no-mutation           | {}                                                 |
      | set-snapshot          | {"fileSchema": ["IFC2X3", "IFC2X3-SAV-MARKER"]}    |
      | set-view-definition   | {"view": "CoordinationView_V2.0"}                  |
      | set-analysis-model    | {"id": 9200001, "model": null}                     |
      | set-load-group        | {"id": 9200002, "group": null}                     |
      | set-group-assignment  | {"id": 9200003, "assignment": null}                |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the document without passing bytes through
    Given the real input document shared://🏗️wellness-center-sama-structural-seed.ifc
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the independently read projections of input and output agree
