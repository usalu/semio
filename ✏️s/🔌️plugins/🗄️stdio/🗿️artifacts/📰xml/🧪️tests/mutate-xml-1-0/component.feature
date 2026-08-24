@capability-xml-1-0-mutate
@oracle-quick-xml-1-0-mutate
@comparison-semantic-xml-v1
@mutations-xml-1-0-any
Feature: Apply every typed XML 1.0 mutation to a real-world document
  The input is `word/document.xml`, extracted once (unzip, no other edit) from the real committed
  ECMA-376 example DOCX at ../../📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/📚️examples/🎬️demo/
  🖼️assets/📜️example.docx and copied into this artifact's own fixtures directory, per the wave 7
  fleet brief's first-choice candidate for a genuine XML document. It is a real OOXML WordprocessingML
  part: prefixed element names bound through `xmlns:w`, single- and multi-attribute elements in
  authored order, and self-closing empty elements (`<w:b/>`, `<w:pStyle .../>`) alongside paired ones
  with content. Every scenario copies it into the case work directory before touching it; the
  committed fixture is never written to.

  The part itself carries no XML declaration, no DOCTYPE, no CDATA, no comment and no entity
  reference — genuine OOXML parts are minified and rarely carry any of these. Those five subtleties
  are instead exercised THROUGH the mutation kinds themselves, on the same real document: `set-declaration`
  and `set-doctype` add a real declaration/doctype where none existed (and their `inverse` removes it
  again, proving survival in both directions); `insert-element` inserts one real element carrying a
  comment child, a CDATA child and an entity-escaped text child (`<`, `>`, `&`, `"`) together, with
  three attributes in a deliberately non-alphabetical author order.

  Attribute order is writer freedom: the `semantic-xml-v1` profile projects each element's attributes
  as an unordered name/value map rather than an ordered list, so `quick-xml`'s own attribute-emission
  order and this subset's own append-order never register as a difference. Self-closing vs paired
  empty-element form is invisible once decoded (both denote the same zero-children element), and
  numeric vs named character/entity references are invisible once decoded (both resolve to the same
  literal text) — neither needed an `ignoreKeys` entry, both are structural non-issues once the
  comparison works over the decoded tree rather than raw bytes. Sibling and child order IS normative
  and is never sorted.

  `quick-xml` reads AND writes real XML, so every kind below is genuinely differential: the oracle
  performs the mutation with `quick-xml`, the subject performs it with this subset's own
  `XmlSnapshot`/`XmlMutation`, and both results are read back through the SAME independent `quick-xml`
  projection (`project_xml_1_0`) before comparison.

  Both non-differential laws are asserted IN ROLE, by the handler that plays the role, and are not
  deferred to the oracle-vs-subject comparison: every `inverse-<kind>` row requires apply-then-undo
  to restore that side's OWN reading of the original document's projection, and
  `identity-round-trip` requires that side's own decode → re-encode both to preserve its own
  projection and to move the bytes. XML 1.0 is not a byte-preserving carrier — a conforming writer
  re-derives every tag, quote and character reference from the tree — so the byte half of the law
  applies in full on both sides. A scenario that only proved the reference library did not error
  would be vacuous — it is checkable without a second producer, so it is checked without one.
  ⚠️ OPEN, and left red rather than tuned away: the byte half of `identity-round-trip` FAILS on the
  ORACLE side today. `shared://📰️ooxml-word-document.xml` is byte-identical to the
  `word/document.xml` part of
  ../../📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/📜️example.docx, and
  that DOCX carries no `docProps`, zeroed zip timestamps and a 14-byte `numbering.xml` — it is this
  repository's own minified OOXML writer's output, not Microsoft Word's. `quick-xml`'s canonical
  serialization agrees with it character for character, so `output == input` here is two minifying
  writers coinciding rather than a pass-through, and the assertion cannot tell the two apart. The
  remedy belongs to the FIXTURE — re-derive the part from a genuinely Word-authored DOCX, whose
  `<?xml …?>` declaration and attribute quoting break the coincidence — not to the assertion.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://📰️ooxml-word-document.xml
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id              | params                                                                                                                                                                                                                                       |
      | no-mutation      | {}                                                                                                                                                                                                                                          |
      | set-snapshot     | {"xml": "<w:styles xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:style w:styleId=\"Normal\"><w:name w:val=\"Normal\"/></w:style><w:style w:styleId=\"Heading1\"><w:name w:val=\"heading 1\"/><w:basedOn w:val=\"Normal\"/></w:style></w:styles>"} |
      | set-declaration  | {"version": "1.0", "encoding": "UTF-8", "standalone": true}                                                                                                                                                                                |
      | set-doctype      | {"name": "w:document", "externalId": {"kind": "public", "publicId": "-//SEMIO//XML 1.0 Wave 7 Sample//EN", "systemId": "https://schemas.openxmlformats.org/wordprocessingml/2006/main.dtd"}, "entities": [{"parameter": false, "name": "semio", "value": "Semio End-to-End Testing Wave 7"}]} |
      | insert-element   | {"path": [0,1], "index": 3, "node": {"kind":"element","name":"w:r","attrs":[{"name":"w:id","value":"7"},{"name":"w:rev","value":"26-08-23"},{"name":"w:note","value":"wave7"}],"children":[{"kind":"comment","text":"wave 7 mutation test"},{"kind":"cdata","text":"<raw> unescaped content"},{"kind":"text","text":"Ticket <ENDTOEND> & \"testing\" review"}]}} |
      | remove-element   | {"path": [0,2,0], "index": 1}                                                                                                                                                                                                              |
      | set-attribute    | {"path": [0,0,0,0], "name": "w:val", "value": "Heading2"}                                                                                                                                                                                  |
      | set-text         | {"path": [0,1,2,0,0], "text": "Wave 7 <mutation> & review text"}                                                                                                                                                                           |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://📰️ooxml-word-document.xml
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id              | params                                                                                                                                                                                                                                       |
      | no-mutation      | {}                                                                                                                                                                                                                                          |
      | set-snapshot     | {"xml": "<w:styles xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:style w:styleId=\"Normal\"><w:name w:val=\"Normal\"/></w:style><w:style w:styleId=\"Heading1\"><w:name w:val=\"heading 1\"/><w:basedOn w:val=\"Normal\"/></w:style></w:styles>"} |
      | set-declaration  | {"version": "1.0", "encoding": "UTF-8", "standalone": true}                                                                                                                                                                                |
      | set-doctype      | {"name": "w:document", "externalId": {"kind": "public", "publicId": "-//SEMIO//XML 1.0 Wave 7 Sample//EN", "systemId": "https://schemas.openxmlformats.org/wordprocessingml/2006/main.dtd"}, "entities": [{"parameter": false, "name": "semio", "value": "Semio End-to-End Testing Wave 7"}]} |
      | insert-element   | {"path": [0,1], "index": 3, "node": {"kind":"element","name":"w:r","attrs":[{"name":"w:id","value":"7"},{"name":"w:rev","value":"26-08-23"},{"name":"w:note","value":"wave7"}],"children":[{"kind":"comment","text":"wave 7 mutation test"},{"kind":"cdata","text":"<raw> unescaped content"},{"kind":"text","text":"Ticket <ENDTOEND> & \"testing\" review"}]}} |
      | remove-element   | {"path": [0,2,0], "index": 1}                                                                                                                                                                                                              |
      | set-attribute    | {"path": [0,0,0,0], "name": "w:val", "value": "Heading2"}                                                                                                                                                                                  |
      | set-text         | {"path": [0,1,2,0,0], "text": "Wave 7 <mutation> & review text"}                                                                                                                                                                           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://📰️ooxml-word-document.xml
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
