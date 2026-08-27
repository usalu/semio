@capability-pdf-1-7-ua-mutate
@oracle-lopdf-pdf-1-7-ua-mutate
@comparison-semantic-pdf-conformance-ua-v1
@mutations-pdf-1-7-ua
Feature: Apply every typed ISO 14289-1 (PDF/UA-1) conformance-class mutation to a real document
  The input is the real, committed 6.3 MB bachelor thesis produced by MiKTeX pdfTeX 1.40.21 — 65
  pages, 3,189 indirect objects, a classic cross-reference table, 70 /Type /Font objects and 23
  /Type /FontDescriptor objects, every one of the 23 carrying an embedded font program (5 /FontFile,
  16 /FontFile2, 2 /FontFile3). Scanning the committed file confirms it carries NO /Encrypt, no
  /S /JavaScript action and no /JS key, no /S /Launch action, no /Subtype /Movie or /Sound
  annotation, no /Type /Filespec, no /OutputIntents, no /MarkInfo, no /StructTreeRoot, no /Lang, no
  /ViewerPreferences, no /AcroForm, no /DPartRoot and no /TrimBox or /ArtBox on any page — it is a
  perfectly ordinary PDF that conforms to no conformance class at all, which is exactly what makes
  it the right input here: every scenario moves the real document along ONE axis of the class and
  then back. It is read where the domain already keeps it; every scenario copies it into the case
  work directory before touching it, and the committed document is never written to.

  WHAT THIS VOCABULARY IS, AND WHY IT IS NOT A COPY OF ✳️any. The ✳️any subset of this same standard
  owns the DOCUMENT vocabulary — insert-page, remove-page, move-page, the media/crop box kinds, page
  content, /Info as authoring metadata, and the raw object/dict/trailer edit primitives. This subset
  owns ISO 14289-1 (PDF/UA-1), a CONFORMANCE CLASS, which is a property of the object graph as a whole and
  of no page at all. The catalog is one kind per axis of this subset's own check_ua_conformance, which
  reads six axes, all of them keys of the document CATALOG or of `/Info`: `/Root/MarkInfo` with `/Marked true`, `/Root/StructTreeRoot`, a non-empty `/Root/Lang`, `/Root/ViewerPreferences` with `/DisplayDocTitle true`, a non-empty `Info.title`, and an embedded program on every font's `/FontDescriptor`. No ✳️any mutation moves any of those axes and no mutation here touches
  page content, so the two vocabularies are disjoint by construction.

  PDF/UA is the only conformance class in this standard that is about ACCESSIBILITY rather than about reproduction or archiving, and its vocabulary shows it: not one variant here addresses an object that could appear in a print or archival profile. `set-mark-info` and `set-struct-tree-root` are its two HARD axes — `check_ua_conformance` is the only checker in the sextet that calls `hard()` for a MISSING key rather than for a forbidden one — and `set-lang`/`set-display-doc-title` are its two soft ones. It shares `set-info-title` with `✳️h` and nothing else with anybody: no encryption axis, no action axes, no output intent, because `check_ua_conformance` reads none of them.

  THE REFERENCE. `lopdf` 0.44 parses the complete COS object graph of the real document and writes a
  fresh file from that graph alone — never a patch of the input bytes — and it both performs and
  observes every kind this catalog declares, which is why every mutate scenario is @mode-differential
  rather than a weaker mode. It is test-only; this repository's own PDF codec is hand-written and
  links nothing.

  ONE DELIBERATE SCOPE NOTE, RECORDED RATHER THAN GLOSSED. insert-encryption-dictionary adds a
  free-standing Standard Security Handler dictionary OBJECT and does not link it from the trailer's
  /Encrypt. That is faithful to what this subset actually checks — check_ua_conformance scans the retained
  objects for the /Filter /Standard + /V + /R + /O + /U shape and never reads the trailer — and it is
  the only form the mutation can take and still leave a document both producers can re-read, since a
  genuinely /Encrypt-linked trailer makes every string and stream in the file ciphertext.

  THE ARRANGED SCENARIOS, AND THAT THEY ARE ARRANGED. The committed document carries none of the
  constructs listed above, so a scenario whose mutation REMOVES one runs on the real document after
  the SAME independent implementation has put it there. The mutation under test is still the
  removal, still performed by the reference, still on a genuine 3,189-object graph:
    remove-mark-info — /Root/MarkInfo is installed first.
    remove-struct-tree-root — /Root/StructTreeRoot is installed first.
    remove-lang — /Root/Lang is installed first.
    remove-display-doc-title — /Root/ViewerPreferences is installed first.
    embed-font-file — descriptor 4's embedded program is REMOVED first — all 23 of the fixture's /FontDescriptor objects already carry one.

  THE PROJECTION RECORDS CONTENT, NOT OBJECT NUMBERS. A conformance class is defined on what a
  document contains, not on where a writer chose to put it: re-inserting a removed action at a fresh
  object number is a faithful undo, and a projection that recorded the number would report a false
  divergence for it. What the projection carries instead is exactly the content check_ua_conformance's own
  diagnostics quote, scoped to the axes this subset reads and to no others.

  THE LAWS THE ORACLE ASSERTS IN-ROLE, so a scenario cannot pass merely because `lopdf` did not
  error. mutate-<id> fails unless the mutation actually MOVED the conformance-class projection.
  inverse-<id> applies the mutation, applies its own independently computed inverse, and fails with
  the first diverging field unless the result projects onto exactly what the pre-state projects onto.
  identity-round-trip fails unless the re-serialized bytes differ from the input AND their projection
  is identical to the input's.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    Then the oracle and the subject agree on the conformance-class projection
    Examples:
      | id                       | params                                                            |
      | set-mark-info            | {"marked": true}                                                  |
      | remove-mark-info         | {}                                                                |
      | set-struct-tree-root     | {}                                                                |
      | remove-struct-tree-root  | {}                                                                |
      | set-lang                 | {"lang": "en-GB"}                                                 |
      | remove-lang              | {}                                                                |
      | set-display-doc-title    | {"displayDocTitle": true}                                         |
      | remove-display-doc-title | {}                                                                |
      | set-info-title           | {"title": "Reuse of load-bearing timber components"}              |
      | embed-font-file          | {"descriptorOrdinal": 4, "key": "FontFile2", "programOrdinal": 0} |
      | remove-font-file         | {"descriptorOrdinal": 4}                                          |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the real document
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the <id> mutation is applied with its parameters
      """
      {"kind": "<id>", "params": <params>}
      """
    And the inverse mutation is applied to that result
    Then the conformance-class projection is the one the document started from
    Examples:
      | id                       | params                                                            |
      | set-mark-info            | {"marked": true}                                                  |
      | remove-mark-info         | {}                                                                |
      | set-struct-tree-root     | {}                                                                |
      | remove-struct-tree-root  | {}                                                                |
      | set-lang                 | {"lang": "en-GB"}                                                 |
      | remove-lang              | {}                                                                |
      | set-display-doc-title    | {"displayDocTitle": true}                                         |
      | remove-display-doc-title | {}                                                                |
      | set-info-title           | {"title": "Reuse of load-bearing timber components"}              |
      | embed-font-file          | {"descriptorOrdinal": 4, "key": "FontFile2", "programOrdinal": 0} |
      | remove-font-file         | {"descriptorOrdinal": 4}                                          |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the real document without passing bytes through
    Given the real input document asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf
    When the document is decoded to the typed snapshot and re-encoded from it alone
    Then the oracle and the subject agree on the conformance-class projection
    And the re-encoded bytes are not bit-identical to the input
