@capability-pptx-ecma-376-mutate
@oracle-pptx-ecma-376-mutate
@comparison-semantic-pptx-mutate-v1
@mutations-pptx-ecma-376-any
Feature: Apply every typed PPTX ECMA-376 mutation to a real-world presentation
  The input is `shared://🎞️semio-talk.pptx`, a real 7-slide, ~110 KB subset derived ONCE (not a
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
  recorded in this ticket's own folder. `temp/` is gitignored; `git check-ignore -v` on the derived
  copy under this artifact's `🧫️fixtures/` confirms it is tracked (the `!**/🧫️fixtures/**` rule
  re-includes it).

  Every scenario copies the immutable fixture into the case work directory before touching it; the
  committed presentation is never written to. `MoveSlide` relocates slide 0 (the real title slide,
  "SemIO") to the end of the real 7-slide deck — genuine reordering of real content, not a
  synthetic stand-in. `InsertShape`/`RemoveShape`/`SetShapeText`/`SetShapePosition` address real
  `(slideIndex, shapeIndex)` pairs on real slides — slide 1's real 5-shape body (title/body/date/
  footer/slide-number placeholders) and slide 6's real picture (the real `Diagrammnotation` photo,
  `blipRelId` `rId2`) among them — never a synthetic two-shape stand-in.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real presentation
    Given the real input presentation shared://🎞️semio-talk.pptx
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
    Given the real input presentation shared://🎞️semio-talk.pptx
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
    Given the real input presentation shared://🎞️semio-talk.pptx
    When the presentation is decoded into the typed snapshot and re-encoded, with no mutation applied
    Then the re-encoded presentation is not a byte-for-byte copy of the input
    And its semantic projection matches the oracle's own decode-then-reencode of the same input
