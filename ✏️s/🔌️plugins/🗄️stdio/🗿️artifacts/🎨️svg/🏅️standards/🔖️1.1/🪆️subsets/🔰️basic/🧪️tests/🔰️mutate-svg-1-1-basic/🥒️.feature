@capability-svg-1-1-basic-mutate
@oracle-quick-xml-svg-1-1-basic-mutate
@comparison-semantic-svg-basic-1-1-v1
@mutations-svg-1-1-basic
Feature: Apply every typed SVG Basic 1.1 mutation to a real 138 KB clipped drawing
  🎨️ **The input is a real 138 KB drawing, composed ONCE out of two real committed ones.** SVG Basic
  1.1's distinguishing rule is the clip-path mechanism, and `shared://🐁️mouse.svg` — the real
  introduction-demonstration mouse graphic the framework's own onboarding UI renders today, copied
  verbatim from `🧰️framework/🔨️modules/🖼️assets/👋️introduction/🔣️mouse.svg` — is the ONLY committed
  SVG in this repository that declares a `<clipPath>` at all. It is also 1 463 bytes, which puts every
  mutation at the document's edge. Every larger real SVG committed here (the brand logos, the QR
  code, the metabolism icons) declares no clip path, so no single committed file is both real,
  complex and in-profile. `shared://🎨️semio-brand-and-onboarding.svg` is therefore composed, body for
  body and byte for byte, by `🐍️derive-svg-basic-fixture.py` in the ticket folder, out of exactly two
  real committed drawings: `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🔣️logo_dark.svg`, the repository's
  real animated brand logo (136 854 bytes, 23 real `<g>` groups, 23 real `<path>` shapes, 69 real
  `<animate>` and 69 real `<animateTransform>` elements with their real key-time and key-spline
  lists, a real `<title>`), which supplies the document element with its own real
  `viewBox="0 0 410 140"`/`version`/`xmlns` and the first 48 children; and the mouse, which supplies
  the remaining 15 — its real XML comment, its real `<defs>` holding the real
  `<clipPath id="introduction-demo-mouse-clip">`, the real `<g>` that references it through
  `clip-path="url(#introduction-demo-mouse-clip)"`, and its four real `<path>` shapes with their real
  `stroke-opacity` and `stroke-width` attributes. Nothing was invented: no element, no attribute and
  no character of the result is absent from one of the two sources. Every scenario copies it into the
  case work directory before touching it; the committed fixtures are never written to, and
  `🐁️mouse.svg` is still read on its own by `identity-round-trip`.

  Unlike the sibling `🔬️tiny` case's input, this document already conforms: SVG Basic 1.1 (W3C Mobile
  SVG Profiles, REC-SVGMobile-20030114 §SVG Basic 1.1) RETAINS opacity, masks, gradients, the
  `clipPath` element and the filter mechanism. What it excludes is narrow — nine expensive raster
  filter primitives, and clipping to text — and those two exclusions are what this vocabulary is
  built around. `insert-basic-element` inserts a real `<filter>` carrying `feGaussianBlur`, which
  Basic retains, and refuses the same insert with `feTurbulence`, which it does not.
  `set-clip-path-reference` points the mouse body at the document's real clip path, and
  `insert-clip-path-shape` adds a real shape to that same clip path; both refuse anything that would
  clip to text. None of the three exists in `🔬️tiny`'s vocabulary, whose profile has neither filters
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
  projection and to move the bytes. SVG 1.1 🔰️basic is XML and is not a byte-preserving carrier — a
  conforming writer re-derives every tag, quote and character reference from the tree — so the byte
  half of the law applies in full on both sides. A scenario that only proved the reference library
  did not error would be vacuous — it is checkable without a second producer, so it is checked
  without one.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real drawing
    Given the real input document shared://🎨️semio-brand-and-onboarding.svg
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    And the semantic projection moved, unless the kind is no-mutation
    Examples:
      | id                      | params                                                                                                                                                                                    |
      | set-snapshot            | {"rootId": "wave8-basic-snapshot-marker", "viewBoxWidth": 96}                                                                                                                             |
      | stamp-base-profile      | {"baseProfile": "basic", "version": "1.1"}                                                                                                                                                |
      | insert-basic-element    | {"parent": [51], "index": 1, "node": {"kind": "element", "name": "filter", "attrs": [{"name": "id", "value": "wave8-basic-blur"}], "children": [{"kind": "element", "name": "feGaussianBlur", "attrs": [{"name": "stdDeviation", "value": "2"}], "children": []}]}} |
      | remove-element          | {"parent": [53], "index": 5}                                                                                                                                                               |
      | set-basic-attribute     | {"path": [55], "name": "stroke-width", "value": "3.5"}                                                                                                                                     |
      | set-clip-path-reference | {"path": [55], "clipPathId": "introduction-demo-mouse-clip"}                                                                                                                               |
      | insert-clip-path-shape  | {"clipPathId": "introduction-demo-mouse-clip", "index": 1, "node": {"kind": "element", "name": "circle", "attrs": [{"name": "cx", "value": "24"}, {"name": "cy", "value": "36"}, {"name": "r", "value": "20"}], "children": []}} |
      | set-text                | {"path": [1, 0], "text": "wave8 basic mutation marker"}                                                                                                                                      |
      | set-view-box            | {"path": [], "viewBox": [0, 0, 96, 144]}                                                                                                                                                  |
      | set-transform           | {"path": [53], "transform": [{"kind": "translate", "x": 4, "y": 4}, {"kind": "scale", "x": 2}]}                                                                                            |

  @id-no-mutation-baseline-mutate
  @level-exhaustive
  @mode-differential
  Scenario: Apply no-mutation to the real drawing
    Given the real input document shared://🎨️semio-brand-and-onboarding.svg
    When the no-mutation mutation is applied with its parameters
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the semantic projection
    And the semantic projection moved, unless the kind is no-mutation

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the real drawing
    Given the real input document shared://🎨️semio-brand-and-onboarding.svg
    When the <id> mutation is applied and then undone with its own inverse
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                      | params                                                                                                                                                                                    |
      | set-snapshot            | {"rootId": "wave8-basic-snapshot-marker", "viewBoxWidth": 96}                                                                                                                             |
      | stamp-base-profile      | {"baseProfile": "basic", "version": "1.1"}                                                                                                                                                |
      | insert-basic-element    | {"parent": [51], "index": 1, "node": {"kind": "element", "name": "filter", "attrs": [{"name": "id", "value": "wave8-basic-blur"}], "children": [{"kind": "element", "name": "feGaussianBlur", "attrs": [{"name": "stdDeviation", "value": "2"}], "children": []}]}} |
      | remove-element          | {"parent": [53], "index": 5}                                                                                                                                                               |
      | set-basic-attribute     | {"path": [55], "name": "stroke-width", "value": "3.5"}                                                                                                                                     |
      | set-clip-path-reference | {"path": [55], "clipPathId": "introduction-demo-mouse-clip"}                                                                                                                               |
      | insert-clip-path-shape  | {"clipPathId": "introduction-demo-mouse-clip", "index": 1, "node": {"kind": "element", "name": "circle", "attrs": [{"name": "cx", "value": "24"}, {"name": "cy", "value": "36"}, {"name": "r", "value": "20"}], "children": []}} |
      | set-text                | {"path": [1, 0], "text": "wave8 basic mutation marker"}                                                                                                                                      |
      | set-view-box            | {"path": [], "viewBox": [0, 0, 96, 144]}                                                                                                                                                  |
      | set-transform           | {"path": [53], "transform": [{"kind": "translate", "x": 4, "y": 4}, {"kind": "scale", "x": 2}]}                                                                                            |

  @id-no-mutation-baseline-inverse
  @level-exhaustive
  @mode-differential
  Scenario: Undoing no-mutation restores the real drawing
    Given the real input document shared://🎨️semio-brand-and-onboarding.svg
    When the no-mutation mutation is applied and then undone with its own inverse
      """
      {"kind": "no-mutation", "params": {}}
      """
    Then the oracle and the subject agree on the semantic projection

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode both real drawings without passing bytes through
    Given the real input document shared://🎨️semio-brand-and-onboarding.svg
    And the onboarding mouse it was composed from shared://🐁️mouse.svg
    When each document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection of both
    And the re-encoded bytes of each are not bit-identical to its input
