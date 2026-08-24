@capability-pptx-ecma-376-strict-mutate
@oracle-pptx-ecma-376-strict-mutate
@comparison-semantic-ooxml-pptx-strict-v1
@mutations-pptx-ecma-376-strict
Feature: Apply every typed PPTX ECMA-376 Strict conformance-class mutation to a real package
  The input is shared://🎞️semio-talk.pptx, a real committed ECMA-376 package: 55 parts — 7 slides,
  11 slide layouts, a slide master, 3 real media parts and 22 relationship parts — from a real
  conference deck. Unzipping the committed file and reading every declared namespace confirms it is
  a genuine ISO/IEC 29500-4 Transitional package that declares no strict-family namespace, no VML,
  no mc:AlternateContent and no conformance attribute — which is exactly what makes it the right
  input for a conformance-class case: every scenario moves the real package along one axis of the
  class and then back.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF ✳️any. The ✳️any subset of this same standard
  owns the DOCUMENT vocabulary — the slide, shape, paragraph and run kinds. This subset owns the
  ISO/IEC 29500-1 Strict CONFORMANCE CLASS, which is a property of the OPC package and of no
  document object at all: check_strict_conformance reads six axes on an already-decoded
  PptxSnapshot, one of which the ✳️strict DOCX subset does not have: besides the Strict
  PresentationML main namespace, the Transitional namespace, VML, the officeDocument relationship
  base, conformance="strict" and mc:AlternateContent, it separately rejects the Transitional
  DrawingML namespace — a second real namespace family a deck carries and a text document does not.
  This catalog is one kind per axis. No ✳️any mutation moves any of those axes and no mutation here
  touches document content, so the two vocabularies are disjoint by construction.

  THE REFERENCE. `quick-xml` 0.42 performs and observes every part edit, over the `zip` 6 container
  codec, which reads every entry of the real package and reassembles the whole container from those
  entries alone — never a patch of the input bytes. That pairing is a genuine second producer for
  every kind this catalog declares, which is why every mutate scenario is @mode-differential rather
  than a weaker mode. Both crates are test-only; this repository's own OPC and XML codecs are
  hand-written and link neither, so the oracle is not production-reachable.

  THE REMOVAL KINDS ARE ARRANGED, AND THAT IS RECORDED. Not one real ECMA-376 package committed to
  this repository carries VML markup, mc:AlternateContent or a conformance attribute — verified by
  unzipping all three committed OOXML fixtures and searching every entry, and a fact about the
  corpus rather than a gap to paper over. remove-conformance-attribute, remove-vml-part,
  remove-alternate-content therefore run on the real package after the SAME independent
  implementation has inserted their target; the mutation under test is still the removal, still
  performed by the reference, still on a genuine OPC container.

  Every scenario copies the fixture into the case work directory before touching it; the committed
  file is never written to.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real presentation package
    Given the real input package shared://🎞️semio-talk.pptx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the conformance-class projection
    Examples:
      | id                           | params                                                              |
      | no-mutation                  | {}                                                                  |
      | set-snapshot                 | {"conformanceClass": "strict"}                                      |
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/presentationml/main"}     |
      | set-drawing-namespace        | {"namespace": "http://purl.oclc.org/ooxml/drawingml/main"}          |
      | set-relationship-base        | {"base": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                 |
      | remove-conformance-attribute | {}                                                                  |
      | insert-vml-part              | {"path": "ppt/drawings/vmlDrawing1.vml"}                            |
      | remove-vml-part              | {"path": "ppt/drawings/vmlDrawing1.vml"}                            |
      | insert-alternate-content     | {"path": "ppt/slides/slide1.xml"}                                   |
      | remove-alternate-content     | {"path": "ppt/slides/slide1.xml"}                                   |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real package
    Given the real input package shared://🎞️semio-talk.pptx
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
      | set-main-namespace           | {"namespace": "http://purl.oclc.org/ooxml/presentationml/main"}     |
      | set-drawing-namespace        | {"namespace": "http://purl.oclc.org/ooxml/drawingml/main"}          |
      | set-relationship-base        | {"base": "http://purl.oclc.org/ooxml/officeDocument/relationships"} |
      | set-conformance-attribute    | {"value": "strict"}                                                 |
      | remove-conformance-attribute | {}                                                                  |
      | insert-vml-part              | {"path": "ppt/drawings/vmlDrawing1.vml"}                            |
      | remove-vml-part              | {"path": "ppt/drawings/vmlDrawing1.vml"}                            |
      | insert-alternate-content     | {"path": "ppt/slides/slide1.xml"}                                   |
      | remove-alternate-content     | {"path": "ppt/slides/slide1.xml"}                                   |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real package without passing bytes through
    Given the real input package shared://🎞️semio-talk.pptx
    When the package is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance-class projection
    And the re-encoded bytes are not bit-identical to the input
