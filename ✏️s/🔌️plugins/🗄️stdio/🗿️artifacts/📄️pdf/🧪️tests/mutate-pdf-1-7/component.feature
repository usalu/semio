@capability-pdf-1-7-mutate
@oracle-lopdf-pdf-1-7-mutate
@comparison-semantic-pdf-v1
@mutations-pdf-1-7-any
Feature: Apply every typed PDF 1.7 mutation to a real-world document
  The input is a real 65-page bachelor thesis produced by LaTeX, not a synthetic fixture, and it is
  read where the domain already keeps it. Every scenario copies it into the case work directory
  before touching it; the committed document is never written to. The oracle drives the registered
  `lopdf` reference implementation over this subset's own real object-graph model (18 mutation
  kinds: page insert/remove/reorder/media-box/crop-box/rotate/content-replace/content-append, plus
  the raw object-graph vocabulary — insert/remove/set-object, dict-entry and trailer-entry edits).
  `remove-page` and `set-info` route through the shared `document` module's own
  `oracle_delete_page`/`oracle_replace_metadata`; every other kind is this module's own. Both the
  oracle's and the subject's results are read back by the SAME independent `lopdf`-backed projection
  before comparison, never against each other's own writing.

  THE LAWS THE ORACLE ASSERTS IN-ROLE, so a scenario cannot pass merely because `lopdf` did not
  error. `inverse-<kind>` applies the mutation, applies its own independently computed inverse, and
  fails with the first diverging field unless the result projects onto exactly what the original
  document projects onto. `identity-round-trip` fails unless the re-serialized bytes differ from the
  input AND their projection is identical to the input's. Neither law is scoped down: the whole
  `semantic-pdf-v1` projection — declared version, page count, and every page's media box, content
  operators, shown text and rotation — has to come back, with one exception, on one axis, for three
  kinds, stated here in full.

  THE ONE AXIS THIS VOCABULARY CANNOT CARRY, FOUND BY ASSERTING THE LAW RATHER THAN BY REASONING
  ABOUT IT. remove-page, append-page-content and set-page-content all have to REBUILD a page's
  content stream on the way back, and PdfPage's only content field is text
  (../../🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs), so InsertPage and
  SetPageContent carry extracted text and nothing else and both producers regenerate a minimal
  BT /F1 12 Tf 72 720 Td (…) Tj ET stream from it. Page 8 of this thesis carries 294 operators —
  glyph positioning, graphics state, the lot — and no round trip through a single text field can
  bring them back. AppendPageContent was documented from the start as having no minimal inverse in
  this vocabulary; this is the same gap, now measured. Those three inverse scenarios therefore compare
  the projection with pages.N.contentOperators dropped and nothing else dropped: declared version,
  page count, every page's media box, rotation and — critically — the shown text the vocabulary DOES
  carry all stay under the full law, and every other kind in the catalog stays under it on every axis
  including contentOperators. Widening PdfPage to retain a real content stream is the fix, and it
  belongs to whoever owns that snapshot.

  A SECOND, SMALLER DEFECT THE SAME LAW EXPOSED, AND IT WAS FIXED RATHER THAN EXEMPTED. This thesis
  sets its type with TJ, the positioned-array form, so the independent reader projects most of its
  pages as text: []. The oracle's own writer used to encode an empty text as () Tj — a text-showing
  operator showing the empty string — which turned such a page into one projecting as text: [""].
  It now writes BT ET, which is the faithful reconstruction of "no text", and the text axis passes
  under the full law.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                    |
      | no-mutation          | {}                                                                                                                                                                        |
      | set-snapshot         | {"declaredVersion": "2.0", "title": "Wave 7 Snapshot Title"}                                                                                                             |
      | insert-page          | {"index": 30, "page": {"mediaBox": [0, 0, 612, 792], "rotate": 0, "text": "Inserted page for wave 7 mutation testing"}}                                                  |
      | remove-page          | {"index": 7}                                                                                                                                                              |
      | set-page-media-box   | {"index": 15, "mediaBox": [0, 0, 595, 842]}                                                                                                                              |
      | set-page-crop-box    | {"index": 16, "cropBox": [10, 10, 580, 820]}                                                                                                                             |
      | append-page-content  | {"index": 17, "text": "Appended content line for wave 7 testing"}                                                                                                        |
      | set-info             | {"title": "Wave 7 Replaced Title", "author": "Wave 7 Test Author"}                                                                                                       |
      | insert-object        | {"id": {"num": 900001, "gen": 0}, "value": {"kind": "dict", "entries": [{"key": "Type", "value": {"kind": "name", "value": "SemioWave7Marker"}}, {"key": "Note", "value": {"kind": "str", "value": "inserted by wave 7"}}]}} |
      | remove-object        | {"id": {"num": 3015, "gen": 0}}                                                                                                                                          |
      | set-object-value     | {"id": {"num": 145, "gen": 0}, "value": {"kind": "dict", "entries": [{"key": "S", "value": {"kind": "name", "value": "GoToR"}}, {"key": "Note", "value": {"kind": "str", "value": "replaced by wave 7"}}]}} |
      | set-dict-entry       | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "PageMode", "value": {"kind": "name", "value": "UseNone"}}                                                           |
      | remove-dict-entry    | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "Outlines"}                                                                                                           |
      | set-trailer-entry    | {"key": "SemioWave7Marker", "value": {"kind": "int", "value": 42}}                                                                                                       |
      | remove-trailer-entry | {"key": "ID"}                                                                                                                                                            |
      | move-page            | {"from": 10, "to": 40}                                                                                                                                                   |
      | set-page-content     | {"index": 20, "text": "Replaced page content for wave 7 mutation testing"}                                                                                               |
      | set-page-rotation    | {"index": 5, "rotation": 90}                                                                                                                                             |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                    |
      | no-mutation          | {}                                                                                                                                                                        |
      | set-snapshot         | {"declaredVersion": "2.0", "title": "Wave 7 Snapshot Title"}                                                                                                             |
      | insert-page          | {"index": 30, "page": {"mediaBox": [0, 0, 612, 792], "rotate": 0, "text": "Inserted page for wave 7 mutation testing"}}                                                  |
      | remove-page          | {"index": 7}                                                                                                                                                              |
      | set-page-media-box   | {"index": 15, "mediaBox": [0, 0, 595, 842]}                                                                                                                              |
      | set-page-crop-box    | {"index": 16, "cropBox": [10, 10, 580, 820]}                                                                                                                             |
      | append-page-content  | {"index": 17, "text": "Appended content line for wave 7 testing"}                                                                                                        |
      | set-info             | {"title": "Wave 7 Replaced Title", "author": "Wave 7 Test Author"}                                                                                                       |
      | insert-object        | {"id": {"num": 900001, "gen": 0}, "value": {"kind": "dict", "entries": [{"key": "Type", "value": {"kind": "name", "value": "SemioWave7Marker"}}, {"key": "Note", "value": {"kind": "str", "value": "inserted by wave 7"}}]}} |
      | remove-object        | {"id": {"num": 3015, "gen": 0}}                                                                                                                                          |
      | set-object-value     | {"id": {"num": 145, "gen": 0}, "value": {"kind": "dict", "entries": [{"key": "S", "value": {"kind": "name", "value": "GoToR"}}, {"key": "Note", "value": {"kind": "str", "value": "replaced by wave 7"}}]}} |
      | set-dict-entry       | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "PageMode", "value": {"kind": "name", "value": "UseNone"}}                                                           |
      | remove-dict-entry    | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "Outlines"}                                                                                                           |
      | set-trailer-entry    | {"key": "SemioWave7Marker", "value": {"kind": "int", "value": 42}}                                                                                                       |
      | remove-trailer-entry | {"key": "ID"}                                                                                                                                                            |
      | move-page            | {"from": 10, "to": 40}                                                                                                                                                   |
      | set-page-content     | {"index": 20, "text": "Replaced page content for wave 7 mutation testing"}                                                                                               |
      | set-page-rotation    | {"index": 5, "rotation": 90}                                                                                                                                             |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
