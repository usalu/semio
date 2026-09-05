@capability-xlsx-ecma-376-strict-mutate
@oracle-xlsx-ecma-376-strict-mutate
@comparison-semantic-ooxml-xlsx-strict-v1
@mutations-xlsx-ecma-376-strict
Feature: Apply every typed XLSX ECMA-376 Strict conformance-class mutation to a real package
  The input is shared://📕️reuse-marketplaces.xlsx, a real committed ECMA-376 package: 11 parts, two
  worksheets and a genuine 229-entry shared-string table, derived once from the real committed survey
  of 50 European building-component reuse marketplaces.
  Unzipping it and reading xl/workbook.xml's root element shows a workbook that fails this class on
  its first two axes — a Transitional SpreadsheetML xmlns and a Transitional xmlns:r — with no
  conformance attribute and no VML part anywhere. A workbook is also the only OOXML package with a
  PER-WORKSHEET axis: [Content_Types].xml declares an Override for each of the two sheet parts, and
  set-worksheet-content-type moves exactly that declaration, which is why this catalog carries a
  kind neither the DOCX nor the PPTX conformance subsets have.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF 🧱️base. The 🧱️base subset of this same standard
  owns the DOCUMENT vocabulary — insert-sheet, remove-sheet, rename-sheet, set-cell, remove-cell and
  the three shared-string kinds. This subset owns the ISO/IEC 29500-1 Strict CONFORMANCE CLASS,
  which is a property of the OPC package and of no document object at all: check_strict_conformance
  reads exactly five axes on an already-decoded XlsxSnapshot: xl/workbook.xml's root xmlns, its root
  xmlns:r, its root conformance attribute, any part whose content type is the legacy VML drawing
  type, and each worksheet part's own content type. This catalog is one kind per axis. Disjointness here is checkable rather than asserted: set-cell writes a c/v pair inside a
  sheetData row and the shared-string kinds write sst entries, and none of them can reach
  xl/workbook.xml's root attributes or a [Content_Types].xml Override — while nothing in this
  catalog reads a cell.

  THE REFERENCE, AND WHAT IT CAN AND CANNOT WITNESS. `quick-xml` 0.42 rewrites xl/workbook.xml's
  root element and the Override entries of [Content_Types].xml; `zip` 6 reads all 11 entries of the
  real workbook and reassembles the container from those entries alone, never patching input bytes.
  Both read AND write, so this case really does have a second producer and every mutate scenario is
  honestly @mode-differential — worth stating plainly, because the 🧱️base XLSX case is NOT
  differential: no single crate both reads and writes a workbook, so it composes `calamine` for
  reading with `rust_xlsxwriter` for writing. At the CONTAINER level that problem does not arise.
  What this case witnesses is the five class axes only: the two worksheets and the 229-entry
  shared-string table are carried through faithfully and read by no axis at all.

  TWO OF THE NINE KINDS RUN ON AN ARRANGED PRE-STATE, AND THAT IS RECORDED RATHER THAN HIDDEN.
  remove-conformance-attribute and remove-vml-part each need their target to exist, and this workbook
  carries neither — nor does any other ECMA-376 package committed to this repository, verified by
  unzipping all three committed OOXML fixtures and searching every entry. That is a fact about the
  corpus rather than a gap to paper over. Both therefore run after the SAME independent
  implementation has inserted their target into the real 11-part container; the mutation under test
  is still the removal, still performed by the reference, still on genuine OPC. The other seven
  kinds read the committed bytes untouched. There is no alternate-content pair here at all, because
  ISO/IEC 29500-1's spreadsheet rules give this checker no mc:AlternateContent axis to police.

  Every scenario copies the committed .xlsx into the case work directory before touching it, so the
  229-entry shared-string table the 🧱️base case also reads is never written to by this one.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real workbook package
    Given the real input package shared://📕️reuse-marketplaces.xlsx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the conformance-class projection
    Examples:
      | id                           | params                                                                   |
      | set-snapshot                 | {"conformanceClass": "strict"}                                           |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/spreadsheetml/main"}           |
      | set-relationships-namespace  | {"namespace": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                      |
      | remove-conformance-attribute | {}                                                                       |
      | insert-vml-part              | {"path": "xl/drawings/vmlDrawing1.vml"}                                  |
      | remove-vml-part              | {"path": "xl/drawings/vmlDrawing1.vml"}                                  |
      | set-worksheet-content-type   | {"path": "xl/worksheets/sheet1.xml", "contentType": "application/xml"}   |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real workbook package
    Given the real input package shared://📕️reuse-marketplaces.xlsx
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the conformance-class projection

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real package
    Given the real input package shared://📕️reuse-marketplaces.xlsx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the conformance-class projection is the one the package started from
    Examples:
      | id                           | params                                                                   |
      | set-snapshot                 | {"conformanceClass": "strict"}                                           |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/spreadsheetml/main"}           |
      | set-relationships-namespace  | {"namespace": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                      |
      | remove-conformance-attribute | {}                                                                       |
      | insert-vml-part              | {"path": "xl/drawings/vmlDrawing1.vml"}                                  |
      | remove-vml-part              | {"path": "xl/drawings/vmlDrawing1.vml"}                                  |
      | set-worksheet-content-type   | {"path": "xl/worksheets/sheet1.xml", "contentType": "application/xml"}   |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real package
    Given the real input package shared://📕️reuse-marketplaces.xlsx
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    And the inverse mutation is applied to that result
    Then the conformance-class projection is the one the package started from

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real package without passing bytes through
    Given the real input package shared://📕️reuse-marketplaces.xlsx
    When the package is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance-class projection
    And the re-encoded bytes are not bit-identical to the input
