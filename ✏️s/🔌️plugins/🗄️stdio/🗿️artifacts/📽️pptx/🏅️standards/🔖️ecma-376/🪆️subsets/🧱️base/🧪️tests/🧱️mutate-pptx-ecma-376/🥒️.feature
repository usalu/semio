@capability-pptx-ecma-376-mutate
@oracle-pptx-ecma-376-mutate
@comparison-semantic-pptx-mutate-v1
@mutations-pptx-ecma-376-base
Feature: Apply every typed PPTX ECMA-376 mutation to a real-world presentation
  The input is `shared://📽️.pptx`, a real 7-slide, ~110 KB subset derived ONCE (not a
  test step) from a real 62-slide, 16 MB 2020 conference deck ("Eine domänenspezifische
  Programmiersprache für Architekten", presented 27.11.2020) that this ticket nominated. The
  committed `📚️examples/🎬️demo/🖼️assets/🎞️example.pptx` for this artifact is 0 bytes — a placeholder,
  not a fixture — so it could not serve as the real input; this derived subset is the real one.

  The derivation kept the first 6 real slides in presentation order plus real slide 23
  ("Diagrammnotation", the first slide carrying a real embedded picture) and closed the OPC
  relationship graph around them: every `slideLayout` the one real `slideMaster` declares (all 11,
  since trimming any would leave a dangling relationship), both real themes, the real notes master,
  `presProps`/`viewProps`/`tableStyles`, and only the 3 real media images the kept parts actually
  reference (`image1.png`/`image2.png`, the master's own backgrounds; `image3.png`, slide 23's real
  photo). `docProps/app.xml`'s descriptive slide count/title vector was updated to match the 7 kept
  real slide titles rather than left stale at 62; the PowerPoint-only `p:extLst` ("sections"/slide
  guides), which referenced numeric slide ids this derivation drops, was removed rather than left
  dangling. Every other real byte — every kept slide's real German/English text, every real
  `a:xfrm`, the real embedded photo — is untouched. The derivation script and full provenance are
  recorded in this ticket's own folder.

  THE FIXTURE WAS NOT A CONFORMANT OPC PACKAGE UNTIL WAVE 14, AND THE FIRST SUBJECT RUN IS WHAT
  FOUND IT. The derivation says "keep every Default" and read them with `re.findall(r"<Default
  [^/]+/>", ct_xml)` — a character class that cannot span the `/` in `application/vnd.openxml...`,
  so it matched NOTHING and the committed `[Content_Types].xml` shipped with 28 Overrides and zero
  Defaults. Every `.rels`, `.png` and `.jpeg` part in the package was therefore left with no
  resolvable content type, which ECMA-376 Part 2 §10.1.2.2.1 forbids outright, and this subset's
  own `decode_pptx` rightly refused the file with `part docProps/thumbnail.jpeg has no resolvable
  content type` — all 19 subject scenarios red, while the oracle composition read
  the same broken package without complaint. The eight real `<Default>` elements were spliced back
  in from the real source deck and NOTHING else changed: the repair rewrites only the
  `[Content_Types].xml` entry, and every other part keeps its exact bytes, order and zip timestamp
  (verified part-by-part). The regex is fixed in the derivation script too, so re-deriving now
  produces the repaired package rather than the broken one. `temp/` is gitignored; `git check-ignore -v` on the derived
  copy under this artifact's `🧫️fixtures/` confirms it is tracked (the `!**/🧫️fixtures/**` rule
  re-includes it).

  Every scenario copies the immutable fixture into the case work directory before touching it; the
  committed presentation is never written to. Every slide and shape kind addresses its target through a
  revision-bound canonical XML address — the part path, the child-index path from the part root, the
  element's expanded name and the 64-bit FNV-1a revision of its subtree (`E name`, `A name=value` per
  attribute, the children, `/E`; `T text` per text node) — so a row authored against another document
  state is refused rather than applied to whatever now sits there. `move-slide` relocates the real title
  slide ("SemIO", `p:sldId` 256) to the end of the deck; `remove-slide` drops the third slide's list entry;
  `insert-slide` lists the title slide a second time, under the new id 9999, at position 3;
  `insert-shape` adds a text box to the title slide's shape tree; `remove-shape`, `set-shape-text` and
  `set-shape-position` address real shapes by their `p:cNvPr` ids (slide 2's third placeholder, the title
  slide's title, slide 23's real `Diagrammnotation` photo). The rows are derived from the committed deck by
  this ticket's `🧪️s4-stdio-pptx-rows.py`, never hand-copied.

  ALL THREE LAWS ARE ASSERTED IN ROLE, through the shared ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law module,
  so no scenario can pass merely because the reference composition declined to error.
  `mutate-<kind>` fails unless the mutation MOVES the very projection the case is compared through:
  a kind that applies cleanly and changes nothing observable would otherwise report a green for a
  mutation nobody watched, and until wave 14 every one of them did exactly that. `inverse-<kind>`
  applies the mutation, applies its own independently computed inverse, and fails with the first
  diverging field unless the result projects onto exactly what the original presentation projects
  onto. `identity-round-trip` fails unless the rebuilt archive differs from the input AND its
  projection is identical to the input's. NONE of the three is scoped down and NO kind is exempt
  from any of them: `semantic-pptx-mutate-v1` declares no writer freedom, and the whole projection —
  the ordered slide list and every slide's ordered shape list with each shape's kind, text and
  position — has to move for a mutation and come back for an inverse. Slide ORDER is part of that,
  which is what gives `move-slide` real evidence rather than a shape census that a reorder leaves
  untouched. The same three laws are proven again at unit level over these very Examples rows by
  `every_declared_kind_is_observable_and_its_inverse_restores_the_presentation` in
  ../🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🦀️component.rs, which READS this table rather
  than restating it, so the two can never drift apart.

  Every `params` cell is the leaf's own wire payload, exactly what `PptxMutation::payload_value()`
  emits: addresses as above, inserted entries and shapes as `XmlNode` trees, and positions as EMU strings.
  Both implementations read that one wire: the reference by field name over its own zip and XML
  trees, the subject through `Mutation::from_payload_value`, whose re-emitted payload must equal the row
  exactly; the subject undoes every kind with `Mutation::inverse` itself.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real presentation
    Given the real input presentation shared://📽️.pptx
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection

    Examples:
      | id                 | params |
      | insert-slide | {"vacancy":{"container":{"partPath":"ppt/presentation.xml","nodePath":[2],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sldIdLst","revision":"70e812bbdab5f338"},"index":3},"entry":{"kind":"element","name":"p:sldId","attrs":[{"name":"id","value":"9999"},{"name":"r:id","value":"rId2"}],"children":[]}} |
      | remove-slide | {"address":{"entry":{"partPath":"ppt/presentation.xml","nodePath":[2,2],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sldId","revision":"432595ffaed1749b"},"slidePartPath":"ppt/slides/slide3.xml","relationshipId":"rId4","slideId":"257"}} |
      | move-slide | {"address":{"entry":{"partPath":"ppt/presentation.xml","nodePath":[2,0],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sldId","revision":"6a72974b4f753332"},"slidePartPath":"ppt/slides/slide1.xml","relationshipId":"rId2","slideId":"256"},"destinationIndex":6} |
      | insert-shape | {"vacancy":{"container":{"partPath":"ppt/slides/slide1.xml","nodePath":[0,0],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"spTree","revision":"e7868fb394a0437b"},"index":2},"shape":{"kind":"element","name":"p:sp","attrs":[],"children":[{"kind":"element","name":"p:nvSpPr","attrs":[],"children":[{"kind":"element","name":"p:cNvPr","attrs":[{"name":"id","value":"999"},{"name":"name","value":"Added Shape"}],"children":[]},{"kind":"element","name":"p:cNvSpPr","attrs":[{"name":"txBox","value":"1"}],"children":[]},{"kind":"element","name":"p:nvPr","attrs":[],"children":[]}]},{"kind":"element","name":"p:spPr","attrs":[],"children":[{"kind":"element","name":"a:xfrm","attrs":[],"children":[{"kind":"element","name":"a:off","attrs":[{"name":"x","value":"100"},{"name":"y","value":"100"}],"children":[]},{"kind":"element","name":"a:ext","attrs":[{"name":"cx","value":"500"},{"name":"cy","value":"300"}],"children":[]}]}]},{"kind":"element","name":"p:txBody","attrs":[],"children":[{"kind":"element","name":"a:bodyPr","attrs":[],"children":[]},{"kind":"element","name":"a:p","attrs":[],"children":[{"kind":"element","name":"a:r","attrs":[],"children":[{"kind":"element","name":"a:t","attrs":[],"children":[{"kind":"text","text":"Added Shape"}]}]}]}]}]}} |
      | remove-shape | {"address":{"node":{"partPath":"ppt/slides/slide2.xml","nodePath":[0,0,4],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sp","revision":"db2b41ceacd4abf3"},"shapeId":"4"}} |
      | set-shape-text | {"address":{"node":{"partPath":"ppt/slides/slide1.xml","nodePath":[0,0,2],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sp","revision":"0e7d7c1d78ae71c1"},"shapeId":"2"},"text":"Changed Title"} |
      | set-shape-position | {"address":{"node":{"partPath":"ppt/slides/slide23.xml","nodePath":[0,0,3],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"pic","revision":"9142f26053af89ea"},"shapeId":"7"},"position":{"x":"1","y":"2","cx":"3","cy":"4"}} |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the presentation
    Given the real input presentation shared://📽️.pptx
    When the <id> mutation is applied and then undone with its own inverse
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the restored presentation's semantic projection matches what the original presentation's does

    Examples:
      | id                 | params |
      | insert-slide | {"vacancy":{"container":{"partPath":"ppt/presentation.xml","nodePath":[2],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sldIdLst","revision":"70e812bbdab5f338"},"index":3},"entry":{"kind":"element","name":"p:sldId","attrs":[{"name":"id","value":"9999"},{"name":"r:id","value":"rId2"}],"children":[]}} |
      | remove-slide | {"address":{"entry":{"partPath":"ppt/presentation.xml","nodePath":[2,2],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sldId","revision":"432595ffaed1749b"},"slidePartPath":"ppt/slides/slide3.xml","relationshipId":"rId4","slideId":"257"}} |
      | move-slide | {"address":{"entry":{"partPath":"ppt/presentation.xml","nodePath":[2,0],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sldId","revision":"6a72974b4f753332"},"slidePartPath":"ppt/slides/slide1.xml","relationshipId":"rId2","slideId":"256"},"destinationIndex":6} |
      | insert-shape | {"vacancy":{"container":{"partPath":"ppt/slides/slide1.xml","nodePath":[0,0],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"spTree","revision":"e7868fb394a0437b"},"index":2},"shape":{"kind":"element","name":"p:sp","attrs":[],"children":[{"kind":"element","name":"p:nvSpPr","attrs":[],"children":[{"kind":"element","name":"p:cNvPr","attrs":[{"name":"id","value":"999"},{"name":"name","value":"Added Shape"}],"children":[]},{"kind":"element","name":"p:cNvSpPr","attrs":[{"name":"txBox","value":"1"}],"children":[]},{"kind":"element","name":"p:nvPr","attrs":[],"children":[]}]},{"kind":"element","name":"p:spPr","attrs":[],"children":[{"kind":"element","name":"a:xfrm","attrs":[],"children":[{"kind":"element","name":"a:off","attrs":[{"name":"x","value":"100"},{"name":"y","value":"100"}],"children":[]},{"kind":"element","name":"a:ext","attrs":[{"name":"cx","value":"500"},{"name":"cy","value":"300"}],"children":[]}]}]},{"kind":"element","name":"p:txBody","attrs":[],"children":[{"kind":"element","name":"a:bodyPr","attrs":[],"children":[]},{"kind":"element","name":"a:p","attrs":[],"children":[{"kind":"element","name":"a:r","attrs":[],"children":[{"kind":"element","name":"a:t","attrs":[],"children":[{"kind":"text","text":"Added Shape"}]}]}]}]}]}} |
      | remove-shape | {"address":{"node":{"partPath":"ppt/slides/slide2.xml","nodePath":[0,0,4],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sp","revision":"db2b41ceacd4abf3"},"shapeId":"4"}} |
      | set-shape-text | {"address":{"node":{"partPath":"ppt/slides/slide1.xml","nodePath":[0,0,2],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"sp","revision":"0e7d7c1d78ae71c1"},"shapeId":"2"},"text":"Changed Title"} |
      | set-shape-position | {"address":{"node":{"partPath":"ppt/slides/slide23.xml","nodePath":[0,0,3],"namespaceUri":"http://schemas.openxmlformats.org/presentationml/2006/main","localName":"pic","revision":"9142f26053af89ea"},"shapeId":"7"},"position":{"x":"1","y":"2","cx":"3","cy":"4"}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real presentation without passing bytes through
    Given the real input presentation shared://📽️.pptx
    When the presentation is decoded into the typed snapshot and re-encoded, with no mutation applied
    Then the re-encoded presentation is not a byte-for-byte copy of the input
    And its semantic projection matches the oracle's own decode-then-reencode of the same input
