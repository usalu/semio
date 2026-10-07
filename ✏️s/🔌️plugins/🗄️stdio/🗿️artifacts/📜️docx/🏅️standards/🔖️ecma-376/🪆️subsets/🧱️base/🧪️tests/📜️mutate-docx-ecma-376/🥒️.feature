@capability-docx-ecma-376-mutate
@oracle-jszip-docx-ecma-376-mutate-reader
@comparison-semantic-docx-ecma-376-jszip-v1
@mutations-docx-ecma-376-base
Feature: Apply every typed DOCX ECMA-376 mutation to a real-world document
  The committed `example.docx` under this artifact's own demo example is a genuine OOXML package but
  only 1,648 bytes -- thin for exercising all 14 `DocxMutation` kinds. No larger real `.docx` exists
  anywhere in this repository (`♻️mit-bestand`, `temp/` and every other tree were searched first) --
  a real `.pptx` and other office-adjacent binaries exist under `temp/`, but no `.docx`. Rather than
  a synthetic 2-paragraph stub, a substantial real DOCX was DERIVED ONCE from this repository's own
  real `README.md` (951 lines of real prose, 77 real headings, a real 37-row/7-column color-reference
  table, real fenced code blocks, real inline **bold**/*italic* markdown) by a hand-rolled OPC/
  WordprocessingML builder (Python stdlib `zipfile` only, no new dependency -- the script is a
  disposable ticket-folder artifact, never imported by production or test code) that maps markdown
  headings to `Heading1`/`Heading2`/`Heading3` paragraphs, the real markdown table to a real `w:tbl`,
  fenced code to `Code`-styled paragraphs, and inline `**bold**`/`*italic*` spans to real multi-run
  paragraphs with `w:b`/`w:i` -- real styles (`Normal`, `Title`, `Heading1..3`, `Code`, `TableCell`),
  real multiple parts (`word/document.xml`, `word/styles.xml`, `docProps/core.xml`,
  `docProps/app.xml`, `[Content_Types].xml`, `_rels/.rels`, `word/_rels/document.xml.rels`), 414
  top-level body blocks including a real nested table (37 rows). Derivation is fully reproducible:
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w7-docx-ecma-376-mutate/
  derive_fixture.py`. Committed once at `shared://📜️example-readme.docx`; every scenario copies it
  into the case work directory before touching it, and the committed fixture is never written to.

  `InsertBlock`/`RemoveBlock` -- the document-structure analogue of this wave's page operations --
  target the real color table's own cells (`segments: [{blockIndex, row, cell}]`, mirroring
  `DocxBlockPath`), not a flat top-level index: `insert-block` adds an annotation paragraph inside
  the "Primary" swatch row's first cell, `remove-block` deletes the sole paragraph from the
  "Secondary" swatch row's first cell, both exercising the full `Table -> rows -> cells -> blocks`
  path-segment traversal against real, pre-existing structure rather than a synthetic one-level tree.

  `SetSnapshot` replaces `document.body` + `document.styles` only (the typed semantic view this
  subset's own `DocxDocument` models) -- real OPC parts outside that typed view are exercised
  separately by `SetPart`/`RemovePart` and are deliberately left untouched by `SetSnapshot` here, per
  `../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🔣️.json`'s own comparison-profile note.

  `set-part` overwrites the real, pre-existing `docProps/app.xml` (exercising the "replace" branch of
  "inserting or replacing"); `remove-part` deletes the real, pre-existing `docProps/core.xml` --
  both real parts this derivation's own builder wrote, restored exactly by their own inverse.

  Both `zip` (OPC container) and `quick-xml` (every OOXML part) read AND write for real, so every
  kind below is genuinely differential: the oracle performs the mutation with the two composed
  reference libraries, the subject performs it with this subset's own `DocxSnapshot`/`DocxMutation`,
  and both results are read back through the SAME independent `project_docx_ecma_376` before
  comparison.

  ONE INVERSE GENUINELY DOES NOT EXIST, AND THE CASE SAYS SO RATHER THAN DODGING IT. The committed
  fixture's word/styles.xml declares seven styles in order — Normal, Title, Heading1, Heading2,
  Heading3, Code, TableCell — and DocxMutation::InsertStyle carries only a style and APPENDS
  (../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:181), so no
  declared kind can put a style back at an interior position. remove-style {"id": "Title"} is
  therefore not invertible in this vocabulary at all: undoing it leaves Heading1 where Title was, and
  the inverse law caught exactly that. The oracle now refuses such a request outright instead of
  returning an undo that does not undo, and the Examples row removes TableCell — the LAST style,
  which append genuinely restores. Widening the vocabulary (an insert-style that carries a position)
  is the fix, and it belongs to whoever owns that enum.

  THE FIRST DIFFERENTIAL RUN OF THIS CASE FOUND A REAL DIVERGENCE, AND IT WAS FIXED IN OUR CODE.
  `inverse-set-snapshot` came back 12 differences apart from the oracle: `$.styles[1..6]` — every
  interior style — sat in the wrong place. `DocxMutation::SetSnapshot`'s inverse is
  `SetSnapshot{snapshot: base}`, which is correct; what was wrong is that `DocxDiff::between` routed
  the style list through a name-keyed collection triple that transported no ORDER, so applying it
  kept the survivors in their base order and APPENDED the four re-added styles. `set-snapshot` is a
  total replacement, so `apply(base, between(base, next))` has to land on `next` exactly — the
  ordered style list `semantic-docx-ecma-376-mutate-v1` projects by index included. The triple now
  carries the exact final key sequence, populated only when the survivors-then-additions default
  would not reproduce it, and `inverse_named` restores the base's own sequence. No comparison
  profile was touched, no `ignoreKeys` added, no Examples row changed; the oracle was already right.
  This does NOT widen the vocabulary: `InsertStyle` still appends by definition, so the
  interior-`remove-style` gap described above is exactly as non-invertible as it was.

  THE JUDGE. `jszip-docx-ecma-376-mutate-reader` is a third-party READER (jszip + fast-xml-parser): each mutation
  row's expected package is not computed, it is the COMMITTED `➡️after.docx` under
  `🧫️fixtures/🧾️readme-afters/<fixture>/`, written by python-docx (MIT, a second third-party library) applying that very
  row to the real README package through its own package/part/oxml model (generator recorded in each fixture manifest);
  the real README itself is the expected package of every inverse row, both no-mutation baselines and the identity
  round trip. `docx-ecma-376-jszip-compare-v1` reads it and the subject's `actual-docx` with the same reader. The
  `zip`+`quick-xml` composition below stays as the Rust supplement and keeps asserting all three laws in role.

  TWO READINGS OF THE JUDGE OVER-CLAIMED, AND THE SUBJECT'S FIRST RUN AGAINST IT SHOWED BOTH. Every
  scenario, the identity round trip included, came back unequal. First, every other OPC part was
  compared by a digest of its raw BYTES; but this subset's snapshot keeps each XML-bearing part
  (docProps/core.xml, docProps/app.xml, …) as its logical document, exactly as it keeps
  word/document.xml, so its writer materializes line ends and start-tag layout of its own — bytes no
  XML reader distinguishes. An XML-bearing part is now digested over its parsed logical content
  (element names, attributes by name, text, in document order), any other part over its bytes; the
  set-part and remove-part rows still move the projection. Second, run flags were read by the mere
  PRESENCE of w:b/w:i/w:u, while ECMA-376 Part 1 §17.17.4 makes w:b and w:i ST_OnOff toggles that
  w:val="0"/"false"/"off" switches off, and w:u w:val="none" underlines nothing. set-run-formatting's
  bold:false is written as <w:b w:val="0"/> — the explicit switch-off the snapshot keeps distinct
  from an absent toggle — and python-docx's after-document drops the toggle; both are not bold, and
  the reader now says so. The same two readings are corrected in the Rust supplement's projection.
  The subject's inverse rows apply the production inverse (`inverse_docx_mutation`) rather than a
  whole-document restore, so they hold this implementation's own undo to the law.

  ALL THREE LAWS ARE ASSERTED IN ROLE, through the shared ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law module,
  so no scenario can pass merely because the reference composition declined to error.
  `mutate-<kind>` fails unless the mutation MOVES the very projection the case is compared through:
  a kind that applies cleanly and changes nothing observable would otherwise report a green for a
  mutation nobody watched, and until this wave all thirteen of them did exactly that.
  `inverse-<kind>` applies the mutation, applies its own independently computed inverse, and fails
  with the first diverging field unless the result projects onto exactly what the original document
  projects onto. `identity-round-trip` fails unless the rebuilt archive differs from the input AND
  its projection is identical to the input's. NONE of the three is scoped down and NO kind is exempt
  from any of them: `semantic-docx-ecma-376-mutate-v1` declares no writer freedom at all, and the
  whole projection — the ordered block tree, the ordered style list and the path-keyed digest of
  every other OPC part — has to move for a mutation and come back for an inverse. The set-part and
  remove-part kinds reach the projection through that last member, which is why the digest map is
  part of it rather than an afterthought. The same three laws are proven again at unit level over
  these very Examples rows by
  `every_declared_kind_is_observable_and_its_inverse_restores_the_document` in
  ../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️component.rs, which READS this table rather
  than restating it, so the two can never drift apart — and the same module pins the
  remove-style-of-an-interior-style refusal described above.

  The `patch-snapshot` row is one RFC 6901 pointer operation on the subject's own `DocxSnapshot` reading — the logical XML
  parts in archive order, each a retained arena whose `attributes` list every element's attributes in pre-order — and sets
  the first body paragraph's `w:pStyle` `w:val` from Heading1 to Heading2. Its committed after-document was written by
  python-docx through its own package/part/oxml model from that row, and re-read against the patched reading before it was
  committed (this ticket's `🧪️s4-stdio-docx-patch-after.py`); the zip+quick-xml oracle applies the same pointer to its own
  reading of the package.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://📜️example-readme.docx
    And the committed after-document shared://🧾️readme-afters/<fixture>/➡️after.docx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the jszip reader reads the subject's package and the committed after-document as the same DOCX
    Examples:
      | id                 | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | fixture              |
      | insert-block       | {"path": {"segments": [{"blockIndex": 359, "row": 1, "cell": 0}], "index": 1}, "block": {"kind": "paragraph", "style": "TableCell", "runs": [{"text": "(wave 7 annotation)", "bold": false, "italic": true, "underline": false}]}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | ➕️insert-block       |
      | remove-block       | {"path": {"segments": [{"blockIndex": 359, "row": 2, "cell": 0}], "index": 0}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | ➖️remove-block       |
      | set-block-content  | {"path": {"segments": [], "index": 4}, "block": {"kind": "paragraph", "style": "Normal", "runs": [{"text": "Wave 7 replaced this admonition paragraph outright.", "bold": false, "italic": true, "underline": false}]}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | 📝️set-block-content  |
      | set-run-text       | {"address": {"partPath": "word/document.xml", "nodePath": [0, 177, 1], "expectedName": "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}r", "revision": "1dc1281dba7711ea"}, "text": "Wave 7 mutation replaced this run's text entirely, still real."}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | ✍️set-run-text       |
      | set-run-formatting | {"address": {"partPath": "word/document.xml", "nodePath": [0, 177, 1], "expectedName": "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}r", "revision": "1dc1281dba7711ea"}, "bold": false, "italic": true, "underline": true}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | 🪄️set-run-formatting |
      | insert-style       | {"style": {"id": "Callout", "name": "Callout", "basedOn": "Normal"}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | 💬️insert-style       |
      | remove-style       | {"id": "TableCell"}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | 🗨️remove-style       |
      | set-style-name     | {"id": "Heading2", "name": "Section Heading"}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | 🔤️set-style-name     |
      | set-style-based-on | {"id": "Heading3", "based_on": "Heading1"}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | 🌳️set-style-based-on |
      | set-part           | {"path":"docProps/app.xml","content_type":"application/vnd.openxmlformats-officedocument.extended-properties+xml","payload":{"kind":"xml","document":{"root":{"kind":"element","name":"Properties","attrs":[{"name":"xmlns","value":"http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"}],"children":[{"kind":"element","name":"Application","attrs":[],"children":[{"kind":"text","text":"semio-wave7-mutation-test"}]}]}}}} | 🧩️set-part           |
      | remove-part        | {"path": "docProps/core.xml"}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | 🧹️remove-part        |
      | patch-snapshot | {"patch": {"operation": "set", "path": "/xmlParts/0/document/attributes/1/value", "value": "Heading2"}} | 🩹️patch-snapshot |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://📜️example-readme.docx
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the jszip reader reads the restored package and the real README as the same DOCX
    Examples:
      | id                 | params                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
      | insert-block       | {"path": {"segments": [{"blockIndex": 359, "row": 1, "cell": 0}], "index": 1}, "block": {"kind": "paragraph", "style": "TableCell", "runs": [{"text": "(wave 7 annotation)", "bold": false, "italic": true, "underline": false}]}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
      | remove-block       | {"path": {"segments": [{"blockIndex": 359, "row": 2, "cell": 0}], "index": 0}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
      | set-block-content  | {"path": {"segments": [], "index": 4}, "block": {"kind": "paragraph", "style": "Normal", "runs": [{"text": "Wave 7 replaced this admonition paragraph outright.", "bold": false, "italic": true, "underline": false}]}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
      | set-run-text       | {"address": {"partPath": "word/document.xml", "nodePath": [0, 177, 1], "expectedName": "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}r", "revision": "1dc1281dba7711ea"}, "text": "Wave 7 mutation replaced this run's text entirely, still real."}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
      | set-run-formatting | {"address": {"partPath": "word/document.xml", "nodePath": [0, 177, 1], "expectedName": "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}r", "revision": "1dc1281dba7711ea"}, "bold": false, "italic": true, "underline": true}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
      | insert-style       | {"style": {"id": "Callout", "name": "Callout", "basedOn": "Normal"}}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
      | remove-style       | {"id": "TableCell"}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
      | set-style-name     | {"id": "Heading2", "name": "Section Heading"}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
      | set-style-based-on | {"id": "Heading3", "based_on": "Heading1"}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
      | set-part           | {"path":"docProps/app.xml","content_type":"application/vnd.openxmlformats-officedocument.extended-properties+xml","payload":{"kind":"xml","document":{"root":{"kind":"element","name":"Properties","attrs":[{"name":"xmlns","value":"http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"}],"children":[{"kind":"element","name":"Application","attrs":[],"children":[{"kind":"text","text":"semio-wave7-mutation-test"}]}]}}}} |
      | remove-part        | {"path": "docProps/core.xml"}                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
      | patch-snapshot | {"patch": {"operation": "set", "path": "/xmlParts/0/document/attributes/1/value", "value": "Heading2"}} |

  @id-mutate-set-snapshot
  @level-exhaustive
  @mode-differential
  Scenario: Replace the real document with the committed after-document's whole snapshot
    Given the real input document shared://📜️example-readme.docx
    And the committed after-document shared://🧾️readme-afters/📸️set-snapshot/➡️after.docx
    When the whole document is replaced by the snapshot decoded from the committed after-document
    Then the jszip reader reads the subject's package and the committed after-document as the same DOCX

  @id-inverse-set-snapshot
  @level-exhaustive
  @mode-differential
  Scenario: Undoing the whole-document replacement restores the document
    Given the real input document shared://📜️example-readme.docx
    And the committed after-document shared://🧾️readme-afters/📸️set-snapshot/➡️after.docx
    When the whole document is replaced by the committed after-document's snapshot and then undone
    Then the jszip reader reads the restored package and the real README as the same DOCX

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://📜️example-readme.docx
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the jszip reader reads the re-encoded package and the real README as the same DOCX
    And the re-encoded bytes are not bit-identical to the input
