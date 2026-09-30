@capability-xlsx-ecma-376-mutate
@oracle-xlsx-ecma-376-mutate
@comparison-semantic-spreadsheet-v1
@mutations-xlsx-ecma-376-base
Feature: Apply every typed XLSX ECMA-376 mutation to a real-world workbook
  The input is shared://📕️reuse-marketplaces.xlsx, a real two-sheet workbook derived ONCE (never
  synthesised) from the real committed 50-row, 12-column European building-component reuse-
  marketplace survey (✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv,
  itself derived from ♻️mit-bestand/📋️bericht/📋️zwischenbericht/📎️anhang/♻️bauteilboersen.tex): sheet
  "Marktplätze" is the real survey table verbatim, sheet "Länderübersicht" is a real per-country
  tally computed from its own "Land" column. Repeated real values (country names, access categories,
  platform channels) deduplicate into a genuine 229-entry shared-string table built by the reference
  writer itself — `xl/sharedStrings.xml` reports `uniqueCount="229"`, confirmed by unzipping the
  committed fixture. The example.xlsx this artifact's own demo previously pointed at is a 0-byte
  placeholder, never a real fixture; it is untouched by this case and reported separately.

  THE CONSTRAINT THAT SHAPES THIS FEATURE — no single crate both reads and modifies an XLSX.
  `calamine` 0.36 parses a real workbook into resolved cell values but exposes no accessor for its
  own shared-string table (`Xlsx<RS>::strings` and `read_shared_strings` are both private in
  calamine-0.36.1/src/xlsx/mod.rs — confirmed by reading the vendored source); it also collapses a
  `t="s"` shared-string reference and a `t="inlineStr"` literal into the same resolved value.
  `rust_xlsxwriter` 0.96 can only assemble a brand-new package, never open and patch the original,
  and its own shared-string table is populated ONLY as a byproduct of `write_string` on a cell — there
  is no API to insert, remove or target a pool entry independent of a cell write. Five of the nine
  declared kinds are fully representable as "read the whole workbook into a grid, change the grid,
  rebuild the whole workbook from it", which is a genuine second producer, so they stay
  `@mode-differential`: `insert-sheet`, `remove-sheet`, `rename-sheet`, `set-cell`, `remove-cell`.
  `set-snapshot` carries a whole replacement package, and "the workbook becomes this snapshot" is
  reproduced by the shared OPC engine, which writes the package the snapshot describes from its own
  parts, content types and relationships without ever reaching this repository's OPC codec.

  THE REMAINING THREE HAVE A SECOND PRODUCER TOO, AND IT IS NOT THAT PAIRING. `insert-shared-string`,
  `remove-shared-string` and `set-shared-string` address the pool by an INDEX independent of any cell
  reference — the axis neither `calamine` nor `rust_xlsxwriter` exposes. The conclusion once drawn
  from that was that no second producer existed and the oracle had to return the input unchanged.
  That conclusion was wrong. `xl/sharedStrings.xml` is a PART of an OPC package, and the second
  producer for a part is the container codec plus an XML reader/writer: `zip` 6 + `quick-xml` 0.42,
  which this owner has linked all along and which the six ECMA-376 conformance-class subsets already
  run on. The three pool kinds now read the real 229-entry pool out of the package, edit it by index,
  rewrite the part and reassemble the whole container from its parts — and the projection reads the
  result back out of the bytes, entry by entry. Nothing about the pool is adapter-tracked any more,
  and all nine kinds are `@mode-differential`.

  ONE KIND RUNS ON AN ARRANGED PRE-STATE, AND THAT IS RECORDED RATHER THAN HIDDEN. The vocabulary
  refuses to remove a pool entry a cell still references — the entry would dangle — and every one of
  the real workbook's 229 entries is referenced, so `remove-shared-string` runs on the real workbook
  after the reference has appended one unreferenced entry ("Nicht referenzierter Eintrag") and
  removes exactly that one, index 229; the reference refuses a referenced entry the same way the
  subject does. Its own undo is an append, so the last position is also the only one the reference
  can restore: `insert-shared-string` carries only a `value`, and no declared kind puts a string back
  at an interior position.

  Every scenario copies the fixture into the case work directory before touching it; the committed
  file is never written to.

  BOTH ROLES MAKE THE SAME PROJECTOR CHOICE PER SCENARIO ID. The three pool kinds are read back from
  `xl/sharedStrings.xml` with `zip` + `quick-xml` as `{sharedStringCount, sharedStrings}`; every other
  kind is read back through the `calamine` grid. A first differential run once projected the pool
  kinds through the grid on the subject side only, which compared projections of two different
  shapes; that wiring is gone.

  THE LAWS THE ORACLE ASSERTS IN-ROLE, so a scenario cannot pass merely because the reference
  pairing did not error. `inverse-<kind>` applies the mutation, undoes it with the reference's own
  independently computed inverse, and fails with the first diverging cell unless the result projects
  onto exactly what the real workbook projects onto — every kind held to the whole projection,
  `set-snapshot` included, since undoing a whole replacement means rebuilding the original's own
  grid. `identity-round-trip` fails unless the rebuilt bytes differ from the input AND their
  projection is identical to the input's.

  Every `params` cell is the leaf's own wire payload, exactly what `XlsxMutation::payload_value()`
  emits: a sheet is the `XlsxSheet` wire with tagged `XlsxCellValue`s, `set-snapshot` carries the whole
  replacement `XlsxSnapshot` (one worksheet "Ersatz" holding an inline string and a number), and
  `set-cell`/`remove-cell` address their cell by the lineage-bound `XlsxCellAddress` the subject takes
  on the real workbook — the worksheet part, the child path to the `c` element and its revision.
  The reference resolves that address independently: it walks the path through its own XML reader,
  reads the element's `r` reference and the part's sheet name, and edits the grid there. Both
  implementations read one wire; the subject decodes it through `Mutation::from_payload_value`, whose
  re-emitted payload must equal the row exactly, and undoes every kind with `Mutation::inverse`.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real workbook (independently reproducible)
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id           | params |
      | set-snapshot | {"snapshot":{"schema":"stdio.xlsx","opc":{"parts":[],"contentTypes":{"defaults":[["rels","application/vnd.openxmlformats-package.relationships+xml"],["xml","application/xml"]],"overrides":[["/xl/workbook.xml","application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"],["/xl/sharedStrings.xml","application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"],["/xl/worksheets/sheet1.xml","application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"]]},"relationships":{"":[{"id":"rId1","relType":"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument","target":"xl/workbook.xml","targetMode":"internal"}],"xl/workbook.xml":[{"id":"rId1","relType":"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet","target":"worksheets/sheet1.xml","targetMode":"internal"},{"id":"rId2","relType":"http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings","target":"sharedStrings.xml","targetMode":"internal"}]},"comment":""},"xmlParts":[{"path":"xl/sharedStrings.xml","contentType":"application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml","document":{"root":{"kind":"element","name":"sst","attrs":[{"name":"xmlns","value":"http://schemas.openxmlformats.org/spreadsheetml/2006/main"},{"name":"count","value":"0"},{"name":"uniqueCount","value":"0"}],"children":[]}}},{"path":"xl/workbook.xml","contentType":"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml","document":{"root":{"kind":"element","name":"workbook","attrs":[{"name":"xmlns","value":"http://schemas.openxmlformats.org/spreadsheetml/2006/main"},{"name":"xmlns:r","value":"http://schemas.openxmlformats.org/officeDocument/2006/relationships"}],"children":[{"kind":"element","name":"sheets","attrs":[],"children":[{"kind":"element","name":"sheet","attrs":[{"name":"name","value":"Ersatz"},{"name":"sheetId","value":"1"},{"name":"r:id","value":"rId1"}],"children":[]}]}]}}},{"path":"xl/worksheets/sheet1.xml","contentType":"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml","document":{"root":{"kind":"element","name":"worksheet","attrs":[{"name":"xmlns","value":"http://schemas.openxmlformats.org/spreadsheetml/2006/main"}],"children":[{"kind":"element","name":"sheetData","attrs":[],"children":[{"kind":"element","name":"row","attrs":[{"name":"r","value":"1"}],"children":[{"kind":"element","name":"c","attrs":[{"name":"r","value":"A1"},{"name":"t","value":"inlineStr"}],"children":[{"kind":"element","name":"is","attrs":[],"children":[{"kind":"element","name":"t","attrs":[{"name":"xml:space","value":"preserve"}],"children":[{"kind":"text","text":"Ersetzt"}]}]}]},{"kind":"element","name":"c","attrs":[{"name":"r","value":"B1"}],"children":[{"kind":"element","name":"v","attrs":[],"children":[{"kind":"text","text":"42"}]}]}]}]}]}}}]}} |
      | insert-sheet | {"sheet":{"name":"Quellen","cells":[{"row":1,"col":0,"value":{"kind":"inlineString","value":"Baustoffbörsen: Eine systematische Erhebung, 2024"}},{"row":2,"col":0,"value":{"kind":"inlineString","value":"Herkunft: mit-bestand/bericht/zwischenbericht/anhang/bauteilboersen.tex"}}]}} |
      | remove-sheet | {"name":"Länderübersicht"} |
      | rename-sheet | {"name":"Länderübersicht","newName":"Länder"} |
      | set-cell     | {"address":{"partPath":"xl/worksheets/sheet1.xml","nodePath":[3,2,2],"namespaceUri":"http://schemas.openxmlformats.org/spreadsheetml/2006/main","localName":"c","revision":"c10cb4fa36844692"},"value":{"kind":"inlineString","value":"Restado (überarbeitet)"}} |
      | remove-cell  | {"address":{"partPath":"xl/worksheets/sheet1.xml","nodePath":[3,5,7],"namespaceUri":"http://schemas.openxmlformats.org/spreadsheetml/2006/main","localName":"c","revision":"b2b766d8b96c5b6b"}} |

  @id-mutate
  @level-exhaustive
  @mode-round-trip
  Scenario Outline: Apply <id> to the real workbook (no independent second producer)
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                         |
      | insert-shared-string | {"value":"Ökobau Referenzquelle 2024"}         |
      | remove-shared-string | {"index":229}                                  |
      | set-shared-string    | {"index":0,"value":"Aktualisierter Quellwert"} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real workbook
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original workbook
    Examples:
      | id                   | params |
      | set-snapshot         | {"snapshot":{"schema":"stdio.xlsx","opc":{"parts":[],"contentTypes":{"defaults":[["rels","application/vnd.openxmlformats-package.relationships+xml"],["xml","application/xml"]],"overrides":[["/xl/workbook.xml","application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"],["/xl/sharedStrings.xml","application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"],["/xl/worksheets/sheet1.xml","application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"]]},"relationships":{"":[{"id":"rId1","relType":"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument","target":"xl/workbook.xml","targetMode":"internal"}],"xl/workbook.xml":[{"id":"rId1","relType":"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet","target":"worksheets/sheet1.xml","targetMode":"internal"},{"id":"rId2","relType":"http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings","target":"sharedStrings.xml","targetMode":"internal"}]},"comment":""},"xmlParts":[{"path":"xl/sharedStrings.xml","contentType":"application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml","document":{"root":{"kind":"element","name":"sst","attrs":[{"name":"xmlns","value":"http://schemas.openxmlformats.org/spreadsheetml/2006/main"},{"name":"count","value":"0"},{"name":"uniqueCount","value":"0"}],"children":[]}}},{"path":"xl/workbook.xml","contentType":"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml","document":{"root":{"kind":"element","name":"workbook","attrs":[{"name":"xmlns","value":"http://schemas.openxmlformats.org/spreadsheetml/2006/main"},{"name":"xmlns:r","value":"http://schemas.openxmlformats.org/officeDocument/2006/relationships"}],"children":[{"kind":"element","name":"sheets","attrs":[],"children":[{"kind":"element","name":"sheet","attrs":[{"name":"name","value":"Ersatz"},{"name":"sheetId","value":"1"},{"name":"r:id","value":"rId1"}],"children":[]}]}]}}},{"path":"xl/worksheets/sheet1.xml","contentType":"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml","document":{"root":{"kind":"element","name":"worksheet","attrs":[{"name":"xmlns","value":"http://schemas.openxmlformats.org/spreadsheetml/2006/main"}],"children":[{"kind":"element","name":"sheetData","attrs":[],"children":[{"kind":"element","name":"row","attrs":[{"name":"r","value":"1"}],"children":[{"kind":"element","name":"c","attrs":[{"name":"r","value":"A1"},{"name":"t","value":"inlineStr"}],"children":[{"kind":"element","name":"is","attrs":[],"children":[{"kind":"element","name":"t","attrs":[{"name":"xml:space","value":"preserve"}],"children":[{"kind":"text","text":"Ersetzt"}]}]}]},{"kind":"element","name":"c","attrs":[{"name":"r","value":"B1"}],"children":[{"kind":"element","name":"v","attrs":[],"children":[{"kind":"text","text":"42"}]}]}]}]}]}}}]}} |
      | insert-sheet         | {"sheet":{"name":"Quellen","cells":[{"row":1,"col":0,"value":{"kind":"inlineString","value":"Baustoffbörsen: Eine systematische Erhebung, 2024"}},{"row":2,"col":0,"value":{"kind":"inlineString","value":"Herkunft: mit-bestand/bericht/zwischenbericht/anhang/bauteilboersen.tex"}}]}} |
      | remove-sheet         | {"name":"Länderübersicht"} |
      | rename-sheet         | {"name":"Länderübersicht","newName":"Länder"} |
      | set-cell             | {"address":{"partPath":"xl/worksheets/sheet1.xml","nodePath":[3,2,2],"namespaceUri":"http://schemas.openxmlformats.org/spreadsheetml/2006/main","localName":"c","revision":"c10cb4fa36844692"},"value":{"kind":"inlineString","value":"Restado (überarbeitet)"}} |
      | remove-cell          | {"address":{"partPath":"xl/worksheets/sheet1.xml","nodePath":[3,5,7],"namespaceUri":"http://schemas.openxmlformats.org/spreadsheetml/2006/main","localName":"c","revision":"b2b766d8b96c5b6b"}} |
      | insert-shared-string | {"value":"Ökobau Referenzquelle 2024"} |
      | remove-shared-string | {"index":229} |
      | set-shared-string    | {"index":0,"value":"Aktualisierter Quellwert"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real workbook without passing bytes through
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the workbook is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
