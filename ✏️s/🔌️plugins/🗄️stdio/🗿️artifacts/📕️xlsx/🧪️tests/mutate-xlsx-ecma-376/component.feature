@capability-xlsx-ecma-376-mutate
@oracle-xlsx-ecma-376-mutate
@comparison-semantic-spreadsheet-v1
@mutations-xlsx-ecma-376-any
Feature: Apply every typed XLSX ECMA-376 mutation to a real-world workbook
  The input is shared://📕️reuse-marketplaces.xlsx, a real two-sheet workbook derived ONCE (never
  synthesised) from the real committed 50-row, 12-column European building-component reuse-
  marketplace survey (✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv,
  itself derived from ♻️mit-bestand/📋️bericht/📋️zwischenbericht/anhang/bauteilboersen.tex): sheet
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
  is no API to insert, remove or target a pool entry independent of a cell write. Seven of the ten
  declared kinds are fully representable as "read the whole workbook into a grid, change the grid,
  rebuild the whole workbook from it", which is a genuine second producer, so they stay
  `@mode-differential`: `no-mutation`, `set-snapshot`, `insert-sheet`, `remove-sheet`,
  `rename-sheet`, `set-cell`, `remove-cell`. The remaining three — `insert-shared-string`,
  `remove-shared-string`, `set-shared-string` — address the shared-string pool by an INDEX
  independent of any cell reference, exactly the axis neither reference crate exposes, so this
  reference pairing cannot independently reproduce them; their `mutate` scenario is `@mode-round-trip`
  rather than `@mode-differential` (the oracle honestly returns the input unchanged — the correct
  answer this pairing can give, since no cell is affected either way — and the comparison instead
  carries `sharedStringCount` as adapter-tracked arithmetic on the oracle side, mirrored against the
  subject's own real `XlsxWorkbook::shared_strings.len()` on the subject side). Every scenario copies
  the fixture into the case work directory before touching it; the committed file is never written
  to.

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
      | id            | params                                                                                                                                                          |
      | no-mutation   | {}                                                                                                                                                              |
      | set-snapshot  | {"sheets": [{"name": "Ersatz", "cells": [{"row": 1, "col": 0, "value": "Ersetzt"}, {"row": 1, "col": 1, "value": 42}]}]}                                       |
      | insert-sheet  | {"name": "Quellen", "cells": [{"row": 1, "col": 0, "value": "Baustoffbörsen: Eine systematische Erhebung, 2024"}, {"row": 2, "col": 0, "value": "Herkunft: mit-bestand/bericht/zwischenbericht/anhang/bauteilboersen.tex"}]} |
      | remove-sheet  | {"name": "Länderübersicht"}                                                                                                                                    |
      | rename-sheet  | {"name": "Länderübersicht", "newName": "Länder"}                                                                                                               |
      | set-cell      | {"sheetName": "Marktplätze", "row": 3, "col": 2, "value": "Restado (überarbeitet)"}                                                                            |
      | remove-cell   | {"sheetName": "Marktplätze", "row": 6, "col": 7}                                                                                                                |

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
      | id                    | params                                                        |
      | insert-shared-string  | {"value": "Ökobau Referenzquelle 2024"}                       |
      | remove-shared-string  | {"index": 0}                                                  |
      | set-shared-string     | {"index": 0, "value": "Aktualisierter Quellwert"}             |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real workbook
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original workbook
    Examples:
      | id                    | params                                                                                                                                                          |
      | no-mutation           | {}                                                                                                                                                              |
      | set-snapshot          | {"sheets": [{"name": "Ersatz", "cells": [{"row": 1, "col": 0, "value": "Ersetzt"}, {"row": 1, "col": 1, "value": 42}]}]}                                       |
      | insert-sheet          | {"name": "Quellen", "cells": [{"row": 1, "col": 0, "value": "Baustoffbörsen: Eine systematische Erhebung, 2024"}, {"row": 2, "col": 0, "value": "Herkunft: mit-bestand/bericht/zwischenbericht/anhang/bauteilboersen.tex"}]} |
      | remove-sheet          | {"name": "Länderübersicht"}                                                                                                                                    |
      | rename-sheet          | {"name": "Länderübersicht", "newName": "Länder"}                                                                                                               |
      | set-cell              | {"sheetName": "Marktplätze", "row": 3, "col": 2, "value": "Restado (überarbeitet)"}                                                                            |
      | remove-cell           | {"sheetName": "Marktplätze", "row": 6, "col": 7}                                                                                                                |
      | insert-shared-string  | {"value": "Ökobau Referenzquelle 2024"}                                                                                                                        |
      | remove-shared-string  | {"index": 0}                                                                                                                                                    |
      | set-shared-string     | {"index": 0, "value": "Aktualisierter Quellwert"}                                                                                                              |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real workbook without passing bytes through
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the workbook is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
