@capability-bcf-2-1-mutate
@oracle-jszip-bcf-2-1-mutate-reader
@comparison-semantic-bcf-jszip-v1
@mutations-bcf-2-1-any
Feature: Apply every typed BCF 2.1 markup mutation and round-trip a real-world coordination review

  A BCF file IS a ZIP of XML markup, viewpoint and snapshot files — no standalone BCF crate exists
  in the Rust ecosystem (BCF support only appears bundled inside much larger MPL-licensed IFC
  toolkits), so the oracle composes two already-linked, genuinely independent crates over the real
  BCF-XML 2.1 shapes: `zip` 6 for the flat bcfzip container and `quick-xml` 0.42 for every XML part
  inside it. `📰️xml`'s own oracle is the precedent for the `quick-xml` half, `🎒️zip`'s for the
  archive half; this subset composes both into one independent reader/writer rather than reusing
  either module directly.

  The committed `💬️example.bcf` under this artifact's own `📚️examples/` is a 0-byte placeholder, not
  a fixture — reported as a finding, not used here. No real BCF file (a genuine export from a
  coordination tool such as Solibri, BIM Collab or Navisworks) exists anywhere in this repository, so
  the input below is DERIVED, once, by the ticket-folder script
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/bcf-2-1-mutate/derive_fixture.py`,
  composing real sources rather than inventing content:

  - Every `IfcGuid` referenced by a viewpoint's component selection/visibility/coloring, and every
    element name quoted in a topic's title/description, was read directly out of the real, committed
    21 MB IFC2X3 export `temp/wellness-center-sama.ifc` (Autodesk Revit 2021 (ENU)) — e.g. wall
    `0HG2A49bzDARlPHy2ZDHwJ` ("Basic Wall:CW 102-50-100p:350250"), column
    `0PfeWE7Aj7GBHCsLa67379` ("UC-Universal Columns-Column:UC305x305x97:552739"), door
    `2JJqxZjqn96xzCFMbZMpfb` ("Door-Exterior-Double-Two_Lite:my door:388452"), curtain-wall mullion
    `2lrUU8Tqz92AICLQu1TLwD`, storey `0a3v3dJi10mxIqGCVATOEH` ("First floor") and project
    `0a3v3dJi10mxIqGCSrYdxN`. Two more real elements (a slab and a second column) are exercised only
    by the mutation scenarios below, never by the base fixture, so `insert-topic`/`set-snapshot`
    introduce genuinely new real content rather than repeating what is already there.
  - Every viewpoint snapshot is real pixel data: one topic's snapshot is the real, unmodified 244 KB
    committed floor plan `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🧫️fixtures/🖼️rathaus-ahlen-grundriss.png`;
    the other is a real 64×64 crop of the same PNG (182 bytes) — a genuine derived excerpt, never
    synthetic filler. The mutation scenarios' own `insert-viewpoint`/`insert-topic`/
    `set-viewpoint-snapshot` payloads reuse that same crop, or a second real 64×64 crop of the same
    source image at a different offset, hex-encoded inline.
  - Topic/comment/viewpoint GUIDs (the `3b1f6a1e-...` series) are fresh identifiers this derivation
    minted, exactly as any real BCF-writing tool mints them — they are never IFC entity identity, so
    minting them here does not compromise the "real GUIDs" claim above, which is about the
    `IfcGuid`s a real tool would also have copied from the model.
  - Camera positions/directions are ordinary BCF viewpoint metadata (where a reviewer was notionally
    standing) — every real BCF export carries numbers exactly like these; they describe a viewing
    pose, not building geometry, so they carry no separate "real-world" claim the way an IfcGuid does.

  Honesty about the fixture's status: this is a DERIVED package assembled by this ticket, not a real
  export from a coordination tool. Every reviewer name/date/text is authored by this derivation
  (`ueli.saluz@iek.uni-hannover.de`, 2026-08-23), not recovered from a real review; what is real is
  the IFC element identity/geometry data and the snapshot pixels, not the review narrative itself.

  Three topics ship in the base fixture: one Open clash (column vs. wall, one comment, one
  perspective-camera viewpoint with a full-size real snapshot), one InProgress clash (door vs.
  mullion, two comments, one orthogonal-camera viewpoint with a real cropped snapshot) and one
  Closed topic with no comments/viewpoints at all (an empty-topic edge case for `remove-topic`). A
  real `project.bcfp` raw part (referencing the real project GUID/name) is retained verbatim by every
  scenario that does not touch it, proving this subset's raw-retention path stays honest under
  mutation.

  Binary payloads travel through mutation params as lowercase hex, the same convention
  `BcfSnapshot::parse_dsl`/`print_dsl` already use for this artifact's own DSL form. A viewpoint's
  PNG snapshot projects as size+digest (never raw bytes) under `semantic-bcf-v1`, matching the fleet
  brief's own raster precedent for opaque binary payloads.

  THE JUDGE. `jszip-bcf-2-1-mutate-reader` is a third-party READER (jszip + fast-xml-parser): each mutation row's
  expected document is not computed, it is the COMMITTED `➡️after.bcf` of a before/after pair this subset's own
  generator (`../../🏭️generator/📜️script.ts`) builds DIRECTLY with jszip + fast-xml-parser from its small two-topic base
  (`⬅️before.bcf` for an inverse row), and the `bcf-2-1-jszip-compare-v1` pipeline reads it and the subject's
  `actual-bcf` with the same reader (`bcf-import` admits both, `bcf-compare` compares the projection). The mutation
  rows therefore run on those small committed pairs — the parameters state exactly each pair's one change — while the
  real coordination review above is judged by the identity round trip, whose expected document is the real review
  itself. The cross-semio `zip`+`quick-xml` composition (`../../🔮️oracles/🦀️.rs`) stays as the Rust supplement and
  asserts its own laws in role.

  📌️ Every Examples row below is required to MOVE the semantic projection, and the adapter fails the
  scenario in role when it does not: a row whose parameters make the mutation a no-op passes whenever
  the reference library merely declined to error, which is not a test. The baseline it is measured
  against runs one unzip/rezip round trip first, so the comparison isolates the mutation rather than
  the writer's own normal form. Every row's `params` is the leaf wire payload (`set-snapshot` carries
  the whole `BcfSnapshot`, a viewpoint snapshot its PNG bytes), decoded by `BcfMutation`'s own payload
  constructor.
  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the committed review pair
    Given the real input document shared://<fixture>/⬅️before.bcf
    And the committed after-document shared://<fixture>/➡️after.bcf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the jszip reader reads the subject's review and the committed after-document as the same BCF
    Examples:
      | id               | fixture                    | params |
      | set-snapshot     | 🗃️set-snapshot-applied     | {"snapshot": {"schema": "stdio.bcf", "version": "2.1", "topics": [{"guid": "topic-replacement-04", "title": "Slab clash near the stair core", "description": "Replacement review: the slab intersects the stair-core column.", "status": "Open", "priority": "High", "labels": ["structural"], "creationDate": "2026-01-08T09:00:00Z", "creationAuthor": "dave@example.com", "comments": [], "viewpoints": []}], "parts": []}} |
      | set-version      | 🔢️set-version-applied      | {"version": "2.2"} |
      | insert-topic     | 📌️insert-topic-applied     | {"topic": {"guid": "topic-new-03", "title": "New topic", "description": "", "status": "Open", "priority": "", "labels": [], "creationDate": "2026-01-07T09:00:00Z", "creationAuthor": "carol@example.com", "comments": [], "viewpoints": []}} |
      | remove-topic     | 🗑️remove-topic-applied     | {"guid": "topic-review-02"} |
      | set-topic-markup | 🖊️set-topic-markup-applied | {"guid": "topic-clash-01", "status": "Closed", "priority": "Low"} |
      | insert-comment   | 🗨️insert-comment-applied   | {"topicGuid": "topic-clash-01", "comment": {"guid": "comment-02", "date": "2026-01-05T11:00:00Z", "author": "bob@example.com", "text": "Confirmed, rerouting duct.", "viewpointRef": null}} |
      | remove-comment   | 🧹️remove-comment-applied   | {"topicGuid": "topic-clash-01", "guid": "comment-01"} |
      | set-comment      | ✏️set-comment-applied      | {"topicGuid": "topic-clash-01", "guid": "comment-01", "text": "Please review — updated."} |
      | patch-snapshot | 🔢️set-version-applied | {"patch": {"operation": "set", "path": "/version", "value": "2.2"}} |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed review
    Given the real input document shared://<fixture>/⬅️before.bcf
    When the <id> mutation is applied and then undone
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the jszip reader reads the restored review and the committed before-document as the same BCF
    Examples:
      | id               | fixture                    | params |
      | set-snapshot     | 🗃️set-snapshot-applied     | {"snapshot": {"schema": "stdio.bcf", "version": "2.1", "topics": [{"guid": "topic-replacement-04", "title": "Slab clash near the stair core", "description": "Replacement review: the slab intersects the stair-core column.", "status": "Open", "priority": "High", "labels": ["structural"], "creationDate": "2026-01-08T09:00:00Z", "creationAuthor": "dave@example.com", "comments": [], "viewpoints": []}], "parts": []}} |
      | set-version      | 🔢️set-version-applied      | {"version": "2.2"} |
      | insert-topic     | 📌️insert-topic-applied     | {"topic": {"guid": "topic-new-03", "title": "New topic", "description": "", "status": "Open", "priority": "", "labels": [], "creationDate": "2026-01-07T09:00:00Z", "creationAuthor": "carol@example.com", "comments": [], "viewpoints": []}} |
      | remove-topic     | 🗑️remove-topic-applied     | {"guid": "topic-review-02"} |
      | set-topic-markup | 🖊️set-topic-markup-applied | {"guid": "topic-clash-01", "status": "Closed", "priority": "Low"} |
      | insert-comment   | 🗨️insert-comment-applied   | {"topicGuid": "topic-clash-01", "comment": {"guid": "comment-02", "date": "2026-01-05T11:00:00Z", "author": "bob@example.com", "text": "Confirmed, rerouting duct.", "viewpointRef": null}} |
      | remove-comment   | 🧹️remove-comment-applied   | {"topicGuid": "topic-clash-01", "guid": "comment-01"} |
      | set-comment      | ✏️set-comment-applied      | {"topicGuid": "topic-clash-01", "guid": "comment-01", "text": "Please review — updated."} |
      | patch-snapshot | 🔢️set-version-applied | {"patch": {"operation": "set", "path": "/version", "value": "2.2"}} |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real coordination review without passing bytes through
    Given the real input document shared://🏥️wellness-center-coordination-review.bcf
    When the document is fully parsed into the subset's own snapshot model and re-encoded from it alone
    Then the jszip reader reads the re-encoded review and the real review as the same BCF
    And the re-encoded bytes are not bit-identical to the input
