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
  Scenario Outline: Both implementations apply <id> to the real 24792-entity exchange structure
    Given the real input document shared://🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then both independently produced documents carry the same semantic projection
    Examples:
      | id                   | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
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

  @id-differential-official-round-trip
  @level-quick
  @mode-differential
  Scenario Outline: Preserve the full graph of an unchanged official IFC4 reference sample
    Given the official input document shared://🏛️official/<file>
    When the IFC4 boundary imports and exports the document through its own snapshot
      """
      {"input": "shared://🏛️official/<file>", "entityCount": <count>}
      """
    Then independent ruststep and IfcOpenShell readers agree on the complete exchange graph
    Examples:
      | file                                        | count |
      | tessellation-with-individual-colors.ifc      | 32    |
      | column-straight-rectangle-tessellation.ifc   | 26    |

  @id-differential-geometry-placed-extrusion
  @level-quick
  @mode-differential
  Scenario: Independently project placed-extrusion geometry
    Given the language-neutral IFC4 representation graph
    When both implementations independently tessellate and place the source representation
      """
      {"name":"placed-extrusion","data":"#1=IFCCARTESIANPOINT((10.,0.,0.));\n#2=IFCAXIS2PLACEMENT3D(#1,$,$);\n#3=IFCLOCALPLACEMENT($,#2);\n#4=IFCCARTESIANPOINT((0.,20.,0.));\n#5=IFCDIRECTION((0.,1.,0.));\n#6=IFCAXIS2PLACEMENT3D(#4,#11,#5);\n#7=IFCLOCALPLACEMENT(#3,#6);\n#8=IFCCARTESIANPOINT((0.,0.,0.));\n#9=IFCAXIS2PLACEMENT3D(#8,$,$);\n#10=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,0.00001,#9,$);\n#11=IFCDIRECTION((0.,0.,1.));\n#12=IFCRECTANGLEPROFILEDEF(.AREA.,$,$,2.,4.);\n#13=IFCEXTRUDEDAREASOLID(#12,#9,#11,3.);\n#14=IFCSHAPEREPRESENTATION(#10,'Body','SweptSolid',(#13));\n#15=IFCPRODUCTDEFINITIONSHAPE($,$,(#14));\n#16=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Extrusion',$,$,#7,#15,$,$);","expected":{"products":[16],"vertices":8,"triangles":12,"bounds":[[8,19,0],[12,21,3]],"volume":24}}
      """
    Then real vertices, triangles, world bounds and geometric measures agree

  @id-differential-geometry-triangulated
  @level-quick
  @mode-differential
  Scenario: Independently project triangulated geometry
    Given the language-neutral IFC4 representation graph
    When both implementations independently tessellate and place the source representation
      """
      {"name":"triangulated","data":"#1=IFCCARTESIANPOINTLIST3D(((0.,0.,0.),(2.,0.,0.),(0.,3.,0.)));\n#2=IFCTRIANGULATEDFACESET(#1,$,.F.,((1,2,3)),$);\n#3=IFCSHAPEREPRESENTATION(#8,'Body','Tessellation',(#2));\n#4=IFCPRODUCTDEFINITIONSHAPE($,$,(#3));\n#5=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Triangle',$,$,$,#4,$,$);\n#6=IFCCARTESIANPOINT((0.,0.,0.));\n#7=IFCAXIS2PLACEMENT3D(#6,$,$);\n#8=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,0.00001,#7,$);","expected":{"products":[5],"vertices":3,"triangles":1,"bounds":[[0,0,0],[2,3,0]]}}
      """
    Then real vertices, triangles, world bounds and geometric measures agree

  @id-differential-geometry-mapped-triangle
  @level-quick
  @mode-differential
  Scenario: Independently project mapped-triangle geometry
    Given the language-neutral IFC4 representation graph
    When both implementations independently tessellate and place the source representation
      """
      {"name":"mapped-triangle","data":"#1=IFCCARTESIANPOINTLIST3D(((0.,0.,0.),(2.,0.,0.),(0.,3.,0.)));\n#2=IFCTRIANGULATEDFACESET(#1,$,.F.,((1,2,3)),$);\n#3=IFCSHAPEREPRESENTATION(#8,'Body','Tessellation',(#2));\n\n#6=IFCCARTESIANPOINT((0.,0.,0.));\n#7=IFCAXIS2PLACEMENT3D(#6,$,$);\n#8=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,0.00001,#7,$);\n#9=IFCCARTESIANPOINT((1.,0.,0.));\n#10=IFCAXIS2PLACEMENT3D(#9,$,$);\n#11=IFCREPRESENTATIONMAP(#10,#3);\n#12=IFCCARTESIANPOINT((10.,0.,0.));\n#13=IFCCARTESIANTRANSFORMATIONOPERATOR3D($,$,#12,2.,$);\n#14=IFCMAPPEDITEM(#11,#13);\n#15=IFCSHAPEREPRESENTATION(#8,'Body','MappedRepresentation',(#14));\n#16=IFCPRODUCTDEFINITIONSHAPE($,$,(#15));\n#17=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Mapped Triangle',$,$,$,#16,$,$);","expected":{"products":[17],"vertices":3,"triangles":1,"bounds":[[12,0,0],[16,6,0]]}}
      """
    Then real vertices, triangles, world bounds and geometric measures agree

  @id-differential-geometry-concave-polygon
  @level-quick
  @mode-differential
  Scenario: Independently project concave-polygon geometry
    Given the language-neutral IFC4 representation graph
    When both implementations independently tessellate and place the source representation
      """
      {"name":"concave-polygon","data":"#1=IFCCARTESIANPOINTLIST3D(((0.,0.,0.),(2.,0.,0.),(2.,1.,0.),(1.,1.,0.),(1.,2.,0.),(0.,2.,0.)));\n#2=IFCINDEXEDPOLYGONALFACE((1,2,3,4,5,6));\n#9=IFCPOLYGONALFACESET(#1,.F.,(#2),$);\n#3=IFCSHAPEREPRESENTATION(#8,'Body','Tessellation',(#9));\n#4=IFCPRODUCTDEFINITIONSHAPE($,$,(#3));\n#5=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Concave Polygon',$,$,$,#4,$,$);\n#6=IFCCARTESIANPOINT((0.,0.,0.));\n#7=IFCAXIS2PLACEMENT3D(#6,$,$);\n#8=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,0.00001,#7,$);","expected":{"products":[5],"vertices":6,"triangles":4,"bounds":[[0,0,0],[2,2,0]],"area":3}}
      """
    Then real vertices, triangles, world bounds and geometric measures agree

  @id-differential-geometry-pn-index
  @level-quick
  @mode-differential
  Scenario: Independently project pn-index geometry
    Given the language-neutral IFC4 representation graph
    When both implementations independently tessellate and place the source representation
      """
      {"name":"pn-index","data":"#1=IFCCARTESIANPOINTLIST3D(((99.,99.,99.),(0.,0.,0.),(2.,0.,0.),(0.,3.,0.)));\n#2=IFCTRIANGULATEDFACESET(#1,$,.F.,((1,2,3)),(2,3,4));\n#3=IFCSHAPEREPRESENTATION(#8,'Body','Tessellation',(#2));\n#4=IFCPRODUCTDEFINITIONSHAPE($,$,(#3));\n#5=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Triangle',$,$,$,#4,$,$);\n#6=IFCCARTESIANPOINT((0.,0.,0.));\n#7=IFCAXIS2PLACEMENT3D(#6,$,$);\n#8=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,0.00001,#7,$);","expected":{"products":[5],"vertices":3,"triangles":1,"bounds":[[0,0,0],[2,3,0]]}}
      """
    Then real vertices, triangles, world bounds and geometric measures agree

  @id-differential-geometry-faceted-brep
  @level-quick
  @mode-differential
  Scenario: Independently project faceted-brep geometry
    Given the language-neutral IFC4 representation graph
    When both implementations independently tessellate and place the source representation
      """
      {"name":"faceted-brep","data":"#1=IFCCARTESIANPOINT((0.,0.,0.));\n#2=IFCCARTESIANPOINT((2.,0.,0.));\n#3=IFCCARTESIANPOINT((0.,3.,0.));\n#4=IFCCARTESIANPOINT((0.,0.,4.));\n#5=IFCPOLYLOOP((#1,#3,#2));\n#6=IFCPOLYLOOP((#1,#2,#4));\n#7=IFCPOLYLOOP((#1,#4,#3));\n#8=IFCPOLYLOOP((#2,#3,#4));\n#9=IFCFACEOUTERBOUND(#5,.T.);\n#10=IFCFACEOUTERBOUND(#6,.T.);\n#11=IFCFACEOUTERBOUND(#7,.T.);\n#12=IFCFACEOUTERBOUND(#8,.T.);\n#13=IFCFACE((#9));\n#14=IFCFACE((#10));\n#15=IFCFACE((#11));\n#16=IFCFACE((#12));\n#17=IFCCLOSEDSHELL((#13,#14,#15,#16));\n#18=IFCFACETEDBREP(#17);\n#19=IFCSHAPEREPRESENTATION(#102,'Body','Brep',(#18));\n#20=IFCPRODUCTDEFINITIONSHAPE($,$,(#19));\n#21=IFCBUILDINGELEMENTPROXY('0123456789012345678901',$,'Tetrahedron',$,$,$,#20,$,$);\n#100=IFCCARTESIANPOINT((0.,0.,0.));\n#101=IFCAXIS2PLACEMENT3D(#100,$,$);\n#102=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,0.00001,#101,$);","expected":{"products":[21],"vertices":4,"triangles":4,"bounds":[[0,0,0],[2,3,4]],"volume":4}}
      """
    Then real vertices, triangles, world bounds and geometric measures agree
