@capability-svg-1-1-basic-mutate
@oracle-quick-xml-svg-1-1-basic-mutate
@comparison-semantic-svg-basic-1-1-v1
@mutations-svg-1-1-basic
Feature: Apply every typed SVG Basic 1.1 mutation to a real-world clipped drawing
  The input is `shared://mouse.svg`, the real committed introduction-demonstration mouse graphic
  (provenance: copied verbatim from `🧰️framework/🔨️modules/🖼️assets/👋️introduction/🔣️mouse.svg` into
  this artifact's own `🧫️fixtures/`), which the framework's own onboarding UI renders today. It is
  small — 1,463 bytes — and it is chosen for what it CONTAINS rather than for its size: a real
  `<clipPath id="introduction-demo-mouse-clip">` holding a real `<path>`, a real
  `clip-path="url(#introduction-demo-mouse-clip)"` reference on a real `<g>`, real `fill-opacity`
  and `stroke-opacity` attributes, a real XML comment, a real `viewBox="0 0 48 72"`, and eight
  `<path>` elements with real path data. It is the only real, committed SVG in this repository that
  declares a clip path at all, and SVG Basic 1.1's clip-path rule is half of what distinguishes the
  profile — a synthetic fixture would have made both clip-path mutations vacuous.

  Unlike the sibling `✳️tiny` case's input, this document already conforms: SVG Basic 1.1 (W3C Mobile
  SVG Profiles, REC-SVGMobile-20030114 §SVG Basic 1.1) RETAINS opacity, masks, gradients, the
  `clipPath` element and the filter mechanism. What it excludes is narrow — nine expensive raster
  filter primitives, and clipping to text — and those two exclusions are what this vocabulary is
  built around. `insert-basic-element` inserts a real `<filter>` carrying `feGaussianBlur`, which
  Basic retains, and refuses the same insert with `feTurbulence`, which it does not.
  `set-clip-path-reference` points the mouse body at the document's real clip path, and
  `insert-clip-path-shape` adds a real shape to that same clip path; both refuse anything that would
  clip to text. None of the three exists in `✳️tiny`'s vocabulary, whose profile has neither filters
  nor `clipPath`; and `✳️any`'s ungated `set-attribute`/`insert-element` can leave the profile in one
  step, which is why they are absent here.

  The two profiles' vocabularies do share a shape — a gated insert, a gated attribute set, the
  profile stamp, and the geometry/text kinds — and that is a real fact about them, not a copy: Basic
  and Tiny are two restrictions of ONE schema, so the operations differ in what each GATE rejects and
  in the profile-specific kinds each adds, not in how a document is addressed. Where they genuinely
  differ they differ: Tiny declares `strip-non-tiny` and has no clip-path kinds; Basic declares the
  two clip-path kinds and has no strip, because this repository holds no real SVG carrying an
  excluded raster primitive for such a strip to remove — a vacuous scenario would have been worse
  than an absent one.

  Every scenario copies the fixture into the case work directory before touching it; the committed
  asset is never written to. The oracle drives `quick-xml` 0.42 over an element tree built by the
  shared `📰markup` family module, resolving `url(#id)` references and applying the profile rules from
  its own transcription of REC-SVGMobile-20030114 — never importing this repository's
  `check_svg_basic_conformance` or its SVG codec. Both sides' results are read back by that SAME
  independent projection before the `semantic-svg-basic-1-1-v1` profile compares them.

  Both non-differential laws are asserted IN ROLE, by the handler that plays the role, and are not
  deferred to the oracle-vs-subject comparison: every `inverse-<kind>` row requires apply-then-undo
  to restore that side's OWN reading of the original document's projection, and
  `identity-round-trip` requires that side's own decode → re-encode both to preserve its own
  projection and to move the bytes. SVG 1.1 ✳️basic is XML and is not a byte-preserving carrier — a
  conforming writer re-derives every tag, quote and character reference from the tree — so the byte
  half of the law applies in full on both sides. A scenario that only proved the reference library
  did not error would be vacuous — it is checkable without a second producer, so it is checked
  without one.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real drawing
    Given the real input document shared://mouse.svg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                      | params                                                                                                                                                                                    |
      | no-mutation             | {}                                                                                                                                                                                        |
      | set-snapshot            | {"rootId": "wave8-basic-snapshot-marker", "viewBoxWidth": 96}                                                                                                                             |
      | stamp-base-profile      | {"baseProfile": "basic", "version": "1.1"}                                                                                                                                                |
      | insert-basic-element    | {"parent": [3], "index": 1, "node": {"kind": "element", "name": "filter", "attrs": [{"name": "id", "value": "wave8-basic-blur"}], "children": [{"kind": "element", "name": "feGaussianBlur", "attrs": [{"name": "stdDeviation", "value": "2"}], "children": []}]}} |
      | remove-element          | {"parent": [5], "index": 5}                                                                                                                                                               |
      | set-basic-attribute     | {"path": [7], "name": "stroke-width", "value": "3.5"}                                                                                                                                     |
      | set-clip-path-reference | {"path": [7], "clipPathId": "introduction-demo-mouse-clip"}                                                                                                                               |
      | insert-clip-path-shape  | {"clipPathId": "introduction-demo-mouse-clip", "index": 1, "node": {"kind": "element", "name": "circle", "attrs": [{"name": "cx", "value": "24"}, {"name": "cy", "value": "36"}, {"name": "r", "value": "20"}], "children": []}} |
      | set-text                | {"path": [0], "text": "wave8 basic mutation marker"}                                                                                                                                      |
      | set-view-box            | {"path": [], "viewBox": [0, 0, 96, 144]}                                                                                                                                                  |
      | set-transform           | {"path": [5], "transform": [{"kind": "translate", "x": 4, "y": 4}, {"kind": "scale", "x": 2}]}                                                                                            |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real drawing
    Given the real input document shared://mouse.svg
    When the <id> mutation is applied and then undone with its own inverse
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                      | params                                                                                                                                                                                    |
      | no-mutation             | {}                                                                                                                                                                                        |
      | set-snapshot            | {"rootId": "wave8-basic-snapshot-marker", "viewBoxWidth": 96}                                                                                                                             |
      | stamp-base-profile      | {"baseProfile": "basic", "version": "1.1"}                                                                                                                                                |
      | insert-basic-element    | {"parent": [3], "index": 1, "node": {"kind": "element", "name": "filter", "attrs": [{"name": "id", "value": "wave8-basic-blur"}], "children": [{"kind": "element", "name": "feGaussianBlur", "attrs": [{"name": "stdDeviation", "value": "2"}], "children": []}]}} |
      | remove-element          | {"parent": [5], "index": 5}                                                                                                                                                               |
      | set-basic-attribute     | {"path": [7], "name": "stroke-width", "value": "3.5"}                                                                                                                                     |
      | set-clip-path-reference | {"path": [7], "clipPathId": "introduction-demo-mouse-clip"}                                                                                                                               |
      | insert-clip-path-shape  | {"clipPathId": "introduction-demo-mouse-clip", "index": 1, "node": {"kind": "element", "name": "circle", "attrs": [{"name": "cx", "value": "24"}, {"name": "cy", "value": "36"}, {"name": "r", "value": "20"}], "children": []}} |
      | set-text                | {"path": [0], "text": "wave8 basic mutation marker"}                                                                                                                                      |
      | set-view-box            | {"path": [], "viewBox": [0, 0, 96, 144]}                                                                                                                                                  |
      | set-transform           | {"path": [5], "transform": [{"kind": "translate", "x": 4, "y": 4}, {"kind": "scale", "x": 2}]}                                                                                            |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real drawing without passing bytes through
    Given the real input document shared://mouse.svg
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
