@capability-svg-1-1-tiny-mutate
@oracle-quick-xml-svg-1-1-tiny-mutate
@comparison-semantic-svg-tiny-1-1-v1
@mutations-svg-1-1-tiny
Feature: Apply every typed SVG Tiny 1.1 mutation to a real-world Full 1.1 drawing
  The input is `shared://qr-code.svg`, the real committed QR-code logo drawing (provenance: copied
  verbatim from `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️qr-code.svg` into this artifact's own
  `🧫️fixtures/`), a genuine 76 KB Inkscape export: 664 nested `<g>` groups (329 of them carrying a
  real `transform="matrix(...)"`), 329 `<rect>` leaves, 3 `<path>` leaves with real path data, a real
  `viewBox="0 0 1015 1015"`, an `<?xml version="1.0" encoding="UTF-8" standalone="no"?>` declaration,
  five namespace-declaration attributes on the root, a real `<sodipodi:namedview>`, and one `<image>`
  whose `xlink:href` carries a ~74 KB embedded base64 data URI.

  It is deliberately NOT a Tiny document. It carries **335 real `style` attributes**, which SVG Tiny
  1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG Tiny 1.1) forbids on every element. That
  is the point: a profile subset's mutation vocabulary only means something when it is exercised
  against a real document that violates the profile, and the Full→Tiny down-conversion
  `strip-non-tiny` has 335 real attributes to remove rather than a synthetic one.

  This is `✳️tiny`'s own vocabulary, not `✳️any`'s. `✳️any` declares `insert-element`,
  `set-attribute` and `set-element-name`, each of which can put a document outside the profile in a
  single step; this subset declares `insert-tiny-element` and `set-tiny-attribute`, which reject an
  excluded element or a forbidden presentation attribute with a real diagnostic instead of writing
  it, plus two operations Full 1.1 has no use for at all — `stamp-base-profile`, the profile
  declaration itself, and `strip-non-tiny`, the down-conversion. `✳️any`'s `set-declaration` and
  `set-doctype` are absent: neither is a profile operation, and neither is what this subset exists
  to say anything about.

  Every scenario copies the fixture into the case work directory before touching it; the committed
  document is never written to. The oracle drives `quick-xml` 0.42 over an element tree built by the
  shared `📰markup` family module, applying each kind with its own transcription of
  REC-SVGMobile-20030114's excluded-element and excluded-attribute lists — never importing this
  repository's `check_svg_tiny_conformance` or its `xml_document_from_text`/`write_svg_xml` codec.
  Both the oracle's and the subject's results are read back by that SAME independent projection
  before the `semantic-svg-tiny-1-1-v1` profile compares them, never against each other's writing.

  Two honest notes on the algebra. `strip-non-tiny` inverts to a whole-document restore on both
  sides: a strip that removed 335 attributes across a real drawing has no smaller undo, and the
  subject's own `SvgTinyMutation::inverse` returns exactly that `SetSnapshot`. And `remove-element`
  is exercised at `[], 0` — the real `<defs id="defs663"/>` — because its inverse is a gated
  `insert-tiny-element`: in a profile-closed vocabulary you cannot undo the removal of a node the
  profile itself would refuse, which is a real property of the design and is stated here rather than
  hidden behind a target chosen to avoid it.

  Both non-differential laws are asserted IN ROLE, by the handler that plays the role, and are not
  deferred to the oracle-vs-subject comparison: every `inverse-<kind>` row requires apply-then-undo
  to restore that side's OWN reading of the original document's projection, and
  `identity-round-trip` requires that side's own decode → re-encode both to preserve its own
  projection and to move the bytes. SVG 1.1 ✳️tiny is XML and is not a byte-preserving carrier — a
  conforming writer re-derives every tag, quote and character reference from the tree — so the byte
  half of the law applies in full on both sides. A scenario that only proved the reference library
  did not error would be vacuous — it is checkable without a second producer, so it is checked
  without one.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real drawing
    Given the real input document shared://qr-code.svg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    And the semantic projection moved, unless the kind is no-mutation
    Examples:
      | id                  | params                                                                                                                                                                                        |
      | no-mutation         | {}                                                                                                                                                                                            |
      | set-snapshot        | {"rootId": "wave8-tiny-snapshot-marker", "viewBoxWidth": 2030}                                                                                                                                |
      | stamp-base-profile  | {"baseProfile": "tiny", "version": "1.1"}                                                                                                                                                     |
      | insert-tiny-element | {"parent": [4, 0, 0], "index": 1, "node": {"kind": "element", "name": "rect", "attrs": [{"name": "x", "value": "0"}, {"name": "y", "value": "0"}, {"name": "width", "value": "35"}, {"name": "height", "value": "35"}, {"name": "id", "value": "wave8-tiny-marker"}], "children": []}} |
      | remove-element      | {"parent": [], "index": 0}                                                                                                                                                                    |
      | set-tiny-attribute  | {"path": [4, 0, 0], "name": "fill", "value": "#123456"}                                                                                                                                       |
      | set-text            | {"path": [2], "text": "wave8 tiny mutation marker"}                                                                                                                                           |
      | set-view-box        | {"path": [], "viewBox": [0, 0, 2030, 2030]}                                                                                                                                                   |
      | set-transform       | {"path": [4, 0, 0], "transform": [{"kind": "translate", "x": 50, "y": 50}, {"kind": "rotate", "angle": 45}]}                                                                                  |
      | strip-non-tiny      | {}                                                                                                                                                                                            |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real drawing
    Given the real input document shared://qr-code.svg
    When the <id> mutation is applied and then undone with its own inverse
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                  | params                                                                                                                                                                                        |
      | no-mutation         | {}                                                                                                                                                                                            |
      | set-snapshot        | {"rootId": "wave8-tiny-snapshot-marker", "viewBoxWidth": 2030}                                                                                                                                |
      | stamp-base-profile  | {"baseProfile": "tiny", "version": "1.1"}                                                                                                                                                     |
      | insert-tiny-element | {"parent": [4, 0, 0], "index": 1, "node": {"kind": "element", "name": "rect", "attrs": [{"name": "x", "value": "0"}, {"name": "y", "value": "0"}, {"name": "width", "value": "35"}, {"name": "height", "value": "35"}, {"name": "id", "value": "wave8-tiny-marker"}], "children": []}} |
      | remove-element      | {"parent": [], "index": 0}                                                                                                                                                                    |
      | set-tiny-attribute  | {"path": [4, 0, 0], "name": "fill", "value": "#123456"}                                                                                                                                       |
      | set-text            | {"path": [2], "text": "wave8 tiny mutation marker"}                                                                                                                                           |
      | set-view-box        | {"path": [], "viewBox": [0, 0, 2030, 2030]}                                                                                                                                                   |
      | set-transform       | {"path": [4, 0, 0], "transform": [{"kind": "translate", "x": 50, "y": 50}, {"kind": "rotate", "angle": 45}]}                                                                                  |
      | strip-non-tiny      | {}                                                                                                                                                                                            |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real drawing without passing bytes through
    Given the real input document shared://qr-code.svg
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
