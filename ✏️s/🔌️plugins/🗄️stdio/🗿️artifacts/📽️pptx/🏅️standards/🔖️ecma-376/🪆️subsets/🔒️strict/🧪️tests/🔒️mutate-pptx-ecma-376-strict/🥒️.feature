@capability-pptx-ecma-376-strict-mutate
@oracle-pptx-ecma-376-strict-mutate
@comparison-semantic-ooxml-pptx-strict-v1
@mutations-pptx-ecma-376-strict
Feature: Apply every typed PPTX ECMA-376 Strict conformance-class mutation to a real package
  The input is shared://📽️.pptx, a real committed ECMA-376 package: 55 parts — 7 slides,
  11 slide layouts, a slide master, 3 real media parts and 22 relationship parts — from a real 2020
  conference deck.
  Unzipping it and reading every namespace across all 55 entries shows a package that fails this
  class on two independent namespace families at once: ppt/presentation.xml and all 7 slide parts
  declare Transitional PresentationML, and their shape trees declare Transitional DrawingML. That
  second family is what makes this deck the right input for a PPTX conformance case and what no
  DOCX or XLSX package can supply — set-main-namespace and set-drawing-namespace move the real
  package onto the class along two axes a text document and a workbook simply do not have.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF 🧱️base. The 🧱️base subset of this same standard
  owns the DOCUMENT vocabulary — the slide, shape, paragraph and run kinds. This subset owns the
  ISO/IEC 29500-1 Strict CONFORMANCE CLASS, which is a property of the OPC package and of no
  document object at all: check_strict_conformance reads six axes on an already-decoded
  PptxSnapshot, one of which the 🔒️strict DOCX subset does not have: besides the Strict
  PresentationML main namespace, the Transitional namespace, VML, the officeDocument relationship
  base, conformance="strict" and mc:AlternateContent, it separately rejects the Transitional
  DrawingML namespace — a second real namespace family a deck carries and a text document does not.
  This catalog is one kind per axis. Disjointness here is checkable rather than asserted: move-slide reorders p:sldIdLst and
  set-shape-position rewrites an a:xfrm, and neither can reach a root xmlns or a relationship type —
  while nothing in this catalog opens a slide's shape tree.

  THE REFERENCE, AND WHAT IT CAN AND CANNOT WITNESS. `quick-xml` 0.42 rewrites the root element of
  ppt/presentation.xml, of each of the 7 slide parts and of every one of the 22 *.rels parts; `zip` 6
  reads all 55 entries — the 3 real media binaries included — and reassembles the container from
  those entries alone, never patching input bytes. Both read AND write, so this case has a real second producer for all eleven kinds and every mutate scenario is honestly @mode-differential. The evidence is
  the six class axes and nothing else: the ordered slide list, the shapes on each slide and their
  EMU geometry that mutate-pptx-ecma-376 measures are invisible here, and so are the media parts,
  which are carried through the container faithfully but read by no axis of any conformance class.

  THREE OF THE ELEVEN KINDS RUN ON AN ARRANGED PRE-STATE, AND THAT IS RECORDED RATHER THAN HIDDEN.
  remove-conformance-attribute, remove-vml-part and remove-alternate-content each need their target
  to exist, and this deck carries none of the three — nor does any other ECMA-376 package committed
  to this repository, verified by unzipping all three committed OOXML fixtures and searching every
  entry. A 2020 conference deck exported by a modern authoring tool has no reason to carry legacy
  VML, and that is a fact about the corpus rather than a gap to paper over. Each of those three
  therefore runs after the SAME independent implementation has inserted its target into the real
  55-part container; the mutation under test is still the removal, still performed by the reference,
  still on genuine OPC. The other eight kinds read the committed bytes untouched.

  Every scenario copies the committed .pptx into the case work directory before touching it, so the
  7 real slides and 3 real media parts the 🧱️base case also reads are never written to by this one.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real presentation package
    Given the real input package shared://📽️.pptx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the conformance-class projection
    Examples:
      | id                           | params                                                              |
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

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real presentation package
    Given the real input package shared://📽️.pptx
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the conformance-class projection

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real package
    Given the real input package shared://📽️.pptx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the conformance-class projection is the one the package started from
    Examples:
      | id                           | params                                                              |
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

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-property
  Scenario: Undoing no-mutation restores the real package
    Given the real input package shared://📽️.pptx
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
    Given the real input package shared://📽️.pptx
    When the package is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance-class projection
    And the re-encoded bytes are not bit-identical to the input
