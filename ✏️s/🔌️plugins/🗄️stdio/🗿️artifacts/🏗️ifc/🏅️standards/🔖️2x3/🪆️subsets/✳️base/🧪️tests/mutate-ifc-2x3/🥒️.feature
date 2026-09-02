@capability-ifc-2x3-base-mutate
@oracle-ruststep-ifc-2x3-base-mutate
@comparison-semantic-ifc-v1
@mutations-ifc-2x3-base
Feature: Apply every typed IFC2X3 mutation to a real-world building model
  The input is a real, self-consistent SUBSET of a genuine 21 MB IFC2X3 building model
  (`temp/wellness-center-sama.ifc`, an EDM StepFileFactory export, 2021, `FILE_SCHEMA(('IFC2X3'))`
  confirmed by reading the header directly -- the only real IFC2X3 file in this repository), not a
  synthetic fixture: `shared://🧪️wellness-center-sama-street-level/🏗️.ifc`. 21 MB is too large to copy
  into every work directory, so this fixture was derived ONCE (script in this ticket's own scratch
  folder) rather than committing the whole file: it keeps the real `IFCBUILDINGSTOREY` #139
  ("Street level"), its full real spatial-structure ancestor chain (`IFCRELAGGREGATES` up through
  `IFCBUILDING`/`IFCSITE`/`IFCPROJECT`), the real `IFCRELCONTAINEDINSPATIALSTRUCTURE` relationship
  naming its 14 real contained building elements (slabs, walls, columns, a ramp, a stair, building-
  element proxies), every real `IFCREL*` relationship that references the storey or any of those
  elements (property sets, material associations, type definitions, voids -- 72 real relationships),
  then the full forward-reference closure of that root set. 3464 of the source's 409102 real
  entities are kept; every id, coordinate, geometry definition and relationship is real and
  untouched -- nothing renumbered, nothing synthesised. `git check-ignore -v` on the committed
  fixture confirms it is tracked (the repository-wide `temp/` exclusion does not reach
  `🧫️fixtures/`, which is what makes committing a derived slice of a `temp/` asset possible at all).
  Every scenario copies the fixture into the case work directory before touching it; the committed
  asset is never written to.

  This subset's own vocabulary (`Ifc2x3Mutation`, `../../🏅️standards/🔖️2x3/🪆️subsets/✳️base/🧬️schema/
  🧬️mutations/🦀️component.rs`) is real per-instance editing, richer than the sibling `4` standard's
  `{NoMutation, SetSnapshot}` stub: `UpsertInstance` replaces an existing id's whole instance or
  appends a brand-new one (never a positional insert), `RemoveInstance` deletes an id with no
  cascading reference-integrity check, and `SetHeader` replaces the header wholesale. `remove-
  instance`'s own scenario below is the deliberate real-integrity case: it removes `#270549`, a real
  `IFCWALLSTANDARDCASE` referenced by 8 real entities in the full source (7 of which -- 5 property-
  set relationships, 1 material association, 1 type-definition relationship, plus the storey's own
  spatial-containment relationship -- are carried into this fixture's own closure). The mutation is
  mechanical, matching production `Ifc2x3Mutation::RemoveInstance`'s own bare `retain`: the result
  genuinely leaves those real relationships holding a dangling `#270549` reference afterward. That is
  recorded as the chosen integrity strategy, not hidden by picking an unreferenced entity instead.
  Its own inverse re-inserts the exact original entity via `upsert-instance` (cross-kind inversion,
  the same pattern the sibling `step/🔖️ap214/✳️base` subset's `insert-entity`/`remove-entity` pair
  uses), which heals the dangling reference by restoring the same id.

  IFC2X3 is physically ISO 10303-21 (STEP physical file) syntax under a different EXPRESS schema --
  `ruststep` 0.4, already linked and registered for the sibling `step/🔖️ap214/✳️base` subset this
  wave, parses it exactly as it parses STEP AP214 (confirmed here again: it parsed this fixture's
  3464 real entities with zero errors). It has NO writer at all -- reading its own source finds no
  `Display`/`fmt::Formatter` impl anywhere on `Exchange`/`DataSection`/`Record`/`Parameter`, and
  `ast::ser::to_record` only builds an in-memory `Record` from an already-typed Rust struct. Per the
  fleet brief's §6, this rules out `@mode-differential`: ruststep cannot be a second PRODUCER of
  mutated bytes to diff the subject against. It is registered as the INDEPENDENT READER instead:
  every mutation this subset's own oracle dispatcher performs (`ruststep` parses the real input;
  this subset's own from-scratch Part-21 writer, in `../../🏅️standards/🔖️2x3/🪆️subsets/✳️base/
  🧪️oracle/🦀️component.rs`, re-serializes it -- independent of this subset's own production
  `step::engine::part21` codec, since comparing that codec against itself would be the exact failure
  mode this platform exists to prevent) is read back through a FRESH, independent
  `ruststep::ast::Exchange::from_str` call (`project_ifc_2x3_any`) before `semantic-ifc-v1` compares
  it, and the identity round trip is checked the same way. That is real third-party evidence about
  the entity graph's structure, argument values and declared `FILE_SCHEMA` as ruststep's own parser
  sees them -- but it cannot discharge a byte-level differential claim, which every scenario below is
  typed to reflect honestly rather than claim.


  📌️ Every Examples row below other than `no-mutation` is required to MOVE the semantic projection,
  and the adapter fails the scenario in role when it does not: a row whose parameters make the
  mutation a no-op passes whenever the reference library merely declined to error, which is not a
  test. The baseline it is measured against runs one `no-mutation` cycle first, so the comparison
  isolates the mutation rather than the writer's own normal form.
  @id-mutate
  @level-exhaustive
  @mode-property
  Scenario Outline: Apply <id> to the real building model
    Given the real input document shared://🧪️wellness-center-sama-street-level/🏗️.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the independently read projection shows the mutation's real effect
    Examples:
      | id              | params                                                                                                                                                                                                                                                                                                                              |
      | set-snapshot    | {"fileSchema": ["IFC2X3", "IFC2X3-WAVE8-SNAPSHOT-MARKER"]}                                                                                                                                                                                                                                                                         |
      | upsert-instance | {"instance": {"id": 619887, "entities": [{"name": "IFCCOLUMN", "args": [{"t": "string", "v": "0PfeWE7Aj7GBHCsLa67379"}, {"t": "reference", "v": 41}, {"t": "string", "v": "WAVE8-RENAMED-COLUMN"}, {"t": "unset"}, {"t": "string", "v": "UC-Universal Columns-Column:UC305x305x97"}, {"t": "reference", "v": 619886}, {"t": "reference", "v": 619879}, {"t": "string", "v": "552739"}]}]}} |
      | remove-instance | {"id": 270549}                                                                                                                                                                                                                                                                                                                      |
      | set-header      | {"header": {"fileDescription": [{"t": "aggregate", "v": [{"t": "string", "v": "ViewDefinition [CoordinationView_V2.0]"}]}, {"t": "string", "v": "2;1"}], "fileName": [{"t": "string", "v": "wellness-center-sama-street-level-wave8"}, {"t": "string", "v": "2021-11-21T06:45:25"}, {"t": "aggregate", "v": [{"t": "string", "v": ""}]}, {"t": "aggregate", "v": [{"t": "string", "v": ""}]}, {"t": "string", "v": "The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013"}, {"t": "string", "v": "21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383"}, {"t": "string", "v": ""}], "fileSchema": [{"t": "aggregate", "v": [{"t": "string", "v": "IFC2X3"}]}]}} |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-property
  Scenario: Apply no-mutation to the real building model
    Given the real input document shared://🧪️wellness-center-sama-street-level/🏗️.ifc
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the independently read projection shows the mutation's real effect

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real building model
    Given the real input document shared://🧪️wellness-center-sama-street-level/🏗️.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then the independently read projection matches its pre-mutation projection
    Examples:
      | id              | params                                                                                                                                                                                                                                                                                                                              |
      | set-snapshot    | {"fileSchema": ["IFC2X3", "IFC2X3-WAVE8-SNAPSHOT-MARKER"]}                                                                                                                                                                                                                                                                         |
      | upsert-instance | {"instance": {"id": 619887, "entities": [{"name": "IFCCOLUMN", "args": [{"t": "string", "v": "0PfeWE7Aj7GBHCsLa67379"}, {"t": "reference", "v": 41}, {"t": "string", "v": "WAVE8-RENAMED-COLUMN"}, {"t": "unset"}, {"t": "string", "v": "UC-Universal Columns-Column:UC305x305x97"}, {"t": "reference", "v": 619886}, {"t": "reference", "v": 619879}, {"t": "string", "v": "552739"}]}]}} |
      | remove-instance | {"id": 270549}                                                                                                                                                                                                                                                                                                                      |
      | set-header      | {"header": {"fileDescription": [{"t": "aggregate", "v": [{"t": "string", "v": "ViewDefinition [CoordinationView_V2.0]"}]}, {"t": "string", "v": "2;1"}], "fileName": [{"t": "string", "v": "wellness-center-sama-street-level-wave8"}, {"t": "string", "v": "2021-11-21T06:45:25"}, {"t": "aggregate", "v": [{"t": "string", "v": ""}]}, {"t": "aggregate", "v": [{"t": "string", "v": ""}]}, {"t": "string", "v": "The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013"}, {"t": "string", "v": "21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383"}, {"t": "string", "v": ""}], "fileSchema": [{"t": "aggregate", "v": [{"t": "string", "v": "IFC2X3"}]}]}} |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real building model
    Given the real input document shared://🧪️wellness-center-sama-street-level/🏗️.ifc
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    And the mutation's own inverse is applied to the result
    Then the independently read projection matches its pre-mutation projection

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real building model without passing bytes through
    Given the real input document shared://🧪️wellness-center-sama-street-level/🏗️.ifc
    When the document is decoded into the subset's own snapshot and re-encoded from it alone
    Then the output is not bit-identical to the input
    And the independently read projections of input and output agree
