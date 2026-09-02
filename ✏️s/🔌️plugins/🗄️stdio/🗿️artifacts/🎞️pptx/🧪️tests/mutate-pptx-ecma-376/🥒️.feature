@capability-pptx-ecma-376-mutate
@oracle-pptx-ecma-376-mutate
@comparison-semantic-pptx-mutate-v1
@mutations-pptx-ecma-376-any
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
  committed presentation is never written to. `MoveSlide` relocates slide 0 (the real title slide,
  "SemIO") to the end of the real 7-slide deck — genuine reordering of real content, not a
  synthetic stand-in. `InsertShape`/`RemoveShape`/`SetShapeText`/`SetShapePosition` address real
  `(slideIndex, shapeIndex)` pairs on real slides — slide 1's real 5-shape body (title/body/date/
  footer/slide-number placeholders) and slide 6's real picture (the real `Diagrammnotation` photo,
  `blipRelId` `rId2`) among them — never a synthetic two-shape stand-in.

  ALL THREE LAWS ARE ASSERTED IN ROLE, through the shared ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law module,
  so no scenario can pass merely because the reference composition declined to error.
  `mutate-<kind>` fails unless the mutation MOVES the very projection the case is compared through:
  a kind that applies cleanly and changes nothing observable would otherwise report a green for a
  mutation nobody watched, and until this wave all nine of them did exactly that. `inverse-<kind>`
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
  ../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs, which READS this table rather
  than restating it, so the two can never drift apart.

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
      | id                  | params                                                                                                             |
      | no-mutation          | {}                                                                                                                |
      | set-snapshot         | {"slides": [{"shapes": [{"kind": "textBox", "text": "Replacement Deck", "position": {"x": 0, "y": 0, "cx": 100, "cy": 100}}]}]} |
      | insert-slide         | {"index": 3, "slide": {"shapes": [{"kind": "textBox", "text": "Inserted Slide", "position": {"x": 457200, "y": 274638, "cx": 8229600, "cy": 1143000}}]}} |
      | remove-slide         | {"index": 2}                                                                                                      |
      | move-slide           | {"from": 0, "to": 6}                                                                                              |
      | insert-shape         | {"slideIndex": 0, "shapeIndex": 2, "shape": {"kind": "textBox", "text": "Added Shape", "position": {"x": 100, "y": 100, "cx": 500, "cy": 300}}} |
      | remove-shape         | {"slideIndex": 1, "shapeIndex": 2}                                                                                |
      | set-shape-text       | {"slideIndex": 0, "shapeIndex": 0, "text": "Changed Title"}                                                      |
      | set-shape-position   | {"slideIndex": 6, "shapeIndex": 1, "position": {"x": 1, "y": 2, "cx": 3, "cy": 4}}                                |

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
      | id                  | params                                                                                                             |
      | no-mutation          | {}                                                                                                                |
      | set-snapshot         | {"slides": [{"shapes": [{"kind": "textBox", "text": "Replacement Deck", "position": {"x": 0, "y": 0, "cx": 100, "cy": 100}}]}]} |
      | insert-slide         | {"index": 3, "slide": {"shapes": [{"kind": "textBox", "text": "Inserted Slide", "position": {"x": 457200, "y": 274638, "cx": 8229600, "cy": 1143000}}]}} |
      | remove-slide         | {"index": 2}                                                                                                      |
      | move-slide           | {"from": 0, "to": 6}                                                                                              |
      | insert-shape         | {"slideIndex": 0, "shapeIndex": 2, "shape": {"kind": "textBox", "text": "Added Shape", "position": {"x": 100, "y": 100, "cx": 500, "cy": 300}}} |
      | remove-shape         | {"slideIndex": 1, "shapeIndex": 2}                                                                                |
      | set-shape-text       | {"slideIndex": 0, "shapeIndex": 0, "text": "Changed Title"}                                                      |
      | set-shape-position   | {"slideIndex": 6, "shapeIndex": 1, "position": {"x": 1, "y": 2, "cx": 3, "cy": 4}}                                |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real presentation without passing bytes through
    Given the real input presentation shared://📽️.pptx
    When the presentation is decoded into the typed snapshot and re-encoded, with no mutation applied
    Then the re-encoded presentation is not a byte-for-byte copy of the input
    And its semantic projection matches the oracle's own decode-then-reencode of the same input
