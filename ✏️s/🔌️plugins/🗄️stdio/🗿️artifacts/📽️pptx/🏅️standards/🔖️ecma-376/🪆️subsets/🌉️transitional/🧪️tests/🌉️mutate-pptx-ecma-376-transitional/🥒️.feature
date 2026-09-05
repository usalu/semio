@capability-pptx-ecma-376-transitional-mutate
@oracle-pptx-ecma-376-transitional-mutate
@comparison-semantic-ooxml-pptx-transitional-v1
@mutations-pptx-ecma-376-transitional
Feature: Apply every typed PPTX ECMA-376 Transitional conformance-class mutation to a real package
  The input is shared://📽️.pptx, a real committed ECMA-376 package: 55 parts — 7 slides,
  11 slide layouts, a slide master, 3 real media parts and 22 relationship parts — from a real 2020
  conference deck.
  Unzipping it and reading every namespace across all 55 entries shows a package that ALREADY
  satisfies this class on all three of its axes, with both of its Transitional namespace families —
  PresentationML on ppt/presentation.xml and the 7 slide parts, DrawingML inside their shape trees —
  declared and no strict-family purl.oclc.org/ooxml namespace anywhere. This case is therefore the
  mirror of mutate-pptx-ecma-376-strict over the same bytes: every scenario starts INSIDE the class
  and moves the deck out of it along one of two independently addressable namespace families, then
  back.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF 🧱️base. The 🧱️base subset of this same standard
  owns the DOCUMENT vocabulary — the slide, shape, paragraph and run kinds. This subset owns the
  ISO/IEC 29500-4 Transitional CONFORMANCE CLASS, which is a property of the OPC package and of no
  document object at all: check_transitional_conformance reads three axes — the Transitional
  PresentationML main namespace, any strict-family namespace in a part or a relationship type, and a
  contradicting conformance="strict" — over a package carrying TWO Transitional namespace families,
  PresentationML and DrawingML, each addressable on its own. Disjointness here is checkable rather than asserted: insert-slide and set-shape-text
  write slide markup and never a namespace declaration, and no kind in this catalog reads a shape.

  THE REFERENCE, AND WHAT IT CAN AND CANNOT WITNESS. `quick-xml` 0.42 rewrites the root element of
  ppt/presentation.xml, of each of the 7 slide parts and of every one of the 22 *.rels parts; `zip` 6
  reads all 55 entries — the 3 real media binaries included — and reassembles the container from
  those entries alone, never patching input bytes. Both read AND write, so this case has a real second producer for all seven kinds and every mutate scenario is honestly @mode-differential. The evidence
  stops at the three axes ISO/IEC 29500-4 gives this class: VML and mc:AlternateContent are LEGAL
  Transitional markup, so nothing here polices them and this case says nothing whatever about them.
  What it does carry that no DOCX or XLSX conformance case can is the DrawingML axis, exercised on
  its own by set-drawing-namespace.

  ONE OF THE SEVEN KINDS RUNS ON AN ARRANGED PRE-STATE, AND THAT IS RECORDED RATHER THAN HIDDEN.
  remove-conformance-attribute needs a conformance attribute to remove, and this deck has none — nor
  does any other ECMA-376 package committed to this repository, verified by unzipping all three
  committed OOXML fixtures and searching every entry. It therefore runs after the SAME independent
  implementation has stamped one onto ppt/presentation.xml inside the real 55-part container; the
  mutation under test is still the removal, still performed by the reference, still on genuine OPC.
  The other six kinds read the committed bytes untouched — three arrangements fewer than the
  🔒️strict sibling needs, because a Transitional fixture already sits where this class wants it.

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
