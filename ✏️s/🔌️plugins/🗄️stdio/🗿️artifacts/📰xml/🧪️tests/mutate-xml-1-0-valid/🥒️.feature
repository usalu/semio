@capability-xml-1-0-valid-mutate
@oracle-quick-xml-xml-1-0-valid-mutate
@comparison-semantic-xml-valid-1-0-v1
@mutations-xml-1-0-valid
Feature: Apply every typed XML 1.0 valid-subset mutation to a real 40 KB DOCTYPE-bearing document
  📰️ **The input is a real 40 KB property list, derived ONCE from real committed content.** XML 1.0
  Fifth Edition §2.8 makes a document *valid* only if it carries a document type declaration whose
  Name is the document element's name, and exactly ONE committed document in this repository
  satisfies that: `shared://📰️macos-uttype-plist.xml`, copied verbatim from
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🖥️associations/macos/tech.semio.document.uttype.plist`,
  the real Uniform Type Identifier declaration macOS reads to associate `.semio` files with the app.
  It is 631 bytes, which puts every mutation at the document's edge. Every larger committed XML
  document in this repository — the OOXML parts, the SVG drawings, the HTML article — carries no
  DOCTYPE at all, so none of them is in this subset.

  `shared://📰️reuse-marketplaces-plist.xml` is therefore derived, by `🐍️derive-xml-valid-fixture.py`
  in the ticket folder, from real committed CONTENT in the same Apple PropertyList 1.0 dialect and
  under the same real Apple DOCTYPE: the real 50-row German building-material-reuse survey
  `../../📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv` — 50 real rows over 12 real columns — read with
  Python's own stdlib `csv` module and serialised with Python's own stdlib `plistlib`. Two
  independent standard-library implementations, neither of which knows anything about this
  repository; every key is a real column header of the real survey, every string is a real cell of
  it, and the `<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">`,
  the `<plist version="1.0">` document element and the tab indentation are `plistlib`'s own output
  for Apple's own format. The result is 40 440 bytes with real indentation whitespace as text nodes
  at five nesting levels and real German content with umlauts throughout, against 631 bytes at four
  levels for the UTType declaration.

  The UTType declaration is NOT gone: `identity-round-trip` still reads it on its own beside the
  survey, because it is the real production document this repository ships and that tie is worth
  keeping. Every scenario copies the fixture it reads into the case work directory before touching
  it; the committed documents are never written to.

  This is `✳️valid`'s own vocabulary, not `✳️any`'s — the two catalogs have exactly ONE kind in
  common (`set-text`). XML 1.0 Fifth Edition §2.8 makes a document *valid* only if it carries a
  document type declaration whose Name is the document element's name, and this subset's own
  conformance checker turns that into four axes: `stdio.xml.valid.doctype-missing`,
  `stdio.xml.valid.root-name-mismatch`, `stdio.xml.valid.standalone-external-subset` (§2.9) and
  `stdio.xml.valid.validity-not-fully-verified`. `✳️any` can leave all of them in one step and has no
  notion that it did: its `set-doctype` takes an arbitrary doctype, so `{"name": "book"}` on this
  document desynchronises §2.8 and `null` deletes the declaration outright. Every kind below is
  subset-closed instead — `declare-doctype` takes NO name (it derives one from the actual document
  element), `rename-document-element` retags the DOCTYPE Name in the SAME step, and `set-snapshot` is
  gated on the two hard axes where `✳️any`'s is ungated by design.

  Two kinds this subset deliberately does NOT declare, stated rather than left as a gap.
  `undeclare-doctype`: every application of it would make the document hard-invalid, so the kind
  would exist only to be rejected — removing the DOCTYPE is `✳️any`'s operation, reached by migrating
  the dialect down. `undeclare-entity`: a per-name removal is only ever applicable to a document that
  already declares that name, and no committed document in this repository declares an internal
  entity at all, so the kind could not be exercised against real content. The internal subset is
  edited by `declare-entity` (positional, because §4.2 binds the FIRST declaration of a name, so
  WHERE a declaration sits is semantic) and by `set-internal-subset`, which is also the operation
  that can empty it again.

  One property of the shared schema that limits what §4.1 can be tested here: `XmlNode::Text` holds
  LITERAL character data, because this artifact's own reader resolves the five predefined entities
  and numeric character references on read and rejects any other `&name;` outright. A general entity
  reference therefore cannot survive into the model, and §4.1's *Entity Declared* validity constraint
  has nothing in this schema to bite on — so `set-text` carries no entity gate and does not pretend
  to. The oracle's DOCTYPE grammar is deliberately MORE permissive than the subject's, keeping
  `<!ELEMENT>`/`<!ATTLIST>` declarations as opaque raw markup where the subject's schema refuses
  them; the projection reports them under `doctype.opaqueDeclarations` so a document carrying real
  content-model markup is visibly out of the subject's reach rather than silently equal to one
  without it.

  `quick-xml` reads AND writes real XML, so every kind below is genuinely differential: the oracle
  builds its tree from quick-xml's event stream through the shared `📰markup` family module, then
  decomposes the raw DOCTYPE with its OWN hand-rolled §2.8/§4.2 grammar and derives the four verdicts
  straight from the W3C text — never importing this repository's `parse_doctype`,
  `xml_document_to_text` or `check_valid_conformance`. Both sides' results are read back by that SAME
  independent projection before the `semantic-xml-valid-1-0-v1` profile compares them, never against
  each other's writing.

  Three laws are asserted IN ROLE, by the handler that plays the role, and are not deferred to the
  oracle-vs-subject comparison. Every `mutate-<kind>` row other than `no-mutation` requires the
  semantic projection to MOVE — a row whose parameters make the mutation a no-op against the real
  document tests nothing, and every `Examples` value below is chosen against this document's actual
  content for that reason: `set-text` addresses `[1,3,0]`, the real
  `<string>reuse-marketplaces</string>` text node, and `set-standalone` sets the pseudo-attribute the
  document does NOT have, which additionally flips the §2.9 verdict because the DOCTYPE really does
  reference an external subset. Every `inverse-<kind>` row requires apply-then-undo to restore that
  side's OWN reading of the original document's projection. And `identity-round-trip` requires that
  side's own decode → re-encode both to preserve its own projection and to move the bytes: XML 1.0 is
  no byte-preserving carrier, and this document's prolog is line-broken between the declaration, the
  DOCTYPE and the document element, whitespace a canonical writer does not re-derive.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real property-list document
    Given the real input document shared://📰️reuse-marketplaces-plist.xml
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    And the semantic projection moved, unless the kind is no-mutation
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"xml": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><!DOCTYPE plist SYSTEM \"PropertyList-1.0.dtd\"><plist version=\"1.0\"><dict><key>UTTypeIdentifier</key><string>tech.semio.kit</string></dict></plist>"} |
      | declare-doctype | {"externalId": {"kind": "system", "systemId": "https://www.apple.com/DTDs/PropertyList-1.0.dtd"}} |
      | rename-document-element | {"name": "propertyList"} |
      | set-external-subset | {"externalId": null} |
      | set-standalone | {"standalone": true} |
      | declare-entity | {"index": 0, "parameter": false, "name": "semioVendor", "value": "tech.semio"} |
      | set-internal-subset | {"declarations": [{"parameter": false, "name": "semioVendor", "value": "tech.semio"}, {"parameter": true, "name": "semioShared", "value": "tech.semio.shared"}]} |
      | set-text | {"path": [1, 3, 0], "text": "reuse-marketplaces-2026"} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real property-list document
    Given the real input document shared://📰️reuse-marketplaces-plist.xml
    When the <id> mutation is applied and then its own computed inverse is applied to that result
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the restored document's semantic projection equals the original document's own
    Examples:
      | id | params |
      | no-mutation | {} |
      | set-snapshot | {"xml": "<?xml version=\"1.0\" encoding=\"UTF-8\"?><!DOCTYPE plist SYSTEM \"PropertyList-1.0.dtd\"><plist version=\"1.0\"><dict><key>UTTypeIdentifier</key><string>tech.semio.kit</string></dict></plist>"} |
      | declare-doctype | {"externalId": {"kind": "system", "systemId": "https://www.apple.com/DTDs/PropertyList-1.0.dtd"}} |
      | rename-document-element | {"name": "propertyList"} |
      | set-external-subset | {"externalId": null} |
      | set-standalone | {"standalone": true} |
      | declare-entity | {"index": 0, "parameter": false, "name": "semioVendor", "value": "tech.semio"} |
      | set-internal-subset | {"declarations": [{"parameter": false, "name": "semioVendor", "value": "tech.semio"}, {"parameter": true, "name": "semioShared", "value": "tech.semio.shared"}]} |
      | set-text | {"path": [1, 3, 0], "text": "reuse-marketplaces-2026"} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode both real documents without passing bytes through
    Given the real input document shared://📰️reuse-marketplaces-plist.xml
    And the real macOS UTType declaration this case used to rest on shared://📰️macos-uttype-plist.xml
    When each document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection of both
    And the re-encoded bytes of each are not bit-identical to its input
