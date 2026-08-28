@capability-docx-ecma-376-strict-mutate
@oracle-docx-ecma-376-strict-mutate
@comparison-semantic-ooxml-docx-strict-v1
@mutations-docx-ecma-376-strict
Feature: Apply every typed DOCX ECMA-376 Strict conformance-class mutation to a real package
  The input is shared://📜️example-readme.docx, a real committed ECMA-376 package: 7 parts whose
  word/document.xml is 92,873 bytes of real content, derived once from this repository's own 47 KB
  README.
  Unzipping it and reading every namespace it declares shows a package that fails this class on the
  very first axis: word/document.xml declares the Transitional WordprocessingML namespace, every
  relationship carries the Transitional officeDocument base, and there is no conformance attribute
  at all. That is the right starting point for an ISO/IEC 29500-1 STRICT case precisely because it
  is not strict — set-main-namespace, set-relationship-base and set-conformance-attribute each move
  the real package ONTO the class from outside it, and their inverses move it back off. The mirror
  case, mutate-docx-ecma-376-transitional, starts from the same bytes already inside its class and
  moves them out; between the two, the same real package is driven across the boundary in both
  directions.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF ✳️any. The ✳️any subset of this same standard
  owns the DOCUMENT vocabulary — insert-block, remove-block, set-run-text, set-run-formatting, the
  style kinds and the part kinds. This subset owns the ISO/IEC 29500-1 Strict CONFORMANCE CLASS,
  which is a property of the OPC package and of no document object at all: check_strict_conformance
  reads six axes on an already-decoded DocxSnapshot: the main document part's Strict
  WordprocessingML namespace, the Transitional namespace anywhere in the package, the VML namespace
  anywhere in the package, the officeDocument relationship base of every relationship, the main
  part's root conformance="strict", and mc:AlternateContent compatibility markup. This catalog is
  one kind per axis. Disjointness here is checkable rather than asserted: insert-block edits w:body and
  set-run-text edits a w:t, and neither can reach a root xmlns, a relationship base or a *.rels
  part — while nothing in this catalog opens the block tree at all.

  THE REFERENCE, AND WHAT IT CAN AND CANNOT WITNESS. `quick-xml` 0.42 rewrites word/document.xml's
  root element and every *.rels part; `zip` 6 reads all 7 entries of the real package and reassembles
  the container from those entries alone, never patching input bytes. Both read AND write, so this case has a real second producer for all ten kinds and every mutate scenario is honestly @mode-differential.
  What it witnesses is the six-axis conformance projection and nothing else: the 414-block body and
  the seven declared styles that mutate-docx-ecma-376 measures do not appear here at all, so a
  mutation that silently corrupted a paragraph would pass this case. That is a division of labour
  between two vocabularies, not coverage this case claims.

  THREE OF THE TEN KINDS RUN ON AN ARRANGED PRE-STATE, AND THAT IS RECORDED RATHER THAN HIDDEN.
  remove-conformance-attribute, remove-vml-part and remove-alternate-content each need their target
  to exist, and this package carries none of the three — nor does any other ECMA-376 package
  committed to this repository, verified by unzipping all three committed OOXML fixtures and
  searching every entry. That is a fact about the corpus, not a gap to paper over. Each of those
  three therefore runs after the SAME independent implementation has inserted its target into the
  real 7-part container; the mutation under test is still the removal, still performed by the
  reference, still on genuine OPC. The other seven kinds read the committed bytes untouched.

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
      | no-mutation                  | {}                                                                  |
      | set-snapshot                 | {"conformanceClass": "strict"}                                      |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/wordprocessingml/main"}   |
      | set-relationship-base        | {"base": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                 |
      | remove-conformance-attribute | {}                                                                  |
      | insert-vml-part              | {"path": "word/vmlDrawing1.vml"}                                    |
      | remove-vml-part              | {"path": "word/vmlDrawing1.vml"}                                    |
      | insert-alternate-content     | {"path": "word/document.xml"}                                       |
      | remove-alternate-content     | {"path": "word/document.xml"}                                       |

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
      | no-mutation                  | {}                                                                  |
      | set-snapshot                 | {"conformanceClass": "strict"}                                      |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/wordprocessingml/main"}   |
      | set-relationship-base        | {"base": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                 |
      | remove-conformance-attribute | {}                                                                  |
      | insert-vml-part              | {"path": "word/vmlDrawing1.vml"}                                    |
      | remove-vml-part              | {"path": "word/vmlDrawing1.vml"}                                    |
      | insert-alternate-content     | {"path": "word/document.xml"}                                       |
      | remove-alternate-content     | {"path": "word/document.xml"}                                       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real package without passing bytes through
    Given the real input package shared://📜️example-readme.docx
    When the package is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance-class projection
    And the re-encoded bytes are not bit-identical to the input
