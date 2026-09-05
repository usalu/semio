@capability-docx-ecma-376-transitional-mutate
@oracle-docx-ecma-376-transitional-mutate
@comparison-semantic-ooxml-docx-transitional-v1
@mutations-docx-ecma-376-transitional
Feature: Apply every typed DOCX ECMA-376 Transitional conformance-class mutation to a real package
  The input is shared://📜️example-readme.docx, a real committed ECMA-376 package: 7 parts whose
  word/document.xml is 92,873 bytes of real content, derived once from this repository's own 47 KB
  README.
  Unzipping it and reading every namespace it declares shows a package that ALREADY satisfies this
  class on all three of its axes: word/document.xml declares the Transitional WordprocessingML
  namespace, no part and no relationship type mentions the strict-family purl.oclc.org/ooxml
  namespace, and no root conformance attribute contradicts the stamp. This case is therefore the
  mirror of mutate-docx-ecma-376-strict over the same bytes: every scenario starts INSIDE the class
  and moves the real package out of it — set-main-namespace to the strict namespace, then back —
  where the strict case starts outside and moves in.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF ✳️any. The ✳️any subset of this same standard
  owns the DOCUMENT vocabulary — insert-block, remove-block, set-run-text, set-run-formatting, the
  style kinds and the part kinds. This subset owns the ISO/IEC 29500-4 Transitional CONFORMANCE
  CLASS, which is a property of the OPC package and of no document object at all:
  check_transitional_conformance reads three axes: the main document part's Transitional
  WordprocessingML namespace, any strict-family (purl.oclc.org/ooxml) namespace in a part or a
  relationship type, and a root conformance="strict" that would contradict the stamp. VML and
  mc:AlternateContent are legal Transitional markup and are not policed, which is why this catalog
  declares four kinds fewer than its 📏️strict sibling. Disjointness here is checkable rather than asserted: the ✳️any style and part kinds
  rewrite w:styles and add OPC parts, and none of them can reach word/document.xml's root namespace
  declaration — while nothing in this catalog reads a paragraph.

  THE REFERENCE, AND WHAT IT CAN AND CANNOT WITNESS. `quick-xml` 0.42 rewrites word/document.xml's
  root element and every *.rels part; `zip` 6 reads all 7 entries of the real package and reassembles
  the container from those entries alone, never patching input bytes. Both read AND write, so this case has a real second producer for all six kinds and every mutate scenario is honestly @mode-differential.
  The evidence stops at the three axes ISO/IEC 29500-4 gives this class. VML and mc:AlternateContent
  are LEGAL Transitional markup, so nothing here polices them and this case says nothing whatever
  about them — that is why it declares four kinds fewer than its 📏️strict sibling, and the reason is
  the specification's rather than an editorial economy.

  ONE OF THE SIX KINDS RUNS ON AN ARRANGED PRE-STATE, AND THAT IS RECORDED RATHER THAN HIDDEN.
  remove-conformance-attribute needs a conformance attribute to remove, and this package has none —
  nor does any other ECMA-376 package committed to this repository, verified by unzipping all three
  committed OOXML fixtures and searching every entry. It therefore runs after the SAME independent
  implementation has stamped one onto the real 7-part container; the mutation under test is still
  the removal, still performed by the reference, still on genuine OPC. The other five kinds read the
  committed bytes untouched, which is the practical difference a conforming fixture makes: this case
  arranges one pre-state where its 📏️strict sibling arranges three.

  Every scenario copies the committed .docx into the case work directory before touching it, so the
  6-figure body part the ✳️any case also reads is never written to by this one.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document package
    Given the real input package shared://📜️example-readme.docx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the conformance-class projection
    Examples:
      | id                           | params                                                              |
      | set-snapshot                 | {"conformanceClass": "strict"}                                      |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/wordprocessingml/main"}   |
      | set-relationship-base        | {"base": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                 |
      | remove-conformance-attribute | {}                                                                  |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real document package
    Given the real input package shared://📜️example-readme.docx
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the conformance-class projection

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real package
    Given the real input package shared://📜️example-readme.docx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the conformance-class projection is the one the package started from
    Examples:
      | id                           | params                                                              |
      | set-snapshot                 | {"conformanceClass": "strict"}                                      |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/wordprocessingml/main"}   |
      | set-relationship-base        | {"base": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                 |
      | remove-conformance-attribute | {}                                                                  |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real package
    Given the real input package shared://📜️example-readme.docx
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
    Given the real input package shared://📜️example-readme.docx
    When the package is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance-class projection
    And the re-encoded bytes are not bit-identical to the input
