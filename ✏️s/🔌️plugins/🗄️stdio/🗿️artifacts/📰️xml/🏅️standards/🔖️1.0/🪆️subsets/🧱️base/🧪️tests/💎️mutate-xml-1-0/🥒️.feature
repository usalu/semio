@capability-xml-1-0-mutate
@oracle-quick-xml-1-0-mutate-reader
@comparison-semantic-xml-v1
@mutations-xml-1-0-base
Feature: Apply every typed XML 1.0 mutation to a real 92 KB OOXML document part
  The input is `word/document.xml`, extracted once (unzip, no other edit) from the real committed
  ../../📜️docx/🧫️fixtures/📜️example-readme.docx and copied into this artifact's own fixtures
  directory as `shared://🧪️ooxml-readme-document/🏷️.xml`. That DOCX was itself derived once from this
  repository's own real `README.md` (951 lines of real prose, 77 real headings, a real 37-row/
  7-column colour-reference table, real fenced code, real inline bold/italic spans) — see
  ../../📜️docx/🧪️tests/⚪️mutate-docx-ecma-376/🥒️.feature for that derivation. The part is
  92 873 bytes of real OOXML WordprocessingML: a real `<?xml version="1.0" encoding="UTF-8"
  standalone="yes"?>` declaration, prefixed element names bound through `xmlns:w`, 414 top-level
  body blocks, a real `w:tbl` of 37 real `w:tr` rows each carrying 7 real `w:tc` cells, paragraphs
  carrying up to nine sibling runs, and self-closing empty elements (`<w:b/>`, `<w:i/>`,
  `<w:pStyle .../>`) alongside paired ones with content. Every scenario copies it into the case work
  directory before touching it; the committed fixture is never written to.

  It replaced the 747-byte minified part this case used to rest on, which was the `word/document.xml`
  of the 1 648-byte demo DOCX — a document too small to place a mutation anywhere but at its edges.
  That part is NOT gone: `identity-round-trip` still reads it, because it is the one committed
  document on which this repository's writer and `quick-xml` are known to converge character for
  character, and that convergence is the whole reason the serialization-form probe below exists.

  The part carries no DOCTYPE, no CDATA, no comment and no entity reference — genuine OOXML parts
  rarely carry any of these. Those four subtleties are exercised THROUGH the mutation kinds
  themselves, on the same real document: `set-doctype` adds a real doctype where none existed (and
  its `inverse` removes it again, proving survival in both directions); `insert-element` inserts one
  real element carrying a comment child, a CDATA child and an entity-escaped text child (`<`, `>`,
  `&`, `"`) together, with three attributes in a deliberately non-alphabetical author order. The
  declaration is the one of the five this document DOES carry, so `set-declaration` overwrites a real
  declaration rather than inventing one: it flips the real `standalone="yes"` this document really
  carries to `standalone="no"` and leaves the version and the encoding where they are, so an
  implementation that drops the pseudo-attribute on write, or defaults it on read, fails.

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

  Three laws are asserted IN ROLE, by the handler that plays the role, and are not deferred to the
  oracle-vs-subject comparison. Every `mutate-<kind>` row other than `no-mutation` requires the
  semantic projection to MOVE — a row whose parameters make the mutation a no-op against the real
  document tests nothing, and every `Examples` value below is chosen against this part's actual
  content for that reason: `set-attribute` retags the real `w:pStyle` of the document's first
  paragraph from `Heading1` to `Heading2`, `insert-element` lands at index 3 of the real nine-run
  paragraph at body block 275, `remove-element` deletes the SECOND real cell of the real table's
  header row (body block 359, row 0 of 37, cell 1 of 7), and `set-text` rewrites the real text node
  inside that nine-run paragraph's fourth run. Every `inverse-<kind>` row requires apply-then-undo to
  restore that side's OWN reading of the original document's projection. And `identity-round-trip`
  requires that side's own decode → re-encode to preserve its own projection AND to prove it
  re-derived its output rather than handing the input back. A scenario that only proved the reference
  library did not error would be vacuous — all three are checkable without a second producer, so all
  three are checked without one.

  ⚠️ RESOLVED, and worth recording because the resolution was to change the ASSERTION rather than
  the fixture. `identity-round-trip` used to require the re-encoded bytes to differ from the input,
  and it failed. `shared://🏷️.xml` is byte-identical to the `word/document.xml`
  part of ../../📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/📜️example.docx
  — this repository's own minified OOXML writer's output, with no XML declaration and no
  inter-element whitespace — and `quick-xml`'s canonical serialization agrees with it character for
  character. `output == input` there is two minifying writers CONVERGING, which a byte-difference
  check cannot tell apart from a `read`/`write` shortcut that never parsed anything: it fails on a
  correct implementation and would pass on an incorrect one the moment a declaration-bearing fixture
  were swapped in. It is not evidence either way, so swapping the fixture would have hidden the
  problem rather than fixed it. What replaced it is a probe a byte copy cannot satisfy —
  serialization-form invariance. Each input is re-rendered with one insignificant space before the
  `>`/`/>` that terminates each start tag (XML 1.0 §3.1 admits `S?` in exactly that position, so the
  perturbed bytes denote the SAME document while being markup no writer would emit), and both
  renderings must re-encode to the SAME bytes. A shortcut that hands its input back returns the two
  different byte strings unchanged and fails immediately; only an implementation that parsed both
  into one tree and re-derived the output from it can pass. The probe is additionally required to be
  non-vacuous. Both sides assert it, over BOTH documents — the minified part whose convergence made
  the naive check useless, and the 92 KB declaration-bearing part where it would not have been.

  ⚠️ One real defect the inverse law found and that WAS fixed at its cause, in the oracle module:
  `oracles::apply_mutation_inverse` used to re-serialize between the forward step and the undo and
  re-parse those bytes, so the two steps did not see the same tree. XML parsing coalesces adjacent
  character data, so removing an element that sat between two whitespace text nodes left an index
  space the undo could no longer address. The minified part never showed it; the sibling SVG case
  did, on a pretty-printed real drawing. Both routings now apply the forward step and its inverse to
  ONE parsed tree, which is what the law claims and what the subject does.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://🧪️ooxml-readme-document/🏷️.xml
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    And the semantic projection moved, unless the kind is no-mutation
    Examples:
      | id              | params                                                                                                                                                                                                                                       |
      | set-declaration  | {"version": "1.0", "encoding": "UTF-8", "standalone": false}                                                                                                                                                                                |
      | set-doctype      | {"name": "w:document", "externalId": {"kind": "public", "publicId": "-//SEMIO//XML 1.0 Wave 7 Sample//EN", "systemId": "https://schemas.openxmlformats.org/wordprocessingml/2006/main.dtd"}, "entities": [{"parameter": false, "name": "semio", "value": "Semio End-to-End Testing Wave 7"}]} |
      | insert-element   | {"path": [0,275], "index": 3, "node": {"kind":"element","name":"w:r","attrs":[{"name":"w:id","value":"7"},{"name":"w:rev","value":"26-08-23"},{"name":"w:note","value":"wave7"}],"children":[{"kind":"comment","text":"wave 7 mutation test"},{"kind":"cdata","text":"<raw> unescaped content"},{"kind":"text","text":"Ticket <ENDTOEND> & \"testing\" review"}]}} |
      | remove-element   | {"path": [0,359,0], "index": 1}                                                                                                                                                                                                              |
      | set-attribute    | {"path": [0,0,0,0], "name": "w:val", "value": "Heading2"}                                                                                                                                                                                  |
      | set-text         | {"path": [0,275,3,0,0], "text": "Wave 7 <mutation> & review text"}                                                                                                                                                                           |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://🧪️ooxml-readme-document/🏷️.xml
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id              | params                                                                                                                                                                                                                                       |
      | set-declaration  | {"version": "1.0", "encoding": "UTF-8", "standalone": false}                                                                                                                                                                                |
      | set-doctype      | {"name": "w:document", "externalId": {"kind": "public", "publicId": "-//SEMIO//XML 1.0 Wave 7 Sample//EN", "systemId": "https://schemas.openxmlformats.org/wordprocessingml/2006/main.dtd"}, "entities": [{"parameter": false, "name": "semio", "value": "Semio End-to-End Testing Wave 7"}]} |
      | insert-element   | {"path": [0,275], "index": 3, "node": {"kind":"element","name":"w:r","attrs":[{"name":"w:id","value":"7"},{"name":"w:rev","value":"26-08-23"},{"name":"w:note","value":"wave7"}],"children":[{"kind":"comment","text":"wave 7 mutation test"},{"kind":"cdata","text":"<raw> unescaped content"},{"kind":"text","text":"Ticket <ENDTOEND> & \"testing\" review"}]}} |
      | remove-element   | {"path": [0,359,0], "index": 1}                                                                                                                                                                                                              |
      | set-attribute    | {"path": [0,0,0,0], "name": "w:val", "value": "Heading2"}                                                                                                                                                                                  |
      | set-text         | {"path": [0,275,3,0,0], "text": "Wave 7 <mutation> & review text"}                                                                                                                                                                           |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode both real documents without passing bytes through
    Given the real input document shared://🧪️ooxml-readme-document/🏷️.xml
    And the minified OOXML part this case used to rest on shared://🏷️.xml
    When each document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection of both
    And a byte-different rendering of each document re-encodes to exactly those bytes
