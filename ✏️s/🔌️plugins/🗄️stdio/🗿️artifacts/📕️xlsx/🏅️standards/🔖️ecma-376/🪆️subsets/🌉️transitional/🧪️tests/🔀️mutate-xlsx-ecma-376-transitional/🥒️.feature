@capability-xlsx-ecma-376-transitional-mutate
@oracle-xlsx-ecma-376-transitional-mutate
@comparison-semantic-ooxml-xlsx-transitional-v1
@mutations-xlsx-ecma-376-transitional
Feature: Apply every typed XLSX ECMA-376 Transitional conformance-class mutation to a real package
  The input is shared://📕️reuse-marketplaces.xlsx, a real committed ECMA-376 package: 11 parts, two
  worksheets and a genuine 229-entry shared-string table, derived once from the real committed survey
  of 50 European building-component reuse marketplaces.
  Unzipping it and reading xl/workbook.xml's root element shows a workbook that ALREADY satisfies
  this class on all four of its axes: a Transitional SpreadsheetML xmlns, a Transitional xmlns:r, no
  conformance attribute claiming "strict", and the ordinary worksheet content type on both sheet
  parts. This case is therefore the mirror of mutate-xlsx-ecma-376-strict over the same bytes: every
  scenario starts INSIDE the class and moves the workbook out of it, including along the
  per-worksheet content-type axis that no DOCX or PPTX conformance subset has.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF 🧱️base. The 🧱️base subset of this same standard
  owns the DOCUMENT vocabulary — insert-sheet, remove-sheet, rename-sheet, set-cell, remove-cell and
  the three shared-string kinds. This subset owns the ISO/IEC 29500-4 Transitional CONFORMANCE
  CLASS, which is a property of the OPC package and of no document object at all:
  check_transitional_conformance reads four axes: xl/workbook.xml's root xmlns, its root xmlns:r, a
  root conformance attribute that must NOT say "strict", and each worksheet part's content type. It
  has no VML rule at all, because ISO/IEC 29500-4 Transitional deliberately retains VML — so this
  catalog declares two kinds fewer than its 🔒️strict sibling, and that difference is the
  specification's, not an editorial one. Disjointness here is checkable rather than asserted: the 🧱️base sheet and shared-string
  kinds write sheetData and sst markup, never a root attribute of workbook.xml or a content-type
  Override — while nothing in this catalog reads a cell.

  THE REFERENCE, AND WHAT IT CAN AND CANNOT WITNESS. `quick-xml` 0.42 rewrites xl/workbook.xml's
  root element and the Override entries of [Content_Types].xml; `zip` 6 reads all 11 entries of the
  real workbook and reassembles the container from those entries alone, never patching input bytes.
  Both read AND write, so this case really does have a second producer and every mutate scenario is
  honestly @mode-differential — worth stating plainly, because the 🧱️base XLSX case is NOT
  differential: no single crate both reads and writes a workbook, so it composes `calamine` for
  reading with `rust_xlsxwriter` for writing. At the CONTAINER level that problem does not arise.
  What this case witnesses is the four class axes only, and it has no VML rule at all: ISO/IEC
  29500-4 Transitional deliberately RETAINS VML, so policing it here would be inventing a rule the
  specification does not have. That is the whole of why this catalog declares two kinds fewer than
  its 🔒️strict sibling.

  ONE OF THE SEVEN KINDS RUNS ON AN ARRANGED PRE-STATE, AND THAT IS RECORDED RATHER THAN HIDDEN.
  remove-conformance-attribute needs a conformance attribute to remove, and this workbook has none —
  nor does any other ECMA-376 package committed to this repository, verified by unzipping all three
  committed OOXML fixtures and searching every entry. It therefore runs after the SAME independent
  implementation has stamped one onto xl/workbook.xml inside the real 11-part container; the
  mutation under test is still the removal, still performed by the reference, still on genuine OPC.
  The other six kinds read the committed bytes untouched.

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
