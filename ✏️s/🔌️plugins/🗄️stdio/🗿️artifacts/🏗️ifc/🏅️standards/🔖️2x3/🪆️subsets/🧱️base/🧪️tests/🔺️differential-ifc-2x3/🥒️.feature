@capability-ifc-2x3-base-mutate
@oracle-ifcopenshell-ifc-2x3-base-differential
@comparison-semantic-ifc-v1
Feature: Produce every schema-expressible IFC2X3 mutation twice, in two independent implementations
  This case exists for one reason: the sibling `../🧱️mutate-ifc-2x3` cannot make a differential claim.
  `ruststep` 0.4 READS ISO 10303-21 and has no writer anywhere in the crate, so all 9 of its
  scenarios are honestly typed `@mode-property`/`@mode-round-trip` — a second READER standing beside
  this repository's own producer, never a second producer. **IfcOpenShell 0.8.4.post1 both reads and
  writes IFC**, and every scenario below is therefore `@mode-differential`: IfcOpenShell applies the
  named mutation to the real building model through its own API and re-serializes the whole document
  with its own C++ Part-21 writer (`ifcopenshell.file.to_string`), and this repository's own
  `Ifc2x3Snapshot` codec does the same, independently, from the same input. The two written
  documents are then projected onto `semantic-ifc-v1` and compared. Neither producer reads the
  other; neither is derived from the other.

  The artifact is the real one this subset already commits and is not substituted for anything
  smaller: `shared://🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc`, a real self-consistent 3 464-entity
  slice of a real 21 MB EDM StepFileFactory IFC2X3 export of the Wellness Center Sama,
  `FILE_SCHEMA(('IFC2X3'))`, **193 915 bytes**, every id, coordinate, geometry definition and
  relationship real and untouched. The whole entity graph is compared on every scenario. Every
  scenario copies the fixture into the case work directory before touching it; the committed asset
  is never written to. The oracle reads back its own written bytes through a from-scratch ISO
  10303-21 reader (`🐍️component.py`'s `🔖️Part21Reader`, written from clause 6 and clause 8, importing
  no semio module and shelling out to nothing), so the projection is a reading of BYTES that
  IfcOpenShell actually emitted, never of an in-memory Python object graph that never survived
  serialization.

  📌️ The byte tripwire on the round trip is REAL on this fixture, and that is why it is here rather
  than in the IFC4 sibling. This file was written by EDM StepFileFactory, so IfcOpenShell's own
  writer is not a fixed point of it: 193 915 bytes in, **188 288 bytes out**, and the two projections
  still agree instance for instance. The IFC4 sibling reproduces its own earlier export byte for
  byte and therefore states, rather than asserts, its no-pass-through evidence.

  📌️ THREE of this subset's four mutation kinds appear below. `remove-instance` does not, and the
  reason is a measurement rather than an assumption: `ifcopenshell.file.remove`'s own documentation
  states it repairs references ("in the case of a list or set of references, the reference to the
  deleted will be removed from the aggregate"), while this subset's `Ifc2x3Mutation::RemoveInstance`
  is a bare `retain` that deliberately leaves them dangling — the sibling case's own feature file
  records that as the chosen integrity strategy and removes `#270549` precisely BECAUSE it is
  referenced. `get_total_inverses(#270549)` on this fixture is **8**, so the two verbs would visibly
  differ on this exact row. Comparing two different verbs is not a differential, so it is not
  claimed as one; `remove-instance` keeps its `ruststep`-backed scenarios in `../🧱️mutate-ifc-2x3`,
  unchanged and unweakened. The removal primitive is guarded the same way everywhere in this
  oracle: any path that would reach it with a non-zero inverse count refuses instead.

  🧾️ Every row's `params` IS the leaf's wire payload, exactly as `payload_value()` emits it — camelCase,
  the artifact's own value wire — so the subject decodes
  it through the derive-generated `from_payload_value` and the reference reads the same wire; nothing maps a
  parameter onto an operation by hand.

  📌️ Every Examples row is required to MOVE the semantic projection, and
  the oracle fails the scenario in role when it does not: a row whose parameters make the mutation a
  no-op passes whenever the reference library merely declined to error, which is not a test. The
  baseline it is measured against runs one identity rewrite first, so the comparison isolates the
  mutation rather than IfcOpenShell's own normal form.
  @id-differential
  @level-exhaustive
  @mode-differential
  Scenario Outline: Both implementations apply <id> to the real 3464-entity building model
    Given the real input document shared://🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then both independently produced documents carry the same semantic projection
    Examples:
      | id              | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
      | upsert-instance | {"instance": {"id": 619887, "entities": [{"typeName": "IFCCOLUMN", "arguments": [{"kind": "str", "value": "0PfeWE7Aj7GBHCsLa67379"}, {"kind": "ref", "value": 41}, {"kind": "str", "value": "WAVE8-RENAMED-COLUMN"}, {"kind": "unset"}, {"kind": "str", "value": "UC-Universal Columns-Column:UC305x305x97"}, {"kind": "ref", "value": 619886}, {"kind": "ref", "value": 619879}, {"kind": "str", "value": "552739"}]}]}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
      | set-header      | {"header": {"fileDescription": [{"kind": "list", "values": [{"kind": "str", "value": "ViewDefinition [CoordinationView_V2.0]"}]}, {"kind": "str", "value": "2;1"}], "fileName": [{"kind": "str", "value": "wellness-center-sama-street-level-wave8"}, {"kind": "str", "value": "2021-11-21T06:45:25"}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "str", "value": "The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013"}, {"kind": "str", "value": "21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383"}, {"kind": "str", "value": ""}], "fileSchema": [{"kind": "list", "values": [{"kind": "str", "value": "IFC2X3"}]}]}}                                                                                                                                                                                                                                                                                                                                                                                                             |

  @id-differential-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Both implementations undo <id> and restore the real building model
    Given the real input document shared://🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then both independently restored documents carry the same semantic projection
    Examples:
      | id              | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
      | upsert-instance | {"instance": {"id": 619887, "entities": [{"typeName": "IFCCOLUMN", "arguments": [{"kind": "str", "value": "0PfeWE7Aj7GBHCsLa67379"}, {"kind": "ref", "value": 41}, {"kind": "str", "value": "WAVE8-RENAMED-COLUMN"}, {"kind": "unset"}, {"kind": "str", "value": "UC-Universal Columns-Column:UC305x305x97"}, {"kind": "ref", "value": 619886}, {"kind": "ref", "value": 619879}, {"kind": "str", "value": "552739"}]}]}}                                                                                                                                                                                                                                                                                                         |
      | set-header      | {"header": {"fileDescription": [{"kind": "list", "values": [{"kind": "str", "value": "ViewDefinition [CoordinationView_V2.0]"}]}, {"kind": "str", "value": "2;1"}], "fileName": [{"kind": "str", "value": "wellness-center-sama-street-level-wave8"}, {"kind": "str", "value": "2021-11-21T06:45:25"}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "list", "values": [{"kind": "str", "value": ""}]}, {"kind": "str", "value": "The EXPRESS Data Manager Version 5.02.0100.07 : 28 Aug 2013"}, {"kind": "str", "value": "21.0.0.383 - Exporter 21.0.0.383 - Alternate UI 21.0.0.383"}, {"kind": "str", "value": ""}], "fileSchema": [{"kind": "list", "values": [{"kind": "str", "value": "IFC2X3"}]}]}} |

  @id-differential-identity-round-trip
  @level-long
  @mode-differential
  Scenario: Both implementations decode and re-encode the real building model from the model alone
    Given the real input document shared://🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc
    When the document is decoded into each implementation's own model and re-encoded from it alone
    Then both independently re-encoded documents carry the same semantic projection
