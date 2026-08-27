@capability-pdf-1-7-mutate
@oracle-lopdf-pdf-1-7-mutate
@comparison-semantic-pdf-v1
@mutations-pdf-1-7-any
Feature: Apply every typed PDF 1.7 mutation to a real-world document
  The input is a real 65-page bachelor thesis produced by LaTeX, not a synthetic fixture, and it is
  read where the domain already keeps it. Every scenario copies it into the case work directory
  before touching it; the committed document is never written to. The oracle drives the registered
  `lopdf` reference implementation over this subset's own real object-graph model (16 direct mutation
  kinds: page insert/remove/reorder/media-box/crop-box/rotate/content-replace/content-append, plus
  the raw object-graph vocabulary — insert/remove/set-object, dict-entry and trailer-entry edits).
  `remove-page` and `set-info` route through the shared `document` module's own
  `oracle_delete_page`/`oracle_replace_metadata`; every other kind is this module's own. Both the
  oracle's and the subject's results are read back by the SAME independent `lopdf`-backed projection
  before comparison, never against each other's own writing.

  ALL THREE LAWS ARE ASSERTED IN ROLE, through the shared ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law module
  and under `semantic-pdf-v1`'s own tolerance, so no scenario can pass merely because `lopdf`
  declined to error. `mutate-<kind>` fails unless the mutation MOVES the compared projection — a
  kind that applies cleanly and changes nothing observable would otherwise report a green for a
  mutation nobody watched, and until this wave all sixteen of them did exactly that.
  `inverse-<kind>` applies the mutation, applies its own independently computed inverse, and fails
  with the first diverging field unless the result projects onto exactly what the original document
  projects onto. `identity-round-trip` fails unless the re-serialized bytes differ from the input
  AND their projection is identical to the input's.

  WHAT THE PROJECTION HAD TO GROW BEFORE THE OBSERVABILITY LAW COULD BE HONEST. The shared PDF
  projection reports declared version, page count and per-page media box, content operators and
  shown text — a page-and-metadata surface. Seven of this catalog's sixteen kinds never touch a
  page: insert-object, remove-object, set-object-value, set-dict-entry, remove-dict-entry,
  set-trailer-entry and remove-trailer-entry all edit the COS object graph, and an eighth,
  set-page-crop-box, moves a page field the shared surface does not report. Asserting observability
  against that surface would have meant declaring eight kinds unobservable, which would be shrinking
  the law to fit the projection. This subset's own project_pdf_1_7 therefore reports two things more
  — each page's /CropBox, and an objectGraph member carrying the trailer (minus the /Size, /Prev and
  /XRefStm bookkeeping the writer recomputes on every save) and the document catalog resolved three
  references deep, with /Pages omitted because pageCount and pages already project the page tree in
  full and re-reporting it would make every page edit register twice. Object NUMBERS never appear in
  it: semantic-pdf-v1 calls them writer freedom, and a resolved value is what a conforming reader
  sees anyway (ISO 32000-1 §7.3.10). On the real thesis that surface is where set-dict-entry's
  /PageMode, remove-dict-entry's /Outlines, remove-object's #3015 (the outline root the catalog
  resolves to), set-object-value's #145 (the /OpenAction the catalog resolves to) and both trailer
  kinds become visible — seven of the eight, under the full law, with no exemption.

  THE ONE KIND THAT STAYS UNOBSERVABLE, AND WHY NO PROJECTION CAN FIX IT. insert-object adds an
  indirect object and links it to nothing. ISO 32000-1 §7.5.4 has a conforming reader reach objects
  only by following references from the trailer, so an object nothing references changes nothing
  readable. That is not a thin projection, it was measured: the real thesis carries 3,173 objects,
  3,173 references, ZERO orphans and ZERO dangling references, so there is no id at which an
  insertion could land somewhere already pointed at. The vocabulary is what cannot express it —
  InsertObject carries no reference site, and only SetDictEntry can create one. Widening it to carry
  the linking site is the fix, and it belongs to whoever owns that enum. Its INVERSE stays under the
  full law, as does every other kind; the exemption is one kind on one law, named in the subset's own
  oracle module as UNOBSERVABLE and pinned there by a test that flips red the moment the vocabulary
  or the fixture changes.

  The inverse law is not scoped down either, with one exception, on one axis, for three kinds,
  stated here in full.

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
  page count, every page's media box, crop box, rotation, the whole objectGraph surface and —
  critically — the shown text the vocabulary DOES carry all stay under the full law, and every other
  kind in the catalog stays under it on every axis including contentOperators. Widening PdfPage to retain a real content stream is the fix, and it
  belongs to whoever owns that snapshot.

  All three laws are proven again at unit level, against the same real document and the same
  Examples rows, by `every_declared_kind_is_observable_and_its_inverse_restores_the_document` in
  ../../🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs, so the argument holds without the
  runner too.

  A SECOND, SMALLER DEFECT THE SAME LAW EXPOSED, AND IT WAS FIXED RATHER THAN EXEMPTED. This thesis
  sets its type with TJ, the positioned-array form, so the independent reader projects most of its
  pages as text: []. The oracle's own writer used to encode an empty text as () Tj — a text-showing
  operator showing the empty string — which turned such a page into one projecting as text: [""].
  It now writes BT ET, which is the faithful reconstruction of "no text", and the text axis passes
  under the full law.

  THE DEFECT THE DIFFERENTIAL RUN FOUND, AND IT WAS FIXED IN THE CODEC RATHER THAN EXEMPTED. The
  first time this case actually ran oracle AGAINST subject it scored the ratio recorded in the
  ticket, not here, and ten of its thirteen failures were one bug: PdfSnapshot carries the document twice — pages/info are the
  resolved authoring lanes every page and metadata mutation edits, objects/trailer are the retained
  native carrier — and encode_pdf serialized the carrier ALONE whenever it was non-empty. On this
  thesis it always is, so set-page-rotation, set-page-media-box, set-page-crop-box, set-page-content,
  append-page-content, insert-page, remove-page, move-page and set-info all applied
  cleanly to the snapshot and then vanished on export: the subject's own reader projected 65 pages
  where the oracle had 64, rotate 0 where the oracle had 90, title "" where the oracle had the
  stamped one. That is a mutation reporting as applied that no reader can find in the bytes, and it
  contradicted PdfPage's own docstring ("the writer regenerates a fresh content stream from it on
  encode"). ../../🏅️standards/🔖️1.7/🪆️subsets/✳️any/🚪️io/🦀️component.rs now writes the authored lanes
  back onto the retained graph before serializing — patching the leaf page objects in place,
  rebuilding /Kids and /Count when the page SET changes, appending rather than re-rendering when a
  page's text only grew, and re-stating /Info as the whole record PdfInfo declares itself to be —
  and rewrites nothing that did not move. Parity went 24/37 to 34/37 with no comparison profile
  touched, no ignoreKeys added and no fixture swapped.

  THE THREE THAT REMAIN, AND WHY THEY ARE LEFT RED. inverse-remove-page, inverse-append-page-content
  and inverse-set-page-content still diverge, on ONE axis and one only: pages.N.contentOperators.
  Every other axis agrees exactly — page count, media box, crop box, rotation, the shown text and
  the whole objectGraph. The subject restores the page's ORIGINAL content stream (294, 148 and 289
  operators respectively) because returning the authored lane to its base value leaves the retained
  carrier untouched, so the writer has nothing to rewrite; the reference lands on a two-operator
  BT ET, because its own capture of a page's prior text reads only the Tj operator and this thesis
  sets its type with TJ, so it undoes the mutation from an empty string. Both sides project text: []
  and agree there — the loss is in the reference's round trip, not in the projection and not in this
  implementation. The subject half proves it rather than asserting it in prose: its inverse-<kind>
  handler holds all sixteen kinds to the inverse law with NO carve-out at all, contentOperators
  included, and all sixteen pass. Dropping the axis from the comparison would make these three go
  green while hiding exactly that fact, so they stay red and attributed. The oracle half keeps its
  own documented one-axis exemption for the same three kinds, because on ITS side the loss is real.

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
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params                                                                                                                                                                    |
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
