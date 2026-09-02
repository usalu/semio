@capability-svg-1-1-mutate
@oracle-quick-xml-svg-1-1-mutate-reader
@comparison-semantic-svg-1-1-v1
@mutations-svg-1-1-base
Feature: Apply every typed SVG 1.1 mutation to a real-world document
  The input is a real, committed QR-code logo drawing (`🔣️qr-code.svg`, provenance: copied verbatim
  from `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️qr-code.svg` into this artifact's own
  `🧫️fixtures/`), not a synthetic fixture: 664 nested `<g>` groups (many carrying a real
  `transform="matrix(...)"`), 329 `<rect>` leaves, 3 `<path>` leaves with real path data, a real
  `viewBox="0 0 1015 1015"`, an `<?xml version="1.0" encoding="UTF-8" standalone="no"?>` declaration,
  five namespace-declaration attributes on the root (`xmlns`, `xmlns:svg`, `xmlns:xlink`,
  `xmlns:inkscape`, `xmlns:sodipodi`), and one `<image>` element whose real `xlink:href` carries a
  ~74 KB embedded base64 data URI folded across many lines with literal `&#10;` character
  references. Every scenario copies it into the case work directory before touching it; the
  committed document is never written to. The oracle drives `quick-xml` 0.42 (registered here for
  the first time under the `svg-1-1-mutate` capability — the shared stdio manifest never reaches it
  for SVG) over this subset's own 11-kind `SvgMutation` vocabulary, reimplemented independently in
  `../../🏅️standards/🔖️1.1/🪆️subsets/✳️base/🧪️oracle/🦀️component.rs` against a small hand-rolled
  element tree, never importing this subset's own `xml_document_from_text`/`write_svg_xml` codec.
  Both the oracle's and the subject's results are read back by the SAME independent `quick-xml`
  projection before comparison, never against each other's own writing.

  `set-view-box` and `set-transform` are the SVG-specific geometry mutations this wave calls out:
  both target real, already-populated geometry on the real drawing (the root's own
  `viewBox="0 0 1015 1015"`, and `<g id="g3">`'s real `transform="matrix(0.35,0,0,0.35,280,0)"` three
  levels inside the QR module tree), and the projection decomposes both into typed numeric fields
  rather than comparing raw attribute-string formatting — real writer freedom between two
  independently-written formatters, narrowed out here rather than chased byte-for-byte. `set-doctype`
  sets the document's real SVG 1.1 public DOCTYPE (`-//W3C//DTD SVG 1.1//EN`), which the source file
  does not carry. `set-attribute` targets the real `<image>`'s `xlink:href`.

  FINDING (not narrowed, not worked around): the production `xml` codec's `xml_escape_attr`
  (`../../../📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️component.rs`, a shared
  module this subset does not own) escapes only `&`, `<` and `"` when re-emitting an attribute
  value — it does not re-escape a literal tab/newline/CR (produced when it decodes a `&#10;`-style
  numeric character reference such as the real `xlink:href` above carries) back into a character
  reference. `quick-xml`'s own attribute writer does escape all three, symmetrically with its own
  reader's attribute-value normalization. So once the SUBJECT phase compiles, any scenario that
  leaves this specific attribute untouched (`identity-round-trip` foremost) is expected to show the
  subject's re-serialized value differing from the oracle's under exact-string comparison: the
  subject's raw embedded newlines collapse to spaces on the NEXT parse (XML attribute-value
  normalization applies to literal whitespace bytes, never to character-reference-produced ones),
  where the oracle's re-escaped `&#10;` sequences do not. This is real loss in a shared module out of
  this subset's ownership, reported here rather than hidden by narrowing the projection.

  Both non-differential laws are asserted IN ROLE, by the handler that plays the role, and are not
  deferred to the oracle-vs-subject comparison: every `inverse-<kind>` row requires apply-then-undo
  to restore that side's OWN reading of the original document's projection, and
  `identity-round-trip` requires that side's own decode → re-encode both to preserve its own
  projection and to move the bytes. SVG 1.1 is XML and is not a byte-preserving carrier — a
  conforming writer re-derives every tag, quote and character reference from the tree — so the byte
  half of the law applies in full on both sides. A scenario that only proved the reference library
  did not error would be vacuous — it is checkable without a second producer, so it is checked
  without one.
  A note on `inverse-remove-element`, which WAS red and is not any more — kept here because the
  defect is instructive and the remedy is the one this feature named rather than a relaxed law. The
  reference module's `oracles::apply_mutation_inverse` used to re-serialize between the forward step
  and the undo step and re-parse those bytes, so the two steps did not see the same tree: in the real
  drawing every `<g …>\n<rect …/>\n</g>` group holds `[text "\n", rect, text "\n"]`, removing index
  1 leaves two ADJACENT text nodes, and XML parsing coalesces them into the single node `"\n\n"` on
  the way back in — so the undo inserted the rect at index 1 of a one-child list and the restored
  drawing projected as `[text "\n\n", rect]`. It was fixed in the ORACLE MODULE, by applying the
  forward step and its inverse to ONE parsed tree, exactly as prescribed; the law here was never
  relaxed. `mutate-xml-1-0` shared the routing and never showed it, because its minified fixture
  carries no inter-element whitespace at all.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document shared://qr-code.svg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    And the semantic projection moved, unless the kind is no-mutation
    Examples:
      | id                | params                                                                                                                                                      |
      | set-declaration    | {"version": "1.1", "encoding": "UTF-8", "standalone": true}                                                                                                |
      | set-doctype        | {"doctype": "svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\""}                                                |
      | insert-element     | {"parent": [4, 0, 0, 0], "index": 1, "node": {"kind": "element", "name": "circle", "attrs": [{"name": "cx", "value": "50"}, {"name": "cy", "value": "50"}, {"name": "r", "value": "10"}, {"name": "id", "value": "wave7-marker-circle"}], "children": []}} |
      | remove-element     | {"parent": [4, 0, 0, 0], "index": 1}                                                                                                                       |
      | set-element-name   | {"path": [4, 0, 0, 0], "name": "g-wave7"}                                                                                                                  |
      | set-attribute      | {"path": [3, 0], "name": "xlink:href", "value": "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciLz4="}                     |
      | set-text           | {"path": [2], "text": "wave7 mutation marker"}                                                                                                             |
      | set-view-box       | {"path": [], "viewBox": [0, 0, 2030, 2030]}                                                                                                                |
      | set-transform      | {"path": [4, 0, 0], "transform": [{"kind": "translate", "x": 50, "y": 50}, {"kind": "rotate", "angle": 45}]}                                              |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document shared://qr-code.svg
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                | params                                                                                                                                                      |
      | set-declaration    | {"version": "1.1", "encoding": "UTF-8", "standalone": true}                                                                                                |
      | set-doctype        | {"doctype": "svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\""}                                                |
      | insert-element     | {"parent": [4, 0, 0, 0], "index": 1, "node": {"kind": "element", "name": "circle", "attrs": [{"name": "cx", "value": "50"}, {"name": "cy", "value": "50"}, {"name": "r", "value": "10"}, {"name": "id", "value": "wave7-marker-circle"}], "children": []}} |
      | remove-element     | {"parent": [4, 0, 0, 0], "index": 1}                                                                                                                       |
      | set-element-name   | {"path": [4, 0, 0, 0], "name": "g-wave7"}                                                                                                                  |
      | set-attribute      | {"path": [3, 0], "name": "xlink:href", "value": "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciLz4="}                     |
      | set-text           | {"path": [2], "text": "wave7 mutation marker"}                                                                                                             |
      | set-view-box       | {"path": [], "viewBox": [0, 0, 2030, 2030]}                                                                                                                |
      | set-transform      | {"path": [4, 0, 0], "transform": [{"kind": "translate", "x": 50, "y": 50}, {"kind": "rotate", "angle": 45}]}                                              |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document shared://qr-code.svg
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
