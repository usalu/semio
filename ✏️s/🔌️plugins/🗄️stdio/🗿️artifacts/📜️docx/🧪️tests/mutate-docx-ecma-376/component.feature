@capability-docx-ecma-376-mutate
@oracle-zip-quick-xml-docx-ecma-376-mutate
@comparison-semantic-docx-ecma-376-mutate-v1
@mutations-docx-ecma-376-any
Feature: Apply every typed DOCX ECMA-376 mutation to a real-world document
  The committed `example.docx` under this artifact's own demo example is a genuine OOXML package but
  only 1,648 bytes -- thin for exercising all 13 `DocxMutation` kinds. No larger real `.docx` exists
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
  `../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`'s own comparison-profile note.

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

  ALL THREE LAWS ARE ASSERTED IN ROLE, through the shared ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law module,
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
  ../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs, which READS this table rather
  than restating it, so the two can never drift apart — and the same module pins the
  remove-style-of-an-interior-style refusal described above.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://📜️example-readme.docx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                  | params                                                                                                                                                                                                                                                                             |
      | no-mutation         | {}                                                                                                                                                                                                                                                                                 |
      | set-snapshot        | {"body": [{"kind":"paragraph","style":"Heading1","runs":[{"text":"Wave 7 replacement document","bold":false,"italic":false,"underline":false}]},{"kind":"paragraph","style":"Normal","runs":[{"text":"This whole document was replaced by a ","bold":false,"italic":false,"underline":false},{"text":"set-snapshot","bold":true,"italic":false,"underline":false},{"text":" mutation.","bold":false,"italic":false,"underline":false}]},{"kind":"table","rows":[{"cells":[{"blocks":[{"kind":"paragraph","style":"TableCell","runs":[{"text":"Left","bold":false,"italic":false,"underline":false}]}]},{"blocks":[{"kind":"paragraph","style":"TableCell","runs":[{"text":"Right","bold":false,"italic":false,"underline":false}]}]}]}]}], "styles": [{"id":"Normal","name":"Normal","basedOn":null},{"id":"Heading1","name":"heading 1","basedOn":"Normal"},{"id":"TableCell","name":"Table Cell","basedOn":"Normal"}]} |
      | insert-block        | {"path": {"segments": [{"blockIndex": 359, "row": 1, "cell": 0}], "index": 1}, "block": {"kind":"paragraph","style":"TableCell","runs":[{"text":"(wave 7 annotation)","bold":false,"italic":true,"underline":false}]}}                                                          |
      | remove-block        | {"path": {"segments": [{"blockIndex": 359, "row": 2, "cell": 0}], "index": 0}}                                                                                                                                                                                                    |
      | set-block-content   | {"path": {"segments": [], "index": 4}, "block": {"kind":"paragraph","style":"Normal","runs":[{"text":"Wave 7 replaced this admonition paragraph outright.","bold":false,"italic":true,"underline":false}]}}                                                                     |
      | set-run-text        | {"path": {"segments": [], "index": 177}, "runIndex": 0, "text": "Wave 7 mutation replaced this run's text entirely, still real."}                                                                                                                                                |
      | set-run-formatting  | {"path": {"segments": [], "index": 177}, "runIndex": 0, "bold": false, "italic": true, "underline": true}                                                                                                                                                                        |
      | insert-style        | {"style": {"id": "Callout", "name": "Callout", "basedOn": "Normal"}}                                                                                                                                                                                                              |
      | remove-style        | {"id": "TableCell"}                                                                                                                                                                                                                                                                    |
      | set-style-name      | {"id": "Heading2", "name": "Section Heading"}                                                                                                                                                                                                                                     |
      | set-style-based-on  | {"id": "Heading3", "basedOn": "Heading1"}                                                                                                                                                                                                                                         |
      | set-part            | {"path": "docProps/app.xml", "contentType": "application/vnd.openxmlformats-officedocument.extended-properties+xml", "content": "<Properties xmlns=\"http://schemas.openxmlformats.org/officeDocument/2006/extended-properties\"><Application>semio-wave7-mutation-test</Application></Properties>"} |
      | remove-part         | {"path": "docProps/core.xml"}                                                                                                                                                                                                                                                     |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://📜️example-readme.docx
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                  | params                                                                                                                                                                                                                                                                             |
      | no-mutation         | {}                                                                                                                                                                                                                                                                                 |
      | set-snapshot        | {"body": [{"kind":"paragraph","style":"Heading1","runs":[{"text":"Wave 7 replacement document","bold":false,"italic":false,"underline":false}]},{"kind":"paragraph","style":"Normal","runs":[{"text":"This whole document was replaced by a ","bold":false,"italic":false,"underline":false},{"text":"set-snapshot","bold":true,"italic":false,"underline":false},{"text":" mutation.","bold":false,"italic":false,"underline":false}]},{"kind":"table","rows":[{"cells":[{"blocks":[{"kind":"paragraph","style":"TableCell","runs":[{"text":"Left","bold":false,"italic":false,"underline":false}]}]},{"blocks":[{"kind":"paragraph","style":"TableCell","runs":[{"text":"Right","bold":false,"italic":false,"underline":false}]}]}]}]}], "styles": [{"id":"Normal","name":"Normal","basedOn":null},{"id":"Heading1","name":"heading 1","basedOn":"Normal"},{"id":"TableCell","name":"Table Cell","basedOn":"Normal"}]} |
      | insert-block        | {"path": {"segments": [{"blockIndex": 359, "row": 1, "cell": 0}], "index": 1}, "block": {"kind":"paragraph","style":"TableCell","runs":[{"text":"(wave 7 annotation)","bold":false,"italic":true,"underline":false}]}}                                                          |
      | remove-block        | {"path": {"segments": [{"blockIndex": 359, "row": 2, "cell": 0}], "index": 0}}                                                                                                                                                                                                    |
      | set-block-content   | {"path": {"segments": [], "index": 4}, "block": {"kind":"paragraph","style":"Normal","runs":[{"text":"Wave 7 replaced this admonition paragraph outright.","bold":false,"italic":true,"underline":false}]}}                                                                     |
      | set-run-text        | {"path": {"segments": [], "index": 177}, "runIndex": 0, "text": "Wave 7 mutation replaced this run's text entirely, still real."}                                                                                                                                                |
      | set-run-formatting  | {"path": {"segments": [], "index": 177}, "runIndex": 0, "bold": false, "italic": true, "underline": true}                                                                                                                                                                        |
      | insert-style        | {"style": {"id": "Callout", "name": "Callout", "basedOn": "Normal"}}                                                                                                                                                                                                              |
      | remove-style        | {"id": "TableCell"}                                                                                                                                                                                                                                                                    |
      | set-style-name      | {"id": "Heading2", "name": "Section Heading"}                                                                                                                                                                                                                                     |
      | set-style-based-on  | {"id": "Heading3", "basedOn": "Heading1"}                                                                                                                                                                                                                                         |
      | set-part            | {"path": "docProps/app.xml", "contentType": "application/vnd.openxmlformats-officedocument.extended-properties+xml", "content": "<Properties xmlns=\"http://schemas.openxmlformats.org/officeDocument/2006/extended-properties\"><Application>semio-wave7-mutation-test</Application></Properties>"} |
      | remove-part         | {"path": "docProps/core.xml"}                                                                                                                                                                                                                                                     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://📜️example-readme.docx
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
