@capability-ifc-4-any-mutate
@oracle-ifcopenshell-ifc-4-any-differential
@comparison-semantic-ifc-v1
Feature: Produce every schema-expressible IFC4 mutation twice, in two independent implementations
  This case exists for one reason: the sibling `../🏗️mutate-ifc-4` cannot make a differential claim.
  `ruststep` 0.4 READS ISO 10303-21 and has no writer anywhere in the crate, so all 21 of its
  scenarios are honestly typed `@mode-property`/`@mode-round-trip` — a second READER standing beside
  this repository's own producer, never a second producer. **IfcOpenShell 0.8.4.post1 both reads and
  writes IFC**, and every scenario below is therefore `@mode-differential`: IfcOpenShell applies the
  named mutation to the real exchange structure through its own API and re-serializes the whole
  document with its own C++ Part-21 writer (`ifcopenshell.file.to_string`), and this repository's
  own `IfcSnapshot` codec does the same, independently, from the same input. The two written
  documents are then projected onto `semantic-ifc-v1` and compared. Neither producer reads the
  other; neither is derived from the other.

  The artifact is the real one this subset already commits and is not substituted for anything
  smaller: `shared://🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc`, a real IfcOpenShell 0.8.4.post1 export of Kisho
  Kurokawa's Nakagin Capsule Tower, `FILE_SCHEMA(('IFC4'))`, **2 496 437 bytes and 24 792 entity
  instances**. The whole entity graph is compared on every scenario, not a sampled corner of it.
  Every scenario copies the fixture into the case work directory before touching it; the committed
  asset is never written to. The oracle reads back its own written bytes through a from-scratch ISO
  10303-21 reader (`🐍️component.py`'s `🔖️Part21Reader`, written from clause 6 and clause 8, importing
  no semio module and shelling out to nothing), so the projection is a reading of BYTES that
  IfcOpenShell actually emitted, never of an in-memory Python object graph that never survived
  serialization.

  📌️ SIX of this subset's ten mutation kinds appear below, and the four that do not are named
  with the measurement that excluded them, not with an assumption. `IfcMutation` is a Part-21
  RECORD-level vocabulary; IfcOpenShell is bound to the IFC4 EXPRESS schema.
    · `set-entity-name` — creating `RENAMED_PROXY` raises `Entity with name 'RENAMED_PROXY' not
      found in schema 'IFC4'`, and a file carrying it reads back through
      `ifcopenshell.file.from_string` as 16 975 of 24 792 entities with NO error raised. A reader
      that silently truncates is worse evidence than `ruststep`, which reads the same file whole.
    · `insert-entity-arg` — a tenth positional argument on the nine-attribute
      `IfcBuildingElementProxy` raises `IndexError` on assignment, and a hand-written file carrying
      one reads back with the extra argument silently dropped.
    · `remove-entity-arg` — arity cannot be reduced through the schema-bound API at all; assigning
      `None` writes `$` and keeps nine arguments, which is a different mutation.
    · `remove-entity` — `ifcopenshell.file.remove`'s own documentation states it repairs references
      ("the reference to the deleted will be removed from the aggregate"), confirmed against this
      fixture: `#16976` disappears from `#16991`'s member aggregate. This subset's
      `IfcMutation::RemoveEntity` deliberately does not cascade and leaves that reference dangling.
      Comparing two different verbs is not a differential, so it is not claimed as one.
  All four keep their `ruststep`-backed scenarios in `../🏗️mutate-ifc-4`, unchanged and unweakened.
  The removal primitive is used here only as the inverse of `insert-entity`, and only behind an
  explicit `get_total_inverses(...) == 0` guard, so the cascading path can never be taken silently.

  📌️ There is deliberately NO "the output is not bit-identical to the input" tripwire on the oracle
  side of the round trip, and the reason is recorded rather than hidden: this fixture was itself
  exported by IfcOpenShell 0.8.4.post1, so its own writer is a FIXED POINT of it — measured, 2 496 437
  bytes in and 2 496 437 identical bytes out. Asserting non-identity would fail a correct
  implementation. What the oracle asserts instead is that the document was genuinely parsed into a
  typed model: IfcOpenShell's own count of materialized entity instances must equal the entity count
  the from-scratch text reader finds in the bytes IfcOpenShell wrote — two independent counts of one
  model, which a byte copy could not report. The subject keeps its own no-pass-through check.

  📌️ `set-snapshot` replaces the whole capsule tower with a one-entity `IfcSnapshot` record — the real
  `IFCPROJECT` `#1` with its references cleared — which IfcOpenShell builds through its own typed API
  into a fresh `IFC4` model. Its inverse is the model it replaced: IfcOpenShell re-reads the untouched
  fixture and writes it back, while the subject applies `IfcMutation`'s own inverse.

  🧾️ Every row's `params` IS the leaf's wire payload, exactly as `payload_value()` emits it — camelCase,
  the artifact's own value wire, `set-snapshot` carrying a whole snapshot record — so the subject decodes
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
  Scenario Outline: Both implementations apply <id> to the real 24792-entity exchange structure
    Given the real input document shared://🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then both independently produced documents carry the same semantic projection
    Examples:
      | id                   | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
      | set-snapshot         | {"snapshot": {"schema": "stdio.ifc", "header": {"fileDescription": [{"kind": "aggregate", "value": [{"kind": "string", "value": "ViewDefinition[DesignTransferView]"}]}, {"kind": "string", "value": "2;1"}], "fileName": [{"kind": "string", "value": "nakagin-capsule-tower-project.ifc"}, {"kind": "string", "value": "2026-03-20T21:51:27+00:00"}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "Nobody"}], "fileSchema": [{"kind": "aggregate", "value": [{"kind": "string", "value": "IFC4"}]}]}, "entities": [{"id": 1, "name": "IFCPROJECT", "args": [{"kind": "string", "value": "1K7GYWVD13lBqV_e0J2CvF"}, {"kind": "unset"}, {"kind": "string", "value": "Metabolism"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}]}]}} |
      | set-file-description | {"values": [{"kind": "aggregate", "value": [{"kind": "string", "value": "wave-7 mutation"}]}, {"kind": "string", "value": "2;1"}]}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
      | set-file-name        | {"values": [{"kind": "string", "value": "wave-7-mutated.ifc"}, {"kind": "string", "value": "2026-08-23T00:00:00"}, {"kind": "aggregate", "value": [{"kind": "string", "value": "Ueli"}]}, {"kind": "aggregate", "value": [{"kind": "string", "value": "semio"}]}, {"kind": "string", "value": "semio-ifc"}, {"kind": "string", "value": "semio"}, {"kind": "string", "value": ""}]}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
      | set-file-schema      | {"values": [{"kind": "aggregate", "value": [{"kind": "string", "value": "IFC4X3"}]}]}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
      | insert-entity        | {"index": 24792, "entity": {"id": 90001, "name": "IFCCARTESIANPOINT", "args": [{"kind": "aggregate", "value": [{"kind": "real", "value": 1000.0}, {"kind": "real", "value": 2000.0}, {"kind": "real", "value": 3000.0}]}]}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
      | set-entity-arg       | {"id": 16976, "index": 2, "value": {"kind": "string", "value": "origin-marker"}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

  @id-differential-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Both implementations undo <id> and restore the real exchange structure
    Given the real input document shared://🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the mutation's own inverse is applied to the result
    Then both independently restored documents carry the same semantic projection
    Examples:
      | id                   | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
      | set-snapshot         | {"snapshot": {"schema": "stdio.ifc", "header": {"fileDescription": [{"kind": "aggregate", "value": [{"kind": "string", "value": "ViewDefinition[DesignTransferView]"}]}, {"kind": "string", "value": "2;1"}], "fileName": [{"kind": "string", "value": "nakagin-capsule-tower-project.ifc"}, {"kind": "string", "value": "2026-03-20T21:51:27+00:00"}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "aggregate", "value": [{"kind": "string", "value": ""}]}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "IfcOpenShell 0.8.4.post1"}, {"kind": "string", "value": "Nobody"}], "fileSchema": [{"kind": "aggregate", "value": [{"kind": "string", "value": "IFC4"}]}]}, "entities": [{"id": 1, "name": "IFCPROJECT", "args": [{"kind": "string", "value": "1K7GYWVD13lBqV_e0J2CvF"}, {"kind": "unset"}, {"kind": "string", "value": "Metabolism"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}, {"kind": "unset"}]}]}} |
      | set-file-description | {"values": [{"kind": "aggregate", "value": [{"kind": "string", "value": "wave-7 mutation"}]}, {"kind": "string", "value": "2;1"}]}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
      | set-file-name        | {"values": [{"kind": "string", "value": "wave-7-mutated.ifc"}, {"kind": "string", "value": "2026-08-23T00:00:00"}, {"kind": "aggregate", "value": [{"kind": "string", "value": "Ueli"}]}, {"kind": "aggregate", "value": [{"kind": "string", "value": "semio"}]}, {"kind": "string", "value": "semio-ifc"}, {"kind": "string", "value": "semio"}, {"kind": "string", "value": ""}]}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
      | set-file-schema      | {"values": [{"kind": "aggregate", "value": [{"kind": "string", "value": "IFC4X3"}]}]}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
      | insert-entity        | {"index": 24792, "entity": {"id": 90001, "name": "IFCCARTESIANPOINT", "args": [{"kind": "aggregate", "value": [{"kind": "real", "value": 1000.0}, {"kind": "real", "value": 2000.0}, {"kind": "real", "value": 3000.0}]}]}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
      | set-entity-arg       | {"id": 16976, "index": 2, "value": {"kind": "string", "value": "origin-marker"}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

  @id-differential-identity-round-trip
  @level-long
  @mode-differential
  Scenario: Both implementations decode and re-encode the real exchange structure from the model alone
    Given the real input document shared://🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc
    When the document is decoded into each implementation's own model and re-encoded from it alone
    Then both independently re-encoded documents carry the same semantic projection
