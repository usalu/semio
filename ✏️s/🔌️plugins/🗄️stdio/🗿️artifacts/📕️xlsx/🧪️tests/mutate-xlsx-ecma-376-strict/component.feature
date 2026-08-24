@capability-xlsx-ecma-376-strict-mutate
@oracle-xlsx-ecma-376-strict-mutate
@comparison-semantic-ooxml-xlsx-strict-v1
@mutations-xlsx-ecma-376-strict
Feature: Apply every typed XLSX ECMA-376 Strict conformance-class mutation to a real package
  The input is shared://📕️reuse-marketplaces.xlsx, a real committed ECMA-376 package: 11 parts, two
  worksheets and a genuine 229-entry shared-string table, derived once from the real committed
  survey of 50 European building-component reuse marketplaces. Unzipping the committed file and
  reading every declared namespace confirms it is a genuine ISO/IEC 29500-4 Transitional package
  that declares no strict-family namespace, no VML, no mc:AlternateContent and no conformance
  attribute — which is exactly what makes it the right input for a conformance-class case: every
  scenario moves the real package along one axis of the class and then back.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF ✳️any. The ✳️any subset of this same standard
  owns the DOCUMENT vocabulary — insert-sheet, remove-sheet, rename-sheet, set-cell, remove-cell and
  the three shared-string kinds. This subset owns the ISO/IEC 29500-1 Strict CONFORMANCE CLASS,
  which is a property of the OPC package and of no document object at all: check_strict_conformance
  reads exactly five axes on an already-decoded XlsxSnapshot: xl/workbook.xml's root xmlns, its root
  xmlns:r, its root conformance attribute, any part whose content type is the legacy VML drawing
  type, and each worksheet part's own content type. This catalog is one kind per axis. No ✳️any
  mutation moves any of those axes and no mutation here touches document content, so the two
  vocabularies are disjoint by construction.

  THE REFERENCE. `quick-xml` 0.42 performs and observes every part edit, over the `zip` 6 container
  codec, which reads every entry of the real package and reassembles the whole container from those
  entries alone — never a patch of the input bytes. That pairing is a genuine second producer for
  every kind this catalog declares, which is why every mutate scenario is @mode-differential rather
  than a weaker mode. Both crates are test-only; this repository's own OPC and XML codecs are
  hand-written and link neither, so the oracle is not production-reachable.

  THE REMOVAL KINDS ARE ARRANGED, AND THAT IS RECORDED. Not one real ECMA-376 package committed to
  this repository carries VML markup, mc:AlternateContent or a conformance attribute — verified by
  unzipping all three committed OOXML fixtures and searching every entry, and a fact about the
  corpus rather than a gap to paper over. remove-conformance-attribute, remove-vml-part therefore
  run on the real package after the SAME independent implementation has inserted their target; the
  mutation under test is still the removal, still performed by the reference, still on a genuine OPC
  container.

  Every scenario copies the fixture into the case work directory before touching it; the committed
  file is never written to.

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
      | no-mutation                  | {}                                                                       |
      | set-snapshot                 | {"conformanceClass": "strict"}                                           |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/spreadsheetml/main"}           |
      | set-relationships-namespace  | {"namespace": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                      |
      | remove-conformance-attribute | {}                                                                       |
      | insert-vml-part              | {"path": "xl/drawings/vmlDrawing1.vml"}                                  |
      | remove-vml-part              | {"path": "xl/drawings/vmlDrawing1.vml"}                                  |
      | set-worksheet-content-type   | {"path": "xl/worksheets/sheet1.xml", "contentType": "application/xml"}   |

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
      | no-mutation                  | {}                                                                       |
      | set-snapshot                 | {"conformanceClass": "strict"}                                           |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/spreadsheetml/main"}           |
      | set-relationships-namespace  | {"namespace": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                      |
      | remove-conformance-attribute | {}                                                                       |
      | insert-vml-part              | {"path": "xl/drawings/vmlDrawing1.vml"}                                  |
      | remove-vml-part              | {"path": "xl/drawings/vmlDrawing1.vml"}                                  |
      | set-worksheet-content-type   | {"path": "xl/worksheets/sheet1.xml", "contentType": "application/xml"}   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real package without passing bytes through
    Given the real input package shared://📕️reuse-marketplaces.xlsx
    When the package is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance-class projection
    And the re-encoded bytes are not bit-identical to the input
