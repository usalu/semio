@capability-pdf-1-7-mutate
@oracle-lopdf-pdf-1-7-base-mutate-reader
@comparison-semantic-pdf-v1
@mutations-pdf-1-7-base
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

  ALL THREE LAWS ARE ASSERTED IN ROLE, through the shared ✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law module
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

  The inverse law is not scoped down at all: every kind, on every axis, contentOperators included.

  THE AXIS THE REFERENCE'S UNDO USED TO DROP, AND HOW IT CARRIES IT NOW. remove-page,
  append-page-content and set-page-content all have to put a page's content stream back on the way
  back. The vocabulary carries the page's typed operator list (PdfPage.content; every Examples row
  below is the leaf wire payload those three kinds decode from, BT /F1 12 Tf 72 720 Td (…) Tj ET
  spelled as PdfOp records), so the subject restores the original stream exactly. The reference's
  undo used to capture a page's prior text through Tj alone and rebuild a minimal stream from it,
  while page 8 of this thesis carries 294 operators set with TJ — so those three inverses were
  compared with pages.N.contentOperators dropped and still left red. The undo now captures the
  page's operators verbatim, as lopdf decodes them, in the wire's generic unknown PdfOp record,
  and re-encodes exactly those; the carve-out is gone and all three are held to the full law.

  All three laws are proven again at unit level, against the same real document and the same
  Examples rows, by `every_declared_kind_is_observable_and_its_inverse_restores_the_document` in
  ../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs, so the argument holds without the
  runner too.

  THE DEFECTS THE DIFFERENTIAL RUNS FOUND, EACH FIXED IN THE CODEC RATHER THAN EXEMPTED. PdfSnapshot
  carries the document twice — pages/info and the other typed lanes are the authoring surface every
  typed mutation edits, objects/trailer the retained native carrier every COS-level mutation edits.
  First, encode_pdf serialized the carrier ALONE, so every page and metadata edit applied to the
  snapshot and vanished on export. Its first fix then regenerated every typed object whenever any
  lane moved: a page edit re-stated the whole catalog (inlining /Names /Dests, adding
  /OpenAction /Type), a direct COS edit to an object a typed lane owns (set-object-value #145, the
  catalog's own dict entries, remove-object #3015) was overwritten by the stale typed lane, and
  set-trailer-entry's custom key was never written. Now a COS-level mutation carries every typed lane
  its edit moves (io's carry_graph_edit), and ../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🦀️.rs
  reconciles with incremental-writer semantics: a graph that spells every typed lane is written as
  it stands; a moved lane re-states only what it owns — a page's moved entries in place, the page
  tree flat when the page SEQUENCE changed, /Info in place, a catalog lane's own entries — and the
  grafted graph is read back until it spells the typed lanes; every other object, dictionary entry
  and trailer entry survives untouched. No comparison profile was touched, no ignoreKeys added and no
  fixture swapped.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🎓️bachelor-thesis/🎓️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params |
      | insert-page          | {"index": 30, "page": {"mediaBox": [0, 0, 612, 792], "rotate": 0, "content": [{"op": "beginText"}, {"op": "setFont", "name": "F1", "size": 12}, {"op": "moveText", "tx": 72, "ty": 720}, {"op": "showText", "text": {"kind": "text", "text": "Inserted page for wave 7 mutation testing"}}, {"op": "endText"}]}} |
      | remove-page          | {"index": 7} |
      | set-page-media-box   | {"index": 15, "mediaBox": [0, 0, 595, 842]} |
      | set-page-crop-box    | {"index": 16, "cropBox": [10, 10, 580, 820]} |
      | append-page-content  | {"index": 17, "content": [{"op": "beginText"}, {"op": "setFont", "name": "F1", "size": 12}, {"op": "moveText", "tx": 72, "ty": 720}, {"op": "showText", "text": {"kind": "text", "text": "Appended content line for wave 7 testing"}}, {"op": "endText"}]} |
      | set-info             | {"info": {"title": "Wave 7 Replaced Title", "author": "Wave 7 Test Author"}} |
      | insert-object        | {"id": {"num": 900001, "gen": 0}, "value": {"kind": "dict", "value": [{"key": "Type", "value": {"kind": "name", "value": "SemioWave7Marker"}}, {"key": "Note", "value": {"kind": "str", "value": [105, 110, 115, 101, 114, 116, 101, 100, 32, 98, 121, 32, 119, 97, 118, 101, 32, 55]}}]}} |
      | remove-object        | {"id": {"num": 3015, "gen": 0}} |
      | set-object-value     | {"id": {"num": 145, "gen": 0}, "value": {"kind": "dict", "value": [{"key": "S", "value": {"kind": "name", "value": "GoToR"}}, {"key": "Note", "value": {"kind": "str", "value": [114, 101, 112, 108, 97, 99, 101, 100, 32, 98, 121, 32, 119, 97, 118, 101, 32, 55]}}]}} |
      | set-dict-entry       | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "PageMode", "value": {"kind": "name", "value": "UseNone"}} |
      | remove-dict-entry    | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "Outlines"} |
      | set-trailer-entry    | {"key": "SemioWave7Marker", "value": {"kind": "int", "value": 42}} |
      | remove-trailer-entry | {"key": "ID"} |
      | move-page            | {"from": 10, "to": 40} |
      | set-page-content     | {"index": 20, "content": [{"op": "beginText"}, {"op": "setFont", "name": "F1", "size": 12}, {"op": "moveText", "tx": 72, "ty": 720}, {"op": "showText", "text": {"kind": "text", "text": "Replaced page content for wave 7 mutation testing"}}, {"op": "endText"}]} |
      | set-page-rotation    | {"index": 5, "rotation": 90} |
      | patch-snapshot | {"patch": {"operation": "set", "path": "/pages/15/mediaBox", "value": [0, 0, 595, 842]}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the document
    Given the real input document asset://🎓️bachelor-thesis/🎓️bachelor-thesis.pdf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the semantic projection
    Examples:
      | id                   | params |
      | insert-page          | {"index": 30, "page": {"mediaBox": [0, 0, 612, 792], "rotate": 0, "content": [{"op": "beginText"}, {"op": "setFont", "name": "F1", "size": 12}, {"op": "moveText", "tx": 72, "ty": 720}, {"op": "showText", "text": {"kind": "text", "text": "Inserted page for wave 7 mutation testing"}}, {"op": "endText"}]}} |
      | remove-page          | {"index": 7} |
      | set-page-media-box   | {"index": 15, "mediaBox": [0, 0, 595, 842]} |
      | set-page-crop-box    | {"index": 16, "cropBox": [10, 10, 580, 820]} |
      | append-page-content  | {"index": 17, "content": [{"op": "beginText"}, {"op": "setFont", "name": "F1", "size": 12}, {"op": "moveText", "tx": 72, "ty": 720}, {"op": "showText", "text": {"kind": "text", "text": "Appended content line for wave 7 testing"}}, {"op": "endText"}]} |
      | set-info             | {"info": {"title": "Wave 7 Replaced Title", "author": "Wave 7 Test Author"}} |
      | insert-object        | {"id": {"num": 900001, "gen": 0}, "value": {"kind": "dict", "value": [{"key": "Type", "value": {"kind": "name", "value": "SemioWave7Marker"}}, {"key": "Note", "value": {"kind": "str", "value": [105, 110, 115, 101, 114, 116, 101, 100, 32, 98, 121, 32, 119, 97, 118, 101, 32, 55]}}]}} |
      | remove-object        | {"id": {"num": 3015, "gen": 0}} |
      | set-object-value     | {"id": {"num": 145, "gen": 0}, "value": {"kind": "dict", "value": [{"key": "S", "value": {"kind": "name", "value": "GoToR"}}, {"key": "Note", "value": {"kind": "str", "value": [114, 101, 112, 108, 97, 99, 101, 100, 32, 98, 121, 32, 119, 97, 118, 101, 32, 55]}}]}} |
      | set-dict-entry       | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "PageMode", "value": {"kind": "name", "value": "UseNone"}} |
      | remove-dict-entry    | {"id": {"num": 3188, "gen": 0}, "path": [], "key": "Outlines"} |
      | set-trailer-entry    | {"key": "SemioWave7Marker", "value": {"kind": "int", "value": 42}} |
      | remove-trailer-entry | {"key": "ID"} |
      | move-page            | {"from": 10, "to": 40} |
      | set-page-content     | {"index": 20, "content": [{"op": "beginText"}, {"op": "setFont", "name": "F1", "size": 12}, {"op": "moveText", "tx": 72, "ty": 720}, {"op": "showText", "text": {"kind": "text", "text": "Replaced page content for wave 7 mutation testing"}}, {"op": "endText"}]} |
      | set-page-rotation    | {"index": 5, "rotation": 90} |
      | patch-snapshot | {"patch": {"operation": "set", "path": "/pages/15/mediaBox", "value": [0, 0, 595, 842]}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🎓️bachelor-thesis/🎓️bachelor-thesis.pdf
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the oracle and the subject agree on the semantic projection
    And the re-encoded bytes are not bit-identical to the input
