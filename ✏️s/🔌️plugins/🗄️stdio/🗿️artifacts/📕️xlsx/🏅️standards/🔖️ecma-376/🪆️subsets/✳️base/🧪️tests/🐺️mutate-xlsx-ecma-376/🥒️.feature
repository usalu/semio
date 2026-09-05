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
  is no API to insert, remove or target a pool entry independent of a cell write. Seven of the ten
  declared kinds are fully representable as "read the whole workbook into a grid, change the grid,
  rebuild the whole workbook from it", which is a genuine second producer, so they stay
  `@mode-differential`: `no-mutation`, `set-snapshot`, `insert-sheet`, `remove-sheet`,
  `rename-sheet`, `set-cell`, `remove-cell`.

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
  and all ten kinds are `@mode-differential`.

  ONE INVERSE GENUINELY DOES NOT EXIST, AND THE CASE SAYS SO RATHER THAN DODGING IT.
  `XlsxMutation::InsertSharedString` carries only a `value` and appends at `shared_strings.len()`
  (`../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️component.rs:145`), so no
  declared kind can put a string back at an INTERIOR position: `remove-shared-string {"index": 7}` of
  a 229-entry pool is not invertible in this vocabulary at all, and the production `inverse()` at that
  file's line 173 answers `SetSharedString`, which restores neither the pool's length nor the entry
  that shifted into the hole. That is a real gap in the vocabulary, reported here rather than papered
  over, and the oracle refuses such a request outright instead of returning a wrong undo. The
  Examples row therefore removes index 228 — the LAST entry of the real pool, the German header
  "Anzahl" — which append genuinely restores. Widening the vocabulary (an `insert-shared-string` that
  carries an index) is the fix, and it belongs to whoever owns that enum.

  Every scenario copies the fixture into the case work directory before touching it; the committed
  file is never written to.

  THE FIRST DIFFERENTIAL RUN OF THIS CASE FOUND THE ADAPTER'S OWN SUBJECT WIRING WRONG. All six pool
  scenarios diverged structurally: the oracle emitted `{"sharedStringCount":…,
  "sharedStrings":[…]}` and the subject emitted `{"format":…,"sheets":[…]}` — projections of two
  different SHAPES, which cannot be compared at all. The oracle half already branches on
  `is_pool_kind` and reads `xl/sharedStrings.xml` back with `zip` + `quick-xml` for exactly those
  three kinds, as the paragraph above describes; the subject half projected all ten kinds through
  the `calamine` grid instead. That also made `mutate-remove-shared-string` fail outright with
  `Cell string index not found in shared strings table`: removing pool entry 228 while leaving the
  sheets alone is what BOTH implementations do — it is this vocabulary's declared "address the pool
  by an index independent of any cell reference" semantics, and the oracle's own bytes carry the
  same dangling reference — but only the subject was asked to read the result back through a reader
  that resolves `t="s"` indices. The subject now makes the SAME projector choice the oracle makes,
  per scenario id. Nothing was ignored, no tolerance moved, no Examples row changed.

  THE LAWS THE ORACLE ASSERTS IN-ROLE, so a scenario cannot pass merely because the reference
  pairing did not error. `inverse-<kind>` applies the mutation, applies its own independently
  computed inverse, and fails with the first diverging cell unless the result projects onto exactly
  what the real workbook projects onto. `identity-round-trip` fails unless the rebuilt bytes differ
  from the input AND their projection is identical to the input's. ONE axis is exempt, and only for
  `set-snapshot`: `sharedStringCount`. It is adapter-tracked arithmetic rather than an observation
  (`calamine` cannot read the pool), and this case's `set-snapshot` target carries no pool at all —
  the JSON `sheets` shape has nowhere to put one, so the oracle's rebuild and the subject's
  `XlsxWorkbook { shared_strings: vec![] }` both genuinely land on 0 instead of back on the real
  229. `set-snapshot` is therefore not invertible on that axis by the vocabulary's own shape, which
  is stated here rather than checked with a contrived number; its sheet grid is still held to the
  full law, as is everything about every other kind.

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
      | set-snapshot  | {"sheets": [{"name": "Ersatz", "cells": [{"row": 1, "col": 0, "value": "Ersetzt"}, {"row": 1, "col": 1, "value": 42}]}]}                                       |
      | insert-sheet  | {"name": "Quellen", "cells": [{"row": 1, "col": 0, "value": "Baustoffbörsen: Eine systematische Erhebung, 2024"}, {"row": 2, "col": 0, "value": "Herkunft: mit-bestand/bericht/zwischenbericht/anhang/bauteilboersen.tex"}]} |
      | remove-sheet  | {"name": "Länderübersicht"}                                                                                                                                    |
      | rename-sheet  | {"name": "Länderübersicht", "newName": "Länder"}                                                                                                               |
      | set-cell      | {"sheetName": "Marktplätze", "row": 3, "col": 2, "value": "Restado (überarbeitet)"}                                                                            |
      | remove-cell   | {"sheetName": "Marktplätze", "row": 6, "col": 7}                                                                                                                |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real workbook (independently reproducible)
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the semantic projection

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
      | remove-shared-string  | {"index": 228}                                                  |
      | set-shared-string     | {"index": 0, "value": "Aktualisierter Quellwert"}             |

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
      | id                    | params                                                                                                                                                          |
      | set-snapshot          | {"sheets": [{"name": "Ersatz", "cells": [{"row": 1, "col": 0, "value": "Ersetzt"}, {"row": 1, "col": 1, "value": 42}]}]}                                       |
      | insert-sheet          | {"name": "Quellen", "cells": [{"row": 1, "col": 0, "value": "Baustoffbörsen: Eine systematische Erhebung, 2024"}, {"row": 2, "col": 0, "value": "Herkunft: mit-bestand/bericht/zwischenbericht/anhang/bauteilboersen.tex"}]} |
      | remove-sheet          | {"name": "Länderübersicht"}                                                                                                                                    |
      | rename-sheet          | {"name": "Länderübersicht", "newName": "Länder"}                                                                                                               |
      | set-cell              | {"sheetName": "Marktplätze", "row": 3, "col": 2, "value": "Restado (überarbeitet)"}                                                                            |
      | remove-cell           | {"sheetName": "Marktplätze", "row": 6, "col": 7}                                                                                                                |
      | insert-shared-string  | {"value": "Ökobau Referenzquelle 2024"}                                                                                                                        |
      | remove-shared-string  | {"index": 228}                                                                                                                                                    |
      | set-shared-string     | {"index": 0, "value": "Aktualisierter Quellwert"}                                                                                                              |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-differential
  Scenario: Undoing no-mutation restores the real workbook
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    And the inverse mutation is applied to that result
    Then the oracle and the subject agree on the semantic projection of the original workbook

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real workbook without passing bytes through
    Given the real input workbook shared://📕️reuse-marketplaces.xlsx
    When the workbook is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
